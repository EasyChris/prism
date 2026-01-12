# Proxy 模块

## 功能说明

动态代理服务模块，负责：

1. **本地 HTTP 服务监听**
   - 在本地端口启动 HTTP 服务
   - 伪装成 Anthropic API 端点

2. **请求转发**
   - 接收来自 Claude Code 的请求
   - 根据当前激活的配置档案转发到目标 API

3. **流式响应处理**
   - 支持 SSE (Server-Sent Events) 协议
   - 实时透传流式响应

4. **错误处理**
   - 捕获上游 API 错误
   - 返回符合 Claude Code 格式的错误信息

## 技术栈

- Axum/Actix-web: HTTP 服务器框架
- Reqwest: HTTP 客户端
- Tokio: 异步运行时
