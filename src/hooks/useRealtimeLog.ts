import { useEffect, useCallback } from 'react'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
import { RequestLog } from '../lib/api'

interface UseRealtimeLogOptions {
  onNewLog?: (log: RequestLog) => void
  onLogUpdated?: (log: RequestLog) => void
  enabled?: boolean
}

/**
 * Hook for listening to real-time log events from Tauri backend
 *
 * @param options - Configuration options
 * @param options.onNewLog - Callback when a new log is created
 * @param options.onLogUpdated - Callback when an existing log is updated (e.g., stream completion)
 * @param options.enabled - Whether to enable event listening (default: true)
 *
 * @example
 * ```tsx
 * const [logs, setLogs] = useState<RequestLog[]>([])
 *
 * useRealtimeLog({
 *   onNewLog: (log) => {
 *     setLogs(prev => [log, ...prev].slice(0, 100))
 *   },
 *   onLogUpdated: (log) => {
 *     setLogs(prev => prev.map(l => l.requestId === log.requestId ? log : l))
 *   }
 * })
 * ```
 */
export function useRealtimeLog(options: UseRealtimeLogOptions = {}) {
  const { onNewLog, onLogUpdated, enabled = true } = options

  const handleNewLog = useCallback((log: RequestLog) => {
    console.log('[useRealtimeLog] New log received:', log.requestId)
    onNewLog?.(log)
  }, [onNewLog])

  const handleLogUpdated = useCallback((log: RequestLog) => {
    console.log('[useRealtimeLog] Log updated:', log.requestId)
    onLogUpdated?.(log)
  }, [onLogUpdated])

  useEffect(() => {
    if (!enabled) {
      return
    }

    let unlistenNewLog: UnlistenFn | undefined
    let unlistenLogUpdated: UnlistenFn | undefined

    // Listen to new-log events
    const setupNewLogListener = async () => {
      try {
        unlistenNewLog = await listen<RequestLog>('new-log', (event) => {
          handleNewLog(event.payload)
        })
        console.log('[useRealtimeLog] Listening to new-log events')
      } catch (error) {
        console.error('[useRealtimeLog] Failed to setup new-log listener:', error)
      }
    }

    // Listen to log-updated events
    const setupLogUpdatedListener = async () => {
      try {
        unlistenLogUpdated = await listen<RequestLog>('log-updated', (event) => {
          handleLogUpdated(event.payload)
        })
        console.log('[useRealtimeLog] Listening to log-updated events')
      } catch (error) {
        console.error('[useRealtimeLog] Failed to setup log-updated listener:', error)
      }
    }

    setupNewLogListener()
    setupLogUpdatedListener()

    // Cleanup listeners on unmount
    return () => {
      if (unlistenNewLog) {
        unlistenNewLog()
        console.log('[useRealtimeLog] Unlistened from new-log events')
      }
      if (unlistenLogUpdated) {
        unlistenLogUpdated()
        console.log('[useRealtimeLog] Unlistened from log-updated events')
      }
    }
  }, [enabled, handleNewLog, handleLogUpdated])
}
