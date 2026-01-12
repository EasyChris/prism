---
name: task-create
description: Create new task files following CLAUDE.md workflow. Use when user wants to create a new task, start new work, or initialize a task file with proper structure and numbering.
allowed-tools: Read, Write, Glob, Bash
---

# Task Create

创建新的任务文件，遵循 CLAUDE.md 中定义的任务管理规范。

## 使用方法

用户调用：`/task-create [任务名称]`

## 执行步骤

### 1. 获取任务信息

- 从用户输入获取任务名称
- 如果未提供，询问用户任务名称和目标

### 2. 创建目录结构

```bash
# 获取当前日期（格式：YYYYMMDD）
DATE=$(date +%Y%m%d)

# 创建日期目录和 done 子目录
mkdir -p todo/${DATE}/done
```

### 3. 计算任务序号

- 列出 `todo/{YYYYMMDD}/` 下所有 `.md` 文件
- 提取文件名中的序号（格式：`{序号}_*.md`）
- 新任务序号 = 最大序号 + 1（如果没有文件，则为 1）

### 4. 创建任务文件

文件名格式：`{序号}_{任务名称}.md`

使用以下模板：

```markdown
Date: {YYYYMMDD}

----

## 任务目标

{task_target}

----

## 任务拆分

- [ ] {task_1}
- [ ] {task_2}

---

## CHANGE

修改:
新增:
删除:

## NOTE
{如何执行、执行测试等内容，简短概要}
```

### 5. 输出结果

显示创建的任务文件路径，提示用户：
- 填写任务目标的详细描述
- 将任务拆分为可执行的原子操作
- 使用 `/task-work` 开始执行任务

## 注意事项

- 任务名称使用英文或拼音，避免中文和特殊字符
- 任务名称应简短且具有描述性
- 每天的任务序号从 1 开始独立计数
