// 数据库模块：SQLite 日志存储

mod schema;
mod logs;
mod stats;
mod config;

// 重新导出公共 API
pub use schema::{get_db_path, init_database};
pub use logs::{save_log_to_db, update_log_to_db, get_logs_from_db};
pub use stats::{DashboardStats, TokenDataPoint, get_dashboard_stats, get_token_stats};
pub use config::{
    save_profile_to_db, load_profiles_from_db, delete_profile_from_db,
    save_app_config, load_app_config
};
