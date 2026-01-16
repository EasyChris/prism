import { useState } from "react"
import { Eye, EyeOff, AlertCircle } from "lucide-react"

type ModelMappingMode = "passthrough" | "override" | "map"

interface Profile {
  id: string
  name: string
  apiBaseUrl: string
  apiKey: string
  modelMappingMode: ModelMappingMode
  overrideModel?: string
  modelMappings: Record<string, string>
}

interface ProfileFormProps {
  profile?: Profile
  onSubmit: (profile: Omit<Profile, "id" | "isActive">) => void
  onCancel: () => void
}

export function ProfileForm({ profile, onSubmit, onCancel }: ProfileFormProps) {
  const [formData, setFormData] = useState({
    name: profile?.name || "",
    apiBaseUrl: profile?.apiBaseUrl || "https://api.anthropic.com",
    apiKey: profile?.apiKey || "",
    modelMappingMode: profile?.modelMappingMode || "passthrough" as ModelMappingMode,
    overrideModel: profile?.overrideModel || "",
    modelMappings: profile?.modelMappings || {},
  })

  // UI 状态
  const [showApiKey, setShowApiKey] = useState(false)
  const [errors, setErrors] = useState<Record<string, string>>({})

  // 用于 Map 模式的映射规则编辑
  const [mappingKey, setMappingKey] = useState("")
  const [mappingValue, setMappingValue] = useState("")

  // 添加映射规则
  const handleAddMapping = () => {
    if (mappingKey && mappingValue) {
      setFormData({
        ...formData,
        modelMappings: {
          ...formData.modelMappings,
          [mappingKey]: mappingValue,
        },
      })
      setMappingKey("")
      setMappingValue("")
    }
  }

  // 删除映射规则
  const handleRemoveMapping = (key: string) => {
    const newMappings = { ...formData.modelMappings }
    delete newMappings[key]
    setFormData({ ...formData, modelMappings: newMappings })
  }

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()

    // 清空之前的错误
    const newErrors: Record<string, string> = {}

    // 验证覆盖模式必须填写目标模型
    if (formData.modelMappingMode === "override" && !formData.overrideModel?.trim()) {
      newErrors.overrideModel = "覆盖模式下必须指定目标模型"
    }

    // 如果有错误，显示并阻止提交
    if (Object.keys(newErrors).length > 0) {
      setErrors(newErrors)
      return
    }

    console.log("[ProfileForm] Submitting form data:", formData)
    onSubmit(formData)
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      {/* 基础信息区域 */}
      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            配置名称 *
          </label>
          <input
            type="text"
            required
            value={formData.name}
            onChange={(e) => setFormData({ ...formData, name: e.target.value })}
            className="w-full px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500 transition-colors"
            placeholder="例如：默认配置"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            API Base URL *
          </label>
          <input
            type="url"
            required
            value={formData.apiBaseUrl}
            onChange={(e) => setFormData({ ...formData, apiBaseUrl: e.target.value })}
            className="w-full px-4 py-2.5 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500 transition-colors"
            placeholder="https://api.anthropic.com"
          />
        </div>

        {/* API Key 字段 - 带显示/隐藏功能 */}
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
            API Key *
          </label>
          <div className="relative">
            <input
              type={showApiKey ? "text" : "password"}
              required
              value={formData.apiKey}
              onChange={(e) => setFormData({ ...formData, apiKey: e.target.value })}
              className="w-full px-4 py-2.5 pr-12 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500 transition-colors"
              placeholder="sk-ant-..."
            />
            <button
              type="button"
              onClick={() => setShowApiKey(!showApiKey)}
              className="absolute right-3 top-1/2 -translate-y-1/2 text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
              aria-label={showApiKey ? "隐藏 API Key" : "显示 API Key"}
            >
              {showApiKey ? <EyeOff size={20} /> : <Eye size={20} />}
            </button>
          </div>
        </div>
      </div>

      {/* 模型映射配置区域 */}
      <div className="border-t border-gray-200 dark:border-gray-700 pt-6">
        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-4">
          模型映射模式
        </label>

        {/* Tab 切换 */}
        <div className="flex gap-2 mb-4 border-b border-gray-200 dark:border-gray-700">
          <button
            type="button"
            onClick={() => setFormData({ ...formData, modelMappingMode: "passthrough" })}
            className={`px-4 py-2 text-sm font-medium transition-colors border-b-2 ${
              formData.modelMappingMode === "passthrough"
                ? "border-blue-500 text-blue-600 dark:text-blue-400"
                : "border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300"
            }`}
          >
            透传模式
          </button>
          <button
            type="button"
            onClick={() => setFormData({ ...formData, modelMappingMode: "override" })}
            className={`px-4 py-2 text-sm font-medium transition-colors border-b-2 ${
              formData.modelMappingMode === "override"
                ? "border-blue-500 text-blue-600 dark:text-blue-400"
                : "border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300"
            }`}
          >
            覆盖模式
          </button>
          <button
            type="button"
            onClick={() => setFormData({ ...formData, modelMappingMode: "map" })}
            className={`px-4 py-2 text-sm font-medium transition-colors border-b-2 ${
              formData.modelMappingMode === "map"
                ? "border-blue-500 text-blue-600 dark:text-blue-400"
                : "border-transparent text-gray-500 dark:text-gray-400 hover:text-gray-700 dark:hover:text-gray-300"
            }`}
          >
            映射模式
          </button>
        </div>

        {/* Tab 内容区域 */}
        <div className="mt-4">
          {/* 透传模式说明 */}
          {formData.modelMappingMode === "passthrough" && (
            <div className="p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg">
              <p className="text-sm text-gray-700 dark:text-gray-300">
                使用请求中的原始模型，不做任何修改。适合直接对接 Anthropic API 或兼容服务。
              </p>
            </div>
          )}

          {/* 覆盖模式 */}
          {formData.modelMappingMode === "override" && (
            <div className="space-y-3">
              <div className="p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg">
                <p className="text-sm text-gray-700 dark:text-gray-300">
                  强制使用指定的目标模型，忽略请求中的模型参数。适合统一使用某个特定模型。
                </p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                  目标模型 *
                </label>
                <input
                  type="text"
                  value={formData.overrideModel || ""}
                  onChange={(e) => {
                    setFormData({ ...formData, overrideModel: e.target.value })
                    // 清除错误
                    if (errors.overrideModel) {
                      setErrors({ ...errors, overrideModel: "" })
                    }
                  }}
                  className={`w-full px-4 py-2.5 border rounded-lg focus:outline-none focus:ring-2 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500 transition-colors ${
                    errors.overrideModel
                      ? "border-red-300 dark:border-red-700 focus:ring-red-500"
                      : "border-gray-300 dark:border-gray-600 focus:ring-blue-500 dark:focus:ring-blue-400"
                  }`}
                  placeholder="例如：glm-4-plus 或 deepseek-chat"
                />
                {errors.overrideModel && (
                  <div className="flex items-center gap-1 mt-2 text-sm text-red-600 dark:text-red-400">
                    <AlertCircle size={16} />
                    <span>{errors.overrideModel}</span>
                  </div>
                )}
              </div>
            </div>
          )}

          {/* 映射模式 */}
          {formData.modelMappingMode === "map" && (
            <div className="space-y-4">
              <div className="p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg">
                <p className="text-sm text-gray-700 dark:text-gray-300">
                  根据映射规则表转换模型。可以为不同的原始模型指定不同的目标模型。
                </p>
              </div>

              {/* 添加映射规则 */}
              <div className="flex gap-2">
                <input
                  type="text"
                  value={mappingKey}
                  onChange={(e) => setMappingKey(e.target.value)}
                  className="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500 text-sm transition-colors"
                  placeholder="原始模型（如：claude-3-5-sonnet-20241022）"
                />
                <span className="flex items-center text-gray-400 dark:text-gray-500">→</span>
                <input
                  type="text"
                  value={mappingValue}
                  onChange={(e) => setMappingValue(e.target.value)}
                  className="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 placeholder:text-gray-400 dark:placeholder:text-gray-500 text-sm transition-colors"
                  placeholder="目标模型（如：glm-4-plus）"
                />
                <button
                  type="button"
                  onClick={handleAddMapping}
                  className="px-4 py-2 bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 text-sm transition-colors"
                >
                  添加
                </button>
              </div>

              {/* 显示已有的映射规则 */}
              {Object.keys(formData.modelMappings).length > 0 && (
                <div className="space-y-2">
                  <div className="text-sm font-medium text-gray-700 dark:text-gray-300">映射规则：</div>
                  {Object.entries(formData.modelMappings).map(([key, value]) => (
                    <div key={key} className="flex items-center gap-2 p-3 bg-gray-50 dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-lg">
                      <span className="flex-1 text-sm text-gray-700 dark:text-gray-300">
                        <span className="font-mono">{key}</span>
                        <span className="text-gray-400 dark:text-gray-500 mx-2">→</span>
                        <span className="font-mono">{value}</span>
                      </span>
                      <button
                        type="button"
                        onClick={() => handleRemoveMapping(key)}
                        className="px-3 py-1 text-red-600 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 rounded text-sm transition-colors"
                      >
                        删除
                      </button>
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}
        </div>
      </div>

      {/* 底部按钮 */}
      <div className="flex gap-3 pt-4 border-t border-gray-200 dark:border-gray-700">
        <button
          type="button"
          onClick={onCancel}
          className="flex-1 px-4 py-2.5 border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
        >
          取消
        </button>
        <button
          type="submit"
          className="flex-1 px-4 py-2.5 bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors font-medium"
        >
          {profile ? "保存" : "添加"}
        </button>
      </div>
    </form>
  )
}
