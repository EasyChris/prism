import { useState, useEffect } from "react"
import * as api from "@/lib/api"
import { ProfileSelector } from "@/components/ProfileSelector"
import { SetupGuideModal } from "@/components/SetupGuideModal"

export function Dashboard() {
  const [activeProfile, setActiveProfile] = useState<api.Profile | null>(null)
  const [allProfiles, setAllProfiles] = useState<api.Profile[]>([])
  const [profileCount, setProfileCount] = useState(0)
  const [stats, setStats] = useState<api.DashboardStats>({
    todayRequests: 0,
    todayTokens: 0,
    totalRequests: 0,
    totalTokens: 0,
  })
  const [timeRange, setTimeRange] = useState<api.TimeRange>('hour')
  const [tokenData, setTokenData] = useState<api.TokenDataPoint[]>([])
  const [isLoadingTokenData, setIsLoadingTokenData] = useState(false)
  const [proxyApiKey, setProxyApiKey] = useState<string | null>(null)
  const [proxyServerUrl, setProxyServerUrl] = useState<string>("")
  const [copySuccess, setCopySuccess] = useState(false)
  const [isSetupGuideOpen, setIsSetupGuideOpen] = useState(false)

  // 格式化数字显示（K/M 单位）
  const formatNumber = (num: number): string => {
    if (num >= 1000000) {
      return (num / 1000000).toFixed(1) + 'M'
    }
    if (num >= 1000) {
      return (num / 1000).toFixed(1) + 'K'
    }
    return num.toLocaleString()
  }

  // 加载当前激活的配置
  const loadActiveProfile = async () => {
    try {
      const profiles = await api.getAllProfiles()
      setAllProfiles(profiles)
      setProfileCount(profiles.length)

      const profile = profiles.find(p => p.isActive) || null
      setActiveProfile(profile)
    } catch (error) {
      console.error("Failed to load active profile:", error)
    }
  }

  // 加载统计数据
  const loadStats = async () => {
    try {
      const dashboardStats = await api.getDashboardStats()
      setStats(dashboardStats)
    } catch (error) {
      console.error("Failed to load dashboard stats:", error)
    }
  }

  // 加载 API Key 和代理服务器地址
  const loadApiKeySettings = async () => {
    try {
      const apiKey = await api.getProxyApiKey()
      setProxyApiKey(apiKey)

      const serverUrl = await api.getProxyServerUrl()
      setProxyServerUrl(serverUrl)
    } catch (error) {
      console.error("Failed to load API key settings:", error)
    }
  }

  // 刷新 API Key
  const handleRefreshApiKey = async () => {
    try {
      const newKey = await api.refreshProxyApiKey()
      setProxyApiKey(newKey)
    } catch (error) {
      console.error("Failed to refresh API key:", error)
    }
  }

  // 复制 API Key
  const handleCopyApiKey = async () => {
    if (proxyApiKey) {
      try {
        await navigator.clipboard.writeText(proxyApiKey)
        setCopySuccess(true)
        setTimeout(() => setCopySuccess(false), 2000)
      } catch (error) {
        console.error("Failed to copy API key:", error)
      }
    }
  }


  // 加载 Token 统计数据
  const loadTokenStats = async () => {
    setIsLoadingTokenData(true)
    try {
      const data = await api.getTokenStats(timeRange)
      setTokenData(data)
    } catch (error) {
      console.error("Failed to load token stats:", error)
      setTokenData([])
    } finally {
      setIsLoadingTokenData(false)
    }
  }

  // 处理配置切换
  const handleProfileChange = async (profileId: string) => {
    try {
      await api.activateProfile(profileId)
      await loadActiveProfile()
    } catch (error) {
      console.error("Failed to activate profile:", error)
    }
  }

  // 处理添加配置（跳转到 Profiles 页面）
  const handleAddProfile = () => {
    // 触发父组件的 tab 切换
    window.dispatchEvent(new CustomEvent('switchTab', { detail: 'profiles' }))
  }

  // 组件挂载时加载配置和统计数据
  useEffect(() => {
    loadActiveProfile()
    loadStats()
    loadTokenStats()
    loadApiKeySettings()

    // 每隔 2 秒刷新一次配置和统计数据（用于实时更新）
    // 注意：不包括 Token 统计数据，Token 数据只在切换时间范围或 tab 切换时刷新
    const interval = setInterval(() => {
      loadActiveProfile()
      loadStats()
    }, 2000)
    return () => clearInterval(interval)
  }, [])

  // 当时间范围改变时，重新加载 Token 统计数据
  useEffect(() => {
    loadTokenStats()
  }, [timeRange])

  // 使用真实数据或空数组
  const currentData = tokenData.length > 0 ? tokenData : []
  const maxTokens = currentData.length > 0 ? Math.max(...currentData.map(d => d.tokens)) : 0
  const maxCacheTokens = currentData.length > 0 ? Math.max(...currentData.map(d => d.cacheReadTokens || 0)) : 0
  const totalCacheHits = currentData.reduce((sum, d) => sum + (d.cacheReadTokens || 0), 0)

  // 时间范围标签映射
  const timeRangeLabels = {
    hour: '小时',
    day: '天',
    week: '周',
    month: '月'
  }

  return (
    <div className="space-y-4">
      {/* 第一行：统计数据卡片 */}
      <div className="grid grid-cols-4 gap-3">
        {/* 总配置数卡片 */}
        <div className="bg-white dark:bg-gray-800 rounded-lg p-4 shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-all">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-gray-500 dark:text-gray-400 mb-0.5">总配置数</p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">{profileCount}</p>
            </div>
            <div className="w-10 h-10 bg-blue-100 dark:bg-blue-900/30 rounded-lg flex items-center justify-center">
              <svg className="w-5 h-5 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
            </div>
          </div>
        </div>

        {/* 服务状态卡片 */}
        <div className="bg-white dark:bg-gray-800 rounded-lg p-4 shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-all">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-gray-500 dark:text-gray-400 mb-0.5">服务状态</p>
              <p className={`text-xl font-bold ${activeProfile ? 'text-green-600 dark:text-green-400' : 'text-gray-400 dark:text-gray-500'}`}>
                {activeProfile ? "运行中" : "未激活"}
              </p>
            </div>
            <div className={`w-10 h-10 rounded-lg flex items-center justify-center ${activeProfile ? 'bg-green-100 dark:bg-green-900/30' : 'bg-gray-100 dark:bg-gray-700'}`}>
              <svg className={`w-5 h-5 ${activeProfile ? 'text-green-600 dark:text-green-400' : 'text-gray-400 dark:text-gray-500'}`} fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>
        </div>

        {/* 今日请求卡片 */}
        <div className="bg-white dark:bg-gray-800 rounded-lg p-4 shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-all">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-gray-500 dark:text-gray-400 mb-0.5">今日请求</p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">{formatNumber(stats.todayRequests)}</p>
            </div>
            <div className="w-10 h-10 bg-purple-100 dark:bg-purple-900/30 rounded-lg flex items-center justify-center">
              <svg className="w-5 h-5 text-purple-600 dark:text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
            </div>
          </div>
        </div>

        {/* 今日 Token 卡片 */}
        <div className="bg-white dark:bg-gray-800 rounded-lg p-4 shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-all">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-xs text-gray-500 dark:text-gray-400 mb-0.5">今日 Token</p>
              <p className="text-2xl font-bold text-gray-900 dark:text-white">{formatNumber(stats.todayTokens)}</p>
            </div>
            <div className="w-10 h-10 bg-orange-100 dark:bg-orange-900/30 rounded-lg flex items-center justify-center">
              <svg className="w-5 h-5 text-orange-600 dark:text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z" />
              </svg>
            </div>
          </div>
        </div>
      </div>

      {/* 第二行：当前配置和 API 密钥 */}
      <div className="grid grid-cols-2 gap-3">
        {/* 当前配置卡片 */}
        <div className="bg-white dark:bg-gray-800 rounded-lg p-4 shadow-sm border border-gray-200 dark:border-gray-700 transition-colors">
          <h3 className="text-xs font-semibold text-gray-700 dark:text-gray-300 mb-3">当前配置</h3>
          <ProfileSelector
            profiles={allProfiles}
            activeProfile={activeProfile}
            onProfileChange={handleProfileChange}
            onAddProfile={handleAddProfile}
          />
        </div>

        {/* API 密钥卡片 */}
        <div className="bg-white dark:bg-gray-800 rounded-lg p-4 shadow-sm border border-gray-200 dark:border-gray-700 transition-colors">
          <div className="flex items-center justify-between mb-3">
            <h3 className="text-xs font-semibold text-gray-700 dark:text-gray-300">代理服务 API 密钥</h3>
            <button
              onClick={() => setIsSetupGuideOpen(true)}
              className="flex items-center gap-1 px-2 py-1 text-xs font-medium text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded-lg transition-colors"
            >
              <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              说明
            </button>
          </div>
          <div className="flex items-center gap-2">
            <div className="flex-1 bg-gray-50 dark:bg-gray-700 rounded-lg px-3 py-2 font-mono text-xs text-gray-700 dark:text-gray-300 border border-gray-200 dark:border-gray-600 truncate">
              {proxyApiKey || '点击刷新生成'}
            </div>
            <button
              onClick={handleCopyApiKey}
              disabled={!proxyApiKey}
              className="p-2 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 disabled:bg-gray-50 dark:disabled:bg-gray-800 disabled:text-gray-400 dark:disabled:text-gray-600 text-gray-700 dark:text-gray-300 rounded-lg transition-colors"
              title="复制"
            >
              {copySuccess ? (
                <svg className="w-4 h-4 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                </svg>
              ) : (
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                </svg>
              )}
            </button>
            <button
              onClick={handleRefreshApiKey}
              className="p-2 bg-blue-600 dark:bg-blue-500 hover:bg-blue-700 dark:hover:bg-blue-600 text-white rounded-lg transition-colors"
              title="刷新密钥"
            >
              <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
            </button>
          </div>
        </div>
      </div>

      {/* Token Usage Chart with Time Range Tabs */}
      <div className="bg-white dark:bg-gray-800 rounded-lg p-4 shadow-sm border border-gray-200 dark:border-gray-700 transition-colors">
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-sm font-semibold text-gray-900 dark:text-white">Token 使用量</h3>
          {/* Tab 切换 */}
          <div className="flex bg-gray-100 dark:bg-gray-700 rounded-lg p-0.5">
            {(['hour', 'day', 'week', 'month'] as api.TimeRange[]).map((range) => (
              <button
                key={range}
                onClick={() => setTimeRange(range)}
                className={`px-3 py-1 rounded-md text-xs font-medium transition-all ${
                  timeRange === range
                    ? 'bg-white dark:bg-gray-600 text-gray-900 dark:text-white shadow-sm'
                    : 'text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white'
                }`}
              >
                {timeRangeLabels[range]}
              </button>
            ))}
          </div>
        </div>

        {/* Simple Bar Chart */}
        <div className="space-y-1.5">
          {isLoadingTokenData ? (
            <div className="flex items-center justify-center h-40">
              <div className="text-gray-500 dark:text-gray-400 text-sm">加载中...</div>
            </div>
          ) : currentData.length === 0 ? (
            <div className="flex items-center justify-center h-40">
              <div className="text-gray-400 dark:text-gray-500 text-sm">暂无数据</div>
            </div>
          ) : (
            <>
              {/* Bars */}
              <div className="flex items-end justify-between h-40 gap-1">
                {currentData.map((data, index) => {
                  const totalHeight = (data.tokens / maxTokens) * 100
                  const totalHeightPx = (totalHeight / 100) * 160 // 160px = h-40
                  const cacheTokens = data.cacheReadTokens || 0
                  const regularTokens = data.tokens - cacheTokens
                  const cacheHeightPx = cacheTokens > 0 ? (cacheTokens / data.tokens) * totalHeightPx : 0
                  const regularHeightPx = totalHeightPx - cacheHeightPx

                  return (
                    <div key={index} className="flex-1 flex items-end" style={{ height: '160px' }}>
                      <div className="w-full flex flex-col relative group cursor-pointer" style={{ height: `${totalHeightPx}px` }}>
                        {/* 缓存命中部分（顶部，琥珀色） */}
                        {cacheHeightPx > 0 && (
                          <div
                            className="w-full bg-gradient-to-t from-amber-500 to-amber-400 rounded-t hover:from-amber-600 hover:to-amber-500 transition-all"
                            style={{ height: `${cacheHeightPx}px` }}
                          />
                        )}
                        {/* 常规 token 部分（底部，蓝色） */}
                        <div
                          className="w-full bg-gradient-to-t from-blue-500 to-blue-400 hover:from-blue-600 hover:to-blue-500 transition-all min-h-[2px]"
                          style={{
                            height: `${regularHeightPx}px`,
                            borderTopLeftRadius: cacheHeightPx > 0 ? '0' : '0.25rem',
                            borderTopRightRadius: cacheHeightPx > 0 ? '0' : '0.25rem'
                          }}
                        />
                        {/* Tooltip on hover */}
                        <div className="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-1.5 px-2 py-1 bg-gray-900 dark:bg-gray-700 text-white text-xs rounded opacity-0 group-hover:opacity-100 transition-opacity whitespace-nowrap pointer-events-none z-10">
                          {data.label}<br/>
                          总计: {data.tokens.toLocaleString()} tokens
                          {cacheTokens > 0 && (
                            <>
                              <br/>
                              <span className="text-amber-400">缓存: {cacheTokens.toLocaleString()} tokens</span>
                            </>
                          )}
                        </div>
                      </div>
                    </div>
                  )
                })}
              </div>

              {/* Time axis labels - separate row */}
              <div className="flex justify-between">
                {currentData.map((data, index) => {
                  // 根据数据量决定显示间隔
                  const showLabel = timeRange === 'hour'
                    ? index % 3 === 0  // 小时视图：每3个显示一次
                    : timeRange === 'month'
                    ? index % 2 === 0  // 月视图：每2个显示一次
                    : true  // 天和周视图：全部显示

                  return (
                    <div key={index} className="flex-1 text-center">
                      {showLabel && (
                        <span className="text-xs text-gray-500 dark:text-gray-400">{data.label}</span>
                      )}
                    </div>
                  )
                })}
              </div>
            </>
          )}
        </div>

        {/* Legend */}
        <div className="mt-3 pt-3 border-t border-gray-100 dark:border-gray-700 flex items-center justify-between text-xs">
          <div className="flex items-center gap-3">
            <div className="flex items-center gap-1.5">
              <div className="w-2.5 h-2.5 bg-blue-500 rounded"></div>
              <span className="text-gray-600 dark:text-gray-400">常规 Token</span>
            </div>
            <div className="flex items-center gap-1.5">
              <div className="w-2.5 h-2.5 bg-amber-500 rounded"></div>
              <span className="text-gray-600 dark:text-gray-400">缓存命中</span>
            </div>
          </div>
          <div className="flex items-center gap-3 text-gray-500 dark:text-gray-400">
            <div>
              峰值: <span className="font-semibold text-gray-900 dark:text-white">{maxTokens.toLocaleString()}</span> tokens
            </div>
            {totalCacheHits > 0 && (
              <div>
                缓存: <span className="font-semibold text-amber-600 dark:text-amber-400">{totalCacheHits.toLocaleString()}</span> tokens
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Setup Guide Modal */}
      <SetupGuideModal
        isOpen={isSetupGuideOpen}
        onClose={() => setIsSetupGuideOpen(false)}
        proxyApiKey={proxyApiKey}
        proxyServerUrl={proxyServerUrl}
      />
    </div>
  )
}
