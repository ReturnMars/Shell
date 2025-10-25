// SSH连接服务
use crate::models::{ConnectionConfig, ConnectionStatus, Session as AppSession, TabInfo, CommandOptions};
use crate::services::connection_storage::ConnectionStorage;
use ssh2::{Session, Sftp};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

/// SSH连接管理器
pub struct SshService {
    /// 活跃的SSH连接
    connections: Arc<RwLock<HashMap<String, SshConnection>>>,
    /// 会话管理
    sessions: Arc<RwLock<HashMap<String, AppSession>>>,
    /// 标签页管理
    tabs: Arc<RwLock<HashMap<String, TabInfo>>>,
    /// 连接存储管理器
    storage: Arc<ConnectionStorage>,
}

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

impl SshService {
    /// 创建新的SSH服务实例
    pub fn new() -> Result<Self, String> {
        let storage = ConnectionStorage::new()?;

        Ok(Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            tabs: Arc::new(RwLock::new(HashMap::new())),
            storage: Arc::new(storage),
        })
    }

    /// 建立SSH连接
    pub async fn connect(&self, config: ConnectionConfig) -> Result<String, String> {
        // 验证配置
        config
            .validate()
            .map_err(|e| format!("配置验证失败: {}", e))?;

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
        let tcp = TcpStream::connect(&addr).map_err(|e| format!("TCP连接失败: {}", e))?;

        // 创建SSH会话
        let mut session = Session::new().map_err(|e| format!("创建SSH会话失败: {}", e))?;

        session.set_tcp_stream(tcp);
        session
            .handshake()
            .map_err(|e| format!("SSH握手失败: {}", e))?;

        // 认证
        self.authenticate(&mut session, &config).await?;

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
        }

        log::info!(
            "SSH连接建立成功: {}@{}:{}",
            config.username,
            config.host,
            config.port
        );
        Ok(connection_id)
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
                // 主动关闭SSH会话
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
    pub async fn get_ssh_connection(&self, connection_id: &str) -> Result<SshConnection, String> {
        let connections = self.connections.read().await;

        if let Some(connection) = connections.get(connection_id) {
            // 克隆SSH连接信息
            Ok(SshConnection {
                id: connection.id.clone(),
                config: connection.config.clone(),
                session: connection.session.clone(),
                status: connection.status.clone(),
                sftp: None,          // Sftp不能clone，暂时设为None
                shell_channel: None, // Channel不能clone，暂时设为None
                created_at: connection.created_at,
            })
        } else {
            Err("SSH连接不存在".to_string())
        }
    }

    /// 执行SSH命令 - 优化版本
    pub async fn execute_command(
        &self,
        connection_id: &str,
        command: &str,
    ) -> Result<String, String> {
        self.execute_command_with_options(connection_id, command, CommandOptions::default()).await
    }

    /// 执行SSH命令 - 带选项的完整版本
    pub async fn execute_command_with_options(
        &self,
        connection_id: &str,
        command: &str,
        options: CommandOptions,
    ) -> Result<String, String> {
        if options.debug_output {
            println!("=== execute_command_with_options 开始 ===");
            println!("connection_id: {}", connection_id);
            println!("command: {:?}", command);
            println!("options: {:?}", options);
        }

        // 获取连接并执行命令
        let mut connections = self.connections.write().await;

        if let Some(connection) = connections.get_mut(connection_id) {
            if !matches!(connection.status, ConnectionStatus::Connected) {
                return Err("连接未建立".to_string());
            }

            // 使用持久化的shell通道
            if let Some(ref mut shell_channel) = connection.shell_channel {
                // 发送命令
                let command_to_send = if command.ends_with('\n') || command.ends_with('\r') {
                    command.to_string()
                } else {
                    format!("{}\n", command)
                };
                
                shell_channel
                    .write_all(command_to_send.as_bytes())
                    .map_err(|e| format!("发送命令失败: {}", e))?;
                shell_channel
                    .flush()
                    .map_err(|e| format!("刷新缓冲区失败: {}", e))?;

                // 智能读取输出
                self.read_command_output(shell_channel, &connection.config, &options).await
            } else {
                Err("Shell通道不存在".to_string())
            }
        } else {
            Err("连接不存在".to_string())
        }
    }

    /// 智能读取命令输出
    async fn read_command_output(
        &self,
        shell_channel: &mut ssh2::Channel,
        config: &ConnectionConfig,
        options: &CommandOptions,
    ) -> Result<String, String> {
        let mut output = String::new();
        let mut buffer = [0u8; 4096];
        
        // 获取提示符模式
        let prompt_patterns = options.custom_prompts.as_ref()
            .unwrap_or(&config.prompt_config.patterns);
        
        // 获取配置参数
        let max_wait_time = options.timeout.unwrap_or(config.prompt_config.max_wait_time);
        let max_empty_reads = config.prompt_config.max_empty_reads;
        
        let start_time = std::time::Instant::now();
        let mut empty_reads = 0;
        let mut last_data_time = std::time::Instant::now();
        
        loop {
            // 检查超时
            if start_time.elapsed().as_millis() as u64 > max_wait_time {
                if options.debug_output {
                    println!("命令执行超时，停止读取");
                }
                break;
            }
            
            // 检查是否等待提示符
            if !options.wait_for_prompt {
                // 不等待提示符，只读取一次
                match shell_channel.read(&mut buffer) {
                    Ok(0) => break,
                    Ok(n) => {
                        let chunk = String::from_utf8_lossy(&buffer[..n]);
                        output.push_str(&chunk);
                        if options.debug_output {
                            println!("读取到{}字节: {:?}", n, chunk);
                        }
                        break;
                    }
                    Err(e) => {
                        if options.debug_output {
                            println!("读取失败: {:?}", e);
                        }
                        break;
                    }
                }
            } else {
                // 等待提示符模式
                match shell_channel.read(&mut buffer) {
                    Ok(0) => {
                        empty_reads += 1;
                        if options.debug_output {
                            println!("读取到0字节，连续空读取: {}", empty_reads);
                        }
                        
                        // 如果连续空读取次数过多，可能命令已完成
                        if empty_reads >= max_empty_reads {
                            if options.debug_output {
                                println!("连续{}次空读取，可能命令已完成", max_empty_reads);
                            }
                            break;
                        }
                        
                        // 如果长时间没有数据，可能命令已完成
                        if last_data_time.elapsed().as_millis() > 1000 {
                            if options.debug_output {
                                println!("长时间无数据，可能命令已完成");
                            }
                            break;
                        }
                        
                        // 短暂等待更多数据
                        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                    }
                    Ok(n) => {
                        empty_reads = 0; // 重置空读取计数
                        last_data_time = std::time::Instant::now();
                        
                        let chunk = String::from_utf8_lossy(&buffer[..n]);
                        output.push_str(&chunk);
                        
                        if options.debug_output {
                            println!("读取到{}字节: {:?}", n, chunk);
                        }

                        // 检查是否包含提示符（命令完成）
                        if self.detect_prompt(&chunk, prompt_patterns, &config.prompt_config) {
                            if options.debug_output {
                                println!("检测到提示符，命令执行完成");
                            }
                            break;
                        }

                        // 短暂等待更多数据
                        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    }
                    Err(e) => {
                        if options.debug_output {
                            println!("读取失败: {:?}，停止读取", e);
                        }
                        break; // 读取错误，停止
                    }
                }
            }
        }

        Ok(output)
    }

    /// 检测提示符
    fn detect_prompt(
        &self,
        chunk: &str,
        patterns: &[String],
        config: &crate::models::PromptConfig,
    ) -> bool {
        if !config.smart_detection {
            // 简单模式：直接检查是否包含任何提示符
            return patterns.iter().any(|pattern| chunk.contains(pattern));
        }
        
        // 智能模式：检查提示符是否在行尾
        for pattern in patterns {
            if chunk.contains(pattern) {
                // 检查提示符是否在行尾（更准确的检测）
                let lines: Vec<&str> = chunk.split('\n').collect();
                if let Some(last_line) = lines.last() {
                    if last_line.trim_end().ends_with(pattern.trim_end()) {
                        return true;
                    }
                }
            }
        }
        
        false
    }

    /// 认证处理
    async fn authenticate(
        &self,
        session: &mut Session,
        config: &ConnectionConfig,
    ) -> Result<(), String> {
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
                // 尝试密码认证
                if let Some(password) = &config.password {
                    if session
                        .userauth_password(&config.username, password)
                        .is_ok()
                    {
                        return Ok(());
                    }
                }

                // 尝试密钥认证
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

    /// 获取标签页列表
    pub async fn get_tabs_list(&self) -> Result<Vec<TabInfo>, String> {
        // 从持久化存储加载标签页
        let tabs = self.storage.load_tabs()?;

        // 更新内存中的标签页
        {
            let mut memory_tabs = self.tabs.write().await;
            *memory_tabs = tabs.clone();
        }

        // 按创建时间倒序排序（最新的在前）
        let mut tab_list: Vec<TabInfo> = tabs.values().cloned().collect();
        tab_list.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(tab_list)
    }

    /// 添加标签页
    pub async fn add_tab(&self, connection_id: String, title: String) -> Result<String, String> {
        // 从持久化存储检查是否已存在相同链接的标签页
        let existing_tab = {
            let tabs = self.storage.load_tabs()?;
            tabs.values()
                .find(|t| t.connection_id == connection_id)
                .cloned()
        };

        if let Some(existing) = existing_tab {
            // 如果已存在，直接激活
            self.set_active_tab(&existing.id).await?;
            return Ok(existing.id);
        }

        // 创建新标签页
        let tab = TabInfo::new(connection_id, title);
        let tab_id = tab.id.clone();

        // 保存到持久化存储
        self.storage.add_tab(tab.clone())?;

        // 更新内存中的标签页
        {
            let mut tabs = self.tabs.write().await;
            tabs.insert(tab_id.clone(), tab);

            // 取消其他标签页的激活状态
            for tab in tabs.values_mut() {
                tab.active = false;
            }

            // 激活新标签页
            if let Some(tab) = tabs.get_mut(&tab_id) {
                tab.active = true;
            }
        }

        log::info!("添加标签页成功: {}", tab_id);
        Ok(tab_id)
    }

    /// 删除标签页
    pub async fn remove_tab(&self, tab_id: &str) -> Result<(), String> {
        // 从持久化存储删除
        self.storage.remove_tab(tab_id)?;

        // 从内存中删除
        let mut tabs = self.tabs.write().await;

        if let Some(removed_tab) = tabs.remove(tab_id) {
            // 如果删除的是活动标签页，激活下一个标签页
            if removed_tab.active && !tabs.is_empty() {
                if let Some(next_tab) = tabs.values().next() {
                    let next_tab_id = next_tab.id.clone();
                    drop(tabs); // 释放锁
                    self.storage.set_active_tab(&next_tab_id)?;
                }
            }
            log::info!("删除标签页成功: {}", tab_id);
            Ok(())
        } else {
            Err("标签页不存在".to_string())
        }
    }

    /// 设置活动标签页
    pub async fn set_active_tab(&self, tab_id: &str) -> Result<(), String> {
        // 更新持久化存储中的活动状态
        self.storage.set_active_tab(tab_id)?;

        // 更新内存中的活动状态
        let mut tabs = self.tabs.write().await;

        // 先取消所有标签页的激活状态
        for tab in tabs.values_mut() {
            tab.active = false;
        }

        // 激活指定标签页
        if let Some(tab) = tabs.get_mut(tab_id) {
            tab.active = true;
            tab.update();
            log::info!("设置活动标签页: {}", tab_id);
            Ok(())
        } else {
            Err("标签页不存在".to_string())
        }
    }

    /// 获取活动标签页
    pub async fn get_active_tab(&self) -> Option<TabInfo> {
        let tabs = self.tabs.read().await;
        tabs.values().find(|t| t.active).cloned()
    }

    /// 关闭所有标签页
    pub async fn close_all_tabs(&self) -> Result<(), String> {
        // 清空持久化存储
        self.storage.clear_all_tabs()?;

        // 清空内存
        let mut tabs = self.tabs.write().await;
        tabs.clear();
        log::info!("关闭所有标签页");
        Ok(())
    }

    /// 关闭其他标签页
    pub async fn close_other_tabs(&self, keep_tab_id: &str) -> Result<(), String> {
        let mut tabs = self.tabs.write().await;

        if let Some(keep_tab) = tabs.get(keep_tab_id).cloned() {
            // 清空持久化存储
            self.storage.clear_all_tabs()?;

            // 重新添加要保留的标签页
            self.storage.add_tab(keep_tab.clone())?;

            // 更新内存
            tabs.clear();
            tabs.insert(keep_tab_id.to_string(), keep_tab);
            log::info!("关闭其他标签页，保留: {}", keep_tab_id);
            Ok(())
        } else {
            Err("要保留的标签页不存在".to_string())
        }
    }

    /// 根据链接ID获取标签页
    pub async fn get_tab_by_connection_id(&self, connection_id: &str) -> Option<TabInfo> {
        let tabs = self.tabs.read().await;
        tabs.values()
            .find(|t| t.connection_id == connection_id)
            .cloned()
    }
}

impl Default for SshService {
    fn default() -> Self {
        Self::new().expect("无法创建SSH服务")
    }
}
