// SSH连接相关的Tauri命令
use tauri::command;
use crate::models::{ConnectionConfig, ConnectionStatus, TabInfo};
use crate::services::SSH_SERVICE;

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

/// 获取已连接数
#[command]
pub async fn get_connected_count() -> Result<usize, String> {
    let service = SSH_SERVICE.read().await;
    Ok(service.get_connected_count().await)
}

/// 获取已连接的连接列表
#[command]
pub async fn get_connected_connections() -> Result<Vec<ConnectionConfig>, String> {
    let service = SSH_SERVICE.read().await;
    Ok(service.get_connected_connections().await)
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

/// 重连SSH
#[command]
pub async fn reconnect_ssh(config: ConnectionConfig) -> Result<String, String> {
    // 先断开现有连接
    {
        let service = SSH_SERVICE.read().await;
        if let Err(e) = service.disconnect(&config.id).await {
            log::warn!("重连时断开现有连接失败: {}", e);
        }
    } // 释放读锁
    
    // 重新建立连接
    let service = SSH_SERVICE.read().await;
    service.connect(config).await
}

/// 检查连接健康状态
#[command]
pub async fn check_connection_status(connection_id: String) -> Result<bool, String> {
    let service = SSH_SERVICE.read().await;
    service.check_connection_health(&connection_id).await
}

/// 生成UUID
#[command]
pub async fn generate_uuid() -> Result<String, String> {
    Ok(uuid::Uuid::new_v4().to_string())
}

/// 测试连接（不保存）
#[command]
pub async fn test_connection(config: ConnectionConfig) -> Result<String, String> {
    let service = SSH_SERVICE.read().await;
    
    // 建立连接进行测试
    let connection_id = service.connect(config).await?;
    
    // 立即断开测试连接
    drop(service); // 释放读锁
    let service = SSH_SERVICE.read().await;
    if let Err(e) = service.disconnect(&connection_id).await {
        log::warn!("测试连接断开时出现警告: {}", e);
    }
    
    Ok(format!("连接测试成功: {}", connection_id))
}

/// 获取标签页列表
#[command]
pub async fn get_tabs_list() -> Result<Vec<TabInfo>, String> {
    let service = SSH_SERVICE.read().await;
    service.get_tabs_list().await
}

/// 添加标签页
#[command]
pub async fn add_tab(connection_id: String, title: String) -> Result<String, String> {
    let service = SSH_SERVICE.read().await;
    service.add_tab(connection_id, title).await
}

/// 删除标签页
#[command]
pub async fn remove_tab(tab_id: String) -> Result<(), String> {
    let service = SSH_SERVICE.read().await;
    service.remove_tab(&tab_id).await
}

/// 设置活动标签页
#[command]
pub async fn set_active_tab(tab_id: String) -> Result<(), String> {
    let service = SSH_SERVICE.read().await;
    service.set_active_tab(&tab_id).await
}

/// 获取活动标签页
#[command]
pub async fn get_active_tab() -> Result<Option<TabInfo>, String> {
    let service = SSH_SERVICE.read().await;
    Ok(service.get_active_tab().await)
}

/// 关闭所有标签页
#[command]
pub async fn close_all_tabs() -> Result<(), String> {
    let service = SSH_SERVICE.read().await;
    service.close_all_tabs().await
}

/// 关闭其他标签页
#[command]
pub async fn close_other_tabs(keep_tab_id: String) -> Result<(), String> {
    let service = SSH_SERVICE.read().await;
    service.close_other_tabs(&keep_tab_id).await
}

/// 根据链接ID获取标签页
#[command]
pub async fn get_tab_by_connection_id(connection_id: String) -> Result<Option<TabInfo>, String> {
    let service = SSH_SERVICE.read().await;
    Ok(service.get_tab_by_connection_id(&connection_id).await)
}
