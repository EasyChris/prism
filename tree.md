# 项目目录结构

```
/Users/chrischen/code/prism/
├── CLAUDE.md                      # 开发流程规范和项目说明
├── 需求文档.md                    # 产品需求文档 (PRD)
├── package.json                   # 前端项目配置文件
├── pnpm-lock.yaml                 # 依赖锁定文件
├── index.html                     # HTML 入口文件
├── vite.config.ts                 # Vite 构建配置
├── tsconfig.json                  # TypeScript 配置
├── tsconfig.node.json             # Node.js TypeScript 配置
│
├── docs/                          # 开发文档目录
│
├── todo/                          # 任务管理目录
│   └── 20260112/                  # 2026年01月12日的任务
│       ├── 2_init_tauri_project.md  # 初始化 Tauri 项目任务
│       └── done/                  # 已完成的任务
│           └── 1_create_task_management_skills.md  # 已完成：创建任务管理 skills
│
├── .claude/                       # Claude Code 配置目录
│   └── skills/                    # 自定义 skills
│       ├── task-create/           # 任务创建 skill
│       ├── task-work/             # 任务执行 skill
│       ├── task-done/             # 任务完成 skill
│       ├── task-list/             # 任务列表 skill
│       ├── task-search/           # 任务搜索 skill
│       └── task-archive/          # 任务归档 skill
│
├── src/                           # 前端源代码目录
│   ├── main.tsx                   # React 应用入口
│   ├── App.tsx                    # 主应用组件
│   └── index.css                  # 全局样式
│
└── src-tauri/                     # Tauri 后端目录
    ├── Cargo.toml                 # Rust 项目配置
    ├── Cargo.lock                 # Rust 依赖锁定
    ├── build.rs                   # 构建脚本
    ├── tauri.conf.json            # Tauri 配置文件
    │
    ├── capabilities/              # Tauri 权限配置
    │   └── default.json           # 默认权限
    │
    ├── icons/                     # 应用图标
    │   ├── icon.png               # 主图标
    │   ├── icon.icns              # macOS 图标
    │   └── icon.ico               # Windows 图标
    │
    └── src/                       # Rust 源代码
        ├── main.rs                # 主程序入口
        ├── lib.rs                 # 库入口
        │
        ├── proxy/                 # 代理服务模块
        │   └── INFO.md            # 模块说明文档
        │
        ├── config/                # 配置管理模块
        │   └── INFO.md            # 模块说明文档
        │
        └── logger/                # 日志审计模块
            └── INFO.md            # 模块说明文档
```
