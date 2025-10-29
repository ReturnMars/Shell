// SSH连接服务 - 统一入口模块
use super::modules::*;
use crate::models::{
    CommandOptions, ConnectionConfig, ConnectionStatus, HardwareInfo, Session as AppSession,
    TabInfo,
};
use crate::services::storage::ConnectionStorage;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// SSH连接管理器 - 统一对外接口
pub struct SshService {
    /// SSH连接管理器
    connection_manager: Arc<SshConnectionManager>,
    /// 会话管理
    sessions: Arc<RwLock<HashMap<String, AppSession>>>,
    /// 标签页管理器
    tab_manager: Arc<SshTabManager>,
    /// 硬件信息服务
    hardware_service: Arc<SshHardwareService>,
    /// 连接存储管理器
    storage: Arc<ConnectionStorage>,
}

impl SshService {
    /// 创建新的SSH服务实例
    pub fn new() -> Result<Self, String> {
        let storage = ConnectionStorage::new()?;
        let storage_arc = Arc::new(storage);

        let connection_manager = Arc::new(SshConnectionManager::new(storage_arc.clone()));
        let tab_manager = Arc::new(SshTabManager::new(storage_arc.clone()));
        let hardware_service = Arc::new(SshHardwareService::new(connection_manager.clone()));

        Ok(Self {
            connection_manager,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            tab_manager,
            hardware_service,
            storage: storage_arc,
        })
    }

    // ==================== SSH连接管理方法 ====================

    /// 建立SSH连接
    pub async fn connect(&self, config: ConnectionConfig) -> Result<String, String> {
        self.connection_manager.connect(config).await
    }

    /// 检查连接健康状态
    pub async fn check_connection_health(&self, connection_id: &str) -> Result<bool, String> {
        self.connection_manager
            .check_connection_health(connection_id)
            .await
    }

    /// 断开SSH连接
    pub async fn disconnect(&self, connection_id: &str) -> Result<(), String> {
        self.connection_manager.disconnect(connection_id).await
    }

    /// 断开所有连接
    pub async fn disconnect_all(&self) -> Result<(), String> {
        self.connection_manager.disconnect_all().await
    }

    /// 获取连接状态
    pub async fn get_connection_status(
        &self,
        connection_id: &str,
    ) -> Result<ConnectionStatus, String> {
        self.connection_manager
            .get_connection_status(connection_id)
            .await
    }

    /// 获取所有连接
    pub async fn get_connections(&self) -> Vec<ConnectionConfig> {
        self.connection_manager.get_connections().await
    }

    /// 获取已连接数
    pub async fn get_connected_count(&self) -> usize {
        self.connection_manager.get_connected_count().await
    }

    /// 获取已连接的连接列表
    pub async fn get_connected_connections(&self) -> Vec<ConnectionConfig> {
        self.connection_manager.get_connected_connections().await
    }

    /// 获取SSH连接（用于终端服务）
    pub async fn get_ssh_connection(&self, connection_id: &str) -> Result<SshConnection, String> {
        self.connection_manager
            .get_ssh_connection(connection_id)
            .await
    }

    // ==================== 命令执行方法 ====================

    /// 执行SSH命令
    pub async fn execute_command(
        &self,
        connection_id: &str,
        command: &str,
    ) -> Result<String, String> {
        self.execute_command_with_options(connection_id, command, CommandOptions::default())
            .await
    }

    /// 执行SSH命令 - 带选项的完整版本
    pub async fn execute_command_with_options(
        &self,
        connection_id: &str,
        command: &str,
        options: CommandOptions,
    ) -> Result<String, String> {
        let mut connections = self.connection_manager.connections.write().await;

        if let Some(connection) = connections.get_mut(connection_id) {
            if !matches!(connection.status, ConnectionStatus::Connected) {
                return Err("连接未建立".to_string());
            }

            if let Some(ref mut shell_channel) = connection.shell_channel {
                SshCommandExecutor::execute_command_with_options(
                    shell_channel,
                    &connection.config,
                    command,
                    options,
                )
                .await
            } else {
                Err("Shell通道不存在".to_string())
            }
        } else {
            Err("连接不存在".to_string())
        }
    }

    // ==================== 标签页管理方法 ====================

    /// 获取标签页列表
    pub async fn get_tabs_list(&self) -> Result<Vec<TabInfo>, String> {
        self.tab_manager.get_tabs_list().await
    }

    /// 添加标签页
    pub async fn add_tab(&self, connection_id: String, title: String) -> Result<String, String> {
        self.tab_manager.add_tab(connection_id, title).await
    }

    /// 删除标签页
    pub async fn remove_tab(&self, tab_id: &str) -> Result<(), String> {
        self.tab_manager.remove_tab(tab_id).await
    }

    /// 设置活动标签页
    pub async fn set_active_tab(&self, tab_id: &str) -> Result<(), String> {
        self.tab_manager.set_active_tab(tab_id).await
    }

    /// 获取活动标签页
    pub async fn get_active_tab(&self) -> Option<TabInfo> {
        self.tab_manager.get_active_tab().await
    }

    /// 关闭所有标签页
    pub async fn close_all_tabs(&self) -> Result<(), String> {
        self.tab_manager.close_all_tabs().await
    }

    /// 关闭其他标签页
    pub async fn close_other_tabs(&self, keep_tab_id: &str) -> Result<(), String> {
        self.tab_manager.close_other_tabs(keep_tab_id).await
    }

    /// 根据链接ID获取标签页
    pub async fn get_tab_by_connection_id(&self, connection_id: &str) -> Option<TabInfo> {
        self.tab_manager
            .get_tab_by_connection_id(connection_id)
            .await
    }

    // ==================== 硬件信息获取方法 ====================

    /// 获取完整的硬件信息
    pub async fn get_hardware_info(&self, connection_id: &str) -> Result<HardwareInfo, String> {
        self.hardware_service.get_hardware_info(connection_id).await
    }

    /// 获取CPU信息
    pub async fn get_cpu_info(
        &self,
        connection_id: &str,
    ) -> Result<crate::models::CpuInfo, String> {
        self.hardware_service.get_cpu_info(connection_id).await
    }

    /// 获取内存信息
    pub async fn get_memory_info(
        &self,
        connection_id: &str,
    ) -> Result<crate::models::MemoryInfo, String> {
        self.hardware_service.get_memory_info(connection_id).await
    }

    /// 获取存储信息
    pub async fn get_storage_info(
        &self,
        connection_id: &str,
    ) -> Result<Vec<crate::models::StorageInfo>, String> {
        self.hardware_service.get_storage_info(connection_id).await
    }

    /// 获取网络信息
    pub async fn get_network_info(
        &self,
        connection_id: &str,
    ) -> Result<crate::models::NetworkInfo, String> {
        self.hardware_service.get_network_info(connection_id).await
    }
}

impl Default for SshService {
    fn default() -> Self {
        Self::new().expect("无法创建SSH服务")
    }
}
