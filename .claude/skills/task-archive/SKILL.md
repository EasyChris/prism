---
name: task-archive
description: Archive old task directories to keep todo folder clean. Use when user wants to archive old tasks, clean up todo directory, or organize historical tasks.
allowed-tools: Read, Bash, Glob
---

# Task Archive

归档旧任务，保持 todo 目录整洁。

## 使用方法

用户调用：`/task-archive [选项]`

选项：
- `--before YYYYMMDD`: 归档指定日期之前的任务
- `--days N`: 归档 N 天前的任务（默认：30）
- `--dry-run`: 预览将要归档的任务，不实际执行

## 执行步骤

### 1. 确定归档范围

根据参数计算截止日期：
```bash
# 归档 30 天前的任务
CUTOFF_DATE=$(date -d "30 days ago" +%Y%m%d)
```

### 2. 筛选待归档任务

- 扫描 `todo/` 目录下的所有日期目录
- 找出早于截止日期的目录
- 统计任务数量（进行中/已完成）

### 3. 执行归档

```bash
# 创建归档目录
mkdir -p todo/archive/

# 移动整个日期目录到 archive/
mv todo/${DATE}/ todo/archive/${DATE}/
```

保持原有结构：`archive/{YYYYMMDD}/`

### 4. 生成报告

显示：
- 归档的日期范围
- 归档的任务数量
- 归档的目录列表
- 未完成任务警告（如有）

## 归档结构

```
/todo
├── archive/              # 归档目录
│   ├── 20251101/
│   ├── 20251102/
│   └── ...
├── 20260110/            # 最近的任务
├── 20260111/
└── 20260112/
```

## 注意事项

- 归档前检查是否有未完成的任务
- 如有未完成任务会给出警告
- 建议先使用 --dry-run 预览
- 归档后的任务仍可通过 /task-search 搜索
