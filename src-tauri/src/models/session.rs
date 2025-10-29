use crate::models::ConnectionStatus;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// SSH会话信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub connection_id: String,
    pub title: String,
    pub status: ConnectionStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

/// 会话类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SessionType {
    Terminal,
    FileManager,
    Both,
}

impl Default for Session {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            connection_id: String::new(),
            title: String::new(),
            status: ConnectionStatus::Disconnected,
            created_at: now,
            last_activity: now,
        }
    }
}

impl Session {
    /// 创建新的会话
    pub fn new(connection_id: String, title: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            connection_id,
            title,
            status: ConnectionStatus::Disconnected,
            created_at: now,
            last_activity: now,
        }
    }

    /// 更新最后活动时间
    pub fn update_activity(&mut self) {
        self.last_activity = chrono::Utc::now();
    }

    /// 更新会话状态
    pub fn update_status(&mut self, status: ConnectionStatus) {
        self.status = status;
        self.update_activity();
    }

    /// 检查会话是否活跃
    pub fn is_active(&self) -> bool {
        matches!(self.status, ConnectionStatus::Connected)
    }

    /// 检查会话是否过期（超过30分钟无活动）
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(self.last_activity);
        duration.num_minutes() > 30
    }
}
