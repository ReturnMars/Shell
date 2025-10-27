// 硬件信息获取命令
use crate::models::HardwareInfo;
use crate::services::SSH_SERVICE;

/// 获取服务器硬件信息
#[tauri::command]
pub async fn get_hardware_info(connection_id: String) -> Result<HardwareInfo, String> {
    log::info!("前端请求获取硬件信息: {}", connection_id);

    let service = SSH_SERVICE.read().await;

    // 调试：检查实际连接状态
    let connected_connections = service.get_connected_connections().await;
    log::info!("实际已连接数量: {}", connected_connections.len());
    for conn in &connected_connections {
        log::info!(
            "已连接 - ID: {}, 名称: {}, 主机: {}",
            conn.id,
            conn.name,
            conn.host
        );
    }

    // 检查指定连接是否真的存在
    let is_connected = connected_connections
        .iter()
        .any(|conn| conn.id == connection_id);
    log::info!(
        "连接 {} 是否在已连接列表中: {}",
        connection_id,
        is_connected
    );

    service
        .get_hardware_info(&connection_id)
        .await
        .map_err(|e| {
            log::error!("获取硬件信息失败: {}", e);
            e
        })
}

/// 获取CPU信息
#[tauri::command]
pub async fn get_cpu_info(connection_id: String) -> Result<crate::models::CpuInfo, String> {
    log::info!("前端请求获取CPU信息: {}", connection_id);

    let service = SSH_SERVICE.read().await;
    service.get_cpu_info(&connection_id).await.map_err(|e| {
        log::error!("获取CPU信息失败: {}", e);
        e
    })
}

/// 获取内存信息
#[tauri::command]
pub async fn get_memory_info(connection_id: String) -> Result<crate::models::MemoryInfo, String> {
    log::info!("前端请求获取内存信息: {}", connection_id);

    let service = SSH_SERVICE.read().await;
    service.get_memory_info(&connection_id).await.map_err(|e| {
        log::error!("获取内存信息失败: {}", e);
        e
    })
}

/// 获取存储信息
#[tauri::command]
pub async fn get_storage_info(
    connection_id: String,
) -> Result<Vec<crate::models::StorageInfo>, String> {
    log::info!("前端请求获取存储信息: {}", connection_id);

    let service = SSH_SERVICE.read().await;
    service.get_storage_info(&connection_id).await.map_err(|e| {
        log::error!("获取存储信息失败: {}", e);
        e
    })
}

/// 获取网络信息
#[tauri::command]
pub async fn get_network_info(connection_id: String) -> Result<crate::models::NetworkInfo, String> {
    log::info!("前端请求获取网络信息: {}", connection_id);

    let service = SSH_SERVICE.read().await;
    service.get_network_info(&connection_id).await.map_err(|e| {
        log::error!("获取网络信息失败: {}", e);
        e
    })
}
