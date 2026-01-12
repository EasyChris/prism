# Logger 模块

## 功能说明

日志与审计模块，负责：

1. **请求日志记录**
   - 记录所有经过代理的请求
   - 包含时间戳、请求路径、HTTP 方法

2. **Token 统计**
   - 从流式响应中解析 Token 数量
   - 分别记录 Input Tokens 和 Output Tokens

3. **路由追踪**
   - 记录请求转发的目标 Provider
   - 记录使用的 Model ID

4. **性能监控**
   - 记录请求耗时
   - 记录 HTTP 状态码

5. **数据持久化**
   - 日志存储到本地数据库（SQLite）
   - 支持日志查询和导出

## 数据结构

```rust
struct RequestLog {
    id: String,
    timestamp: DateTime<Utc>,
    provider: String,
    model: String,
    input_tokens: u32,
    output_tokens: u32,
    duration_ms: u64,
    status_code: u16,
}
```
