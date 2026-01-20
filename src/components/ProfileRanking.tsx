import { useState, useEffect } from "react"
import * as api from "@/lib/api"

interface ProfileRankingProps {
  timeRange?: api.TimeRange
  limit?: number
  className?: string
}

export function ProfileRanking({ timeRange, limit = 10, className = "" }: ProfileRankingProps) {
  const [rankings, setRankings] = useState<api.ProfileConsumption[]>([])
  const [isLoading, setIsLoading] = useState(false)

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

  // 加载排名数据
  const loadRankings = async () => {
    setIsLoading(true)
    try {
      const data = await api.getProfileConsumptionRanking(timeRange, limit)
      setRankings(data)
    } catch (error) {
      console.error("Failed to load profile rankings:", error)
      setRankings([])
    } finally {
      setIsLoading(false)
    }
  }

  // 当时间范围改变时重新加载
  useEffect(() => {
    loadRankings()
  }, [timeRange, limit])

  // 获取排名徽章样式
  const getRankBadgeStyle = (rank: number) => {
    switch (rank) {
      case 1:
        return "bg-amber-100 text-amber-700 border-amber-300 dark:bg-amber-900/30 dark:text-amber-400 dark:border-amber-700"
      case 2:
        return "bg-gray-100 text-gray-700 border-gray-300 dark:bg-gray-700 dark:text-gray-300 dark:border-gray-600"
      case 3:
        return "bg-orange-100 text-orange-700 border-orange-300 dark:bg-orange-900/30 dark:text-orange-400 dark:border-orange-700"
      default:
        return "bg-blue-50 text-blue-700 border-blue-200 dark:bg-blue-900/30 dark:text-blue-400 dark:border-blue-700"
    }
  }

  // 获取进度条颜色
  const getProgressBarColor = (rank: number) => {
    switch (rank) {
      case 1:
        return "bg-gradient-to-r from-amber-400 to-amber-500 dark:from-amber-500 dark:to-amber-600"
      case 2:
        return "bg-gradient-to-r from-gray-400 to-gray-500 dark:from-gray-500 dark:to-gray-600"
      case 3:
        return "bg-gradient-to-r from-orange-400 to-orange-500 dark:from-orange-500 dark:to-orange-600"
      default:
        return "bg-gradient-to-r from-blue-400 to-blue-500 dark:from-blue-500 dark:to-blue-600"
    }
  }

  // 获取排名图标
  const getRankIcon = (rank: number) => {
    switch (rank) {
      case 1:
        return "🥇"
      case 2:
        return "🥈"
      case 3:
        return "🥉"
      default:
        return null
    }
  }

  return (
    <div className={className}>
      <h3 className="text-sm font-semibold text-gray-900 dark:text-white mb-3">配置消耗排名</h3>

      {isLoading ? (
        <div className="flex items-center justify-center h-40">
          <div className="text-gray-500 dark:text-gray-400 text-sm">加载中...</div>
        </div>
      ) : rankings.length === 0 ? (
        <div className="flex items-center justify-center h-40">
          <div className="text-gray-400 dark:text-gray-500 text-sm">暂无数据</div>
        </div>
      ) : (
        <div className="space-y-3">
          {rankings.map((profile) => (
            <div
              key={profile.profileId}
              className="group"
            >
              {/* 排名和配置名称 */}
              <div className="flex items-center justify-between mb-1.5">
                <div className="flex items-center gap-2">
                  {/* 排名徽章 */}
                  <div
                    className={`flex items-center justify-center w-8 h-6 rounded text-xs font-bold border ${getRankBadgeStyle(profile.rank)}`}
                  >
                    {getRankIcon(profile.rank) || profile.rank}
                  </div>
                  {/* 配置名称 */}
                  <span className="text-sm font-medium text-gray-900 dark:text-white truncate max-w-[200px]">
                    {profile.profileName}
                  </span>
                </div>
                {/* Token 数量和百分比 */}
                <div className="flex items-center gap-2">
                  <span className="text-sm font-semibold text-gray-900 dark:text-white">
                    {formatNumber(profile.totalTokens)}
                  </span>
                  <span className="text-xs text-gray-500 dark:text-gray-400">
                    ({profile.percentage.toFixed(1)}%)
                  </span>
                </div>
              </div>

              {/* 进度条 */}
              <div className="w-full bg-gray-100 dark:bg-gray-700 rounded-full h-2 overflow-hidden">
                <div
                  className={`h-full transition-all duration-300 ${getProgressBarColor(profile.rank)}`}
                  style={{ width: `${profile.percentage}%` }}
                />
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
