# 多模型智能路由设计方案

## 1. 需求背景

实现类似 OpenCode 的多模型路由功能，根据不同的任务类型自动选择最合适的模型，提升开发效率和成本控制。

## 2. 核心挑战

**如何识别任务类型？**

Claude Code 发送的请求格式固定，我们需要从有限的信息中提取任务特征：
- 请求体中的 `messages` 内容
- 请求体中的 `model` 字段
- 可能的自定义 HTTP Headers

## 3. 方案设计

### 3.1 方案一：模型名称前缀路由（推荐 MVP）

**原理**：用户在 Claude Code 中切换模型时，使用特殊前缀标记任务类型

**示例**：
```
task:frontend/claude-sonnet-4    → 路由到前端专用配置
task:backend/gpt-4               → 路由到后端专用配置
task:review/claude-opus-4        → 路由到代码审查配置
claude-sonnet-4                  → 使用默认配置
```

**实现流程**：
```
1. 解析请求中的 model 字段
2. 检查是否包含 "task:" 前缀
3. 提取任务类型（如 frontend）
4. 查找对应的 Profile 配置
5. 转发到目标 API
```

**优点**：
- 实现简单，性能好
- 用户可精确控制
- 无需 AI 分析
- 兼容性好

**缺点**：
- 需要用户手动切换模型
- 不够智能

### 3.2 方案二：智能内容分析路由

**原理**：分析请求内容，使用轻量级模型判断任务类型

**实现流程**：
```
1. 接收请求
2. 提取 messages 中的最后一条用户消息
3. 使用快速模型（如 Claude Haiku）分析任务类型
4. 根据分析结果选择目标 Profile
5. 转发请求
```

**分析 Prompt 示例**：
```
分析以下开发任务，返回任务类型（只返回类型名称）：

任务描述：{user_message}

可选类型：
- frontend: 前端开发（React/Vue/CSS/UI）
- backend: 后端开发（API/服务器/业务逻辑）
- database: 数据库操作（SQL/Schema/查询优化）
- testing: 测试相关（单元测试/集成测试）
- documentation: 文档编写
- code_review: 代码审查
- debugging: 调试问题
- refactoring: 代码重构
- general: 通用任务

只返回类型名称，不要解释。
```

**优点**：
- 用户无感知，自动化
- 智能化程度高

**缺点**：
- 增加延迟（需要额外 API 调用）
- 增加成本
- 可能误判

### 3.3 方案三：混合路由（推荐最终方案）

**原理**：结合方案一和方案二的优点

**实现逻辑**：
```rust
async fn route_request(request: &Request) -> Profile {
    // 1. 优先检查模型名称前缀
    if let Some(task_type) = parse_model_prefix(&request.model) {
        return get_profile_by_task(task_type);
    }

    // 2. 检查是否启用智能路由
    if config.enable_smart_routing {
        if let Some(task_type) = analyze_content(&request.messages).await {
            return get_profile_by_task(task_type);
        }
    }

    // 3. 使用默认激活的 Profile
    return get_active_profile();
}
```

**优点**：
- 灵活性高
- 兼顾性能和智能化
- 用户可选择使用方式

## 4. 数据结构设计

### 4.1 扩展 Profile 结构

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    // ... 现有字段

    /// 任务类型标识（用于路由匹配）
    #[serde(default)]
    pub task_types: Vec<TaskType>,

    /// 路由优先级（数字越小优先级越高）
    #[serde(default)]
    pub priority: u32,

    /// 是否为默认配置
    #[serde(default)]
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    Frontend,
    Backend,
    Database,
    Testing,
    Documentation,
    CodeReview,
    Debugging,
    Refactoring,
    General,
}
```

### 4.2 路由配置

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingConfig {
    /// 路由模式
    pub mode: RoutingMode,

    /// 是否启用智能分析
    pub enable_smart_routing: bool,

    /// 智能分析使用的模型
    pub analysis_model: String,

    /// 智能分析使用的 API 配置
    pub analysis_profile_id: String,

    /// 缓存分析结果的时长（秒）
    pub cache_duration: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoutingMode {
    /// 仅使用模型前缀
    ModelPrefix,
    /// 仅使用内容分析
    ContentAnalysis,
    /// 混合模式（优先前缀，回退到分析）
    Hybrid,
    /// 禁用路由（使用激活的 Profile）
    Disabled,
}
```

## 5. 配置示例

### 5.1 用户配置文件示例

```json
{
  "routing_config": {
    "mode": "hybrid",
    "enable_smart_routing": true,
    "analysis_model": "claude-haiku-4",
    "analysis_profile_id": "default-profile",
    "cache_duration": 300
  },
  "profiles": [
    {
      "id": "frontend-dev",
      "name": "前端开发 - Gemini Pro",
      "api_base_url": "https://generativelanguage.googleapis.com/v1beta",
      "api_key": "xxx",
      "task_types": ["frontend"],
      "priority": 1,
      "model_mapping_mode": "override",
      "override_model": "gemini-3-pro"
    },
    {
      "id": "backend-dev",
      "name": "后端开发 - Claude Sonnet",
      "api_base_url": "https://api.anthropic.com",
      "api_key": "xxx",
      "task_types": ["backend", "database"],
      "priority": 1,
      "model_mapping_mode": "passthrough"
    },
    {
      "id": "code-review",
      "name": "代码审查 - Claude Opus",
      "api_base_url": "https://api.anthropic.com",
      "api_key": "xxx",
      "task_types": ["code_review"],
      "priority": 1,
      "model_mapping_mode": "override",
      "override_model": "claude-opus-4"
    },
    {
      "id": "default",
      "name": "默认配置 - Claude Sonnet",
      "api_base_url": "https://api.anthropic.com",
      "api_key": "xxx",
      "is_active": true,
      "is_default": true,
      "task_types": ["general"],
      "priority": 999
    }
  ]
}
```

### 5.2 使用示例

**方式 1：模型前缀路由**
```bash
# 在 Claude Code 中切换模型
claude --model "task:frontend/claude-sonnet-4"

# 请求会自动路由到 frontend-dev 配置
```

**方式 2：智能路由**
```bash
# 用户正常使用，系统自动分析
claude "帮我优化这个 React 组件的性能"
# → 自动识别为 frontend 任务，路由到 frontend-dev

claude "设计一个用户认证的 API 接口"
# → 自动识别为 backend 任务，路由到 backend-dev
```

## 6. 实现优先级

### Phase 1: MVP（模型前缀路由）
- [ ] 扩展 Profile 数据结构
- [ ] 实现模型名称解析逻辑
- [ ] 实现基于任务类型的 Profile 匹配
- [ ] 添加路由日志记录
- [ ] UI 支持配置任务类型

### Phase 2: 智能路由
- [ ] 实现内容分析模块
- [ ] 添加分析结果缓存
- [ ] 实现混合路由逻辑
- [ ] 添加路由统计和可视化

### Phase 3: 优化
- [ ] 支持自定义任务类型
- [ ] 支持正则表达式匹配
- [ ] 支持基于文件类型的路由
- [ ] 支持路由规则导入/导出

## 7. 性能考虑

### 7.1 缓存策略
- 对相似的请求内容缓存分析结果
- 使用 LRU 缓存，限制内存占用
- 可配置缓存时长

### 7.2 降级策略
- 智能分析超时时回退到默认配置
- 分析失败时使用模型前缀路由
- 记录失败日志供后续优化

## 8. 用户体验

### 8.1 UI 设计
- Profile 配置页面添加"任务类型"多选框
- Dashboard 显示当前路由规则
- 日志页面显示路由决策过程

### 8.2 提示和帮助
- 首次使用时显示路由功能介绍
- 提供常见任务类型的配置模板
- 支持一键导入预设配置

## 9. 安全考虑

- 智能分析的请求内容需要脱敏
- 分析结果不应包含敏感信息
- 缓存数据需要加密存储

## 10. 测试计划

- 单元测试：路由逻辑、模型解析
- 集成测试：端到端路由流程
- 性能测试：智能分析延迟、缓存命中率
- 用户测试：路由准确率、用户满意度
