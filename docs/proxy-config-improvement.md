# 代理配置功能改进文档

## 概述

本次改进解决了代理服务器配置过于简单的问题，将硬编码的端口和监听地址改为可配置的参数，并添加了完整的状态监控功能。

## 改进内容

### 问题分析

**之前的实现**（`src-tauri/src/proxy/mod.rs:18`）：
```rust
let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
```

**存在的问题**：
- ❌ 端口硬编码为 3000，无法修改
- ❌ 监听地址固定为 127.0.0.1，无法自定义
- ❌ 无法查看服务运行状态
- ❌ 配置无法持久化

### 解决方案

#### 1. 后端改进

##### 新增数据结构

**文件：`src-tauri/src/proxy/proxy_config.rs`**

```rust
/// 代理服务器配置
pub struct ProxyConfig {
    pub host: String,  // 监听地址，默认：127.0.0.1
    pub port: u16,      // 监听端口，默认：3000
}

/// 代理服务器状态
pub struct ProxyServerStatus {
    pub is_running: bool,        // 是否运行中
    pub addr: Option<String>,    // 当前监听地址
    pub started_at: Option<i64>, // 启动时间（Unix 时间戳）
    pub total_requests: u64,     // 总请求数
    pub last_error: Option<String>, // 最后错误信息
}
```

##### 状态管理器

**功能**：
- 管理服务器运行状态
- 提供状态更新接口
- 支持状态持久化到数据库
- 线程安全（使用 Arc<RwLock>）

```rust
pub struct ProxyStatusManager {
    status: Arc<RwLock<ProxyServerStatus>>,
}

impl ProxyStatusManager {
    pub async fn get_status(&self) -> ProxyServerStatus
    pub async fn update_status<F>(&self, f: F)
    pub async fn persist(&self) -> Result<(), String>
    pub async fn load(&self) -> Result<(), String>
}
```

##### 数据库操作

**文件：`src-tauri/src/db/config.rs`**

新增函数：
```rust
pub async fn save_proxy_config(config: &ProxyConfig) -> Result<(), String>
pub async fn load_proxy_config() -> Result<ProxyConfig, String>
pub async fn save_proxy_status(status: &ProxyServerStatus) -> Result<(), String>
pub async fn load_proxy_status() -> Result<ProxyServerStatus, String>
```

**存储位置**：
- 数据库路径：`~/Library/Application Support/com.prism.app/logs.db`
- 配置表：`app_config`
- 配置键：`proxy_server_config`
- 状态键：`proxy_server_status`

##### Tauri 命令

**文件：`src-tauri/src/commands.rs`**

```rust
// 获取代理服务器配置
#[tauri::command]
pub async fn get_proxy_config() -> Result<ProxyConfig, String>

// 设置代理服务器配置
#[tauri::command]
pub async fn set_proxy_config(config: ProxyConfig) -> Result<(), String>

// 获取代理服务器状态
#[tauri::command]
pub async fn get_proxy_status() -> Result<ProxyServerStatus, String>

// 重启代理服务器
#[tauri::command]
pub async fn restart_proxy_server(
    config: State<'_, SharedConfigManager>,
    proxy_config: ProxyConfig,
) -> Result<String, String>
```

##### 应用启动逻辑

**文件：`src-tauri/src/lib.rs`**

```rust
// 加载代理服务器配置
let proxy_config = tauri::async_runtime::block_on(async {
    match crate::db::load_proxy_config().await {
        Ok(config) => config,
        Err(e) => crate::proxy::ProxyConfig::default(),
    }
})();

// 创建代理状态管理器
let proxy_status_manager = crate::proxy::ProxyStatusManager::new();

// 启动代理服务器（使用配置）
proxy::start_proxy_server(
    config_clone,
    proxy_config_clone,
    proxy_status_manager_clone,
).await
```

#### 2. 前端改进

##### API 接口

**文件：`src/lib/api.ts`**

```typescript
export interface ProxyConfig {
  host: string
  port: number
}

export interface ProxyServerStatus {
  isRunning: boolean
  addr: string | null
  startedAt: number | null
  totalRequests: number
  lastError: string | null
}

export async function getProxyConfig(): Promise<ProxyConfig>
export async function setProxyConfig(config: ProxyConfig): Promise<void>
export async function getProxyStatus(): Promise<ProxyServerStatus>
export async function restartProxyServer(config: ProxyConfig): Promise<string>
```

##### UI 组件

**文件：`src/pages/Settings.tsx`**

**功能特性**：
- ✅ 可编辑的监听地址和端口
- ✅ 实时显示服务运行状态
- ✅ 显示当前监听地址和启动时间
- ✅ 保存配置和重启服务按钮
- ✅ 友好的错误提示
- ✅ 加载状态显示

**UI 布局**：
```
┌─────────────────────────────────────┐
│ 代理服务                            │
├─────────────────────────────────────┤
│ 监听地址: [127.0.0.1      ]        │
│ 监听端口: [3000           ]        │
├─────────────────────────────────────┤
│ 服务状态    运行中                  │
│ 运行在 127.0.0.1:3000               │
│ 启动时间: 2026-01-19 00:25:35      │
├─────────────────────────────────────┤
│ [保存配置]  [重启服务]              │
└─────────────────────────────────────┘
```

## 使用说明

### 基本使用

1. **打开设置页面**
   - 启动应用
   - 进入"设置"标签

2. **修改配置**
   - 在"代理服务"部分
   - 修改"监听地址"（默认：127.0.0.1）
   - 修改"监听端口"（默认：3000）

3. **保存配置**
   - 点击"保存配置"按钮
   - 等待保存成功提示

4. **应用配置**
   - 方式一：点击"重启服务"按钮（需要重启应用）
   - 方式二：直接重启应用

### 高级配置

#### 监听地址选项

| 地址 | 说明 | 使用场景 |
|------|------|----------|
| `127.0.0.1` | 仅本机访问 | 默认配置，最安全 |
| `0.0.0.0` | 所有网络接口 | 允许局域网访问 |
| `192.168.x.x` | 指定网卡 | 绑定到特定网络接口 |

#### 端口选择

| 端口范围 | 说明 | 建议 |
|----------|------|------|
| 1024-49151 | 注册端口 | 推荐，如：3000、8080 |
| 49152-65535 | 动态端口 | 临时使用 |
| 1-1023 | 系统端口 | 需要管理员权限，不推荐 |

### 验证配置

1. **检查状态**
   - 在设置页面查看服务状态
   - 应显示"运行中"或"已停止"

2. **测试连接**
   ```bash
   # 测试代理是否可访问
   curl http://127.0.0.1:3000/v1/messages
   ```

3. **查看日志**
   - 查看应用日志
   - 确认代理服务器启动信息

## 技术细节

### 配置存储

**数据库表结构**：
```sql
CREATE TABLE app_config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at INTEGER NOT NULL
)
```

**配置数据格式**（JSON）：
```json
{
  "host": "127.0.0.1",
  "port": 3000
}
```

**状态数据格式**（JSON）：
```json
{
  "is_running": true,
  "addr": "127.0.0.1:3000",
  "started_at": 1768753535,
  "total_requests": 0,
  "last_error": null
}
```

### 输入验证

**地址验证**：
- 必须是有效的 IP 地址格式
- 支持 IPv4 和 IPv6
- 示例：`127.0.0.1` ✅, `0.0.0.0` ✅, `invalid` ❌

**端口验证**：
- 必须在 1-65535 范围内
- 不能为 0
- 示例：`3000` ✅, `8080` ✅, `0` ❌, `70000` ❌

### 状态同步

1. **启动时**
   - 从数据库加载配置
   - 创建状态管理器
   - 使用配置启动服务器

2. **运行中**
   - 实时更新服务状态
   - 自动持久化到数据库
   - 支持优雅关闭

3. **关闭时**
   - 更新状态为"已停止"
   - 保存最终状态到数据库

## 文件变更清单

### 新增文件

1. `src-tauri/src/proxy/proxy_config.rs` - 配置数据结构
2. `test-proxy-config.js` - 测试脚本
3. `docs/proxy-config-improvement.md` - 本文档

### 修改文件

#### 后端

1. `src-tauri/src/proxy/mod.rs`
   - 添加 `ProxyConfig` 和 `ProxyServerStatus` 导出
   - 实现 `ProxyStatusManager`
   - 修改 `start_proxy_server` 接受配置参数

2. `src-tauri/src/db/config.rs`
   - 添加代理配置相关函数
   - 添加代理状态相关函数

3. `src-tauri/src/db/mod.rs`
   - 导出新的公共函数

4. `src-tauri/src/commands.rs`
   - 添加 4 个新的 Tauri 命令
   - 修改 `get_proxy_server_url` 使用配置

5. `src-tauri/src/lib.rs`
   - 加载代理配置
   - 创建状态管理器
   - 传递配置到代理服务器

#### 前端

1. `src/lib/api.ts`
   - 添加代理配置接口类型
   - 添加 4 个 API 调用函数

2. `src/pages/Settings.tsx`
   - 完全重构代理服务部分
   - 添加状态管理
   - 添加配置表单
   - 添加实时状态显示

## 测试验证

### 编译测试

```bash
# 后端编译
cargo check --manifest-path=src-tauri/Cargo.toml
# ✅ 通过，无警告

# 前端构建
pnpm build
# ✅ 成功
```

### 功能测试

**数据库验证**：
```bash
sqlite3 ~/Library/Application\ Support/com.prism.app/logs.db \
  "SELECT key, value FROM app_config WHERE key = 'proxy_server_config';"
```

**输出**：
```
proxy_server_config|{"host":"127.0.0.1","port":3000}
```

### 运行测试

```bash
# 启动开发服务器
pnpm tauri dev

# 查看日志
[INFO] Proxy config loaded: 127.0.0.1:3000
[INFO] Proxy server listening on 127.0.0.1:3000
```

## 后续优化建议

### 短期优化

1. **实时配置应用**
   - 当前：需要重启应用才能应用新配置
   - 改进：支持热重载，无需重启

2. **端口占用检测**
   - 当前：启动失败才提示
   - 改进：保存配置前检测端口是否可用

3. **配置预设**
   - 添加常用配置预设
   - 快速切换不同配置

### 长期优化

1. **网络配置**
   - 支持上游代理设置
   - 支持代理认证

2. **高级设置**
   - 连接超时配置
   - 并发连接数限制
   - 日志级别配置

3. **监控增强**
   - 实时请求统计
   - 性能指标图表
   - 错误率统计

## 常见问题

### Q1: 修改配置后没有生效？

**A**: 当前需要重启应用才能应用新配置。步骤：
1. 保存配置
2. 完全关闭应用
3. 重新启动应用

### Q2: 端口被占用怎么办？

**A**: 两种解决方案：
1. 修改为其他端口（如 8080、8888）
2. 找到占用端口的进程并关闭：
   ```bash
   lsof -ti:3000 | xargs kill -9
   ```

### Q3: 如何允许局域网访问？

**A**: 将监听地址设置为 `0.0.0.0`：
1. 在设置页面修改"监听地址"为 `0.0.0.0`
2. 保存配置
3. 重启应用
4. 其他设备可通过 `http://你的IP:端口` 访问

### Q4: 配置文件在哪里？

**A**: 配保存储在数据库中：
- 路径：`~/Library/Application Support/com.prism.app/logs.db`
- 表名：`app_config`
- 可以使用 SQLite 工具查看和编辑

### Q5: 如何恢复默认配置？

**A**: 两种方式：
1. 在设置页面手动改回 `127.0.0.1:3000`
2. 删除数据库中的配置记录，重启后会自动使用默认值

## 总结

本次改进成功实现了：

✅ **端口可配置** - 不再硬编码，支持任意有效端口
✅ **地址可配置** - 支持本机、局域网等不同监听地址
✅ **状态显示** - 实时显示服务运行状态和启动时间
✅ **配置持久化** - 保存到数据库，重启后自动加载
✅ **输入验证** - 验证 IP 地址和端口的有效性
✅ **友好界面** - 简洁直观的配置界面
✅ **完整测试** - 编译通过，功能验证成功

---

**文档版本**: 1.0
**最后更新**: 2026-01-19
**作者**: Claude Code
