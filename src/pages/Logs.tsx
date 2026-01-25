import { useEffect, useState } from "react"
import { useTranslation } from "react-i18next"
import { getLogs, RequestLog } from "../lib/api"
import { LogDetailModal } from "../components/LogDetailModal"
import { useRealtimeLog } from "../hooks/useRealtimeLog"

const MAX_LOGS = 100 // 限制前端保存的日志数量

export function Logs() {
  const { t } = useTranslation('logs')
  const [logs, setLogs] = useState<RequestLog[]>([])
  const [loading, setLoading] = useState(true)
  const [selectedLog, setSelectedLog] = useState<RequestLog | null>(null)
  const [realtimeEnabled, setRealtimeEnabled] = useState(true) // 实时更新开关

  // 初始加载
  useEffect(() => {
    loadLogs()
  }, [])

  // 实时监听日志事件
  useRealtimeLog({
    enabled: realtimeEnabled,
    onNewLog: (log) => {
      // 确保数据有效性
      const validLog = validateLog(log)
      setLogs((prev) => [validLog, ...prev].slice(0, MAX_LOGS))
    },
    onLogUpdated: (log) => {
      // 确保数据有效性
      const validLog = validateLog(log)
      setLogs((prev) =>
        prev.map((l) => (l.requestId === validLog.requestId ? validLog : l))
      )
    },
  })

  // 验证并修复日志数据
  const validateLog = (log: RequestLog): RequestLog => {
    return {
      ...log,
      inputTokens: log.inputTokens || 0,
      outputTokens: log.outputTokens || 0,
      durationMs: log.durationMs || 0,
      statusCode: log.statusCode || 0,
      requestSizeBytes: log.requestSizeBytes || 0,
      responseSizeBytes: log.responseSizeBytes || 0,
    }
  }

  const loadLogs = async () => {
    try {
      setLoading(true)
      const data = await getLogs(100, 0)

      // 验证所有日志数据
      const validatedLogs = data.map(validateLog)

      setLogs(validatedLogs)
    } catch (error) {
      console.error("[Logs] Failed to load logs:", error)
    } finally {
      setLoading(false)
    }
  }

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp).toLocaleString("zh-CN")
  }

  const formatDuration = (ms: number) => {
    if (ms < 1000) return `${ms}ms`
    return `${(ms / 1000).toFixed(2)}s`
  }

  const formatBytes = (bytes?: number) => {
    if (!bytes) return "-"
    if (bytes < 1024) return `${bytes}B`
    return `${(bytes / 1024).toFixed(2)}KB`
  }

  const getModeModeColor = (mode: string) => {
    switch (mode) {
      case "passthrough":
        return "bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-400"
      case "override":
        return "bg-orange-100 dark:bg-orange-900/30 text-orange-800 dark:text-orange-400"
      case "mapping":
        return "bg-green-100 dark:bg-green-900/30 text-green-800 dark:text-green-400"
      default:
        return "bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-300"
    }
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-semibold text-gray-900 dark:text-white">{t('title')}</h2>
        <div className="flex items-center gap-3">
          <button
            onClick={() => setRealtimeEnabled(!realtimeEnabled)}
            className={`px-4 py-2 rounded-lg transition-colors ${
              realtimeEnabled
                ? "bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 hover:bg-green-200 dark:hover:bg-green-900/50"
                : "bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600"
            }`}
          >
            {realtimeEnabled ? t('pauseRealtime') : t('resumeRealtime')}
          </button>
          <input
            type="text"
            placeholder={t('searchPlaceholder')}
            className="px-4 py-2 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400"
          />
          <button
            onClick={loadLogs}
            disabled={loading}
            className="px-4 py-2 bg-blue-500 dark:bg-blue-600 text-white rounded-lg hover:bg-blue-600 dark:hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {loading ? t('common:actions.refreshing') : t('common:actions.refresh')}
          </button>
          <button className="px-4 py-2 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-lg hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors">
            {t('common:actions.export')}
          </button>
        </div>
      </div>

      {/* Logs Table */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead className="bg-gray-50 dark:bg-gray-700 border-b border-gray-200 dark:border-gray-600">
              <tr>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  {t('table.time')}
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  {t('table.profile')}
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  {t('table.originalModel')}
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  {t('table.processingMode')}
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  {t('table.forwardedModel')}
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  {t('table.tokens')}
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  {t('table.requestSize')}
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  {t('table.duration')}
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  {t('table.upstreamDuration')}
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  {t('table.status')}
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider">
                  {t('common:actions.viewDetails')}
                </th>
              </tr>
            </thead>
          <tbody className="divide-y divide-gray-200 dark:divide-gray-700">
            {loading ? (
              <tr>
                <td colSpan={11} className="px-6 py-12 text-center text-gray-500 dark:text-gray-400">
                  {t('common:actions.loading')}
                </td>
              </tr>
            ) : logs.length === 0 ? (
              <tr>
                <td colSpan={11} className="px-6 py-12 text-center text-gray-500 dark:text-gray-400">
                  {t('common:empty.noRecords')}
                </td>
              </tr>
            ) : (
              logs.map((log, index) => (
                <tr key={index} className="hover:bg-gray-50 dark:hover:bg-gray-700/50">
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">
                    {formatTimestamp(log.timestamp)}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">
                    {log.profileName}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">
                    {log.originalModel}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap">
                    <span className={`px-2 py-1 text-xs rounded-full ${getModeModeColor(log.modelMode)}`}>
                      {log.modelMode}
                    </span>
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">
                    {log.originalModel === log.forwardedModel ? (
                      <span className="text-gray-500 dark:text-gray-400">-</span>
                    ) : (
                      log.forwardedModel
                    )}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">
                    <div className="flex items-center gap-2">
                      <span>
                        {log.inputTokens + log.outputTokens}
                        <span className="text-gray-500 dark:text-gray-400 text-xs ml-1">
                          ({log.inputTokens}/{log.outputTokens})
                        </span>
                      </span>
                      {log.outputTokens === 0 && (
                        <span className="text-yellow-500 dark:text-yellow-400" title={t('warnings.zeroOutputTokens')}>
                          ⚠️
                        </span>
                      )}
                    </div>
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">
                    {formatBytes(log.requestSizeBytes)}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">
                    {formatDuration(log.durationMs)}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100">
                    {log.upstreamDurationMs ? formatDuration(log.upstreamDurationMs) : "-"}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap">
                    <span
                      className={`px-2 py-1 text-xs rounded-full ${
                        log.statusCode === 200
                          ? "bg-green-100 dark:bg-green-900/30 text-green-800 dark:text-green-400"
                          : "bg-red-100 dark:bg-red-900/30 text-red-800 dark:text-red-400"
                      }`}
                    >
                      {log.statusCode}
                    </span>
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap">
                    <button
                      onClick={() => setSelectedLog(log)}
                      className="px-3 py-1 text-xs bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400 rounded hover:bg-blue-200 dark:hover:bg-blue-900/50 transition-colors"
                    >
                      {t('common:actions.viewDetails')}
                    </button>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
        </div>
      </div>

      {/* 详情 Modal */}
      <LogDetailModal log={selectedLog} onClose={() => setSelectedLog(null)} />
    </div>
  )
}
