import { useEffect, useState } from "react"
import { getLogs, RequestLog } from "../lib/api"

export function Logs() {
  const [logs, setLogs] = useState<RequestLog[]>([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    loadLogs()
  }, [])

  const loadLogs = async () => {
    try {
      setLoading(true)
      const data = await getLogs(100, 0)
      setLogs(data)
    } catch (error) {
      console.error("Failed to load logs:", error)
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
        return "bg-blue-100 text-blue-800"
      case "override":
        return "bg-orange-100 text-orange-800"
      case "mapping":
        return "bg-green-100 text-green-800"
      default:
        return "bg-gray-100 text-gray-800"
    }
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-semibold text-gray-900">请求日志</h2>
        <div className="flex items-center gap-3">
          <input
            type="text"
            placeholder="搜索日志..."
            className="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button className="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors">
            导出
          </button>
        </div>
      </div>

      {/* Logs Table */}
      <div className="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead className="bg-gray-50 border-b border-gray-200">
              <tr>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  时间
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  配置
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  原始模型
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  处理模式
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  转发模型
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Token
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  请求大小
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  耗时
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  上游耗时
                </th>
                <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  状态
                </th>
              </tr>
            </thead>
          <tbody className="divide-y divide-gray-200">
            {loading ? (
              <tr>
                <td colSpan={10} className="px-6 py-12 text-center text-gray-500">
                  加载中...
                </td>
              </tr>
            ) : logs.length === 0 ? (
              <tr>
                <td colSpan={10} className="px-6 py-12 text-center text-gray-500">
                  暂无日志记录
                </td>
              </tr>
            ) : (
              logs.map((log, index) => (
                <tr key={index} className="hover:bg-gray-50">
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900">
                    {formatTimestamp(log.timestamp)}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900">
                    {log.profileName}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900">
                    {log.originalModel}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap">
                    <span className={`px-2 py-1 text-xs rounded-full ${getModeModeColor(log.modelMode)}`}>
                      {log.modelMode}
                    </span>
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900">
                    {log.originalModel === log.forwardedModel ? (
                      <span className="text-gray-500">-</span>
                    ) : (
                      log.forwardedModel
                    )}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900">
                    {log.inputTokens + log.outputTokens}
                    <span className="text-gray-500 text-xs ml-1">
                      ({log.inputTokens}/{log.outputTokens})
                    </span>
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900">
                    {formatBytes(log.requestSizeBytes)}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900">
                    {formatDuration(log.durationMs)}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap text-sm text-gray-900">
                    {log.upstreamDurationMs ? formatDuration(log.upstreamDurationMs) : "-"}
                  </td>
                  <td className="px-4 py-4 whitespace-nowrap">
                    <span
                      className={`px-2 py-1 text-xs rounded-full ${
                        log.statusCode === 200
                          ? "bg-green-100 text-green-800"
                          : "bg-red-100 text-red-800"
                      }`}
                    >
                      {log.statusCode}
                    </span>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
        </div>
      </div>
    </div>
  )
}
