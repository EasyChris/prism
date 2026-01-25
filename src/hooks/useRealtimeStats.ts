import { useEffect, useRef } from 'react'
import { getDashboardStats, DashboardStats } from '../lib/api'

interface UseRealtimeStatsOptions {
  onStatsUpdate?: (stats: DashboardStats) => void
  enabled?: boolean
  refreshInterval?: number // 刷新间隔（毫秒），默认 5000ms
}

/**
 * Hook for periodically fetching and updating dashboard statistics
 *
 * 由于移除了后端的自动统计推送（避免频繁数据库查询），
 * 改为前端定时主动查询统计数据
 *
 * @param options - Configuration options
 * @param options.onStatsUpdate - Callback when statistics are updated
 * @param options.enabled - Whether to enable periodic updates (default: true)
 * @param options.refreshInterval - Refresh interval in milliseconds (default: 5000)
 *
 * @example
 * ```tsx
 * const [stats, setStats] = useState<DashboardStats | null>(null)
 *
 * useRealtimeStats({
 *   onStatsUpdate: (newStats) => {
 *     setStats(newStats)
 *   },
 *   refreshInterval: 5000 // 每 5 秒刷新一次
 * })
 * ```
 */
export function useRealtimeStats(options: UseRealtimeStatsOptions = {}) {
  const { onStatsUpdate, enabled = true, refreshInterval = 5000 } = options

  // 使用 ref 存储回调函数，避免因回调变化导致重新设置定时器
  const onStatsUpdateRef = useRef(onStatsUpdate)

  // 更新 ref 中的回调函数
  useEffect(() => {
    onStatsUpdateRef.current = onStatsUpdate
  }, [onStatsUpdate])

  useEffect(() => {
    if (!enabled) {
      return
    }

    // 立即执行一次查询
    const fetchStats = async () => {
      try {
        const stats = await getDashboardStats()
        onStatsUpdateRef.current?.(stats)
      } catch (error) {
        console.error('[useRealtimeStats] Failed to fetch stats:', error)
      }
    }

    fetchStats()

    // 设置定时器定期查询
    const intervalId = setInterval(fetchStats, refreshInterval)

    console.log(`[useRealtimeStats] Started polling stats every ${refreshInterval}ms`)

    // Cleanup
    return () => {
      clearInterval(intervalId)
      console.log('[useRealtimeStats] Stopped polling stats')
    }
  }, [enabled, refreshInterval]) // 只依赖 enabled 和 refreshInterval
}
