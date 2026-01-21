import { useEffect, useCallback } from 'react'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { DashboardStats } from '../lib/api'

interface UseRealtimeStatsOptions {
  onStatsUpdate?: (stats: DashboardStats) => void
  enabled?: boolean
}

/**
 * Hook for listening to real-time statistics updates from Tauri backend
 *
 * @param options - Configuration options
 * @param options.onStatsUpdate - Callback when statistics are updated
 * @param options.enabled - Whether to enable event listening (default: true)
 *
 * @example
 * ```tsx
 * const [stats, setStats] = useState<DashboardStats | null>(null)
 *
 * useRealtimeStats({
 *   onStatsUpdate: (newStats) => {
 *     setStats(newStats)
 *   }
 * })
 * ```
 */
export function useRealtimeStats(options: UseRealtimeStatsOptions = {}) {
  const { onStatsUpdate, enabled = true } = options

  const handleStatsUpdate = useCallback((stats: DashboardStats) => {
    console.log('[useRealtimeStats] Stats updated:', stats)
    onStatsUpdate?.(stats)
  }, [onStatsUpdate])

  useEffect(() => {
    if (!enabled) {
      return
    }

    let unlisten: UnlistenFn | undefined

    // Listen to stats-update events
    const setupListener = async () => {
      try {
        unlisten = await listen<DashboardStats>('stats-update', (event) => {
          handleStatsUpdate(event.payload)
        })
        console.log('[useRealtimeStats] Listening to stats-update events')
      } catch (error) {
        console.error('[useRealtimeStats] Failed to setup stats-update listener:', error)
      }
    }

    setupListener()

    // Cleanup listener on unmount
    return () => {
      if (unlisten) {
        unlisten()
        console.log('[useRealtimeStats] Unlistened from stats-update events')
      }
    }
  }, [enabled, handleStatsUpdate])
}
