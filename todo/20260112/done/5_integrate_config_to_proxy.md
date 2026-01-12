Date: 20260112

----

## 任务目标

将配置管理集成到代理服务，实现从配置中读取上游 API 地址和 API Key

----

## 任务拆分

- [x] 创建全局配置管理器实例
- [x] 在应用启动时加载配置文件
- [x] 修改代理服务使用配置中的上游 API
- [x] 添加默认配置（如果配置文件不存在）
- [x] 测试配置集成功能

---

## CHANGE

修改:
- src-tauri/src/lib.rs: 在应用启动时加载配置，创建并激活默认 Profile，保存配置到文件
- src-tauri/src/proxy/mod.rs: 修改 start_proxy_server 接受 SharedConfigManager 参数，handle_messages 从配置读取上游 API 和 API Key

新增:
- src-tauri/src/config/mod.rs: 添加 SharedConfigManager 类型定义
- ~/.claude-proxy/config.json: 自动生成的配置文件

删除:
- src-tauri/src/proxy/mod.rs: 移除硬编码的上游 API 地址和未使用的导入

## NOTE

实现细节：
1. 配置加载逻辑（lib.rs）：
   - 检查配置文件是否存在（~/.claude-proxy/config.json）
   - 存在则加载，失败时使用空配置
   - 不存在则创建默认 Profile 并激活，保存到文件

2. 代理服务集成（proxy/mod.rs）：
   - 使用 Axum State 传递 SharedConfigManager
   - handle_messages 从配置读取激活的 Profile
   - 使用 Profile 的 api_base_url 构建上游 URL
   - 将 Profile 的 api_key 添加到请求头（x-api-key）
   - 无激活配置时返回 503 SERVICE_UNAVAILABLE

3. 测试结果：
   - ✅ 应用启动成功
   - ✅ 默认配置自动创建并激活
   - ✅ 配置文件成功保存到 ~/.claude-proxy/config.json
   - ✅ 代理服务器成功启动在 127.0.0.1:3000
   - ✅ 编译无错误，仅清理了未使用的导入
