# 实现请求日志记录与展示功能

**创建时间**: 2026-01-12
**状态**: completed

## 任务目标

实现完整的请求日志记录和展示功能，包括：
- 引入 SQLite 数据库存储日志
- 记录请求的完整信息（Token、耗时、状态码等）
- 在前端展示日志列表

## 背景

当前代理服务只有控制台日志输出，没有持久化存储。前端日志页面只是静态 UI，显示"暂无日志记录"。需要实现完整的日志审计功能。

## 子任务

### 1. 引入 SQLite 依赖
- [x] 在 Cargo.toml 中添加 tauri-plugin-sql 依赖
- [x] 在 lib.rs 中注册 SQL 插件
- [x] 创建数据库初始化脚本

### 2. 设计数据库表结构
- [x] 设计 request_logs 表结构
  - id: 主键
  - timestamp: 时间戳
  - profile_id/profile_name: 配置信息
  - model/provider: 模型和提供商
  - input_tokens/output_tokens: Token 统计
  - duration_ms: 耗时
  - status_code: HTTP 状态码
  - error_message: 错误信息
  - is_stream: 是否流式请求

### 3. 创建日志记录模块
- [ ] 创建 src-tauri/src/logger/mod.rs
- [ ] 实现 RequestLog 结构体
- [ ] 实现日志插入函数
- [ ] 实现日志查询函数（支持分页、筛选）

### 4. 集成到代理服务
- [ ] 在 handle_messages 中记录请求开始时间
- [ ] 解析响应中的 Token 信息
- [ ] 计算请求耗时
- [ ] 调用日志记录函数

### 5. 实现 Tauri Commands
- [ ] get_logs: 获取日志列表
- [ ] get_log_stats: 获取统计信息
- [ ] clear_logs: 清空日志（可选）

### 6. 前端集成
- [ ] 在 api.ts 中添加日志相关 API
- [ ] 更新 Logs.tsx 展示真实数据
- [ ] 实现分页和搜索功能
- [ ] 添加日志导出功能（可选）

## 技术要点

### Token 解析
Claude API 响应格式：
```json
{
  "usage": {
    "input_tokens": 100,
    "output_tokens": 200
  }
}
```

### 流式响应处理
流式响应的最后一个事件包含 usage 信息：
```
data: {"type":"message_delta","usage":{"output_tokens":200}}
```

### 数据库路径
使用 Tauri 的 app data 目录：
```rust
let app_data_dir = app.path().app_data_dir()?;
let db_path = app_data_dir.join("logs.db");
```

## 验收标准

- [ ] 代理服务成功记录每个请求到数据库
- [ ] 日志包含所有必要字段（Token、耗时、状态码等）
- [ ] 前端日志页面能正确展示日志列表
- [ ] 支持按时间倒序排列
- [ ] Token 统计准确
- [ ] 流式和非流式请求都能正确记录

## 参考资料

- [Tauri SQL Plugin 文档](https://v2.tauri.app/plugin/sql/)
- [Claude API 文档](https://docs.anthropic.com/en/api/messages)
- CLAUDE.md 中的日志与审计需求
