// 连接配置存储相关的Tauri命令
use crate::models::ConnectionConfig;
use crate::services::{ConnectionStorage, SSH_SERVICE};
use std::sync::Arc;
use tauri::command;
use tokio::sync::RwLock;

// 全局连接存储实例
lazy_static::lazy_static! {
    static ref CONNECTION_STORAGE: Arc<RwLock<ConnectionStorage>> = Arc::new(RwLock::new(
        ConnectionStorage::new().expect("无法创建连接存储管理器")
    ));
}

/// 保存连接配置
#[command]
pub async fn save_connection(config: ConnectionConfig) -> Result<(), String> {
    let storage = CONNECTION_STORAGE.read().await;
    storage.save_connection(&config)
}

/// 加载连接配置
#[command]
pub async fn load_connection(connection_id: String) -> Result<ConnectionConfig, String> {
    let storage = CONNECTION_STORAGE.read().await;
    storage.load_connection(&connection_id)
}

/// 更新连接配置
#[command]
pub async fn update_connection(config: ConnectionConfig) -> Result<(), String> {
    let storage = CONNECTION_STORAGE.read().await;
    storage.update_connection(&config)
}

/// 删除连接配置
#[command]
pub async fn delete_connection(connection_id: String) -> Result<(), String> {
    log::info!("开始删除连接配置: {}", connection_id);

    // 先尝试断开连接（如果连接存在）
    {
        let ssh_service = SSH_SERVICE.read().await;
        let connections = ssh_service.get_connected_connections().await;
        if connections.iter().any(|conn| conn.id == connection_id) {
            log::info!("删除连接前先断开SSH连接: {}", connection_id);
            if let Err(e) = ssh_service.disconnect(&connection_id).await {
                log::warn!("断开连接时出错: {}", e);
                // 继续执行删除操作
            } else {
                log::info!("SSH连接已断开，硬件服务将无法获取该连接的信息");
            }
        } else {
            log::info!("连接 {} 未处于连接状态，跳过断开操作", connection_id);
        }
    }

    // 删除相关的标签页
    {
        let storage = CONNECTION_STORAGE.read().await;
        let _ = storage.remove_tabs_by_connection_id(&connection_id);
        log::info!("已删除相关标签页");
    }

    // 删除连接配置
    let storage = CONNECTION_STORAGE.read().await;
    storage.delete_connection(&connection_id)?;
    log::info!("连接配置已删除: {}", connection_id);

    Ok(())
}

/// 获取所有保存的连接配置
#[command]
pub async fn get_saved_connections() -> Result<Vec<ConnectionConfig>, String> {
    let storage = CONNECTION_STORAGE.read().await;
    storage.get_all_connections()
}

/// 导出连接配置
#[command]
pub async fn export_connections() -> Result<String, String> {
    let storage = CONNECTION_STORAGE.read().await;
    storage.export_connections()
}

/// 导入连接配置
#[command]
pub async fn import_connections(json_data: String) -> Result<(), String> {
    let storage = CONNECTION_STORAGE.read().await;
    storage.import_connections(&json_data)
}

/// 删除全部保存的连接配置
#[command]
pub async fn delete_all_connections() -> Result<(), String> {
    let storage = CONNECTION_STORAGE.read().await;

    // 先清空所有标签页
    storage.clear_all_tabs()?;

    // 再删除所有连接配置
    storage.delete_all()
}
