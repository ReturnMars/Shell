// 连接配置存储服务
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use serde_json;
use crate::models::{ConnectionConfig, TabInfo};
use crate::utils::{storage, crypto};

/// 连接配置存储管理器
pub struct ConnectionStorage {
    config_dir: PathBuf,
    connections_file: PathBuf,
    tabs_file: PathBuf,
    master_key: Vec<u8>,
}

impl ConnectionStorage {
    /// 创建新的存储管理器
    pub fn new() -> Result<Self, String> {
        let config_dir = storage::get_config_dir()?;
        let connections_file = config_dir.join("connections.json");
        let tabs_file = config_dir.join("tabs.json");
        
        // 确保配置目录存在
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .map_err(|e| format!("创建配置目录失败: {}", e))?;
        }

        // 生成或加载主密钥
        let master_key = Self::load_or_generate_master_key(&config_dir)?;

        Ok(Self {
            config_dir,
            connections_file,
            tabs_file,
            master_key,
        })
    }

    /// 在现有连接中查找相同名称的项，返回其id
    fn find_existing_by_name(
        connections: &HashMap<String, ConnectionConfig>,
        candidate_name: &str,
    ) -> Option<String> {
        connections
            .iter()
            .find(|(_, cfg)| cfg.name == candidate_name)
            .map(|(id, _)| id.clone())
    }

    /// 生成唯一的连接名称，如果名称重复则自动添加数字后缀
    fn generate_unique_name(
        connections: &HashMap<String, ConnectionConfig>,
        base_name: &str,
    ) -> String {
        let mut name = base_name.to_string();
        let mut counter = 1;
        
        while Self::find_existing_by_name(connections, &name).is_some() {
            name = format!("{}({})", base_name, counter);
            counter += 1;
        }
        
        name
    }

    /// 保存连接配置（自动处理名称重复，生成唯一名称）
    pub fn save_connection(&self, config: &ConnectionConfig) -> Result<(), String> {
        let mut connections = self.load_connections()?;

        // 生成唯一的连接名称
        let unique_name = Self::generate_unique_name(&connections, &config.name);
        
        // 创建要保存的配置副本
        let mut config_to_save = config.clone();
        config_to_save.name = unique_name;

        // 如果密码存在，加密保存
        if let Some(password) = &config.password {
            let encrypted_password = crypto::encrypt_password(password, &self.master_key)?;
            config_to_save.password = Some(encrypted_password);
        }

        // 直接保存新连接
        connections.insert(config.id.clone(), config_to_save);
        self.save_connections(&connections)
    }

    /// 加载连接配置
    pub fn load_connection(&self, connection_id: &str) -> Result<ConnectionConfig, String> {
        let connections = self.load_connections()?;
        
        if let Some(mut config) = connections.get(connection_id).cloned() {
            // 如果密码存在，解密
            if let Some(encrypted_password) = &config.password {
                let decrypted_password = crypto::decrypt_password(encrypted_password, &self.master_key)?;
                config.password = Some(decrypted_password);
            }
            Ok(config)
        } else {
            Err("连接配置不存在".to_string())
        }
    }

    /// 更新连接配置
    pub fn update_connection(&self, config: &ConnectionConfig) -> Result<(), String> {
        let mut connections = self.load_connections()?;
        
        // 检查连接是否存在
        if !connections.contains_key(&config.id) {
            return Err(format!("连接配置不存在: {}", config.id));
        }
        
        // 创建要更新的配置副本
        let mut config_to_update = config.clone();
        
        // 如果密码存在，加密保存
        if let Some(password) = &config.password {
            let encrypted_password = crypto::encrypt_password(password, &self.master_key)?;
            config_to_update.password = Some(encrypted_password);
        }
        
        // 更新连接
        connections.insert(config.id.clone(), config_to_update);
        self.save_connections(&connections)
    }

    /// 删除连接配置
    pub fn delete_connection(&self, connection_id: &str) -> Result<(), String> {
        let mut connections = self.load_connections()?;
        connections.remove(connection_id);
        self.save_connections(&connections)
    }

    /// 获取所有连接配置（返回前解密密码，按时间倒序排序）
    pub fn get_all_connections(&self) -> Result<Vec<ConnectionConfig>, String> {
        let connections = self.load_connections()?;
        let mut result = Vec::new();
        
        for (_, mut config) in connections {
            // 解密密码
            if let Some(encrypted_password) = &config.password {
                let decrypted_password = crypto::decrypt_password(encrypted_password, &self.master_key)?;
                config.password = Some(decrypted_password);
            }
            result.push(config);
        }
        
        // 按创建时间倒序排序（最新的在前）
        result.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        Ok(result)
    }

    /// 保存所有连接配置
    fn save_connections(&self, connections: &HashMap<String, ConnectionConfig>) -> Result<(), String> {
        let json = serde_json::to_string_pretty(connections)
            .map_err(|e| format!("序列化连接配置失败: {}", e))?;
        
        fs::write(&self.connections_file, json)
            .map_err(|e| format!("写入连接配置文件失败: {}", e))?;
        
        log::info!("连接配置已保存到: {:?}", self.connections_file);
        Ok(())
    }

    /// 加载所有连接配置
    fn load_connections(&self) -> Result<HashMap<String, ConnectionConfig>, String> {
        if !self.connections_file.exists() {
            return Ok(HashMap::new());
        }

        let content = fs::read_to_string(&self.connections_file)
            .map_err(|e| format!("读取连接配置文件失败: {}", e))?;

        if content.trim().is_empty() {
            return Ok(HashMap::new());
        }

        let connections: HashMap<String, ConnectionConfig> = serde_json::from_str(&content)
            .map_err(|e| format!("解析连接配置文件失败: {}", e))?;

        Ok(connections)
    }

    /// 加载或生成主密钥
    fn load_or_generate_master_key(config_dir: &PathBuf) -> Result<Vec<u8>, String> {
        let key_file = config_dir.join("master.key");
        
        if key_file.exists() {
            // 加载现有密钥
            let key_data = fs::read(&key_file)
                .map_err(|e| format!("读取主密钥失败: {}", e))?;
            Ok(key_data)
        } else {
            // 生成新密钥
            let master_key = crypto::generate_master_key();
            fs::write(&key_file, &master_key)
                .map_err(|e| format!("保存主密钥失败: {}", e))?;
            
            log::info!("已生成新的主密钥: {:?}", key_file);
            Ok(master_key)
        }
    }

    /// 导出连接配置（不包含密码）
    pub fn export_connections(&self) -> Result<String, String> {
        let connections = self.load_connections()?;
        let mut export_data = HashMap::new();
        
        for (id, mut config) in connections {
            // 移除密码信息
            config.password = None;
            export_data.insert(id, config);
        }
        
        serde_json::to_string_pretty(&export_data)
            .map_err(|e| format!("导出连接配置失败: {}", e))
    }

    /// 导入连接配置（自动处理名称重复，生成唯一名称）
    pub fn import_connections(&self, json_data: &str) -> Result<(), String> {
        let imported: HashMap<String, ConnectionConfig> = serde_json::from_str(json_data)
            .map_err(|e| format!("解析导入数据失败: {}", e))?;
        
        let mut existing = self.load_connections()?;
        
        for (_id, mut config) in imported {
            // 验证配置
            config.validate()?;
            
            // 生成唯一的连接名称
            let unique_name = Self::generate_unique_name(&existing, &config.name);
            config.name = unique_name;
            
            // 如果密码为空，保持为空；否则加密保存
            if let Some(password) = &config.password {
                let encrypted_password = crypto::encrypt_password(password, &self.master_key)?;
                config.password = Some(encrypted_password);
            }

            existing.insert(config.id.clone(), config);
        }
        
        self.save_connections(&existing)
    }

    /// 删除全部连接配置
    pub fn delete_all(&self) -> Result<(), String> {
        self.save_connections(&HashMap::new())
    }

    // ========== 标签页持久化方法 ==========

    /// 保存标签页列表
    pub fn save_tabs(&self, tabs: &HashMap<String, TabInfo>) -> Result<(), String> {
        let json_data = serde_json::to_string_pretty(tabs)
            .map_err(|e| format!("序列化标签页数据失败: {}", e))?;
        
        fs::write(&self.tabs_file, json_data)
            .map_err(|e| format!("保存标签页文件失败: {}", e))?;
        
        log::info!("标签页数据已保存到: {:?}", self.tabs_file);
        Ok(())
    }

    /// 加载标签页列表
    pub fn load_tabs(&self) -> Result<HashMap<String, TabInfo>, String> {
        if !self.tabs_file.exists() {
            log::info!("标签页文件不存在，返回空列表");
            return Ok(HashMap::new());
        }

        let json_data = fs::read_to_string(&self.tabs_file)
            .map_err(|e| format!("读取标签页文件失败: {}", e))?;

        let tabs: HashMap<String, TabInfo> = serde_json::from_str(&json_data)
            .map_err(|e| format!("解析标签页数据失败: {}", e))?;

        log::info!("成功加载 {} 个标签页", tabs.len());
        Ok(tabs)
    }

    /// 添加标签页
    pub fn add_tab(&self, tab: TabInfo) -> Result<String, String> {
        let mut tabs = self.load_tabs()?;
        
        // 检查是否已存在相同链接的标签页
        let existing_tab = tabs.values().find(|t| t.connection_id == tab.connection_id);
        if let Some(existing) = existing_tab {
            return Err(format!("链接 {} 已存在标签页", existing.title));
        }

        let tab_id = tab.id.clone();
        tabs.insert(tab_id.clone(), tab);
        
        self.save_tabs(&tabs)?;
        log::info!("添加标签页成功: {}", tab_id);
        Ok(tab_id)
    }

    /// 删除标签页
    pub fn remove_tab(&self, tab_id: &str) -> Result<(), String> {
        let mut tabs = self.load_tabs()?;
        
        if tabs.remove(tab_id).is_some() {
            self.save_tabs(&tabs)?;
            log::info!("删除标签页成功: {}", tab_id);
            Ok(())
        } else {
            Err("标签页不存在".to_string())
        }
    }

    /// 更新标签页
    pub fn update_tab(&self, tab: TabInfo) -> Result<(), String> {
        let mut tabs = self.load_tabs()?;
        
        if tabs.contains_key(&tab.id) {
            let tab_id = tab.id.clone();
            tabs.insert(tab_id.clone(), tab);
            self.save_tabs(&tabs)?;
            log::info!("更新标签页成功: {}", tab_id);
            Ok(())
        } else {
            Err("标签页不存在".to_string())
        }
    }

    /// 设置活动标签页
    pub fn set_active_tab(&self, tab_id: &str) -> Result<(), String> {
        let mut tabs = self.load_tabs()?;
        
        // 先取消所有标签页的激活状态
        for tab in tabs.values_mut() {
            tab.active = false;
        }
        
        // 激活指定标签页
        if let Some(tab) = tabs.get_mut(tab_id) {
            tab.active = true;
            tab.update();
            self.save_tabs(&tabs)?;
            log::info!("设置活动标签页: {}", tab_id);
            Ok(())
        } else {
            Err("标签页不存在".to_string())
        }
    }

    /// 清空所有标签页
    pub fn clear_all_tabs(&self) -> Result<(), String> {
        self.save_tabs(&HashMap::new())?;
        log::info!("清空所有标签页");
        Ok(())
    }

    /// 根据链接ID删除相关标签页
    pub fn remove_tabs_by_connection_id(&self, connection_id: &str) -> Result<(), String> {
        let mut tabs = self.load_tabs()?;
        let mut removed_count = 0;
        
        tabs.retain(|_, tab| {
            if tab.connection_id == connection_id {
                removed_count += 1;
                false
            } else {
                true
            }
        });
        
        if removed_count > 0 {
            self.save_tabs(&tabs)?;
            log::info!("删除 {} 个相关标签页", removed_count);
        }
        
        Ok(())
    }
}

impl Default for ConnectionStorage {
    fn default() -> Self {
        Self::new().expect("无法创建连接存储管理器")
    }
}

