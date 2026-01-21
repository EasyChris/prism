import { RequestLog } from "../lib/api"
import { useTranslation } from "react-i18next"
import { Modal } from "./Modal"

interface LogDetailModalProps {
  log: RequestLog | null
  onClose: () => void
}

export function LogDetailModal({ log, onClose }: LogDetailModalProps) {
  const { t } = useTranslation('logs')

  if (!log) return null

  const formatTimestamp = (timestamp: number) => {
    return new Date(timestamp).toLocaleString("zh-CN")
  }

  const formatDuration = (ms: number) => {
    if (ms < 1000) return `${ms}ms`
    return `${(ms / 1000).toFixed(2)}s`
  }

  const formatJson = (jsonStr: string) => {
    try {
      const parsed = JSON.parse(jsonStr)
      return JSON.stringify(parsed, null, 2)
    } catch {
      return jsonStr
    }
  }

  const copyToClipboard = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text)
      alert(t('common:actions.copySuccess'))
    } catch (error) {
      console.error("Failed to copy:", error)
      alert(t('common:actions.copyFailed'))
    }
  }

  return (
    <Modal isOpen={!!log} onClose={onClose} title={t('detail.title')}>
      <div className="space-y-6">
        {/* 基本信息 */}
        <div className="space-y-3">
          <h3 className="text-sm font-semibold text-gray-900 dark:text-white">{t('detail.basicInfo')}</h3>
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <span className="text-gray-500 dark:text-gray-400">{t('detail.requestTime')}</span>
              <span className="text-gray-900 dark:text-gray-100">{formatTimestamp(log.timestamp)}</span>
            </div>
            <div>
              <span className="text-gray-500 dark:text-gray-400">{t('detail.profileName')}</span>
              <span className="text-gray-900 dark:text-gray-100">{log.profileName}</span>
            </div>
            <div>
              <span className="text-gray-500 dark:text-gray-400">{t('detail.originalModel')}</span>
              <span className="text-gray-900 dark:text-gray-100">{log.originalModel}</span>
            </div>
            <div>
              <span className="text-gray-500 dark:text-gray-400">{t('detail.forwardedModel')}</span>
              <span className="text-gray-900 dark:text-gray-100">{log.forwardedModel}</span>
            </div>
            <div>
              <span className="text-gray-500 dark:text-gray-400">{t('detail.processingMode')}</span>
              <span className="text-gray-900 dark:text-gray-100">{log.modelMode}</span>
            </div>
            <div>
              <span className="text-gray-500 dark:text-gray-400">{t('detail.statusCode')}</span>
              <span className={`font-semibold ${log.statusCode === 200 ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'}`}>
                {log.statusCode}
              </span>
            </div>
          </div>
        </div>

        {/* Token 统计 */}
        <div className="space-y-3">
          <h3 className="text-sm font-semibold text-gray-900 dark:text-white">{t('detail.tokenStats')}</h3>
          <div className="grid grid-cols-2 gap-4 text-sm">
            <div>
              <span className="text-gray-500 dark:text-gray-400">{t('detail.inputTokens')}</span>
              <span className="text-gray-900 dark:text-gray-100">{log.inputTokens}</span>
            </div>
            <div>
              <span className="text-gray-500 dark:text-gray-400">{t('detail.outputTokens')}</span>
              <span className={`font-semibold ${log.outputTokens === 0 ? 'text-red-600 dark:text-red-400' : 'text-gray-900 dark:text-gray-100'}`}>
                {log.outputTokens}
              </span>
            </div>
            <div>
              <span className="text-gray-500 dark:text-gray-400">{t('detail.total')}</span>
              <span className="text-gray-900 dark:text-gray-100">{log.inputTokens + log.outputTokens}</span>
            </div>
            <div>
              <span className="text-gray-500 dark:text-gray-400">{t('detail.duration')}</span>
              <span className="text-gray-900 dark:text-gray-100">{formatDuration(log.durationMs)}</span>
            </div>
          </div>
        </div>

        {/* 响应体 */}
        {log.responseBody && (
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <h3 className="text-sm font-semibold text-gray-900 dark:text-white">{t('detail.responseBody')}</h3>
              <button
                onClick={() => copyToClipboard(log.responseBody!)}
                className="px-3 py-1 text-xs bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
              >
                {t('common:actions.copy')}
              </button>
            </div>
            <div className="relative">
              <pre className="bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-4 text-xs overflow-x-auto max-h-96 overflow-y-auto">
                <code className="text-gray-800 dark:text-gray-200">
                  {formatJson(log.responseBody)}
                </code>
              </pre>
            </div>
          </div>
        )}

        {/* 错误信息 */}
        {log.errorMessage && (
          <div className="space-y-3">
            <h3 className="text-sm font-semibold text-red-600 dark:text-red-400">{t('detail.errorInfo')}</h3>
            <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
              <p className="text-sm text-red-800 dark:text-red-300">{log.errorMessage}</p>
            </div>
          </div>
        )}
      </div>
    </Modal>
  )
}
