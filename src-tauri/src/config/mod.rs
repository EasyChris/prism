use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// 全局配置管理器
pub type SharedConfigManager = Arc<RwLock<ConfigManager>>;

/// API 配置档案
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// 配置 ID
    pub id: String,
    /// 配置名称
    pub name: String,
    /// API Base URL
    pub api_base_url: String,
    /// API Key (加密存储)
    pub api_key: String,
    /// Model ID
    pub model_id: String,
    /// 是否激活
    pub is_active: bool,
}

impl Profile {
    pub fn new(name: String, api_base_url: String, api_key: String, model_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            api_base_url,
            api_key,
            model_id,
            is_active: false,
        }
    }
}

/// 配置管理器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigManager {
    /// 所有配置档案
    profiles: HashMap<String, Profile>,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }

    /// 创建新配置
    pub fn create_profile(&mut self, profile: Profile) -> Result<String, String> {
        let id = profile.id.clone();
        if self.profiles.contains_key(&id) {
            return Err("Profile already exists".to_string());
        }
        self.profiles.insert(id.clone(), profile);
        Ok(id)
    }

    /// 获取配置
    pub fn get_profile(&self, id: &str) -> Option<&Profile> {
        self.profiles.get(id)
    }

    /// 更新配置
    pub fn update_profile(&mut self, id: &str, profile: Profile) -> Result<(), String> {
        if !self.profiles.contains_key(id) {
            return Err("Profile not found".to_string());
        }
        self.profiles.insert(id.to_string(), profile);
        Ok(())
    }

    /// 删除配置
    pub fn delete_profile(&mut self, id: &str) -> Result<(), String> {
        if self.profiles.remove(id).is_none() {
            return Err("Profile not found".to_string());
        }
        Ok(())
    }

    /// 获取所有配置
    pub fn list_profiles(&self) -> Vec<&Profile> {
        self.profiles.values().collect()
    }

    /// 保存配置到文件
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), String> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;

        fs::write(path, json)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }

    /// 从文件加载配置
    pub fn load_from_file(path: &PathBuf) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let config: ConfigManager = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))?;

        Ok(config)
    }

    /// 激活配置（确保只有一个配置处于激活状态）
    pub fn activate_profile(&mut self, id: &str) -> Result<(), String> {
        if !self.profiles.contains_key(id) {
            return Err("Profile not found".to_string());
        }

        // 先将所有配置设为非激活
        for profile in self.profiles.values_mut() {
            profile.is_active = false;
        }

        // 激活指定配置
        if let Some(profile) = self.profiles.get_mut(id) {
            profile.is_active = true;
        }

        Ok(())
    }

    /// 获取当前激活的配置
    pub fn get_active_profile(&self) -> Option<&Profile> {
        self.profiles.values().find(|p| p.is_active)
    }
}

/// 获取配置文件路径
pub fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let config_dir = PathBuf::from(home).join(".claude-proxy");

    // 确保目录存在
    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);
    }

    config_dir.join("config.json")
}
