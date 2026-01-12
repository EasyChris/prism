# Claude Code Proxy Hub - 开发指南

## 项目概述

**项目名称**: Claude Code Proxy Hub
**平台**: macOS (基于 Tauri v2 开发)
**核心定位**: 专为 Claude Code 打造的动态路由网关与流量审计工具

### 产品目标

构建一个运行在 macOS 上的轻量级桌面应用，作为 Claude Code 的**本地中转站**。实现"一次配置，随意切换"的无缝体验，并提供完整的流量去向记录与成本审计。

---

## 开发流程规范


### 目录树维护

在根目录维护 `tree.md` 文件：
- 使用 `tree` 命令输出目录结构
- 为当前任务修改的目录和文件添加中文描述
- 每次创建新目录时动态更新此文件
- 该文件帮助快速理解项目结构

---

## 项目结构

使用 `tree -L 3 -I "node_modules"` 创建根目录下 `tree.md`，后续更新也更新到 `tree.md`，并给目录和文件写上简短的使用说明。

### 核心目录说明

- **`docs/`** - 开发文档目录
- **`todo/`** - 任务管理目录，按日期组织
  - 格式：`todo/{YYYYMMDD}/` 和 `todo/{YYYYMMDD}/done/`
- **`.claude/skills/`** - Claude Code 自定义 skills
- **`src-tauri/`** - Tauri 后端代码（Rust）
- **`src/`** - 前端代码（React/Vue）

---

## 技术栈

### 后端 (Rust + Tauri v2)
- **Tauri v2**: 桌面应用框架
- **Axum/Actix-web**: HTTP 服务器框架（用于代理服务）
- **Tokio**: 异步运行时
- **Serde**: JSON 序列化/反序列化
- **Reqwest**: HTTP 客户端（用于转发请求）
- ** 使用 pnpm 进行报的管理和安装 ** 

### 前端
- **React/Vue**: UI 框架
- **TypeScript**: 类型安全
- **Tailwind CSS v4**: 样式框架

---

## 核心功能模块

### 1. 动态代理服务
- 本地 HTTP 服务监听（伪装成 Anthropic API）
- 请求级别的动态路由切换
- SSE 流式响应透传
- 错误处理与兜底

### 2. 配置管理
- Profile（配置档案）的 CRUD 操作
- 必填字段：名称、API Base URL、API Key、Model ID
- 敏感信息加密存储
- 单一激活配置

### 3. 日志与审计
- 全链路请求日志记录
- Token 统计（Input/Output）
- 路由去向追踪（Provider + Model）
- 请求耗时监控
- HTTP 状态码记录

### 4. 用户界面
- Mac 菜单栏常驻
- 配置档案快速切换
- 服务状态显示
- 日志面板（表格展示）
- 仪表盘（当前配置详情）

---

## 开发阶段规划 (MVP)

### P1 阶段：核心跑通
**目标**: 实现基础代理功能
- 本地 HTTP 服务搭建
- 静态配置切换
- 基本的流式转发
- 简单的错误处理

### P2 阶段：日志集成
**目标**: 完善审计功能
- Token 解析逻辑
- 日志数据库设计
- 日志界面展示
- 统计数据汇总

### P3 阶段：体验优化
**目标**: 提升用户体验
- Mac 菜单栏集成
- 自动更新机制
- 开机自启动
- 配置导入/导出

---

## 非功能性需求

### 性能要求
- **低延迟**: 代理转发延迟控制在毫秒级
- **资源占用**: CPU 静默占用近乎为 0，内存占用低
- **流畅体验**: SSE 流式响应无卡顿

### 安全要求
- **本地运行**: 所有数据仅在本地处理，不上传云端
- **加密存储**: API Key 等敏感信息必须加密存储
- **脱敏展示**: 日志展示时对 API Key 进行脱敏处理

### 网络要求
- **代理支持**: 支持配置上游代理（科学上网环境）
- **错误重试**: 网络异常时的重试机制
- **超时控制**: 合理的请求超时设置

---

## 开发规范

### 代码规范
- Rust 代码遵循 `rustfmt` 和 `clippy` 规范
- TypeScript 代码使用 ESLint + Prettier
- 提交前必须通过 lint 检查

### 测试规范
- 核心功能必须有单元测试
- 代理转发逻辑需要集成测试
- UI 组件使用快照测试

### 文档规范
- 每个模块目录下维护 `INFO.md` 说明文件
- API 接口需要详细的注释说明
- 复杂逻辑必须添加代码注释

---

## 相关资源

### 官方文档
- [Tauri v2 文档](https://v2.tauri.app/)
- [Claude API 文档](https://docs.anthropic.com/)
- [Anthropic API 规范](https://docs.anthropic.com/en/api/messages)

### 项目文档
- [需求文档](需求文档.md) - 产品需求详细说明
- [tree.md](tree.md) - 项目目录结构
- [.claude/skills/](.claude/skills/) - 任务管理 Skills

---

## 快速开始

### 使用任务管理 Skills

项目已配置 6 个任务管理 skills，可以通过以下命令使用：

- `/task-create [任务名称]` - 创建新任务
- `/task-work [任务序号]` - 执行任务
- `/task-done [任务序号]` - 完成任务
- `/task-list [日期]` - 列出任务
- `/task-search <关键词>` - 搜索任务
- `/task-archive [选项]` - 归档任务

详细使用说明请参考各 skill 的 SKILL.md 文件。

