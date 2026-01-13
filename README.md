# Claude Code Proxy Hub

<div align="center">

**专为 Claude Code 打造的动态路由网关与流量审计工具**

一个运行在 macOS 上的轻量级桌面应用，作为 Claude Code 的本地中转站

实现"一次配置，随意切换"的无缝体验，并提供完整的流量去向记录与成本审计

[功能特性](#功能特性) • [快速开始](#快速开始) • [使用指南](#使用指南) • [开发文档](#开发文档)

</div>

---

## 项目简介

Claude Code Proxy Hub 是一个专为 Claude Code 设计的本地代理工具，解决以下痛点：

- **多账号管理困难** - 需要频繁修改配置文件切换不同的 API 提供商
- **成本不透明** - 不知道每次请求消耗了多少 Token，花费了多少钱
- **流量去向不明** - 不清楚请求被路由到了哪个模型和提供商
- **审计困难** - 缺乏完整的请求日志和统计数据

### 核心价值

✨ **一次配置，随意切换** - 在菜单栏一键切换不同的 API 配置档案
📊 **完整审计** - 记录每次请求的 Token 消耗、耗时、状态码
🔍 **流量追踪** - 清晰展示每个请求的路由去向（Provider + Model）
🔒 **本地优先** - 所有数据仅在本地处理，API Key 加密存储
⚡ **低延迟** - 毫秒级代理转发，对 Claude Code 使用体验无感知

---

## 功能特性

### 🎯 动态代理服务

- 本地 HTTP 服务伪装成 Anthropic API
- 请求级别的动态路由切换
- SSE 流式响应完美透传
- 智能错误处理与兜底机制

### ⚙️ 配置管理

- 多配置档案（Profile）管理
- 支持字段：名称、API Base URL、API Key、Model ID
- API Key 加密存储，界面脱敏展示
- 单一激活配置，快速切换

### 📝 日志与审计

- 全链路请求日志记录
- Token 统计（Input/Output/Total）
- 路由去向追踪（Provider + Model）
- 请求耗时监控
- HTTP 状态码记录
- 支持日志搜索和过滤

### 🖥️ 用户界面

- macOS 菜单栏常驻
- 配置档案快速切换
- 实时服务状态显示
- 日志面板（表格展示）
- 仪表盘（当前配置详情）
- 统计数据可视化

---

## 技术栈

### 后端
- **Tauri v2** - 桌面应用框架
- **Rust** - 高性能系统编程语言
- **Axum** - 异步 HTTP 服务器框架
- **Tokio** - 异步运行时
- **SQLite** - 本地数据库

### 前端
- **React** - UI 框架
- **TypeScript** - 类型安全
- **Tailwind CSS v4** - 现代化样式框架
- **Vite** - 快速构建工具

---

## 快速开始

### 系统要求

- macOS 10.15 或更高版本
- 已安装 Claude Code CLI

### 安装

#### 方式一：下载预编译版本（推荐）

1. 前往 [Releases](https://github.com/yourusername/prism/releases) 页面
2. 下载最新版本的 `.dmg` 文件
3. 双击安装，拖拽到 Applications 文件夹
4. 首次打开可能需要在"系统偏好设置 > 安全性与隐私"中允许运行

#### 方式二：从源码构建

```bash
# 克隆仓库
git clone https://github.com/yourusername/prism.git
cd prism

# 安装依赖
pnpm install

# 开发模式运行
pnpm tauri dev

# 构建生产版本
pnpm tauri build
```

### 配置 Claude Code

1. 启动 Claude Code Proxy Hub，默认监听 `http://localhost:3000`
2. 配置 Claude Code 使用本地代理：

```bash
# 设置 API Base URL
export ANTHROPIC_API_URL=http://localhost:3000

# 设置一个占位 API Key（实际 Key 在 Proxy Hub 中配置）
export ANTHROPIC_API_KEY=placeholder
```

3. 在 Proxy Hub 中添加你的真实 API 配置档案
4. 选择激活的配置档案
5. 开始使用 Claude Code，所有请求将通过 Proxy Hub 转发

---

## 使用指南

### 添加配置档案

1. 点击菜单栏图标，选择"打开主界面"
2. 切换到"配置管理"标签页
3. 点击"添加配置"按钮
4. 填写配置信息：
   - **名称**：配置档案的显示名称（如 "OpenRouter - Claude"）
   - **API Base URL**：API 提供商的基础 URL
   - **API Key**：你的 API 密钥
   - **Model ID**：要使用的模型 ID
5. 点击"保存"

### 切换配置档案

**方式一：菜单栏快速切换**
- 点击菜单栏图标
- 在配置列表中选择要激活的配置

**方式二：主界面切换**
- 在"配置管理"页面
- 点击配置卡片上的"激活"按钮

### 查看日志

1. 切换到"日志"标签页
2. 查看所有请求记录，包括：
   - 时间戳
   - 使用的配置档案
   - 模型 ID
   - Token 消耗（Input/Output/Total）
   - 请求耗时
   - HTTP 状态码
3. 使用搜索框过滤日志
4. 点击日志条目查看详细信息

### 查看统计

1. 切换到"仪表盘"标签页
2. 查看当前激活配置的详细信息
3. 查看统计数据：
   - 总请求次数
   - 总 Token 消耗
   - 平均响应时间
   - 成功率

---

## 开发文档

### 项目结构

```
prism/
├── src/                    # 前端代码
│   ├── components/         # React 组件
│   ├── pages/             # 页面组件
│   ├── lib/               # 工具函数
│   └── contexts/          # React Context
├── src-tauri/             # Tauri 后端代码
│   ├── src/
│   │   ├── commands.rs    # Tauri 命令
│   │   ├── config/        # 配置管理模块
│   │   ├── db/            # 数据库模块
│   │   ├── proxy/         # 代理服务模块
│   │   └── logger/        # 日志模块
│   └── Cargo.toml
├── docs/                  # 开发文档
├── todo/                  # 任务管理
└── CLAUDE.md             # 项目开发指南
```

### 开发环境设置

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 pnpm
npm install -g pnpm

# 安装依赖
pnpm install

# 运行开发服务器
pnpm tauri dev
```

### 贡献指南

欢迎贡献！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

详细开发规范请参考 [CLAUDE.md](CLAUDE.md)

---

## 常见问题

### Q: 为什么需要这个工具？

A: 如果你使用 Claude Code 并且：
- 使用多个 API 提供商（如 OpenRouter、Claude API、自建代理）
- 需要追踪 Token 消耗和成本
- 想要完整的请求日志和审计
那么这个工具可以大大提升你的使用体验。

### Q: 会影响 Claude Code 的性能吗？

A: 不会。代理转发延迟控制在毫秒级，对使用体验无感知。

### Q: API Key 安全吗？

A: 是的。所有 API Key 都加密存储在本地数据库中，不会上传到任何服务器。

### Q: 支持 Windows 和 Linux 吗？

A: 目前仅支持 macOS。未来可能会支持其他平台。

### Q: 可以用于其他 AI 工具吗？

A: 理论上可以，只要该工具支持配置自定义 API Base URL。但本项目专为 Claude Code 优化。

---

## 路线图

- [x] 基础代理功能
- [x] 配置管理
- [x] 日志记录
- [x] Token 统计
- [x] 菜单栏集成
- [ ] 统计数据可视化
- [ ] 配置导入/导出
- [ ] 自动更新
- [ ] 开机自启动
- [ ] 多语言支持
- [ ] Windows/Linux 支持

---

## 许可证

本项目采用 [CC BY-NC 4.0](LICENSE) 协议开源。

**仅供个人使用，禁止商业使用。**

如需商业使用，请联系作者获取授权。

---

## 致谢

- [Tauri](https://tauri.app/) - 优秀的桌面应用框架
- [Claude](https://claude.ai/) - 强大的 AI 助手
- [Anthropic](https://www.anthropic.com/) - Claude API 提供商

---

## 联系方式

- 问题反馈：[GitHub Issues](https://github.com/yourusername/prism/issues)
- 功能建议：[GitHub Discussions](https://github.com/yourusername/prism/discussions)

---

<div align="center">

**如果这个项目对你有帮助，请给个 ⭐️ Star 支持一下！**

Made with ❤️ for Claude Code users

</div>
