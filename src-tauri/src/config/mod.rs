use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use regex;

/// 全局配置管理器
pub type SharedConfigManager = Arc<RwLock<ConfigManager>>;

/// 模型映射模式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ModelMappingMode {
    /// 透传：使用请求中的原始模型
    Passthrough,
    /// 覆盖：强制使用指定模型
    Override,
    /// 映射：根据规则表映射
    Map,
}

/// 映射规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingRule {
    /// 匹配模式（可以是精确字符串或正则表达式）
    pub pattern: String,
    /// 目标模型
    pub target: String,
    /// 是否使用正则表达式匹配
    #[serde(default)]
    pub use_regex: bool,
}

impl Default for ModelMappingMode {
    fn default() -> Self {
        ModelMappingMode::Passthrough
    }
}

impl ModelMappingMode {
    pub fn as_str(&self) -> &str {
        match self {
            ModelMappingMode::Passthrough => "passthrough",
            ModelMappingMode::Override => "override",
            ModelMappingMode::Map => "map",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "override" => ModelMappingMode::Override,
            "map" => ModelMappingMode::Map,
            _ => ModelMappingMode::Passthrough,
        }
    }
}

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
    /// 是否激活
    pub is_active: bool,

    /// 模型映射模式
    #[serde(default)]
    pub model_mapping_mode: ModelMappingMode,
    /// 覆盖模式使用的目标模型
    #[serde(default)]
    pub override_model: Option<String>,
    /// 映射模式使用的映射规则列表
    #[serde(default)]
    pub model_mappings: Vec<MappingRule>,
}

impl Profile {
    pub fn new(name: String, api_base_url: String, api_key: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            api_base_url,
            api_key,
            is_active: false,
            model_mapping_mode: ModelMappingMode::Passthrough,
            override_model: None,
            model_mappings: Vec::new(),
        }
    }

    /// 解析模型：根据映射模式返回最终使用的模型
    pub fn resolve_model(&self, original_model: &str) -> String {
        match &self.model_mapping_mode {
            ModelMappingMode::Passthrough => {
                // 透传模式：使用原始模型
                original_model.to_string()
            }
            ModelMappingMode::Override => {
                // 覆盖模式：使用配置的目标模型，如果未配置则回退到原始模型
                self.override_model
                    .clone()
                    .unwrap_or_else(|| original_model.to_string())
            }
            ModelMappingMode::Map => {
                // 映射模式：按顺序遍历规则列表
                for rule in &self.model_mappings {
                    if rule.use_regex {
                        // 正则匹配模式
                        if let Ok(re) = regex::Regex::new(&rule.pattern) {
                            if re.is_match(original_model) {
                                return rule.target.clone();
                            }
                        } else {
                            log::warn!("Invalid regex pattern: {}", rule.pattern);
                        }
                    } else {
                        // 精确匹配模式
                        if rule.pattern == original_model {
                            return rule.target.clone();
                        }
                    }
                }
                // 未找到匹配规则，回退到原始模型
                original_model.to_string()
            }
        }
    }
}

/// 配置管理器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigManager {
    /// 所有配置档案
    profiles: HashMap<String, Profile>,
    /// 代理服务 API Key
    #[serde(default)]
    pub proxy_api_key: Option<String>,
    /// 是否启用访问授权
    #[serde(default)]
    pub enable_auth: bool,
}

impl ConfigManager {
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
            proxy_api_key: None,
            enable_auth: false,
        }
    }

    /// 生成新的 API Key (格式: sk-{32位十六进制})
    pub fn generate_api_key() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        let hex_string: String = random_bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();
        format!("sk-{}", hex_string)
    }

    /// 刷新 API Key
    pub fn refresh_api_key(&mut self) -> String {
        let new_key = Self::generate_api_key();
        self.proxy_api_key = Some(new_key.clone());
        new_key
    }

    /// 获取当前 API Key
    pub fn get_api_key(&self) -> Option<&String> {
        self.proxy_api_key.as_ref()
    }

    /// 设置访问授权开关
    pub fn set_auth_enabled(&mut self, enabled: bool) {
        self.enable_auth = enabled;
    }

    /// 检查是否启用访问授权
    pub fn is_auth_enabled(&self) -> bool {
        self.enable_auth
    }

    /// 验证 API Key
    pub fn verify_api_key(&self, key: &str) -> bool {
        if !self.enable_auth {
            return true; // 未启用授权时，所有请求都通过
        }

        match &self.proxy_api_key {
            Some(stored_key) => stored_key == key,
            None => false,
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

    /// 获取所有配置及其 HashMap key
    pub fn get_profiles_with_keys(&self) -> Vec<(String, &Profile)> {
        self.profiles.iter()
            .map(|(k, v)| (k.clone(), v))
            .collect()
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

    /// 从数据库加载配置
    pub async fn load_from_db() -> Result<Self, String> {
        let profiles = crate::db::load_profiles_from_db().await?;

        let mut profiles_map = HashMap::new();
        for profile in profiles {
            profiles_map.insert(profile.id.clone(), profile);
        }

        let proxy_api_key = crate::db::load_app_config("proxy_api_key").await?;
        let enable_auth = crate::db::load_app_config("enable_auth")
            .await?
            .map(|v| v == "true")
            .unwrap_or(false);

        Ok(Self {
            profiles: profiles_map,
            proxy_api_key,
            enable_auth,
        })
    }

    /// 保存所有配置到数据库
    pub async fn save_to_db(&self) -> Result<(), String> {
        // 保存所有 profiles
        for profile in self.profiles.values() {
            crate::db::save_profile_to_db(profile).await?;
        }

        // 保存应用配置
        if let Some(key) = &self.proxy_api_key {
            crate::db::save_app_config("proxy_api_key", key).await?;
        }
        crate::db::save_app_config("enable_auth", &self.enable_auth.to_string()).await?;

        Ok(())
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
