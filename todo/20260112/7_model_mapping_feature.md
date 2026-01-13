# 任务：实现模型映射功能（方案2：映射规则表）

**创建时间**: 2026-01-12
**完成时间**: 2026-01-12
**状态**: completed

## 任务目标

实现灵活的模型映射功能，支持三种模式：
1. **Passthrough（透传）**: 使用请求中的原始模型
2. **Override（覆盖）**: 强制使用指定模型
3. **Map（映射）**: 根据规则表映射

## 完成总结

✅ **所有子任务已完成**

### 1. 数据库 Schema 更新
- ✅ Profile 使用 JSON 文件存储，已添加模型映射字段
- ✅ 字段支持序列化/反序列化

### 2. Rust 后端实现
- ✅ 定义 ModelMappingMode 枚举类型（Passthrough/Override/Map）
- ✅ 更新 Profile 结构体，添加映射相关字段
- ✅ 实现模型映射逻辑函数 `resolve_model()`
- ✅ 在代理请求处理中集成模型映射
- ✅ 更新 Tauri commands 支持新字段

### 3. 前端 UI 实现
- ✅ 在配置表单中添加模型映射模式选择器（单选按钮）
- ✅ 实现 Override 模式的输入框
- ✅ 实现 Map 模式的映射表编辑器（键值对列表）
- ✅ 更新 API 接口和类型定义
- ✅ 前后端类型一致性验证通过

### 4. 代码质量
- ✅ Rust 代码编译通过（仅有无关警告）
- ✅ TypeScript 类型检查通过
- ✅ 代码结构清晰，注释完整

## 技术方案

### 数据结构设计

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelMappingMode {
    Passthrough,  // 透传
    Override,     // 覆盖
    Map,          // 映射
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    // ... 现有字段
    pub model_mapping_mode: ModelMappingMode,
    pub override_model: Option<String>,
    pub model_mappings: HashMap<String, String>,
}
```

### 映射逻辑

```rust
fn resolve_model(
    original_model: &str,
    mode: &ModelMappingMode,
    override_model: &Option<String>,
    mappings: &HashMap<String, String>,
) -> String {
    match mode {
        ModelMappingMode::Passthrough => original_model.to_string(),
        ModelMappingMode::Override => {
            override_model.clone().unwrap_or_else(|| original_model.to_string())
        }
        ModelMappingMode::Map => {
            mappings.get(original_model)
                .cloned()
                .unwrap_or_else(|| original_model.to_string())
        }
    }
}
```

## 使用场景示例

### 场景 1：转发到智谱 GLM-4
```json
{
  "model_mapping_mode": "Override",
  "override_model": "glm-4-plus"
}
```

### 场景 2：多模型分流
```json
{
  "model_mapping_mode": "Map",
  "model_mappings": {
    "claude-3-5-sonnet-20241022": "glm-4-plus",
    "claude-opus-4-5-20251101": "deepseek-chat"
  }
}
```

## 注意事项

1. 默认值：新配置默认使用 Passthrough 模式
2. 向后兼容：现有配置自动设置为 Passthrough 模式
3. 验证：Override 模式必须提供 override_model
4. 日志：记录模型映射的转换过程（原始 -> 目标）

## 相关文件

- `src-tauri/src/db/schema.sql` - 数据库表结构
- `src-tauri/src/config/profile.rs` - Profile 结构体
- `src-tauri/src/proxy/handler.rs` - 代理请求处理
- `src/components/ProfileForm.tsx` - 配置表单组件
