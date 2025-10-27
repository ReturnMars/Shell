// SSH连接管理模块
use crate::models::{ConnectionConfig, ConnectionStatus};
use crate::services::storage::ConnectionStorage;
use ssh2::{Session, Sftp};
use std::collections::HashMap;
use std::net::TcpStream;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

/// SSH连接信息
pub struct SshConnection {
    pub id: String,
    pub config: ConnectionConfig,
    pub session: Session,
    pub status: ConnectionStatus,
    pub sftp: Option<Sftp>,
    pub shell_channel: Option<ssh2::Channel>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// SSH连接管理器
pub struct SshConnectionManager {
    /// 活跃的SSH连接
    pub connections: Arc<RwLock<HashMap<String, SshConnection>>>,
    /// 连接存储管理器
    pub storage: Arc<ConnectionStorage>,
}

impl SshConnectionManager {
    /// 创建新的连接管理器
    pub fn new(storage: Arc<ConnectionStorage>) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            storage,
        }
    }

    /// 建立SSH连接
    pub async fn connect(
        &self,
        config: ConnectionConfig,
    ) -> Result<String, String> {
        log::info!("SSH连接 - 开始建立连接: {}", config.id);
        log::info!("SSH连接 - 连接配置: {}@{}:{}", config.username, config.host, config.port);

        // 验证配置
        config
            .validate()
            .map_err(|e| format!("配置验证失败: {}", e))?;

        let connection_id = config.id.clone();
        log::info!("SSH连接 - 使用连接ID: {}", connection_id);

        // 检查是否已存在活动连接
        {
            let connections = self.connections.read().await;
            if let Some(conn) = connections.get(&connection_id) {
                if matches!(conn.status, ConnectionStatus::Connected) {
                    log::warn!("SSH连接 - 连接 {} 已经处于连接状态", connection_id);
                    return Err("连接已存在".to_string());
                } else {
                    log::info!("SSH连接 - 连接 {} 存在但状态为 {:?}，将重新连接", connection_id, conn.status);
                    // 移除旧的断开状态的连接
                    drop(connections);
                    let mut connections = self.connections.write().await;
                    if let Some(mut old_conn) = connections.remove(&connection_id) {
                        if let Some(mut shell_channel) = old_conn.shell_channel.take() {
                            let _ = shell_channel.close();
                        }
                        let _ = old_conn.session.disconnect(None, "准备重新连接", None);
                        log::info!("SSH连接 - 已清除旧连接，准备建立新连接");
                    }
                }
            }
        }

        // 建立TCP连接
        let addr = format!("{}:{}", config.host, config.port);
        let tcp = TcpStream::connect(&addr).map_err(|e| format!("TCP连接失败: {}", e))?;

        // 创建SSH会话
        let mut session = Session::new().map_err(|e| format!("创建SSH会话失败: {}", e))?;

        session.set_tcp_stream(tcp);
        session
            .handshake()
            .map_err(|e| format!("SSH握手失败: {}", e))?;

        // 认证
        Self::authenticate(&mut session, &config).await?;

        // 创建SFTP会话
        let sftp = session.sftp().map_err(|e| format!("创建SFTP失败: {}", e))?;

        // 创建持久化shell通道
        let mut shell_channel = session
            .channel_session()
            .map_err(|e| format!("创建shell通道失败: {}", e))?;

        shell_channel
            .request_pty("xterm", None, None)
            .map_err(|e| format!("请求PTY失败: {}", e))?;

        shell_channel
            .shell()
            .map_err(|e| format!("启动shell失败: {}", e))?;

        // 创建连接对象
        let ssh_connection = SshConnection {
            id: connection_id.clone(),
            config: config.clone(),
            session,
            status: ConnectionStatus::Connected,
            sftp: Some(sftp),
            shell_channel: Some(shell_channel),
            created_at: chrono::Utc::now(),
        };

        // 存储连接
        {
            let mut connections = self.connections.write().await;
            connections.insert(connection_id.clone(), ssh_connection);
            log::info!("SSH连接 - 连接已存储，当前连接数量: {}", connections.len());
        }

        log::info!(
            "SSH连接 - 连接建立成功: {}@{}:{}, 连接ID: {}",
            config.username,
            config.host,
            config.port,
            connection_id
        );
        Ok(connection_id)
    }

    /// 检查连接健康状态
    pub async fn check_connection_health(&self, connection_id: &str) -> Result<bool, String> {
        log::info!("SSH连接 - 检查连接健康状态: {}", connection_id);

        let connections = self.connections.read().await;

        if let Some(connection) = connections.get(connection_id) {
            if connection.session.authenticated() {
                log::info!("SSH连接 - 连接 {} 会话仍然认证", connection_id);
                Ok(true)
            } else {
                log::warn!("SSH连接 - 连接 {} 会话未认证", connection_id);
                Ok(false)
            }
        } else {
            log::warn!("SSH连接 - 连接 {} 不存在", connection_id);
            Ok(false)
        }
    }

    /// 断开SSH连接
    pub async fn disconnect(&self, connection_id: &str) -> Result<(), String> {
        let mut connections = self.connections.write().await;

        if let Some(mut connection) = connections.remove(connection_id) {
            // 关闭shell通道
            if let Some(mut shell_channel) = connection.shell_channel.take() {
                if let Err(e) = shell_channel.close() {
                    log::warn!("关闭shell通道时出现警告: {}", e);
                }
            }

            // 主动关闭SSH会话
            if let Err(e) = connection
                .session
                .disconnect(None, "用户主动断开连接", None)
            {
                log::warn!("断开SSH会话时出现警告: {}", e);
            }

            connection.status = ConnectionStatus::Disconnected;
            log::info!("SSH连接已断开: {}", connection_id);
            Ok(())
        } else {
            Err("连接不存在".to_string())
        }
    }

    /// 断开所有连接
    pub async fn disconnect_all(&self) -> Result<(), String> {
        let mut connections = self.connections.write().await;
        let connection_ids: Vec<String> = connections.keys().cloned().collect();

        for connection_id in connection_ids {
            if let Some(mut connection) = connections.remove(&connection_id) {
                if let Err(e) = connection
                    .session
                    .disconnect(None, "用户主动断开所有连接", None)
                {
                    log::warn!("断开SSH会话时出现警告: {}", e);
                }
                connection.status = ConnectionStatus::Disconnected;
                log::info!("SSH连接已断开: {}", connection_id);
            }
        }

        Ok(())
    }

    /// 获取连接状态
    pub async fn get_connection_status(
        &self,
        connection_id: &str,
    ) -> Result<ConnectionStatus, String> {
        let connections = self.connections.read().await;

        if let Some(connection) = connections.get(connection_id) {
            Ok(connection.status.clone())
        } else {
            Err("连接不存在".to_string())
        }
    }

    /// 获取所有连接
    pub async fn get_connections(&self) -> Vec<ConnectionConfig> {
        let connections = self.connections.read().await;
        connections
            .values()
            .map(|conn| conn.config.clone())
            .collect()
    }

    /// 获取已连接数
    pub async fn get_connected_count(&self) -> usize {
        let connections = self.connections.read().await;
        connections
            .values()
            .filter(|conn| matches!(conn.status, ConnectionStatus::Connected))
            .count()
    }

    /// 获取已连接的连接列表
    pub async fn get_connected_connections(&self) -> Vec<ConnectionConfig> {
        let connections = self.connections.read().await;
        connections
            .values()
            .filter(|conn| matches!(conn.status, ConnectionStatus::Connected))
            .map(|conn| conn.config.clone())
            .collect()
    }

    /// 获取SSH连接（用于终端服务）
    pub async fn get_ssh_connection(
        &self,
        connection_id: &str,
    ) -> Result<SshConnection, String> {
        let connections = self.connections.read().await;

        if let Some(connection) = connections.get(connection_id) {
            Ok(SshConnection {
                id: connection.id.clone(),
                config: connection.config.clone(),
                session: connection.session.clone(),
                status: connection.status.clone(),
                sftp: None,
                shell_channel: None,
                created_at: connection.created_at,
            })
        } else {
            Err("SSH连接不存在".to_string())
        }
    }

    /// 认证处理
    async fn authenticate(session: &mut Session, config: &ConnectionConfig) -> Result<(), String> {
        match config.auth_method {
            crate::models::AuthMethod::Password => {
                if let Some(password) = &config.password {
                    session
                        .userauth_password(&config.username, password)
                        .map_err(|e| format!("密码认证失败: {}", e))?;
                } else {
                    return Err("密码认证需要提供密码".to_string());
                }
            }
            crate::models::AuthMethod::PrivateKey => {
                if let Some(key_path) = &config.private_key_path {
                    session
                        .userauth_pubkey_file(&config.username, None, Path::new(key_path), None)
                        .map_err(|e| format!("密钥认证失败: {}", e))?;
                } else {
                    return Err("密钥认证需要提供私钥路径".to_string());
                }
            }
            crate::models::AuthMethod::Both => {
                if let Some(password) = &config.password {
                    if session
                        .userauth_password(&config.username, password)
                        .is_ok()
                    {
                        return Ok(());
                    }
                }

                if let Some(key_path) = &config.private_key_path {
                    session
                        .userauth_pubkey_file(&config.username, None, Path::new(key_path), None)
                        .map_err(|e| format!("混合认证失败: {}", e))?;
                } else {
                    return Err("混合认证需要提供密码或私钥".to_string());
                }
            }
        }

        if !session.authenticated() {
            return Err("认证失败".to_string());
        }

        Ok(())
    }

    /// 获取连接的可变引用（用于其他模块操作）
    pub async fn get_connection_mut(
        &self,
        connection_id: &str,
    ) -> Option<tokio::sync::RwLockWriteGuard<'_, HashMap<String, SshConnection>>> {
        let connections = self.connections.write().await;
        if connections.contains_key(connection_id) {
            drop(connections);
            Some(self.connections.write().await)
        } else {
            None
        }
    }
}
