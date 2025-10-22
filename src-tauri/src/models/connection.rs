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
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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
