// SSH硬件信息获取模块
use super::command::SshCommandExecutor;
use super::connection::SshConnectionManager;
use super::parser::SshDataParser;
use crate::models::{CpuInfo, HardwareInfo, MemoryInfo, NetworkInfo, StorageInfo};
use std::sync::Arc;
use std::collections::HashMap;
use chrono::DateTime;
use std::sync::Mutex;

// 上一次的网络数据缓存
#[allow(dead_code)]
struct NetworkCache {
    rx_bytes: u64,
    tx_bytes: u64,
    timestamp: DateTime<chrono::Utc>,
    interfaces: Vec<(String, u64, u64)>, // (name, rx_bytes, tx_bytes)
}

pub struct SshHardwareService {
    connection_manager: Arc<SshConnectionManager>,
    #[allow(dead_code)]
    network_cache: Arc<Mutex<HashMap<String, NetworkCache>>>,
}

impl SshHardwareService {
    pub fn new(connection_manager: Arc<SshConnectionManager>) -> Self {
        Self { 
            connection_manager,
            network_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 获取CPU信息
    pub async fn get_cpu_info(&self, connection_id: &str) -> Result<CpuInfo, String> {
        let mut cpu_info = CpuInfo::default();

        // 1. 获取CPU型号
        let model_cmd =
            "lscpu | grep 'Model name' || lscpu | grep 'Vendor ID' || echo 'Unknown CPU'";
        let model_output = self.execute_command(connection_id, model_cmd).await?;
        cpu_info.model = SshDataParser::extract_cpu_model(&model_output);

        // 2. 获取核心数
        let cores_cmd = "nproc";
        let cores_output = self.execute_command(connection_id, cores_cmd).await?;
        cpu_info.cores = SshDataParser::extract_cpu_cores(&cores_output);

        // 3. 获取CPU使用率
        let usage_cmd = "top -bn1 | grep 'Cpu(s)'";
        let usage_output = self.execute_command(connection_id, usage_cmd).await?;
        cpu_info.usage = SshDataParser::extract_cpu_usage(&usage_output);

        // 4. 获取CPU频率
        let freq_cmd = "lscpu | grep 'CPU MHz'";
        let freq_output = self.execute_command(connection_id, freq_cmd).await?;
        cpu_info.frequency = SshDataParser::extract_cpu_frequency(&freq_output);

        // 5. 获取CPU温度
        let temp_cmd = "cat /sys/class/thermal/thermal_zone*/temp 2>/dev/null | head -1";
        let temp_output = self.execute_command(connection_id, temp_cmd).await?;
        cpu_info.temperature = SshDataParser::extract_cpu_temperature(&temp_output);

        Ok(cpu_info)
    }

    /// 获取内存信息
    pub async fn get_memory_info(&self, connection_id: &str) -> Result<MemoryInfo, String> {
        let mut memory_info = MemoryInfo::default();

        let meminfo_output = self
            .execute_command(connection_id, "cat /proc/meminfo")
            .await?;
        SshDataParser::parse_meminfo(&meminfo_output, &mut memory_info)?;

        match self.get_swap_info(connection_id).await {
            Ok(swap_info) => {
                memory_info.swap = Some(swap_info);
            }
            Err(_) => {
                memory_info.swap = None;
            }
        }

        Ok(memory_info)
    }

    /// 获取交换分区信息
    async fn get_swap_info(&self, connection_id: &str) -> Result<crate::models::SwapInfo, String> {
        let mut swap_info = crate::models::SwapInfo::default();

        let meminfo_output = self
            .execute_command(connection_id, "cat /proc/meminfo | grep -i swap")
            .await?;

        // 清理ANSI颜色代码
        let cleaned_output = SshDataParser::clean_command_output(&meminfo_output);

        let mut swap_total = 0u64;
        let mut swap_free = 0u64;

        for line in cleaned_output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(value) = parts[1].parse::<u64>() {
                    match parts[0] {
                        "SwapTotal:" => swap_total = value,
                        "SwapFree:" => swap_free = value,
                        _ => {}
                    }
                }
            }
        }

        if swap_total > 0 {
            swap_info.total = swap_total / 1024;
            swap_info.free = swap_free / 1024;
            swap_info.used = swap_info.total - swap_info.free;
            swap_info.usage = if swap_info.total > 0 {
                (swap_info.used as f64 / swap_info.total as f64) * 100.0
            } else {
                0.0
            };
        }

        Ok(swap_info)
    }

    /// 获取存储信息
    pub async fn get_storage_info(&self, connection_id: &str) -> Result<Vec<StorageInfo>, String> {
        let mut storage_list = Vec::new();

        let df_output = self.execute_command(connection_id, "df -h").await?;
        SshDataParser::parse_df_output(&df_output, &mut storage_list)?;

        self.enrich_storage_type_info(connection_id, &mut storage_list)
            .await;

        Ok(storage_list)
    }

    /// 丰富存储类型信息
    async fn enrich_storage_type_info(
        &self,
        connection_id: &str,
        storage_list: &mut Vec<StorageInfo>,
    ) {
        let lsblk_output = self
            .execute_command(connection_id, "lsblk -d -o NAME,TYPE,ROTA")
            .await
            .ok();

        if let Some(output) = lsblk_output {
            for storage in storage_list.iter_mut() {
                if let Some(disk_name) = storage.device.strip_prefix("/dev/") {
                    let disk_name = disk_name
                        .chars()
                        .take_while(|c| c.is_alphabetic())
                        .collect::<String>();

                    for line in output.lines() {
                        if line.contains(&disk_name) {
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

        let netdev_output = self
            .execute_command(connection_id, "cat /proc/net/dev")
            .await?;
        SshDataParser::parse_netdev_output(&netdev_output, &mut network_info)?;

        if let Some(ip_output) = self
            .execute_command(connection_id, "ip addr show")
            .await
            .ok()
        {
            SshDataParser::enrich_network_status(&ip_output, &mut network_info);
        }

        SshDataParser::calculate_network_totals(&mut network_info);

        // 计算网络速度
        self.calculate_network_speed(connection_id, &mut network_info)?;

        Ok(network_info)
    }

    /// 计算网络速度
    fn calculate_network_speed(
        &self,
        connection_id: &str,
        network_info: &mut NetworkInfo,
    ) -> Result<(), String> {
        let cache = self.network_cache.clone();
        let mut cache_guard = cache.lock().unwrap();

        // 检查是否有缓存的网络数据
        if let Some(cached) = cache_guard.get(connection_id) {
            let current_time = chrono::Utc::now();
            let time_diff = current_time.signed_duration_since(cached.timestamp);
            let seconds = time_diff.num_milliseconds() as f64 / 1000.0;

            if seconds > 0.0 && seconds < 60.0 {
                // 计算总速度（字节/秒）
                let rx_diff = network_info.total_rx.saturating_sub(cached.rx_bytes);
                let tx_diff = network_info.total_tx.saturating_sub(cached.tx_bytes);

                network_info.rx_speed = rx_diff as f64 / seconds;
                network_info.tx_speed = tx_diff as f64 / seconds;

                // 为每个接口计算速度
                let interface_cache: std::collections::HashMap<String, (u64, u64)> = 
                    cached.interfaces.iter().map(|(name, rx, tx)| {
                        (name.clone(), (*rx, *tx))
                    }).collect();

                for interface in &mut network_info.interfaces {
                    if let Some((cached_rx, cached_tx)) = interface_cache.get(&interface.name) {
                        let rx_diff = interface.rx.saturating_sub(*cached_rx);
                        let tx_diff = interface.tx.saturating_sub(*cached_tx);
                        
                        interface.rx_speed = rx_diff as f64 / seconds;
                        interface.tx_speed = tx_diff as f64 / seconds;
                    }
                }
            }
        } else {
            // 第一次获取，速度设为0
            network_info.rx_speed = 0.0;
            network_info.tx_speed = 0.0;
            for interface in &mut network_info.interfaces {
                interface.rx_speed = 0.0;
                interface.tx_speed = 0.0;
            }
        }

        // 更新缓存
        let interfaces: Vec<(String, u64, u64)> = network_info.interfaces.iter().map(|i| {
            (i.name.clone(), i.rx, i.tx)
        }).collect();

        cache_guard.insert(
            connection_id.to_string(),
            NetworkCache {
                rx_bytes: network_info.total_rx,
                tx_bytes: network_info.total_tx,
                timestamp: chrono::Utc::now(),
                interfaces,
            },
        );

        Ok(())
    }

    /// 获取完整的硬件信息
    pub async fn get_hardware_info(&self, connection_id: &str) -> Result<HardwareInfo, String> {
        // 先检查连接是否存在且已连接
        {
            let connections = self.connection_manager.connections.read().await;

            if !connections.contains_key(connection_id) {
                return Err(format!("连接不存在: {}", connection_id));
            }

            if let Some(conn) = connections.get(connection_id) {
                if !matches!(conn.status, crate::models::ConnectionStatus::Connected) {
                    return Err(format!("连接状态异常: {:?}", conn.status));
                }
            }
        } // 释放读锁

        // 获取各种硬件信息
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

        Ok(hardware_info)
    }

    /// 执行命令的辅助方法
    async fn execute_command(&self, connection_id: &str, command: &str) -> Result<String, String> {
        // 先检查连接是否存在且已连接
        {
            let connections = self.connection_manager.connections.read().await;

            if !connections.contains_key(connection_id) {
                return Err("连接不存在".to_string());
            }

            if let Some(connection) = connections.get(connection_id) {
                if !matches!(
                    connection.status,
                    crate::models::ConnectionStatus::Connected
                ) {
                    return Err("连接未建立".to_string());
                }
            }
        } // 释放读锁

        // 获取可变的连接来执行命令
        let mut connections = self.connection_manager.connections.write().await;

        if let Some(connection) = connections.get_mut(connection_id) {
            if let Some(ref mut shell_channel) = connection.shell_channel {
                SshCommandExecutor::execute_command(shell_channel, &connection.config, command)
                    .await
            } else {
                Err("Shell通道不存在".to_string())
            }
        } else {
            Err("连接不存在".to_string())
        }
    }
}
