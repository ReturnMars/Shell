// SSH硬件信息获取模块
use super::command::SshCommandExecutor;
use super::connection::SshConnectionManager;
use super::parser::SshDataParser;
use crate::models::{CpuInfo, HardwareInfo, MemoryInfo, NetworkInfo, StorageInfo};
use std::sync::Arc;

pub struct SshHardwareService {
    connection_manager: Arc<SshConnectionManager>,
}

impl SshHardwareService {
    pub fn new(connection_manager: Arc<SshConnectionManager>) -> Self {
        Self { connection_manager }
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

        Ok(network_info)
    }

    /// 获取完整的硬件信息
    pub async fn get_hardware_info(&self, connection_id: &str) -> Result<HardwareInfo, String> {
        let connections = self.connection_manager.connections.read().await;

        if !connections.contains_key(connection_id) {
            return Err(format!("连接不存在: {}", connection_id));
        }

        if let Some(conn) = connections.get(connection_id) {
            if !matches!(conn.status, crate::models::ConnectionStatus::Connected) {
                return Err(format!("连接状态异常: {:?}", conn.status));
            }
        }

        drop(connections);

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
        let mut connections = self.connection_manager.connections.write().await;

        if let Some(connection) = connections.get_mut(connection_id) {
            if !matches!(
                connection.status,
                crate::models::ConnectionStatus::Connected
            ) {
                return Err("连接未建立".to_string());
            }

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
