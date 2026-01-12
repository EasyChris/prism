Date: 20260112

----

## 任务目标

创建任务管理的 skills 系统，实现 CLAUDE.md 中定义的开发流程规范自动化

----

## 任务拆分

- [x] 创建 skills 目录结构
- [x] 实现 /task-create - 任务创建 skill
- [x] 实现 /task-work - 任务执行 skill
- [x] 实现 /task-done - 任务完成 skill
- [x] 实现 /task-list - 任务列表 skill
- [x] 实现 /task-search - 任务搜索 skill
- [x] 实现 /task-archive - 任务归档 skill
- [x] 创建 skills 使用文档 README.md

---

## CHANGE

修改:
新增:
- .claude/skills/ - 任务管理 skills 目录（符合官方规范）
- .claude/skills/task-create/SKILL.md - 任务创建 skill（含 YAML 元数据）
- .claude/skills/task-work/SKILL.md - 任务执行 skill（含 YAML 元数据）
- .claude/skills/task-done/SKILL.md - 任务完成 skill（含 YAML 元数据）
- .claude/skills/task-list/SKILL.md - 任务列表 skill（含 YAML 元数据）
- .claude/skills/task-search/SKILL.md - 任务搜索 skill（含 YAML 元数据）
- .claude/skills/task-archive/SKILL.md - 任务归档 skill（含 YAML 元数据）
删除:
- skills/ - 删除错误位置的 skills 目录

## NOTE

创建 6 个 skills 用于任务管理自动化（符合 Claude Code 官方规范）：
1. task-create: 创建新任务
2. task-work: 执行任务
3. task-done: 完成任务
4. task-list: 列出任务
5. task-search: 搜索任务
6. task-archive: 归档任务

所有 skills 已按官方格式创建：
- 位置：.claude/skills/ 目录
- 文件名：SKILL.md（官方要求）
- 包含 YAML frontmatter 元数据（name, description, allowed-tools）
- 遵循 CLAUDE.md 开发流程规范
