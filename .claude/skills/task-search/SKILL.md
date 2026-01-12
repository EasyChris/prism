---
name: task-search
description: Search for keywords across all task files. Use when user wants to find tasks, search task history, or locate specific task content.
allowed-tools: Grep, Read, Glob, Bash
---

# Task Search

在所有任务中搜索关键词，快速定位相关任务。

## 使用方法

用户调用：`/task-search <关键词> [选项]`

选项：
- `--date YYYYMMDD`: 限定搜索日期
- `--status [pending|done|all]`: 限定任务状态（默认：all）
- `--days N`: 搜索最近 N 天（默认：30）

## 执行步骤

### 1. 解析搜索参数

- 提取关键词
- 解析选项（日期范围、状态等）

### 2. 扫描任务文件

根据日期范围确定搜索目录：
- 包含进行中的任务：`todo/{YYYYMMDD}/*.md`
- 包含已完成的任务：`todo/{YYYYMMDD}/done/*.md`

### 3. 执行搜索

使用 Grep 工具在文件内容中搜索关键词：
- 搜索任务目标
- 搜索子任务列表
- 搜索 CHANGE 部分
- 记录匹配位置和上下文

### 4. 格式化输出

```
🔍 搜索结果: "{关键词}" (找到 N 个任务)

2026-01-12:
  1. task_name ✓
     任务目标: ...
     匹配: - [x] 相关子任务

2026-01-10:
  2. another_task (3/5) 🔄
     任务目标: ...
     匹配: - [ ] 相关子任务
```

## 注意事项

- 按日期倒序显示结果
- 显示匹配的具体内容片段
- 提供任务文件路径供快速访问
