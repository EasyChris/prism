pub mod proxy;
pub mod config;
pub mod commands;
pub mod logger;

use std::sync::{Arc, RwLock};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_sql::Builder::default().build())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // 加载配置
      let config_path = config::get_config_path();
      let config_manager = if config_path.exists() {
        config::ConfigManager::load_from_file(&config_path)
          .unwrap_or_else(|e| {
            log::warn!("Failed to load config: {}, using default", e);
            config::ConfigManager::new()
          })
      } else {
        log::info!("Config file not found, creating default config");
        let mut manager = config::ConfigManager::new();

        // 创建默认配置
        let default_profile = config::Profile::new(
          "Default".to_string(),
          "https://api.anthropic.com".to_string(),
          "".to_string(),
          "claude-3-5-sonnet-20241022".to_string(),
        );

        if let Ok(profile_id) = manager.create_profile(default_profile) {
          // 激活默认配置
          let _ = manager.activate_profile(&profile_id);
          log::info!("Default profile created and activated");
        }

        // 保存配置到文件
        if let Err(e) = manager.save_to_file(&config_path) {
          log::warn!("Failed to save default config: {}", e);
        }

        manager
      };

      let shared_config = Arc::new(RwLock::new(config_manager));

      // 启动代理服务器
      let config_clone = shared_config.clone();
      tauri::async_runtime::spawn(async move {
        if let Err(e) = proxy::start_proxy_server(config_clone).await {
          log::error!("Failed to start proxy server: {}", e);
        }
      });

      // 将配置管理器作为状态管理
      app.manage(shared_config);

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      commands::get_all_profiles,
      commands::create_profile,
      commands::update_profile,
      commands::delete_profile,
      commands::activate_profile,
      commands::get_logs,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
