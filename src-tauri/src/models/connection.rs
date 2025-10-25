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
