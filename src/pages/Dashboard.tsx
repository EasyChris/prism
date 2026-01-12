import { useState, useEffect } from "react"
import * as api from "@/lib/api"

export function Dashboard() {
  const [activeProfile, setActiveProfile] = useState<api.Profile | null>(null)
  const [profileCount, setProfileCount] = useState(0)

  // 加载当前激活的配置
  const loadActiveProfile = async () => {
    try {
      const profile = await api.getActiveProfile()
      setActiveProfile(profile)

      // 同时获取配置总数
      const profiles = await api.getAllProfiles()
      setProfileCount(profiles.length)
    } catch (error) {
      console.error("Failed to load active profile:", error)
    }
  }

  // 组件挂载时加载配置
  useEffect(() => {
    loadActiveProfile()

    // 每隔 2 秒刷新一次配置（用于实时更新）
    const interval = setInterval(loadActiveProfile, 2000)
    return () => clearInterval(interval)
  }, [])

  return (
    <div className="space-y-6">
      {/* Welcome Section */}
      <div>
        <h2 className="text-2xl font-semibold text-gray-900">
          你好，欢迎使用 👋
        </h2>
      </div>

      {/* Stats Cards */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        {/* Card 1 */}
        <div className="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
          <div className="flex items-center gap-3 mb-2">
            <div className="p-2 bg-blue-50 rounded-lg">
              <svg className="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
              </svg>
            </div>
          </div>
          <div className="text-3xl font-bold text-gray-900">{profileCount}</div>
          <div className="text-sm text-gray-600 mt-1">总配置数</div>
        </div>

        {/* Card 2 */}
        <div className="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
          <div className="flex items-center gap-3 mb-2">
            <div className="p-2 bg-green-50 rounded-lg">
              <svg className="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
          <div className="text-3xl font-bold text-gray-900">100%</div>
          <div className="text-sm text-gray-600 mt-1">服务状态</div>
          <div className="text-xs text-green-600 mt-1">✓ 运行中</div>
        </div>

        {/* Card 3 */}
        <div className="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
          <div className="flex items-center gap-3 mb-2">
            <div className="p-2 bg-purple-50 rounded-lg">
              <svg className="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
            </div>
          </div>
          <div className="text-3xl font-bold text-gray-900">0</div>
          <div className="text-sm text-gray-600 mt-1">今日请求数</div>
        </div>

        {/* Card 4 */}
        <div className="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
          <div className="flex items-center gap-3 mb-2">
            <div className="p-2 bg-orange-50 rounded-lg">
              <svg className="w-5 h-5 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
          <div className="text-3xl font-bold text-gray-900">0</div>
          <div className="text-sm text-gray-600 mt-1">Token 使用</div>
        </div>
      </div>

      {/* Current Profile Section */}
      <div className="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
        <h3 className="text-lg font-semibold text-gray-900 mb-4">当前配置</h3>
        {activeProfile ? (
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-600">配置名称</span>
              <span className="text-sm font-medium text-gray-900">{activeProfile.name}</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-600">API 地址</span>
              <span className="text-sm font-medium text-gray-900">{activeProfile.apiBaseUrl}</span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-gray-600">模型</span>
              <span className="text-sm font-medium text-gray-900">{activeProfile.modelId}</span>
            </div>
          </div>
        ) : (
          <div className="text-sm text-gray-500">暂无激活的配置</div>
        )}
      </div>
    </div>
  )
}
