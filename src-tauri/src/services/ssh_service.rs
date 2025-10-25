// SSH连接服务
use crate::models::{ConnectionConfig, ConnectionStatus, Session as AppSession, TabInfo, CommandOptions, HardwareInfo, CpuInfo, MemoryInfo, StorageInfo, NetworkInfo};
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
        log::info!("SSH服务 - 开始建立连接: {}", config.id);
        log::info!("SSH服务 - 连接配置: {}@{}:{}", config.username, config.host, config.port);
        
        // 验证配置
        config
            .validate()
            .map_err(|e| format!("配置验证失败: {}", e))?;

        let connection_id = config.id.clone();
        log::info!("SSH服务 - 使用连接ID: {}", connection_id);

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
            log::info!("SSH服务 - 连接已存储到HashMap中，当前连接数量: {}", connections.len());
        }

        log::info!(
            "SSH服务 - 连接建立成功: {}@{}:{}, 连接ID: {}",
            config.username,
            config.host,
            config.port,
            connection_id
        );
        Ok(connection_id)
    }

    /// 检查连接健康状态
    pub async fn check_connection_health(&self, connection_id: &str) -> Result<bool, String> {
        log::info!("SSH服务 - 检查连接健康状态: {}", connection_id);
        
        let connections = self.connections.read().await;
        
        if let Some(connection) = connections.get(connection_id) {
            // 检查会话是否仍然认证
            if connection.session.authenticated() {
                log::info!("SSH服务 - 连接 {} 会话仍然认证", connection_id);
                return Ok(true);
            } else {
                log::warn!("SSH服务 - 连接 {} 会话未认证", connection_id);
                return Ok(false);
            }
        } else {
            log::warn!("SSH服务 - 连接 {} 不存在", connection_id);
            return Ok(false);
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
                        // 如果是 "Failure while draining incoming flow" 错误，尝试继续
                        if e.to_string().contains("Failure while draining incoming flow") {
                            if options.debug_output {
                                println!("遇到draining flow错误，尝试继续读取");
                            }
                            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                            continue;
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
                            println!("读取失败: {:?}", e);
                        }
                        // 如果是 "Failure while draining incoming flow" 错误，尝试继续
                        if e.to_string().contains("Failure while draining incoming flow") {
                            if options.debug_output {
                                println!("遇到draining flow错误，尝试继续读取");
                            }
                            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                            continue;
                        }
                        break; // 其他读取错误，停止
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

    // ==================== 硬件信息获取方法 ====================

    /// 获取CPU信息 - 使用改进的方法
    pub async fn get_cpu_info(&self, connection_id: &str) -> Result<CpuInfo, String> {
        let mut cpu_info = CpuInfo::default();

        // 分别执行每个命令获取CPU信息
        // 1. 获取CPU型号
        let model_cmd = "lscpu | grep 'Model name' || lscpu | grep 'Vendor ID' || echo 'Unknown CPU'";
        let model_output = self.execute_command(connection_id, model_cmd).await?;
        cpu_info.model = self.extract_cpu_model(&model_output);
        
        // 2. 获取核心数
        let cores_cmd = "nproc";
        let cores_output = self.execute_command(connection_id, cores_cmd).await?;
        cpu_info.cores = self.extract_cpu_cores(&cores_output);
        
        // 3. 获取CPU使用率
        let usage_cmd = "top -bn1 | grep 'Cpu(s)'";
        let usage_output = self.execute_command(connection_id, usage_cmd).await?;
        cpu_info.usage = self.extract_cpu_usage(&usage_output);
        
        // 4. 获取CPU频率
        let freq_cmd = "lscpu | grep 'CPU MHz'";
        let freq_output = self.execute_command(connection_id, freq_cmd).await?;
        cpu_info.frequency = self.extract_cpu_frequency(&freq_output);
        
        // 5. 获取CPU温度
        let temp_cmd = "cat /sys/class/thermal/thermal_zone*/temp 2>/dev/null | head -1";
        let temp_output = self.execute_command(connection_id, temp_cmd).await?;
        cpu_info.temperature = self.extract_cpu_temperature(&temp_output);

        Ok(cpu_info)
    }

    /// 清理命令输出，移除ANSI颜色代码和控制字符
    fn clean_command_output(&self, output: &str) -> String {
        // 移除ANSI颜色代码和控制字符
        let cleaned = output
            .replace("\u{1b}[01;31m", "")
            .replace("\u{1b}[m", "")
            .replace("\u{1b}[K", "")
            .replace("\u{1b}]0;root@iZ2vcc2rr8aqk0prfnjavaZ:~\u{7}", "")
            .replace("\r\n", "\n")
            .replace("\r", "\n");
        
        // 移除命令本身和提示符
        let lines: Vec<&str> = cleaned.lines().collect();
        let mut result = String::new();
        
        for line in lines {
            let trimmed = line.trim();
            // 跳过空行、命令本身、提示符
            if !trimmed.is_empty() 
                && !trimmed.starts_with("lscpu")
                && !trimmed.starts_with("nproc")
                && !trimmed.starts_with("top")
                && !trimmed.starts_with("cat")
                && !trimmed.starts_with("[root@")
                && !trimmed.ends_with("]#")
                && !trimmed.ends_with("$") {
                result.push_str(trimmed);
                result.push('\n');
            }
        }
        
        result.trim().to_string()
    }

    /// 提取CPU型号
    fn extract_cpu_model(&self, output: &str) -> String {
        let cleaned = self.clean_command_output(output);
        cleaned
            .lines()
            .find(|l| l.contains("Model name:") && !l.contains("BIOS"))
            .map(|l| l.replace("Model name:", "").trim().to_string())
            .or_else(|| {
                cleaned.lines()
                    .find(|l| l.contains("Vendor ID:"))
                    .map(|l| l.replace("Vendor ID:", "").trim().to_string())
            })
            .unwrap_or_else(|| "Unknown CPU".to_string())
    }
    
    /// 提取CPU核心数
    fn extract_cpu_cores(&self, output: &str) -> u32 {
        let cleaned = self.clean_command_output(output);
        cleaned
            .lines()
            .find(|l| l.trim().parse::<u32>().is_ok())
            .and_then(|l| l.trim().parse::<u32>().ok())
            .unwrap_or(1)
    }
    
    /// 提取CPU使用率 - 使用 100 - idle 计算总使用率
    fn extract_cpu_usage(&self, output: &str) -> f64 {
        let cleaned = self.clean_command_output(output);
        
        cleaned
            .lines()
            .find(|l| l.contains("Cpu(s)"))
            .and_then(|line| {
                // 查找包含 "id" (idle) 的部分
                let parts: Vec<&str> = line.split(',').collect();
                for part in parts {
                    if part.contains("id") {
                        // 提取idle数字部分
                        let idle_number = part.split_whitespace()
                            .find(|s| s.parse::<f64>().is_ok())
                            .unwrap_or("0");
                        
                        if let Ok(idle) = idle_number.parse::<f64>() {
                            let usage = 100.0 - idle;
                            return Some(usage);
                        }
                    }
                }
                None
            })
            .unwrap_or(0.0)
    }
    
    /// 提取CPU频率
    fn extract_cpu_frequency(&self, output: &str) -> Option<f64> {
        let cleaned = self.clean_command_output(output);
        cleaned
            .lines()
            .find(|l| l.contains("CPU MHz"))
            .and_then(|line| {
                let freq_value = self.extract_number_from_output(line);
                freq_value.parse::<f64>().ok()
            })
    }
    
    /// 提取CPU温度
    fn extract_cpu_temperature(&self, output: &str) -> Option<f64> {
        let cleaned = self.clean_command_output(output);
        cleaned
            .lines()
            .find(|l| l.trim().parse::<f64>().is_ok() && l.trim().len() > 3)
            .and_then(|line| {
                let temp_value = self.extract_number_from_output(line);
                temp_value.parse::<f64>().ok().map(|temp| temp / 1000.0)
            })
    }

    /// 解析CPU信息输出 - 使用改进的方法（保留作为备用）
    fn parse_cpu_info_from_output(&self, output: &str, cpu_info: &mut CpuInfo) -> Result<(), String> {
        log::info!("SSH服务 - 开始解析CPU信息输出");
        
        // 提取 __BEGIN__ 和 __END__ 之间的内容
        let block = output
            .split("__BEGIN__").nth(1).unwrap_or("")
            .split("__END__").next().unwrap_or("")
            .to_string();

        log::info!("SSH服务 - 提取的CPU信息块长度: {} 字符", block.len());
        log::info!("SSH服务 - 提取的CPU信息块内容:\n{}", block);
        
        // 如果标记解析失败，尝试直接解析整个输出
        let content = if block.len() < 10 {
            log::info!("SSH服务 - 标记解析失败，使用整个输出");
            output
        } else {
            &block
        };

        // 解析CPU型号 - 优先使用真正的CPU型号
        let model = content
            .lines()
            .find(|l| l.contains("Model name:") && !l.contains("BIOS"))
            .map(|l| l.replace("Model name:", "").trim().to_string())
            .or_else(|| {
                // 如果没有真正的Model name，尝试获取Vendor ID
                content.lines()
                    .find(|l| l.contains("Vendor ID:"))
                    .map(|l| l.replace("Vendor ID:", "").trim().to_string())
            })
            .unwrap_or_else(|| "Unknown CPU".to_string());
        
        cpu_info.model = model;
        log::info!("SSH服务 - CPU型号: {}", cpu_info.model);

        // 解析核心数
        let cores = content
            .lines()
            .find(|l| l.trim().parse::<u32>().is_ok())
            .and_then(|l| l.trim().parse::<u32>().ok())
            .unwrap_or(1);
        
        cpu_info.cores = cores;
        log::info!("SSH服务 - CPU核心数: {}", cpu_info.cores);

        // 解析CPU使用率
        let cpu_line = content
            .lines()
            .find(|l| l.contains("Cpu(s)"))
            .unwrap_or("")
            .to_string();
        
        log::info!("SSH服务 - CPU使用率原始行: '{}'", cpu_line);
        
        // 解析CPU使用率 - 直接使用us（用户空间使用率）
        if !cpu_line.is_empty() {
            let us: f64 = cpu_line
                .split(',')
                .find(|s| s.contains("us"))
                .and_then(|s| s.split('%').next())
                .and_then(|v| v.trim().parse::<f64>().ok())
                .unwrap_or(0.0);
            
            cpu_info.usage = us;
            log::info!("SSH服务 - CPU使用率(us): {}%", cpu_info.usage);
        } else {
            cpu_info.usage = 0.0;
            log::info!("SSH服务 - 未找到CPU使用率信息");
        }

        // 解析CPU频率
        let freq_line = content
            .lines()
            .find(|l| l.contains("CPU MHz"))
            .unwrap_or("")
            .to_string();
        
        if !freq_line.is_empty() {
            let freq_value = self.extract_number_from_output(&freq_line);
            if let Some(freq) = freq_value.parse::<f64>().ok() {
                cpu_info.frequency = Some(freq);
                log::info!("SSH服务 - CPU频率: {} MHz", freq);
            }
        }

        // 解析CPU温度
        let temp_line = content
            .lines()
            .find(|l| l.trim().parse::<f64>().is_ok() && l.trim().len() > 3)
            .unwrap_or("")
            .to_string();
        
        if !temp_line.is_empty() {
            let temp_value = self.extract_number_from_output(&temp_line);
            if let Some(temp_millicelsius) = temp_value.parse::<f64>().ok() {
                cpu_info.temperature = Some(temp_millicelsius / 1000.0); // 转换为摄氏度
                log::info!("SSH服务 - CPU温度: {}°C", temp_millicelsius / 1000.0);
            }
        }

        Ok(())
    }

    /// 解析 /proc/cpuinfo 输出（保留作为备用）
    fn parse_cpuinfo(&self, output: &str, cpu_info: &mut CpuInfo) -> Result<(), String> {
        log::info!("SSH服务 - 开始解析CPU信息，输出行数: {}", output.lines().count());
        let mut processor_count = 0;
        let mut model_name = String::new();
        let mut cpu_cores = 0;
        let mut physical_id_count = 0;
        let mut seen_physical_ids = std::collections::HashSet::new();

        for line in output.lines() {
            if line.starts_with("processor") {
                processor_count += 1;
            } else if line.starts_with("model name") {
                if model_name.is_empty() {
                    model_name = line.split(':').nth(1)
                        .map(|s| s.trim().to_string())
                        .unwrap_or_else(|| "Unknown CPU".to_string());
                }
            } else if line.starts_with("cpu cores") {
                if let Some(cores_str) = line.split(':').nth(1) {
                    if let Ok(cores) = cores_str.trim().parse::<u32>() {
                        cpu_cores = cores;
                    }
                }
            } else if line.starts_with("physical id") {
                if let Some(id_str) = line.split(':').nth(1) {
                    if let Ok(id) = id_str.trim().parse::<u32>() {
                        if seen_physical_ids.insert(id) {
                            physical_id_count += 1;
                        }
                    }
                }
            }
        }

        // 改进CPU型号解析，获取完整型号
        if !model_name.is_empty() {
            cpu_info.model = model_name;
        } else {
            cpu_info.model = "Unknown CPU".to_string();
        }

        // 改进核心数计算 - 优先使用处理器数量
        if processor_count > 0 {
            // 处理器数量通常更准确，特别是对于虚拟化环境
            cpu_info.cores = processor_count as u32;
            log::info!("SSH服务 - 使用处理器数量作为核心数: {}", processor_count);
        } else if cpu_cores > 0 && physical_id_count > 0 {
            // 如果有物理CPU数量和每CPU核心数，计算总核心数
            cpu_info.cores = cpu_cores * physical_id_count;
            log::info!("SSH服务 - 使用物理CPU计算核心数: {} * {} = {}", cpu_cores, physical_id_count, cpu_info.cores);
        } else if cpu_cores > 0 {
            // 如果只有每CPU核心数，使用它
            cpu_info.cores = cpu_cores;
            log::info!("SSH服务 - 使用每CPU核心数: {}", cpu_cores);
        } else {
            // 最后备用方案
            cpu_info.cores = 1;
            log::info!("SSH服务 - 使用默认核心数: 1");
        }

        log::info!("SSH服务 - CPU解析结果: processor_count={}, cpu_cores={}, physical_id_count={}, 最终model='{}', 最终cores={}", 
                   processor_count, cpu_cores, physical_id_count, cpu_info.model, cpu_info.cores);

        Ok(())
    }

    /// 获取内存信息
    pub async fn get_memory_info(&self, connection_id: &str) -> Result<MemoryInfo, String> {
        log::info!("SSH服务 - 开始获取内存信息: {}", connection_id);
        let mut memory_info = MemoryInfo::default();

        // 获取内存信息
        let meminfo_output = self.execute_command(connection_id, "cat /proc/meminfo").await?;
        log::info!("SSH服务 - 内存信息原始输出长度: {} 字符", meminfo_output.len());
        log::info!("SSH服务 - 内存信息原始输出内容:\n{}", meminfo_output);
        
        self.parse_meminfo(&meminfo_output, &mut memory_info)?;

        // 尝试获取交换分区信息
        match self.get_swap_info(connection_id).await {
            Ok(swap_info) => {
                memory_info.swap = Some(swap_info);
                log::info!("SSH服务 - 交换分区获取成功");
            },
            Err(e) => {
                log::error!("SSH服务 - 交换分区获取失败: {}", e);
                memory_info.swap = None;
            }
        }

        log::info!("SSH服务 - 内存信息解析完成: total={}MB, used={}MB, free={}MB, usage={:.2}%, swap={:?}", 
                   memory_info.total, memory_info.used, memory_info.free, memory_info.usage, memory_info.swap);

        Ok(memory_info)
    }

    /// 解析 /proc/meminfo 输出
    fn parse_meminfo(&self, output: &str, memory_info: &mut MemoryInfo) -> Result<(), String> {
        log::info!("SSH服务 - 开始解析内存信息");
        let mut mem_total = 0u64;
        let mut mem_available = 0u64;
        let mut mem_free = 0u64;
        let mut buffers = 0u64;
        let mut cached = 0u64;

        for line in output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(value) = parts[1].parse::<u64>() {
                    match parts[0] {
                        "MemTotal:" => {
                            mem_total = value;
                            log::info!("SSH服务 - 总内存: {} KB", value);
                        },
                        "MemAvailable:" => {
                            mem_available = value;
                            log::info!("SSH服务 - 可用内存: {} KB", value);
                        },
                        "MemFree:" => {
                            mem_free = value;
                            log::info!("SSH服务 - 空闲内存: {} KB", value);
                        },
                        "Buffers:" => {
                            buffers = value;
                            log::info!("SSH服务 - 缓冲区: {} KB", value);
                        },
                        "Cached:" => {
                            cached = value;
                            log::info!("SSH服务 - 缓存: {} KB", value);
                        },
                        _ => {}
                    }
                }
            }
        }

        // 计算内存使用情况
        memory_info.total = mem_total / 1024; // 转换为MB
        memory_info.free = if mem_available > 0 { mem_available / 1024 } else { mem_free / 1024 };
        memory_info.used = memory_info.total - memory_info.free;
        
        log::info!("SSH服务 - 内存计算: total={}MB, free={}MB, used={}MB", 
                   memory_info.total, memory_info.free, memory_info.used);
        
        if memory_info.total > 0 {
            memory_info.usage = (memory_info.used as f64 / memory_info.total as f64) * 100.0;
            log::info!("SSH服务 - 内存使用率: {:.2}%", memory_info.usage);
        }

        Ok(())
    }

    /// 获取交换分区信息
    async fn get_swap_info(&self, connection_id: &str) -> Result<crate::models::SwapInfo, String> {
        log::info!("SSH服务 - 开始获取交换分区信息: {}", connection_id);
        let mut swap_info = crate::models::SwapInfo::default();
        
        let meminfo_output = self.execute_command(connection_id, "cat /proc/meminfo | grep -i swap").await?;
        log::info!("SSH服务 - 交换分区原始输出: {:?}", meminfo_output);
        
        // 清理ANSI颜色代码
        let cleaned_output = self.clean_command_output(&meminfo_output);
        log::info!("SSH服务 - 清理后的交换分区输出: {:?}", cleaned_output);
        
        let mut swap_total = 0u64;
        let mut swap_free = 0u64;

        for line in cleaned_output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(value) = parts[1].parse::<u64>() {
                    match parts[0] {
                        "SwapTotal:" => {
                            swap_total = value;
                            log::info!("SSH服务 - 交换分区总量: {} KB", value);
                        },
                        "SwapFree:" => {
                            swap_free = value;
                            log::info!("SSH服务 - 交换分区空闲: {} KB", value);
                        },
                        _ => {}
                    }
                }
            }
        }

        if swap_total > 0 {
            swap_info.total = swap_total / 1024; // 转换为MB
            swap_info.free = swap_free / 1024;
            swap_info.used = swap_info.total - swap_info.free;
            swap_info.usage = if swap_info.total > 0 {
                (swap_info.used as f64 / swap_info.total as f64) * 100.0
            } else {
                0.0
            };
            log::info!("SSH服务 - 交换分区解析完成: total={}MB, used={}MB, free={}MB, usage={:.2}%", 
                       swap_info.total, swap_info.used, swap_info.free, swap_info.usage);
        } else {
            log::info!("SSH服务 - 没有交换分区或交换分区为0");
        }

        Ok(swap_info)
    }

    /// 获取存储信息
    pub async fn get_storage_info(&self, connection_id: &str) -> Result<Vec<StorageInfo>, String> {
        let mut storage_list = Vec::new();

        // 获取磁盘使用情况
        let df_output = self.execute_command(connection_id, "df -h").await?;
        self.parse_df_output(&df_output, &mut storage_list)?;

        // 尝试获取磁盘类型信息（SSD/HDD）
        self.enrich_storage_type_info(connection_id, &mut storage_list).await;

        Ok(storage_list)
    }

    /// 解析 df -h 输出
    fn parse_df_output(&self, output: &str, storage_list: &mut Vec<StorageInfo>) -> Result<(), String> {
        for line in output.lines().skip(1) { // 跳过标题行
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                let device = parts[0].to_string();
                let mount_point = parts[5].to_string();
                
                // 跳过特殊文件系统
                if mount_point.starts_with("/proc") || 
                   mount_point.starts_with("/sys") || 
                   mount_point.starts_with("/dev") ||
                   mount_point.starts_with("/run") ||
                   mount_point.starts_with("/snap") ||
                   mount_point.starts_with("/var/lib/docker") ||
                   device.starts_with("tmpfs") ||
                   device.starts_with("devtmpfs") ||
                   device.starts_with("overlay") ||
                   device.starts_with("squashfs") {
                    continue;
                }

                let mut storage_info = StorageInfo::default();
                storage_info.device = device;
                storage_info.mount_point = mount_point;
                storage_info.filesystem = parts[1].to_string();

                // 解析容量信息
                if let Ok(total_str) = parts[2].parse::<String>() {
                    storage_info.total = self.parse_size_to_mb(&total_str);
                }
                if let Ok(used_str) = parts[3].parse::<String>() {
                    storage_info.used = self.parse_size_to_mb(&used_str);
                }
                if let Ok(available_str) = parts[4].parse::<String>() {
                    storage_info.free = self.parse_size_to_mb(&available_str);
                }

                // 计算使用率
                if storage_info.total > 0 {
                    storage_info.usage = (storage_info.used as f64 / storage_info.total as f64) * 100.0;
                }

                storage_list.push(storage_info);
            }
        }

        Ok(())
    }

    /// 将大小字符串转换为MB
    fn parse_size_to_mb(&self, size_str: &str) -> u64 {
        let size_str = size_str.to_uppercase();
        if let Some(num_part) = size_str.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect::<String>().parse::<f64>().ok() {
            if size_str.contains('G') {
                (num_part * 1024.0) as u64
            } else if size_str.contains('T') {
                (num_part * 1024.0 * 1024.0) as u64
            } else if size_str.contains('K') {
                (num_part / 1024.0) as u64
            } else {
                num_part as u64
            }
        } else {
            0
        }
    }

    /// 丰富存储类型信息
    async fn enrich_storage_type_info(&self, connection_id: &str, storage_list: &mut Vec<StorageInfo>) {
        // 尝试使用 lsblk 获取磁盘类型信息
        let lsblk_output = self.execute_command(connection_id, "lsblk -d -o NAME,TYPE,ROTA").await.ok();
        
        if let Some(output) = lsblk_output {
            for storage in storage_list.iter_mut() {
                // 从设备名提取磁盘名（如 /dev/sda1 -> sda）
                if let Some(disk_name) = storage.device.strip_prefix("/dev/") {
                    let disk_name = disk_name.chars().take_while(|c| c.is_alphabetic()).collect::<String>();
                    
                    for line in output.lines() {
                        if line.contains(&disk_name) {
                            // ROTA=1 表示机械硬盘，ROTA=0 表示SSD
                            if line.contains("ROTA=0") {
                                storage.r#type = "ssd".to_string();
                            } else if line.contains("ROTA=1") {
                                storage.r#type = "hdd".to_string();
                            }
                            break;
                        }
                    }
                }
            }
        }
    }

    /// 获取网络信息
    pub async fn get_network_info(&self, connection_id: &str) -> Result<NetworkInfo, String> {
        let mut network_info = NetworkInfo::default();

        // 获取网络接口统计信息
        let netdev_output = self.execute_command(connection_id, "cat /proc/net/dev").await?;
        self.parse_netdev_output(&netdev_output, &mut network_info)?;

        // 获取网络接口状态
        let ip_output = self.execute_command(connection_id, "ip addr show").await.ok();
        if let Some(output) = ip_output {
            self.enrich_network_status(&output, &mut network_info);
        }

        // 计算总流量和速度
        self.calculate_network_totals(&mut network_info);

        Ok(network_info)
    }

    /// 解析 /proc/net/dev 输出
    fn parse_netdev_output(&self, output: &str, network_info: &mut NetworkInfo) -> Result<(), String> {
        for line in output.lines().skip(2) { // 跳过标题行
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let interface_name = parts[0].trim().to_string();
                
                // 跳过回环接口
                if interface_name == "lo" {
                    continue;
                }

                let stats: Vec<&str> = parts[1].split_whitespace().collect();
                if stats.len() >= 16 {
                    let mut interface = crate::models::NetworkInterface::default();
                    interface.name = interface_name;

                    // 解析统计信息
                    if let Ok(rx_bytes) = stats[0].parse::<u64>() {
                        interface.rx = rx_bytes;
                    }
                    if let Ok(tx_bytes) = stats[8].parse::<u64>() {
                        interface.tx = tx_bytes;
                    }

                    network_info.interfaces.push(interface);
                }
            }
        }

        Ok(())
    }

    /// 丰富网络接口状态信息
    fn enrich_network_status(&self, output: &str, network_info: &mut NetworkInfo) {
        for interface in network_info.interfaces.iter_mut() {
            // 检查接口是否在线
            if output.contains(&format!("{}:", interface.name)) {
                // 简单检查：如果接口在输出中且包含 "UP"，则认为是在线状态
                if output.contains("UP") {
                    interface.status = "up".to_string();
                } else {
                    interface.status = "down".to_string();
                }
            }
        }
    }

    /// 计算网络总流量和速度
    fn calculate_network_totals(&self, network_info: &mut NetworkInfo) {
        let mut total_rx = 0u64;
        let mut total_tx = 0u64;

        for interface in &network_info.interfaces {
            total_rx += interface.rx;
            total_tx += interface.tx;
        }

        network_info.total_rx = total_rx;
        network_info.total_tx = total_tx;

        // 计算速度（这里简化处理，实际应该基于时间间隔计算）
        // 在实际应用中，应该保存上次的统计值来计算速度
        network_info.rx_speed = 0.0; // 暂时设为0，需要实现速度计算逻辑
        network_info.tx_speed = 0.0;
    }

    /// 获取完整的硬件信息
    pub async fn get_hardware_info(&self, connection_id: &str) -> Result<HardwareInfo, String> {
        log::info!("开始获取硬件信息: {}", connection_id);

        // 检查连接是否存在（只获取一次读锁）
        let connections = self.connections.read().await;
        log::info!("SSH服务中的连接数量: {}", connections.len());
        
        // 调试：打印所有连接信息
        for (id, conn) in connections.iter() {
            log::info!("SSH连接 - ID: {}, 状态: {:?}", id, conn.status);
        }
        
        // 检查指定连接是否存在
        if !connections.contains_key(connection_id) {
            log::error!("连接不存在: {}, 可用连接: {:?}", connection_id, connections.keys().collect::<Vec<_>>());
            return Err(format!("连接不存在: {}", connection_id));
        }
        
        // 检查连接状态
        if let Some(conn) = connections.get(connection_id) {
            if !matches!(conn.status, ConnectionStatus::Connected) {
                log::error!("连接状态异常: {} 状态为 {:?}", connection_id, conn.status);
                return Err(format!("连接状态异常: {:?}", conn.status));
            }
        }
        
        drop(connections); // 显式释放锁

        // 串行获取各种硬件信息，避免SSH通道并发访问问题
        let cpu = self.get_cpu_info(connection_id).await?;
        let memory = self.get_memory_info(connection_id).await?;
        let storage = self.get_storage_info(connection_id).await?;
        let network = self.get_network_info(connection_id).await?;

        let hardware_info = HardwareInfo {
            cpu,
            memory,
            storage,
            network,
            timestamp: chrono::Utc::now(),
        };

        log::info!("硬件信息获取成功: {}", connection_id);
        Ok(hardware_info)
    }

    /// 从命令输出中提取数字部分
    /// 去除命令本身、提示符等干扰内容
    fn extract_number_from_output(&self, output: &str) -> String {
        // 按行分割，寻找包含数字的行
        for line in output.lines() {
            let trimmed = line.trim();
            // 跳过空行和包含命令的行
            if trimmed.is_empty() || trimmed.contains("grep") || trimmed.contains("awk") || 
               trimmed.contains("sed") || trimmed.contains("top") || trimmed.contains("vmstat") ||
               trimmed.contains("cat") || trimmed.contains("lscpu") || trimmed.contains("sensors") ||
               trimmed.contains("[root@") || trimmed.contains("]#") {
                continue;
            }
            
            // 提取数字部分（包括小数点和负号）
            let mut number = String::new();
            let mut found_digit = false;
            for ch in trimmed.chars() {
                if ch.is_ascii_digit() || ch == '.' || ch == '-' {
                    number.push(ch);
                    found_digit = true;
                } else if found_digit {
                    // 找到数字后遇到非数字字符就停止
                    break;
                }
            }
            
            if !number.is_empty() && number.parse::<f64>().is_ok() {
                return number;
            }
        }
        
        // 如果没有找到有效的数字，返回空字符串
        String::new()
    }

    /// 从 /proc/stat 计算CPU使用率
    /// 注意：单次采样无法计算准确的使用率，这里返回0表示需要使用其他方法
    fn calculate_cpu_usage_from_stat(&self, stat_output: &str) -> f64 {
        // 解析 /proc/stat 的第一行
        // 格式：cpu user nice system idle iowait irq softirq steal guest guest_nice
        let parts: Vec<&str> = stat_output.split_whitespace().collect();
        if parts.len() < 8 {
            log::warn!("SSH服务 - /proc/stat 格式不正确，字段数量: {}", parts.len());
            return 0.0;
        }

        // 解析各个时间值
        let user: u64 = parts[1].parse().unwrap_or(0);
        let nice: u64 = parts[2].parse().unwrap_or(0);
        let system: u64 = parts[3].parse().unwrap_or(0);
        let idle: u64 = parts[4].parse().unwrap_or(0);
        let iowait: u64 = parts[5].parse().unwrap_or(0);
        let irq: u64 = parts[6].parse().unwrap_or(0);
        let softirq: u64 = parts[7].parse().unwrap_or(0);
        let steal: u64 = if parts.len() > 8 { parts[8].parse().unwrap_or(0) } else { 0 };

        // 计算总时间
        let total_time = user + nice + system + idle + iowait + irq + softirq + steal;
        let idle_time = idle + iowait;

        log::info!("SSH服务 - CPU时间统计: user={}, nice={}, system={}, idle={}, iowait={}, irq={}, softirq={}, steal={}", 
                   user, nice, system, idle, iowait, irq, softirq, steal);
        log::info!("SSH服务 - CPU时间统计: total={}, idle={}, used={}", 
                   total_time, idle_time, total_time - idle_time);

        // 单次采样无法计算准确的使用率，返回0让系统使用其他方法
        log::info!("SSH服务 - 单次采样无法计算准确使用率，使用备用方法");
        0.0
    }
}

impl Default for SshService {
    fn default() -> Self {
        Self::new().expect("无法创建SSH服务")
    }
}
