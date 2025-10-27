// SSH标签页管理模块 - Tab管理
use crate::models::TabInfo;
use crate::services::storage::ConnectionStorage;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct SshTabManager {
    /// 标签页管理
    pub tabs: Arc<RwLock<HashMap<String, TabInfo>>>,
    /// 连接存储管理器
    pub storage: Arc<ConnectionStorage>,
}

impl SshTabManager {
    /// 创建新的标签页管理器
    pub fn new(storage: Arc<ConnectionStorage>) -> Self {
        Self {
            tabs: Arc::new(RwLock::new(HashMap::new())),
            storage,
        }
    }

    /// 获取标签页列表
    pub async fn get_tabs_list(&self) -> Result<Vec<TabInfo>, String> {
        let tabs = self.storage.load_tabs()?;

        {
            let mut memory_tabs = self.tabs.write().await;
            *memory_tabs = tabs.clone();
        }

        let mut tab_list: Vec<TabInfo> = tabs.values().cloned().collect();
        tab_list.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(tab_list)
    }

    /// 添加标签页
    pub async fn add_tab(&self, connection_id: String, title: String) -> Result<String, String> {
        let existing_tab = {
            let tabs = self.storage.load_tabs()?;
            tabs.values()
                .find(|t| t.connection_id == connection_id)
                .cloned()
        };

        if let Some(existing) = existing_tab {
            self.set_active_tab(&existing.id).await?;
            return Ok(existing.id);
        }

        let tab = TabInfo::new(connection_id, title);
        let tab_id = tab.id.clone();

        self.storage.add_tab(tab.clone())?;

        {
            let mut tabs = self.tabs.write().await;
            tabs.insert(tab_id.clone(), tab);

            for tab in tabs.values_mut() {
                tab.active = false;
            }

            if let Some(tab) = tabs.get_mut(&tab_id) {
                tab.active = true;
            }
        }

        log::info!("添加标签页成功: {}", tab_id);
        Ok(tab_id)
    }

    /// 删除标签页
    pub async fn remove_tab(&self, tab_id: &str) -> Result<(), String> {
        self.storage.remove_tab(tab_id)?;

        let mut tabs = self.tabs.write().await;

        if let Some(removed_tab) = tabs.remove(tab_id) {
            if removed_tab.active && !tabs.is_empty() {
                if let Some(next_tab) = tabs.values().next() {
                    let next_tab_id = next_tab.id.clone();
                    drop(tabs);
                    self.storage.set_active_tab(&next_tab_id)?;
                }
            }
            log::info!("删除标签页成功: {}", tab_id);
            Ok(())
        } else {
            Err("标签页不存在".to_string())
        }
    }

    /// 设置活动标签页
    pub async fn set_active_tab(&self, tab_id: &str) -> Result<(), String> {
        self.storage.set_active_tab(tab_id)?;

        let mut tabs = self.tabs.write().await;

        for tab in tabs.values_mut() {
            tab.active = false;
        }

        if let Some(tab) = tabs.get_mut(tab_id) {
            tab.active = true;
            tab.update();
            log::info!("设置活动标签页: {}", tab_id);
            Ok(())
        } else {
            Err("标签页不存在".to_string())
        }
    }

    /// 获取活动标签页
    pub async fn get_active_tab(&self) -> Option<TabInfo> {
        let tabs = self.tabs.read().await;
        tabs.values().find(|t| t.active).cloned()
    }

    /// 关闭所有标签页
    pub async fn close_all_tabs(&self) -> Result<(), String> {
        self.storage.clear_all_tabs()?;

        let mut tabs = self.tabs.write().await;
        tabs.clear();
        log::info!("关闭所有标签页");
        Ok(())
    }

    /// 关闭其他标签页
    pub async fn close_other_tabs(&self, keep_tab_id: &str) -> Result<(), String> {
        let mut tabs = self.tabs.write().await;

        if let Some(keep_tab) = tabs.get(keep_tab_id).cloned() {
            self.storage.clear_all_tabs()?;
            self.storage.add_tab(keep_tab.clone())?;

            tabs.clear();
            tabs.insert(keep_tab_id.to_string(), keep_tab);
            log::info!("关闭其他标签页，保留: {}", keep_tab_id);
            Ok(())
        } else {
            Err("要保留的标签页不存在".to_string())
        }
    }

    /// 根据链接ID获取标签页
    pub async fn get_tab_by_connection_id(&self, connection_id: &str) -> Option<TabInfo> {
        let tabs = self.tabs.read().await;
        tabs.values()
            .find(|t| t.connection_id == connection_id)
            .cloned()
    }
}
