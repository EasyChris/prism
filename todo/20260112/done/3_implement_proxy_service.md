Date: 20260112

----

## 任务目标

实现本地 HTTP 代理服务，作为 Claude Code 和上游 API 之间的中转站

----

## 任务拆分

- [x] 添加 Axum 和相关依赖到 Cargo.toml
- [x] 实现基础 HTTP 服务器（监听本地端口）
- [x] 实现请求转发逻辑（转发到上游 API）
- [x] 实现 SSE 流式响应透传
- [x] 添加基本的错误处理
- [x] 测试代理服务功能

---

## CHANGE

修改:
- src-tauri/Cargo.toml - 添加 Axum、Tokio、Reqwest 等依赖
- src-tauri/src/lib.rs - 在应用启动时启动代理服务器

新增:
- src-tauri/src/proxy/mod.rs - 代理服务核心实现
  - start_proxy_server() - 启动 HTTP 服务器
  - handle_messages() - 处理 /v1/messages 请求
  - handle_stream_response() - 处理流式响应
  - convert_headers() - 转换 HTTP headers

删除:

## NOTE

P1 阶段核心功能已完成：
- ✅ 使用 Axum 0.7 框架搭建 HTTP 服务器
- ✅ 监听本地端口 127.0.0.1:3000
- ✅ 实现 /v1/messages 端点（伪装成 Anthropic API）
- ✅ 支持流式响应（SSE）透传
- ✅ 基本的错误处理（BAD_GATEWAY、INTERNAL_SERVER_ERROR）

技术实现：
- 使用 Reqwest 作为 HTTP 客户端转发请求
- 自动检测 stream 参数决定是否使用流式响应
- 流式响应使用 bytes_stream 实现零拷贝透传

编译状态：
- ✅ 编译成功（有 4 个警告，不影响功能）

下一步：
- 实现配置管理模块（从配置中读取上游 API 地址）
- 添加日志记录功能
- 实现前端界面
