import { useState, useEffect } from "react"

interface Profile {
  id: string
  name: string
  apiBaseUrl: string
  apiKey: string
  modelId: string
}

interface ProfileFormProps {
  profile?: Profile
  onSubmit: (profile: Omit<Profile, "id">) => void
  onCancel: () => void
}

export function ProfileForm({ profile, onSubmit, onCancel }: ProfileFormProps) {
  const [formData, setFormData] = useState({
    name: profile?.name || "",
    apiBaseUrl: profile?.apiBaseUrl || "https://api.anthropic.com",
    apiKey: profile?.apiKey || "",
    modelId: profile?.modelId || "claude-3-5-sonnet-20241022",
  })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
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
