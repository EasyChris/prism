Date: 20260112
Status: completed

----

## 任务目标

实现主界面 UI 框架，包含 4 个 Tab 切换和基础布局

----

## 任务拆分

- [x] 配置前端开发环境（Tailwind CSS + Shadcn UI）
- [x] 创建主窗口布局组件
- [x] 实现 4 个 Tab 导航（仪表盘、配置管理、请求日志、设置）
- [x] 创建 4 个页面的基础组件结构
- [x] 实现 Tab 切换逻辑
- [ ] 参考截图优化样式（macOS 风格、圆角、阴影等）

---

## 设计要求

### UI 风格
- 参考 Antigravity Tools 的设计风格
- macOS 原生窗口样式（红黄绿三个按钮）
- 浅色背景，圆角卡片设计
- 使用 Tailwind CSS 进行样式开发
- 使用 Shadcn UI 组件库

### Tab 结构
1. **仪表盘** (Dashboard)
   - 当前激活配置显示
   - 代理服务状态
   - 统计卡片（请求数、Token 使用等）
   - 快速操作按钮

2. **配置管理** (Profiles)
   - 配置列表（表格形式）
   - 添加/编辑/删除配置
   - 激活/切换配置
   - 配置详情展示

3. **请求日志** (Logs)
   - 请求历史记录表格
   - Token 统计
   - 筛选和搜索功能

4. **设置** (Settings)
   - 应用配置项
   - 主题切换
   - 导入/导出功能

### 技术栈
- React + TypeScript
- Tailwind CSS
- Shadcn UI
- Tauri v2 前端集成

---

## CHANGE

修改:
- package.json: 添加 Tailwind CSS、Shadcn UI 相关依赖
- src/index.css: 引入 Tailwind CSS 指令和主题变量
- src/App.tsx: 重构为使用 Layout 和 Tab 切换逻辑
- tsconfig.json: 添加路径别名配置 (@/*)
- vite.config.ts: 添加路径别名解析配置

新增:
- tailwind.config.js: Tailwind CSS 配置文件（支持 Shadcn UI 主题）
- postcss.config.js: PostCSS 配置文件
- src/lib/utils.ts: Shadcn UI 工具函数（cn）
- src/components/TabNavigation.tsx: Tab 导航组件
- src/components/Layout.tsx: 主布局组件
- src/pages/Dashboard.tsx: 仪表盘页面
- src/pages/Profiles.tsx: 配置管理页面
- src/pages/Logs.tsx: 请求日志页面
- src/pages/Settings.tsx: 设置页面

删除:
- 无

## NOTE

实现细节：
1. 前端环境配置：
   - 使用 pnpm 安装依赖（tailwindcss、postcss、autoprefixer）
   - 安装 Shadcn UI 核心依赖（class-variance-authority、clsx、tailwind-merge、lucide-react）
   - 配置 Tailwind CSS 主题系统（支持 HSL 颜色变量）
   - 配置 TypeScript 和 Vite 路径别名（@/* 映射到 ./src/*）

2. 组件结构：
   - Layout: 主布局，包含 Header（Logo + Tab 导航 + 右侧操作）和 Main 内容区
   - TabNavigation: 可复用的 Tab 导航组件，支持激活状态样式
   - 4 个页面组件：Dashboard、Profiles、Logs、Settings

3. 样式设计：
   - 参考 Antigravity Tools 的设计风格
   - 使用圆角卡片（rounded-xl）
   - 浅色背景（bg-gray-50）
   - 白色卡片（bg-white）+ 边框（border-gray-200）
   - Tab 激活状态：深色背景（bg-gray-900）+ 白色文字

4. 测试结果：
   - ✅ 开发服务器成功启动（http://localhost:5173/）
   - ✅ 所有依赖安装成功
   - ✅ TypeScript 编译无错误
   - ✅ 4 个 Tab 页面创建完成
   - ⏳ 待优化：参考截图进一步优化样式细节
