// SSH服务管理器 - 统一管理SSH服务实例
use crate::services::ssh_service::SshService;
use std::sync::Arc;
use tokio::sync::RwLock;

// 全局SSH服务实例 - 确保所有命令使用同一个实例
lazy_static::lazy_static! {
    pub static ref SSH_SERVICE: Arc<RwLock<SshService>> = Arc::new(RwLock::new(
        SshService::new().expect("无法创建SSH服务")
    ));
}

/// 获取SSH服务实例的引用
pub fn get_ssh_service() -> &'static Arc<RwLock<SshService>> {
    &SSH_SERVICE
}
