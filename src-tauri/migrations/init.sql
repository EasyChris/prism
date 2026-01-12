CREATE TABLE IF NOT EXISTS request_logs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp INTEGER NOT NULL,
    profile_id TEXT NOT NULL,
    profile_name TEXT NOT NULL,
    model TEXT NOT NULL,
    provider TEXT NOT NULL,
    input_tokens INTEGER DEFAULT 0,
    output_tokens INTEGER DEFAULT 0,
    duration_ms INTEGER NOT NULL,
    status_code INTEGER NOT NULL,
    error_message TEXT,
    is_stream INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_timestamp ON request_logs(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_profile_id ON request_logs(profile_id);
CREATE INDEX IF NOT EXISTS idx_status_code ON request_logs(status_code);
