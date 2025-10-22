// SSH连接服务
use std::collections::HashMap;
use std::sync::Arc;
use std::path::Path;
use tokio::sync::RwLock;
use ssh2::{Session, Sftp};
use std::net::TcpStream;
use std::io::Read;
use crate::models::{ConnectionConfig, ConnectionStatus, Session as AppSession};

/// SSH连接管理器
pub struct SshService {
    /// 活跃的SSH连接
    connections: Arc<RwLock<HashMap<String, SshConnection>>>,
    /// 会话管理
    sessions: Arc<RwLock<HashMap<String, AppSession>>>,
}

/// SSH连接信息
pub struct SshConnection {
    pub id: String,
    pub config: ConnectionConfig,
    pub session: Session,
    pub status: ConnectionStatus,
    pub sftp: Option<Sftp>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl SshService {
    /// 创建新的SSH服务实例
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 建立SSH连接
    pub async fn connect(&self, config: ConnectionConfig) -> Result<String, String> {
        // 验证配置
        config.validate().map_err(|e| format!("配置验证失败: {}", e))?;

        let connection_id = config.id.clone();
        
        // 检查是否已存在连接
        {
            let connections = self.connections.read().await;
            if connections.contains_key(&connection_id) {
                return Err("连接已存在".to_string());
            }
        }

        // 建立TCP连接
        let addr = format!("{}:{}", config.host, config.port);
        let tcp = TcpStream::connect(&addr)
            .map_err(|e| format!("TCP连接失败: {}", e))?;

        // 创建SSH会话
        let mut session = Session::new()
            .map_err(|e| format!("创建SSH会话失败: {}", e))?;
        
        session.set_tcp_stream(tcp);
        session.handshake()
            .map_err(|e| format!("SSH握手失败: {}", e))?;

        // 认证
        self.authenticate(&mut session, &config).await?;

        // 创建SFTP会话
        let sftp = session.sftp()
            .map_err(|e| format!("创建SFTP失败: {}", e))?;

        // 创建连接对象
        let ssh_connection = SshConnection {
            id: connection_id.clone(),
            config: config.clone(),
            session,
            status: ConnectionStatus::Connected,
            sftp: Some(sftp),
            created_at: chrono::Utc::now(),
        };

        // 存储连接
        {
            let mut connections = self.connections.write().await;
            connections.insert(connection_id.clone(), ssh_connection);
        }

        log::info!("SSH连接建立成功: {}@{}:{}", config.username, config.host, config.port);
        Ok(connection_id)
    }

    /// 断开SSH连接
    pub async fn disconnect(&self, connection_id: &str) -> Result<(), String> {
        let mut connections = self.connections.write().await;
        
        if let Some(mut connection) = connections.remove(connection_id) {
            // 主动关闭SSH会话
            if let Err(e) = connection.session.disconnect(None, "用户主动断开连接", None) {
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
                // 主动关闭SSH会话
                if let Err(e) = connection.session.disconnect(None, "用户主动断开所有连接", None) {
                    log::warn!("断开SSH会话时出现警告: {}", e);
                }
                connection.status = ConnectionStatus::Disconnected;
                log::info!("SSH连接已断开: {}", connection_id);
            }
        }
        
        Ok(())
    }

    /// 获取连接状态
    pub async fn get_connection_status(&self, connection_id: &str) -> Result<ConnectionStatus, String> {
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
        connections.values().map(|conn| conn.config.clone()).collect()
    }

    /// 执行SSH命令
    pub async fn execute_command(&self, connection_id: &str, command: &str) -> Result<String, String> {
        let mut connections = self.connections.write().await;
        
        if let Some(connection) = connections.get_mut(connection_id) {
            if !matches!(connection.status, ConnectionStatus::Connected) {
                return Err("连接未建立".to_string());
            }

            // 创建命令通道
            let mut channel = connection.session.channel_session()
                .map_err(|e| format!("创建命令通道失败: {}", e))?;

            // 执行命令
            channel.exec(command)
                .map_err(|e| format!("执行命令失败: {}", e))?;

            // 读取输出
            let mut output = String::new();
            channel.read_to_string(&mut output)
                .map_err(|e| format!("读取命令输出失败: {}", e))?;

            // 等待命令完成
            channel.wait_eof()
                .map_err(|e| format!("等待命令完成失败: {}", e))?;
            channel.close()
                .map_err(|e| format!("关闭通道失败: {}", e))?;
            channel.wait_close()
                .map_err(|e| format!("等待通道关闭失败: {}", e))?;

            Ok(output)
        } else {
            Err("连接不存在".to_string())
        }
    }

    /// 认证处理
    async fn authenticate(&self, session: &mut Session, config: &ConnectionConfig) -> Result<(), String> {
        match config.auth_method {
            crate::models::AuthMethod::Password => {
                if let Some(password) = &config.password {
                    session.userauth_password(&config.username, password)
                        .map_err(|e| format!("密码认证失败: {}", e))?;
                } else {
                    return Err("密码认证需要提供密码".to_string());
                }
            }
            crate::models::AuthMethod::PrivateKey => {
                if let Some(key_path) = &config.private_key_path {
                    session.userauth_pubkey_file(&config.username, None, Path::new(key_path), None)
                        .map_err(|e| format!("密钥认证失败: {}", e))?;
                } else {
                    return Err("密钥认证需要提供私钥路径".to_string());
                }
            }
            crate::models::AuthMethod::Both => {
                // 尝试密码认证
                if let Some(password) = &config.password {
                    if session.userauth_password(&config.username, password).is_ok() {
                        return Ok(());
                    }
                }
                
                // 尝试密钥认证
                if let Some(key_path) = &config.private_key_path {
                    session.userauth_pubkey_file(&config.username, None, Path::new(key_path), None)
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
}

impl Default for SshService {
    fn default() -> Self {
        Self::new()
    }
}