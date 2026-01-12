# Config 模块

## 功能说明

配置管理模块，负责：

1. **配置档案管理**
   - 创建、读取、更新、删除配置档案
   - 配置档案包含：名称、API Base URL、API Key、Model ID

2. **敏感信息加密**
   - API Key 等敏感信息加密存储
   - 使用系统密钥链或加密算法

3. **配置持久化**
   - 配置数据存储到本地文件
   - 支持配置导入/导出

4. **激活配置管理**
   - 维护当前激活的配置档案
   - 确保只有一个配置处于激活状态

## 数据结构

```rust
struct Profile {
    id: String,
    name: String,
    api_base_url: String,
    api_key: String,  // 加密存储
    model_id: String,
    is_active: bool,
}
```
