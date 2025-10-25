use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// SSH连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub private_key_path: Option<String>,
    pub auth_method: AuthMethod,
    #[serde(default = "default_false")]
    pub connected: bool,
    #[serde(default = "default_false")]
    pub active: bool,
    /// 终端提示符检测配置
    #[serde(default = "default_prompt_config")]
    pub prompt_config: PromptConfig,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 默认值函数
fn default_false() -> bool {
    false
}

/// 终端提示符检测配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptConfig {
    /// 自定义提示符模式列表
    pub patterns: Vec<String>,
    /// 是否启用智能检测
    pub smart_detection: bool,
    /// 最大等待时间（毫秒）
    pub max_wait_time: u64,
    /// 连续空读取的最大次数
    pub max_empty_reads: u32,
}

/// 默认提示符配置
fn default_prompt_config() -> PromptConfig {
    PromptConfig {
        patterns: vec![
            "]# ".to_string(),
            "$ ".to_string(), 
            "> ".to_string(),
            "# ".to_string(),
            "% ".to_string(),
        ],
        smart_detection: true,
        max_wait_time: 10_000, // 5秒
        max_empty_reads: 10,
    }
}

/// 命令执行选项
#[derive(Debug, Clone)]
pub struct CommandOptions {
    /// 自定义提示符模式（覆盖连接配置）
    pub custom_prompts: Option<Vec<String>>,
    /// 超时时间（毫秒）
    pub timeout: Option<u64>,
    /// 是否等待提示符出现
    pub wait_for_prompt: bool,
    /// 是否启用调试输出
    pub debug_output: bool,
}

impl Default for CommandOptions {
    fn default() -> Self {
        Self {
            custom_prompts: None,
            timeout: None,
            wait_for_prompt: true,
            debug_output: false,
        }
    }
}

/// 认证方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthMethod {
    Password,
    PrivateKey,
    Both,
}

/// 连接状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

/// 标签页信息（独立存储，引用链接）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabInfo {
    pub id: String,
    pub connection_id: String,
    pub title: String,
    pub active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}


impl Default for TabInfo {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            connection_id: String::new(),
            title: String::new(),
            active: false,
            created_at: now,
            updated_at: now,
        }
    }
}

impl TabInfo {
    /// 创建新的标签页
    pub fn new(connection_id: String, title: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            connection_id,
            title,
            active: false,
            created_at: now,
            updated_at: now,
        }
    }

    /// 更新标签页
    pub fn update(&mut self) {
        self.updated_at = chrono::Utc::now();
    }
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            host: String::new(),
            port: 22,
            username: String::new(),
            password: None,
            private_key_path: None,
            auth_method: AuthMethod::Password,
            connected: false,
            active: false,
            prompt_config: default_prompt_config(),
            created_at: now,
            updated_at: now,
        }
    }
}



impl ConnectionConfig {
    /// 创建新的连接配置
    pub fn new(name: String, host: String, username: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            host,
            port: 22,
            username,
            password: None,
            private_key_path: None,
            auth_method: AuthMethod::Password,
            connected: false,
            active: false,
            prompt_config: default_prompt_config(),
            created_at: now,
            updated_at: now,
        }
    }

    /// 更新连接配置
    pub fn update(&mut self) {
        self.updated_at = chrono::Utc::now();
    }

    /// 验证连接配置
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("连接名称不能为空".to_string());
        }
        if self.host.is_empty() {
            return Err("主机地址不能为空".to_string());
        }
        if self.username.is_empty() {
            return Err("用户名不能为空".to_string());
        }
        if self.port == 0 {
            return Err("端口号不能为0".to_string());
        }
        
        match self.auth_method {
            AuthMethod::Password => {
                if self.password.is_none() {
                    return Err("密码认证需要提供密码".to_string());
                }
            }
            AuthMethod::PrivateKey => {
                if self.private_key_path.is_none() {
                    return Err("密钥认证需要提供私钥路径".to_string());
                }
            }
            AuthMethod::Both => {
                if self.password.is_none() && self.private_key_path.is_none() {
                    return Err("混合认证需要提供密码或私钥".to_string());
                }
            }
        }
        
        Ok(())
    }
}

// ==================== 硬件信息相关结构体 ====================

/// 服务器硬件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub cpu: CpuInfo,
    pub memory: MemoryInfo,
    pub storage: Vec<StorageInfo>,
    pub network: NetworkInfo,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// CPU信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub model: String,
    pub cores: u32,
    pub usage: f64,
    pub frequency: Option<f64>, // MHz
    pub temperature: Option<f64>, // 摄氏度
}

/// 内存信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total: u64, // MB
    pub used: u64,  // MB
    pub free: u64,  // MB
    pub usage: f64, // 百分比
    pub swap: Option<SwapInfo>,
}

/// 交换分区信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapInfo {
    pub total: u64, // MB
    pub used: u64,  // MB
    pub free: u64,  // MB
    pub usage: f64, // 百分比
}

/// 存储信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub device: String,      // 设备名 (如 /dev/sda1)
    pub mount_point: String, // 挂载点 (如 /, /home)
    pub filesystem: String,  // 文件系统类型 (如 ext4, xfs)
    pub total: u64,          // 总容量 (MB)
    pub used: u64,           // 已使用 (MB)
    pub free: u64,           // 空闲 (MB)
    pub usage: f64,          // 使用率 (百分比)
    pub r#type: String,      // 类型 (ssd, hdd)
}

/// 网络信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterface>,
    pub total_rx: u64,    // 总接收字节数
    pub total_tx: u64,    // 总发送字节数
    pub rx_speed: f64,    // 接收速度 (MB/s)
    pub tx_speed: f64,    // 发送速度 (MB/s)
}

/// 网络接口信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,      // 接口名 (如 eth0, wlan0)
    pub status: String,    // 状态 (up, down)
    pub rx: u64,          // 接收字节数
    pub tx: u64,          // 发送字节数
    pub rx_speed: f64,    // 接收速度 (MB/s)
    pub tx_speed: f64,    // 发送速度 (MB/s)
}

impl Default for HardwareInfo {
    fn default() -> Self {
        Self {
            cpu: CpuInfo::default(),
            memory: MemoryInfo::default(),
            storage: Vec::new(),
            network: NetworkInfo::default(),
            timestamp: chrono::Utc::now(),
        }
    }
}

impl Default for CpuInfo {
    fn default() -> Self {
        Self {
            model: "Unknown".to_string(),
            cores: 1,
            usage: 0.0,
            frequency: None,
            temperature: None,
        }
    }
}

impl Default for MemoryInfo {
    fn default() -> Self {
        Self {
            total: 0,
            used: 0,
            free: 0,
            usage: 0.0,
            swap: None,
        }
    }
}

impl Default for SwapInfo {
    fn default() -> Self {
        Self {
            total: 0,
            used: 0,
            free: 0,
            usage: 0.0,
        }
    }
}

impl Default for StorageInfo {
    fn default() -> Self {
        Self {
            device: "Unknown".to_string(),
            mount_point: "/".to_string(),
            filesystem: "Unknown".to_string(),
            total: 0,
            used: 0,
            free: 0,
            usage: 0.0,
            r#type: "hdd".to_string(),
        }
    }
}

impl Default for NetworkInfo {
    fn default() -> Self {
        Self {
            interfaces: Vec::new(),
            total_rx: 0,
            total_tx: 0,
            rx_speed: 0.0,
            tx_speed: 0.0,
        }
    }
}

impl Default for NetworkInterface {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            status: "down".to_string(),
            rx: 0,
            tx: 0,
            rx_speed: 0.0,
            tx_speed: 0.0,
        }
    }
}
