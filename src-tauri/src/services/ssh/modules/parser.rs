// SSH数据解析模块
use crate::models::{MemoryInfo, NetworkInfo, StorageInfo};

pub struct SshDataParser;

impl SshDataParser {
    /// 清理命令输出，移除ANSI颜色代码和控制字符
    pub fn clean_command_output(output: &str) -> String {
        let cleaned = output
            .replace("\u{1b}[01;31m", "")
            .replace("\u{1b}[m", "")
            .replace("\u{1b}[K", "")
            .replace("\u{1b}]0;root@iZ2vcc2rr8aqk0prfnjavaZ:~\u{7}", "")
            .replace("\r\n", "\n")
            .replace("\r", "\n");

        let lines: Vec<&str> = cleaned.lines().collect();
        let mut result = String::new();

        for line in lines {
            let trimmed = line.trim();
            if !trimmed.is_empty()
                && !trimmed.starts_with("lscpu")
                && !trimmed.starts_with("nproc")
                && !trimmed.starts_with("top")
                && !trimmed.starts_with("cat")
                && !trimmed.starts_with("[root@")
                && !trimmed.ends_with("]#")
                && !trimmed.ends_with("$")
            {
                result.push_str(trimmed);
                result.push('\n');
            }
        }

        result.trim().to_string()
    }

    /// 提取CPU型号
    pub fn extract_cpu_model(output: &str) -> String {
        let cleaned = Self::clean_command_output(output);
        cleaned
            .lines()
            .find(|l| l.contains("Model name:") && !l.contains("BIOS"))
            .map(|l| l.replace("Model name:", "").trim().to_string())
            .or_else(|| {
                cleaned
                    .lines()
                    .find(|l| l.contains("Vendor ID:"))
                    .map(|l| l.replace("Vendor ID:", "").trim().to_string())
            })
            .unwrap_or_else(|| "Unknown CPU".to_string())
    }

    /// 提取CPU核心数
    pub fn extract_cpu_cores(output: &str) -> u32 {
        let cleaned = Self::clean_command_output(output);
        cleaned
            .lines()
            .find(|l| l.trim().parse::<u32>().is_ok())
            .and_then(|l| l.trim().parse::<u32>().ok())
            .unwrap_or(1)
    }

    /// 提取CPU使用率
    pub fn extract_cpu_usage(output: &str) -> f64 {
        let cleaned = Self::clean_command_output(output);

        cleaned
            .lines()
            .find(|l| l.contains("Cpu(s)"))
            .and_then(|line| {
                let parts: Vec<&str> = line.split(',').collect();
                for part in parts {
                    if part.contains("id") {
                        let idle_number = part
                            .split_whitespace()
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
    pub fn extract_cpu_frequency(output: &str) -> Option<f64> {
        let cleaned = Self::clean_command_output(output);
        cleaned
            .lines()
            .find(|l| l.contains("CPU MHz"))
            .and_then(|line| {
                let freq_value = Self::extract_number_from_output(line);
                freq_value.parse::<f64>().ok()
            })
    }

    /// 提取CPU温度
    pub fn extract_cpu_temperature(output: &str) -> Option<f64> {
        let cleaned = Self::clean_command_output(output);
        cleaned
            .lines()
            .find(|l| l.trim().parse::<f64>().is_ok() && l.trim().len() > 3)
            .and_then(|line| {
                let temp_value = Self::extract_number_from_output(line);
                temp_value.parse::<f64>().ok().map(|temp| temp / 1000.0)
            })
    }

    /// 解析 /proc/meminfo 输出
    pub fn parse_meminfo(output: &str, memory_info: &mut MemoryInfo) -> Result<(), String> {
        let mut mem_total = 0u64;
        let mut mem_available = 0u64;
        let mut mem_free = 0u64;

        for line in output.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(value) = parts[1].parse::<u64>() {
                    match parts[0] {
                        "MemTotal:" => {
                            mem_total = value;
                        }
                        "MemAvailable:" => {
                            mem_available = value;
                        }
                        "MemFree:" => {
                            mem_free = value;
                        }
                        "Buffers:" => {}
                        "Cached:" => {}
                        _ => {}
                    }
                }
            }
        }

        memory_info.total = mem_total / 1024;
        memory_info.free = if mem_available > 0 {
            mem_available / 1024
        } else {
            mem_free / 1024
        };
        memory_info.used = memory_info.total - memory_info.free;

        if memory_info.total > 0 {
            memory_info.usage = (memory_info.used as f64 / memory_info.total as f64) * 100.0;
        }

        Ok(())
    }

    /// 解析 df -h 输出
    pub fn parse_df_output(
        output: &str,
        storage_list: &mut Vec<StorageInfo>,
    ) -> Result<(), String> {
        for (index, line) in output.lines().enumerate() {
            // 跳过返回命令行和标题行
            if index <= 1 {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            // 跳过空行
            if parts.len() < 6 {
                continue;
            }

            let device = parts[0].to_string();
            let mount_point = parts[5].to_string();

            // 跳过虚拟文件系统
            if Self::is_virtual_filesystem(&device, &mount_point) {
                continue;
            }

            let mut storage_info = StorageInfo::default();
            storage_info.device = device;
            storage_info.mount_point = mount_point;

            // df -h 输出格式: device size used avail use% mount_point
            // 字段: parts[0]=device, parts[1]=size, parts[2]=used, parts[3]=avail, parts[4]=use%, parts[5]=mount_point
            if parts.len() >= 6 {
                storage_info.total = Self::parse_size_to_mb(parts[1]);
                storage_info.used = Self::parse_size_to_mb(parts[2]);
                storage_info.free = Self::parse_size_to_mb(parts[3]);

                // 计算使用率
                if storage_info.total > 0 {
                    storage_info.usage =
                        (storage_info.used as f64 / storage_info.total as f64) * 100.0;
                }
            }

            storage_list.push(storage_info);
        }
        log::info!("storage_list: {:?}", storage_list);
        Ok(())
    }

    /// 判断是否为虚拟文件系统
    fn is_virtual_filesystem(device: &str, mount_point: &str) -> bool {
        let virtual_devices = ["tmpfs", "devtmpfs", "overlay", "squashfs", "sysfs", "proc"];

        let virtual_mounts = [
            "/proc",
            "/sys",
            "/dev",
            "/run/user",
            "/snap",
            "/var/lib/docker",
        ];

        // 检查设备类型
        if virtual_devices.iter().any(|&v| device.starts_with(v)) {
            return true;
        }

        // 检查挂载点
        if !virtual_mounts.iter().any(|&v| mount_point.starts_with(v)) {
            return false;
        }

        // 特殊处理：/dev下的真实设备应该保留
        if mount_point.starts_with("/dev") && device.starts_with("/dev/") {
            return false;
        }

        // 特殊处理：/run下的真实挂载应该保留
        if mount_point.starts_with("/run") && !device.starts_with("tmpfs") {
            return false;
        }

        true
    }

    /// 将大小字符串转换为MB
    pub fn parse_size_to_mb(size_str: &str) -> u64 {
        let size_str = size_str.to_uppercase();
        if let Some(num_part) = size_str
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.')
            .collect::<String>()
            .parse::<f64>()
            .ok()
        {
            if size_str.contains('G') || size_str.contains("GB") {
                (num_part * 1024.0) as u64
            } else if size_str.contains('T') || size_str.contains("TB") {
                (num_part * 1024.0 * 1024.0) as u64
            } else if size_str.contains('K') || size_str.contains("KB") {
                (num_part / 1024.0) as u64
            } else if size_str.contains('M') || size_str.contains("MB") {
                num_part as u64
            } else {
                (num_part / 1024.0 / 1024.0) as u64
            }
        } else {
            0
        }
    }

    /// 解析 /proc/net/dev 输出
    pub fn parse_netdev_output(output: &str, network_info: &mut NetworkInfo) -> Result<(), String> {
        for line in output.lines().skip(2) {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let interface_name = parts[0].trim().to_string();

                if interface_name == "lo" {
                    continue;
                }

                let stats: Vec<&str> = parts[1].split_whitespace().collect();
                if stats.len() >= 16 {
                    let mut interface = crate::models::NetworkInterface::default();
                    interface.name = interface_name;

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
    pub fn enrich_network_status(output: &str, network_info: &mut NetworkInfo) {
        for interface in network_info.interfaces.iter_mut() {
            if output.contains(&format!("{}:", interface.name)) {
                if output.contains("UP") {
                    interface.status = "up".to_string();
                } else {
                    interface.status = "down".to_string();
                }
            }
        }
    }

    /// 计算网络总流量和速度
    pub fn calculate_network_totals(network_info: &mut NetworkInfo) {
        let mut total_rx = 0u64;
        let mut total_tx = 0u64;

        for interface in &network_info.interfaces {
            total_rx += interface.rx;
            total_tx += interface.tx;
        }

        network_info.total_rx = total_rx;
        network_info.total_tx = total_tx;
        network_info.rx_speed = 0.0;
        network_info.tx_speed = 0.0;
    }

    /// 从命令输出中提取数字部分
    pub fn extract_number_from_output(output: &str) -> String {
        for line in output.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed.contains("grep")
                || trimmed.contains("awk")
                || trimmed.contains("sed")
                || trimmed.contains("top")
                || trimmed.contains("vmstat")
                || trimmed.contains("cat")
                || trimmed.contains("lscpu")
                || trimmed.contains("sensors")
                || trimmed.contains("[root@")
                || trimmed.contains("]#")
            {
                continue;
            }

            let mut number = String::new();
            let mut found_digit = false;
            for ch in trimmed.chars() {
                if ch.is_ascii_digit() || ch == '.' || ch == '-' {
                    number.push(ch);
                    found_digit = true;
                } else if found_digit {
                    break;
                }
            }

            if !number.is_empty() && number.parse::<f64>().is_ok() {
                return number;
            }
        }

        String::new()
    }
}
