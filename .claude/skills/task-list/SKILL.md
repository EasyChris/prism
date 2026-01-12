---
name: task-list
description: List all tasks for a specific date with their status and progress. Use when user wants to see tasks, check task status, or view task overview.
allowed-tools: Read, Glob, Bash
---

# Task List

列出指定日期的所有任务及其状态。

## 使用方法

用户调用：`/task-list [日期]`

参数说明：
- 不提供日期：显示今天的任务
- 提供日期（YYYYMMDD）：显示指定日期的任务
- 使用 "all"：显示最近 7 天的任务

## 执行步骤

### 1. 确定日期

```bash
# 默认使用今天日期
DATE=$(date +%Y%m%d)

# 或使用用户指定的日期
```

### 2. 扫描任务文件

- 读取 `todo/{YYYYMMDD}/` 下的所有 .md 文件
- 读取 `todo/{YYYYMMDD}/done/` 下的已完成任务

### 3. 解析任务状态

对每个任务文件：
- 提取任务目标
- 统计子任务完成情况（如 3/5）
- 判断任务状态（进行中/已完成）

### 4. 格式化输出

```
📅 {日期} 任务列表

进行中:
  1. task_name (2/4) 🔄
  2. another_task (0/3) ⏸️

已完成:
  ✓ completed_task (3/3)

操作提示:
  /task-work 1  - 继续任务 1
  /task-done 1  - 完成任务 1
```

## 注意事项

- 按序号排序显示
- 清晰显示任务状态和进度
- 提供快速操作提示
