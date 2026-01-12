# 代理转发测试指南

## 测试步骤

### 1. 启动 Tauri 应用（包含代理服务器）

在终端中运行：

```bash
pnpm tauri dev
```

这会启动：
- Rust 后端代理服务器（监听 127.0.0.1:3000）
- 前端开发服务器
- 打开 macOS 原生窗口

**等待编译完成**，看到类似以下输出：
```
Proxy server started on 127.0.0.1:3000
```

---

### 2. 运行测试脚本

打开**新的终端窗口**，运行以下任一测试方法：

#### 方法 1: 使用 Node.js 测试脚本（推荐）

```bash
node test-proxy.js
```

#### 方法 2: 使用 Bash 测试脚本

```bash
./test-proxy.sh
```

#### 方法 3: 使用 curl 手动测试

```bash
curl -X POST http://127.0.0.1:3000/v1/messages \
  -H "Content-Type: application/json" \
  -H "anthropic-version: 2023-06-01" \
  -d '{
    "model": "claude-3-5-sonnet-20241022",
    "max_tokens": 100,
    "messages": [
      {
        "role": "user",
        "content": "Hello, this is a test"
      }
    ]
  }'
```

---

## 预期结果

### 成功的情况

✅ HTTP 状态码: 200
✅ 返回 JSON 格式的响应
✅ 包含 Claude 的回复内容

### 可能的错误

❌ **连接被拒绝**: 代理服务器未启动，请先运行 `pnpm tauri dev`
❌ **503 错误**: 没有激活的配置，请在 UI 中添加并激活配置
❌ **401/403 错误**: API Key 无效，请检查配置中的 API Key

---

## 测试流程说明

1. **请求发送**: 测试脚本 → 本地代理 (127.0.0.1:3000)
2. **代理转发**: 本地代理 → 上游 API (配置中的 API Base URL)
3. **响应返回**: 上游 API → 本地代理 → 测试脚本

---

## 调试技巧

### 查看代理日志

在运行 `pnpm tauri dev` 的终端中，你可以看到：
- 接收到的请求
- 转发的目标地址
- 响应状态码

### 修改测试配置

编辑 `test-proxy.js` 文件，可以修改：
- 请求的模型
- 消息内容
- max_tokens 等参数
