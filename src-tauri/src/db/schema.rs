// 数据库表结构和初始化

use std::path::PathBuf;

/// 获取数据库文件路径
pub fn get_db_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("com.prism.app");
    std::fs::create_dir_all(&path).ok();
    path.push("logs.db");
    path
}

/// 初始化数据库
pub async fn init_database() -> Result<(), String> {
    let db_path = get_db_path();
    log::info!("Initializing database at: {:?}", db_path);

    // 使用 rusqlite 直接操作数据库
    let conn = rusqlite::Connection::open(&db_path)
        .map_err(|e| format!("Failed to open database: {}", e))?;

    // 创建日志表（如果不存在）
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS request_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            request_id TEXT NOT NULL UNIQUE,
            timestamp INTEGER NOT NULL,

            -- Profile 信息
            profile_id TEXT NOT NULL,
            profile_name TEXT NOT NULL,
            provider TEXT NOT NULL,

            -- 模型信息
            original_model TEXT NOT NULL,
            model_mode TEXT NOT NULL,
            forwarded_model TEXT NOT NULL,

            -- Token 统计
            input_tokens INTEGER NOT NULL,
            output_tokens INTEGER NOT NULL,

            -- 缓存相关统计
            cache_creation_input_tokens INTEGER NOT NULL DEFAULT 0,
            cache_read_input_tokens INTEGER NOT NULL DEFAULT 0,

            -- 性能指标
            duration_ms INTEGER NOT NULL,
            upstream_duration_ms INTEGER,

            -- 请求/响应信息
            status_code INTEGER NOT NULL,
            error_message TEXT,
            is_stream INTEGER NOT NULL,
            request_size_bytes INTEGER,
            response_size_bytes INTEGER,
            response_body TEXT
        )
        "#,
        [],
    )
    .map_err(|e| format!("Failed to create table: {}", e))?;

    // 迁移：添加 response_body 字段（如果不存在）
    // 检查字段是否存在
    let column_exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('request_logs') WHERE name='response_body'",
            [],
            |row| row.get::<_, i32>(0).map(|count| count > 0),
        )
        .unwrap_or(false);

    if !column_exists {
        log::info!("Adding response_body column to request_logs table");
        conn.execute(
            "ALTER TABLE request_logs ADD COLUMN response_body TEXT",
            [],
        )
        .map_err(|e| format!("Failed to add response_body column: {}", e))?;
        log::info!("Successfully added response_body column");
    }

    // 迁移：为 request_id 添加 UNIQUE 约束（如果不存在）
    // 检查是否已经有 UNIQUE 约束
    let has_unique_constraint: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name='sqlite_autoindex_request_logs_1'",
            [],
            |row| row.get::<_, i32>(0).map(|count| count > 0),
        )
        .unwrap_or(false);

    if !has_unique_constraint {
        log::info!("Adding UNIQUE constraint to request_id and cleaning up duplicates");

        // 先清理重复记录，保留每个 request_id 的第一条记录
        conn.execute(
            r#"
            DELETE FROM request_logs
            WHERE id NOT IN (
                SELECT MIN(id)
                FROM request_logs
                GROUP BY request_id
            )
            "#,
            [],
        )
        .map_err(|e| format!("Failed to clean up duplicate records: {}", e))?;

        // 创建新表结构（带 UNIQUE 约束）
        conn.execute(
            r#"
            CREATE TABLE request_logs_new (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                request_id TEXT NOT NULL UNIQUE,
                timestamp INTEGER NOT NULL,
                profile_id TEXT NOT NULL,
                profile_name TEXT NOT NULL,
                provider TEXT NOT NULL,
                original_model TEXT NOT NULL,
                model_mode TEXT NOT NULL,
                forwarded_model TEXT NOT NULL,
                input_tokens INTEGER NOT NULL,
                output_tokens INTEGER NOT NULL,
                cache_creation_input_tokens INTEGER NOT NULL DEFAULT 0,
                cache_read_input_tokens INTEGER NOT NULL DEFAULT 0,
                duration_ms INTEGER NOT NULL,
                upstream_duration_ms INTEGER,
                status_code INTEGER NOT NULL,
                error_message TEXT,
                is_stream INTEGER NOT NULL,
                request_size_bytes INTEGER,
                response_size_bytes INTEGER,
                response_body TEXT
            )
            "#,
            [],
        )
        .map_err(|e| format!("Failed to create new table: {}", e))?;

        // 复制数据到新表
        conn.execute(
            r#"
            INSERT INTO request_logs_new
            SELECT * FROM request_logs
            "#,
            [],
        )
        .map_err(|e| format!("Failed to copy data to new table: {}", e))?;

        // 删除旧表
        conn.execute("DROP TABLE request_logs", [])
            .map_err(|e| format!("Failed to drop old table: {}", e))?;

        // 重命名新表
        conn.execute("ALTER TABLE request_logs_new RENAME TO request_logs", [])
            .map_err(|e| format!("Failed to rename new table: {}", e))?;

        log::info!("Successfully added UNIQUE constraint and cleaned up duplicates");
    }

    // 创建索引以提高查询性能
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_timestamp ON request_logs(timestamp DESC)",
        [],
    )
    .map_err(|e| format!("Failed to create index: {}", e))?;

    // 创建配置表（Profiles）
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS profiles (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            api_base_url TEXT NOT NULL,
            api_key TEXT NOT NULL,
            is_active INTEGER NOT NULL DEFAULT 0,
            model_mapping_mode TEXT NOT NULL DEFAULT 'passthrough',
            override_model TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )
        "#,
        [],
    )
    .map_err(|e| format!("Failed to create profiles table: {}", e))?;

    // 创建模型映射规则表
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS model_mappings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            profile_id TEXT NOT NULL,
            pattern TEXT NOT NULL,
            target TEXT NOT NULL,
            use_regex INTEGER NOT NULL DEFAULT 0,
            rule_order INTEGER NOT NULL DEFAULT 0,
            FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
        )
        "#,
        [],
    )
    .map_err(|e| format!("Failed to create model_mappings table: {}", e))?;

    // 创建应用配置表（存储全局配置）
    conn.execute(
        r#"
        CREATE TABLE IF NOT EXISTS app_config (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at INTEGER NOT NULL
        )
        "#,
        [],
    )
    .map_err(|e| format!("Failed to create app_config table: {}", e))?;

    log::info!("Database initialized successfully");
    Ok(())
}
