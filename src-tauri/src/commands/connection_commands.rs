// SSH连接相关的Tauri命令
use tauri::command;
use crate::models::{ConnectionConfig, ConnectionStatus};
use crate::services::SshService;
use std::sync::Arc;
use tokio::sync::RwLock;

// 全局SSH服务实例
lazy_static::lazy_static! {
    static ref SSH_SERVICE: Arc<RwLock<SshService>> = Arc::new(RwLock::new(SshService::new()));
}

/// 建立SSH连接
#[command]
pub async fn connect_ssh(config: ConnectionConfig) -> Result<String, String> {
    let service = SSH_SERVICE.read().await;
    service.connect(config).await
}

/// 断开SSH连接
#[command]
pub async fn disconnect_ssh(connection_id: String) -> Result<(), String> {
    let service = SSH_SERVICE.read().await;
    service.disconnect(&connection_id).await
}

/// 获取连接状态
#[command]
pub async fn get_connection_status(connection_id: String) -> Result<ConnectionStatus, String> {
    let service = SSH_SERVICE.read().await;
    service.get_connection_status(&connection_id).await
}

/// 获取所有连接
#[command]
pub async fn get_connections() -> Result<Vec<ConnectionConfig>, String> {
    let service = SSH_SERVICE.read().await;
    Ok(service.get_connections().await)
}

/// 执行SSH命令
#[command]
pub async fn execute_ssh_command(connection_id: String, command: String) -> Result<String, String> {
    let service = SSH_SERVICE.read().await;
    service.execute_command(&connection_id, &command).await
}

/// 断开所有SSH连接
#[command]
pub async fn disconnect_all_ssh() -> Result<(), String> {
    let service = SSH_SERVICE.read().await;
    service.disconnect_all().await
}

/// 测试连接（不保存）
#[command]
pub async fn test_connection(config: ConnectionConfig) -> Result<String, String> {
    let service = SSH_SERVICE.read().await;
    service.connect(config).await
}