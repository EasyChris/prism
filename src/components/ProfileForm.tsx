import { useState } from "react"

type ModelMappingMode = "passthrough" | "override" | "map"

interface Profile {
  id: string
  name: string
  apiBaseUrl: string
  apiKey: string
  modelId: string
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
    modelId: profile?.modelId || "claude-3-5-sonnet-20241022",
    modelMappingMode: profile?.modelMappingMode || "passthrough" as ModelMappingMode,
    overrideModel: profile?.overrideModel || "",
    modelMappings: profile?.modelMappings || {},
  })

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

    // 验证覆盖模式必须填写目标模型
    if (formData.modelMappingMode === "override" && !formData.overrideModel?.trim()) {
      alert("覆盖模式下必须指定目标模型")
      return
    }

    console.log("[ProfileForm] Submitting form data:", formData)
    onSubmit(formData)
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-4">
      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          配置名称 *
        </label>
        <input
          type="text"
          required
          value={formData.name}
          onChange={(e) => setFormData({ ...formData, name: e.target.value })}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="例如：默认配置"
        />
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          API Base URL *
        </label>
        <input
          type="url"
          required
          value={formData.apiBaseUrl}
          onChange={(e) => setFormData({ ...formData, apiBaseUrl: e.target.value })}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="https://api.anthropic.com"
        />
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          API Key *
        </label>
        <input
          type="password"
          required
          value={formData.apiKey}
          onChange={(e) => setFormData({ ...formData, apiKey: e.target.value })}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="sk-ant-..."
        />
      </div>

      <div>
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Model ID *
        </label>
        <input
          type="text"
          required
          value={formData.modelId}
          onChange={(e) => setFormData({ ...formData, modelId: e.target.value })}
          className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          placeholder="claude-3-5-sonnet-20241022"
        />
      </div>

      {/* 模型映射配置 */}
      <div className="border-t pt-4">
        <label className="block text-sm font-medium text-gray-700 mb-3">
          模型映射模式
        </label>
        <div className="space-y-3">
          {/* Passthrough 模式 */}
          <label className="flex items-start gap-3 p-3 border rounded-lg cursor-pointer hover:bg-gray-50">
            <input
              type="radio"
              name="mappingMode"
              value="passthrough"
              checked={formData.modelMappingMode === "passthrough"}
              onChange={(e) => setFormData({ ...formData, modelMappingMode: e.target.value as ModelMappingMode })}
              className="mt-1"
            />
            <div className="flex-1">
              <div className="font-medium text-gray-900">透传模式 (Passthrough)</div>
              <div className="text-sm text-gray-500">使用请求中的原始模型，不做任何修改</div>
            </div>
          </label>

          {/* Override 模式 */}
          <label className="flex items-start gap-3 p-3 border rounded-lg cursor-pointer hover:bg-gray-50">
            <input
              type="radio"
              name="mappingMode"
              value="override"
              checked={formData.modelMappingMode === "override"}
              onChange={(e) => setFormData({ ...formData, modelMappingMode: e.target.value as ModelMappingMode })}
              className="mt-1"
            />
            <div className="flex-1">
              <div className="font-medium text-gray-900">覆盖模式 (Override)</div>
              <div className="text-sm text-gray-500">强制使用指定的目标模型</div>
            </div>
          </label>

          {/* Override 模式的输入框 */}
          {formData.modelMappingMode === "override" && (
            <div className="ml-8 mt-2">
              <input
                type="text"
                value={formData.overrideModel || ""}
                onChange={(e) => setFormData({ ...formData, overrideModel: e.target.value })}
                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                placeholder="例如：glm-4-plus 或 deepseek-chat"
                required
              />
            </div>
          )}

          {/* Map 模式 */}
          <label className="flex items-start gap-3 p-3 border rounded-lg cursor-pointer hover:bg-gray-50">
            <input
              type="radio"
              name="mappingMode"
              value="map"
              checked={formData.modelMappingMode === "map"}
              onChange={(e) => setFormData({ ...formData, modelMappingMode: e.target.value as ModelMappingMode })}
              className="mt-1"
            />
            <div className="flex-1">
              <div className="font-medium text-gray-900">映射模式 (Map)</div>
              <div className="text-sm text-gray-500">根据映射规则表转换模型</div>
            </div>
          </label>

          {/* Map 模式的映射规则编辑器 */}
          {formData.modelMappingMode === "map" && (
            <div className="ml-8 mt-2 space-y-3">
              {/* 添加映射规则 */}
              <div className="flex gap-2">
                <input
                  type="text"
                  value={mappingKey}
                  onChange={(e) => setMappingKey(e.target.value)}
                  className="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
                  placeholder="原始模型（如：claude-3-5-sonnet-20241022）"
                />
                <span className="flex items-center text-gray-400">→</span>
                <input
                  type="text"
                  value={mappingValue}
                  onChange={(e) => setMappingValue(e.target.value)}
                  className="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
                  placeholder="目标模型（如：glm-4-plus）"
                />
                <button
                  type="button"
                  onClick={handleAddMapping}
                  className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm"
                >
                  添加
                </button>
              </div>

              {/* 显示已有的映射规则 */}
              {Object.keys(formData.modelMappings).length > 0 && (
                <div className="space-y-2">
                  <div className="text-sm font-medium text-gray-700">映射规则：</div>
                  {Object.entries(formData.modelMappings).map(([key, value]) => (
                    <div key={key} className="flex items-center gap-2 p-2 bg-gray-50 rounded-lg">
                      <span className="flex-1 text-sm text-gray-700">
                        <span className="font-mono">{key}</span>
                        <span className="text-gray-400 mx-2">→</span>
                        <span className="font-mono">{value}</span>
                      </span>
                      <button
                        type="button"
                        onClick={() => handleRemoveMapping(key)}
                        className="px-2 py-1 text-red-600 hover:bg-red-50 rounded text-sm"
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

      <div className="flex gap-3 pt-4">
        <button
          type="button"
          onClick={onCancel}
          className="flex-1 px-4 py-2 border border-gray-300 text-gray-700 rounded-lg hover:bg-gray-50"
        >
          取消
        </button>
        <button
          type="submit"
          className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700"
        >
          {profile ? "保存" : "添加"}
        </button>
      </div>
    </form>
  )
}
