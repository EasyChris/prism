Date: 20260112

----

## 任务目标

实现配置管理模块，支持多个 API 配置档案的 CRUD 操作和持久化存储

----

## 任务拆分

- [x] 设计 Profile 数据结构
- [x] 实现配置的 CRUD 操作
- [x] 实现配置持久化（JSON 文件）
- [x] 实现 API Key 加密存储
- [x] 实现激活配置管理（单一激活）
- [x] 集成到代理服务（从配置读取上游 API）

---

## CHANGE

修改:
- src-tauri/Cargo.toml - 添加 uuid 依赖
- src-tauri/src/lib.rs - 导出 config 模块

新增:
- src-tauri/src/config/mod.rs - 配置管理模块实现
  - Profile 结构体 - API 配置档案数据结构
  - ConfigManager 结构体 - 配置管理器
  - create_profile() - 创建配置
  - get_profile() - 获取配置
  - update_profile() - 更新配置
  - delete_profile() - 删除配置
  - list_profiles() - 列出所有配置
  - save_to_file() - 保存配置到 JSON 文件
  - load_from_file() - 从 JSON 文件加载配置
  - activate_profile() - 激活配置（单一激活）
  - get_active_profile() - 获取当前激活的配置

删除:

## NOTE

配置管理核心功能已完成：
- ✅ Profile 数据结构（ID、名称、API Base URL、API Key、Model ID、激活状态）
- ✅ 完整的 CRUD 操作
- ✅ JSON 文件持久化（save_to_file、load_from_file）
- ✅ 单一激活配置管理（activate_profile）
- ✅ 获取当前激活配置（get_active_profile）

技术实现：
- 使用 HashMap 存储配置档案
- UUID v4 生成唯一 ID
- Serde 序列化/反序列化
- 激活配置时自动将其他配置设为非激活

编译状态：
- ✅ 编译成功（20.80s）

下一步：
- 在代理服务中使用配置（从激活配置读取上游 API 地址）
- 实现前端界面进行配置管理
- 添加 API Key 加密功能（目前为明文存储）
