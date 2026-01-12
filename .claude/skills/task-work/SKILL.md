---
name: task-work
description: Execute tasks from task files, mark subtasks as completed, and record changes. Use when user wants to work on a task, continue task execution, or update task progress.
allowed-tools: Read, Edit, Glob
---

# Task Work

执行任务文件中的子任务，并实时更新任务状态。

## 使用方法

用户调用：`/task-work [任务序号或路径]`

## 执行步骤

### 1. 定位任务文件

- 如果提供完整路径，直接使用
- 如果提供序号（如 "1"），在当天目录查找 `1_*.md`
- 如果未提供参数，列出当天所有未完成任务供选择

```bash
# 获取当前日期
DATE=$(date +%Y%m%d)

# 根据序号查找任务文件
TASK_FILE=$(ls todo/${DATE}/${序号}_*.md 2>/dev/null | head -1)
```

### 2. 读取并显示任务内容

- 解析任务文件
- 提取所有子任务列表
- 显示任务目标和当前进度

输出格式：
```
📋 任务: {文件名}
🎯 目标: {任务目标}

子任务列表:
- [ ] 子任务 1
- [ ] 子任务 2
- [x] 已完成的子任务 3
```

### 3. 执行子任务

- 找到第一个未完成的子任务 `[ ]`
- 询问用户是否开始执行
- 执行完成后立即更新为 `[x]`
- 继续下一个子任务

### 4. 记录变更

在 CHANGE 部分记录：
- 修改: 修改了哪些文件和内容
- 新增: 新增了哪些文件和功能
- 删除: 删除了什么内容

每完成一个子任务就更新一次。

### 5. 完成检查

所有子任务完成后：
- 提示用户任务已全部完成
- 建议使用 `/task-done` 完成任务

## 注意事项

- 一次只执行一个原子任务
- 每完成一个立即标记，不要批量更新
- CHANGE 部分要具体记录影响的文件
- 如果遇到阻塞，在 NOTE 中记录原因
