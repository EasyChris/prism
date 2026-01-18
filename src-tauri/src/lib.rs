pub mod proxy;
pub mod config;
pub mod commands;
pub mod logger;
pub mod db;
pub mod tray;

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

      // 初始化数据库
      tauri::async_runtime::block_on(async {
        if let Err(e) = db::init_database().await {
          log::error!("Failed to initialize database: {}", e);
        } else {
          log::info!("Database initialized successfully");
        }
      });

      // 加载配置（优先从数据库加载，如果失败则尝试从 JSON 文件迁移）
      let config_manager = tauri::async_runtime::block_on(async {
        // 尝试从数据库加载
        match config::ConfigManager::load_from_db().await {
          Ok(manager) => {
            log::info!("Config loaded from database successfully");
            manager
          }
          Err(e) => {
            log::warn!("Failed to load config from database: {}", e);

            // 尝试从 JSON 文件迁移
            let config_path = config::get_config_path();
            if config_path.exists() {
              log::info!("Attempting to migrate config from JSON file");
              match config::ConfigManager::load_from_file(&config_path) {
                Ok(manager) => {
                  // 迁移到数据库
                  if let Err(e) = manager.save_to_db().await {
                    log::error!("Failed to migrate config to database: {}", e);
                  } else {
                    log::info!("Config migrated to database successfully");
                  }
                  manager
                }
                Err(e) => {
                  log::warn!("Failed to load config from file: {}", e);
                  config::ConfigManager::new()
                }
              }
            } else {
              log::info!("No existing config found, creating default");
              config::ConfigManager::new()
            }
          }
        }
      });

      let shared_config = Arc::new(RwLock::new(config_manager));

      // 启动代理服务器
      let config_clone = shared_config.clone();
      tauri::async_runtime::spawn(async move {
        if let Err(e) = proxy::start_proxy_server(config_clone).await {
          log::error!("Failed to start proxy server: {}", e);
        }
      });

      // 将配置管理器作为状态管理
      app.manage(shared_config.clone());

      // 初始化系统托盘
      if let Err(e) = tray::init_tray(app.handle(), shared_config) {
        log::error!("Failed to initialize tray: {}", e);
      }

      Ok(())
    })
    .on_window_event(|window, event| {
      if let tauri::WindowEvent::CloseRequested { api, .. } = event {
        // 隐藏窗口而不是关闭应用
        window.hide().unwrap();
        api.prevent_close();
      }
    })
    .invoke_handler(tauri::generate_handler![
      commands::get_all_profiles,
      commands::create_profile,
      commands::update_profile,
      commands::delete_profile,
      commands::activate_profile,
      commands::get_logs,
      commands::get_dashboard_stats,
      commands::get_token_stats,
      commands::get_proxy_api_key,
      commands::refresh_proxy_api_key,
      commands::get_auth_enabled,
      commands::set_auth_enabled,
      commands::get_proxy_server_url,
      commands::show_main_window,
      commands::update_tray_menu,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
