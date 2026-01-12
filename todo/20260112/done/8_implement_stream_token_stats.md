# 实现流式响应的 Token 统计

**创建时间**: 2026-01-12
**状态**: completed

## 任务目标

解决流式响应 Token 统计为 0 的问题，实现完整的 Token 统计功能。

## 背景

当前流式响应虽然记录了日志，但 Token 统计始终为 0。Claude API 的流式响应会在最后一个 SSE 事件中发送 usage 信息，需要拦截并解析这些事件。

## 子任务

### 1. 分析流式响应格式
- [ ] 研究 Claude API 流式响应的 SSE 格式
- [ ] 确认 usage 信息在哪个事件中
- [ ] 了解 message_delta 事件的结构

### 2. 实现流式响应拦截
- [ ] 修改 handle_stream_response 函数
- [ ] 拦截并解析每个 SSE 事件
- [ ] 提取 usage 信息（input_tokens/output_tokens）

### 3. 更新日志记录
- [ ] 在流结束后更新日志的 Token 信息
- [ ] 实现日志更新函数（update_log）
- [ ] 确保 Token 统计准确

### 4. 测试验证
- [ ] 测试流式响应的 Token 统计
- [ ] 验证非流式响应仍然正常工作
- [ ] 确认前端显示正确的 Token 数据

## 技术要点

### Claude API 流式响应格式

流式响应使用 Server-Sent Events (SSE) 格式：

```
data: {"type":"message_start","message":{"id":"msg_xxx",...}}

data: {"type":"content_block_start","index":0,...}

data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"Hello"}}

data: {"type":"message_delta","delta":{"stop_reason":"end_turn"},"usage":{"output_tokens":200}}

data: [DONE]
```

关键事件：
- `message_delta`: 包含 `usage` 信息
- `usage.input_tokens`: 输入 Token 数（可能在 message_start 中）
- `usage.output_tokens`: 输出 Token 数（在 message_delta 中）

### 实现方案

**方案 1：拦截并缓存流数据**
- 读取整个流并缓存
- 解析所有 SSE 事件
- 提取 usage 信息
- 将缓存的数据转发给客户端

**方案 2：使用 tee 模式**
- 同时将流数据发送给客户端和解析器
- 在后台解析 usage 信息
- 流结束后更新日志

**推荐方案 1**：更可靠，确保能获取完整的 usage 信息

## 验收标准

- [ ] 流式响应的 Token 统计不再为 0
- [ ] input_tokens 和 output_tokens 都能正确统计
- [ ] 非流式响应的 Token 统计不受影响
- [ ] 前端日志页面显示正确的 Token 数据
- [ ] 性能影响可接受（延迟 < 100ms）

## 参考资料

- [Claude API Streaming 文档](https://docs.anthropic.com/en/api/messages-streaming)
- [Server-Sent Events 规范](https://html.spec.whatwg.org/multipage/server-sent-events.html)
