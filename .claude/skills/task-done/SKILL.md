---
name: task-done
description: Mark task as completed and move to done directory. Use when all subtasks are finished or user wants to complete a task.
allowed-tools: Read, Bash, Glob
---

# Task Done

标记任务完成并移动到 done 目录。

## 使用方法

用户调用：`/task-done [任务序号或路径]`

## 执行步骤

### 1. 定位任务文件

- 如果提供完整路径，直接使用
- 如果提供序号，在当天目录查找
- 如果未提供，列出当天所有进行中的任务

### 2. 验证完成状态

- 检查所有子任务是否标记为 `[x]`
- 检查 CHANGE 部分是否有内容
- 如果未完成，提示用户并询问是否强制完成

### 3. 移动文件

```bash
# 从 todo/{YYYYMMDD}/ 移动到 todo/{YYYYMMDD}/done/
mv todo/${DATE}/${TASK_FILE} todo/${DATE}/done/
```

### 4. 输出结果

显示任务完成信息和新的文件路径。

## 注意事项

- 确保所有子任务已完成
- 确保 CHANGE 已记录
- 保持文件名不变
