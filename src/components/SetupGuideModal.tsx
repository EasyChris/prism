import { useState } from "react"

interface SetupGuideModalProps {
  isOpen: boolean
  onClose: () => void
  proxyApiKey: string | null
  proxyServerUrl: string
}

export function SetupGuideModal({ isOpen, onClose, proxyApiKey, proxyServerUrl }: SetupGuideModalProps) {
  const [copiedStep, setCopiedStep] = useState<string | null>(null)

  if (!isOpen) return null

  const configPath = "~/.config/claude/config.json"

  const configExample = `{
  "apiConfiguration": {
    "baseURL": "${proxyServerUrl || 'http://127.0.0.1:3000'}",
    "apiKey": "${proxyApiKey || 'your-proxy-api-key'}"
  }
}`

  const handleCopy = async (text: string, step: string) => {
    try {
      await navigator.clipboard.writeText(text)
      setCopiedStep(step)
      setTimeout(() => setCopiedStep(null), 2000)
    } catch (error) {
      console.error("Failed to copy:", error)
    }
  }

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onClick={onClose}>
      <div
        className="bg-white dark:bg-gray-800 rounded-xl shadow-2xl max-w-2xl w-full mx-4 max-h-[90vh] overflow-y-auto"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="sticky top-0 bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 px-6 py-4 flex items-center justify-between">
          <h2 className="text-xl font-semibold text-gray-900 dark:text-white">
            如何配置 Claude Code 代理
          </h2>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 transition-colors"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        {/* Content */}
        <div className="p-6 space-y-6">
          {/* Step 1 */}
          <div className="space-y-3">
            <div className="flex items-center gap-2">
              <div className="w-8 h-8 bg-blue-600 text-white rounded-full flex items-center justify-center font-semibold text-sm">
                1
              </div>
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                找到 Claude Code 配置文件
              </h3>
            </div>
            <p className="text-gray-600 dark:text-gray-400 ml-10">
              配置文件位于以下路径：
            </p>
            <div className="ml-10 relative">
              <div className="bg-gray-50 dark:bg-gray-900 rounded-lg px-4 py-3 font-mono text-sm text-gray-700 dark:text-gray-300 border border-gray-200 dark:border-gray-700 flex items-center justify-between">
                <code>{configPath}</code>
                <button
                  onClick={() => handleCopy(configPath, "path")}
                  className="ml-2 p-1.5 hover:bg-gray-200 dark:hover:bg-gray-700 rounded transition-colors"
                  title="复制路径"
                >
                  {copiedStep === "path" ? (
                    <svg className="w-4 h-4 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                    </svg>
                  ) : (
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                    </svg>
                  )}
                </button>
              </div>
            </div>
            <p className="text-sm text-gray-500 dark:text-gray-400 ml-10">
              💡 提示：可以使用命令 <code className="bg-gray-100 dark:bg-gray-700 px-2 py-0.5 rounded">open ~/.config/claude</code> 快速打开配置目录
            </p>
          </div>

          {/* Step 2 */}
          <div className="space-y-3">
            <div className="flex items-center gap-2">
              <div className="w-8 h-8 bg-blue-600 text-white rounded-full flex items-center justify-center font-semibold text-sm">
                2
              </div>
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                修改配置文件
              </h3>
            </div>
            <p className="text-gray-600 dark:text-gray-400 ml-10">
              在配置文件中找到 <code className="bg-gray-100 dark:bg-gray-700 px-2 py-0.5 rounded">apiConfiguration</code> 部分，修改为以下内容：
            </p>
            <div className="ml-10 relative">
              <div className="bg-gray-50 dark:bg-gray-900 rounded-lg px-4 py-3 font-mono text-sm text-gray-700 dark:text-gray-300 border border-gray-200 dark:border-gray-700">
                <div className="flex items-start justify-between">
                  <pre className="flex-1 overflow-x-auto">{configExample}</pre>
                  <button
                    onClick={() => handleCopy(configExample, "config")}
                    className="ml-2 p-1.5 hover:bg-gray-200 dark:hover:bg-gray-700 rounded transition-colors flex-shrink-0"
                    title="复制配置"
                  >
                    {copiedStep === "config" ? (
                      <svg className="w-4 h-4 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 13l4 4L19 7" />
                      </svg>
                    ) : (
                      <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                      </svg>
                    )}
                  </button>
                </div>
              </div>
            </div>
            <div className="ml-10 bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800 rounded-lg p-3">
              <p className="text-sm text-amber-800 dark:text-amber-200">
                ⚠️ 注意：请将 <code className="bg-amber-100 dark:bg-amber-800 px-1.5 py-0.5 rounded">apiKey</code> 替换为上方显示的代理服务 API 密钥
              </p>
            </div>
          </div>

          {/* Step 3 */}
          <div className="space-y-3">
            <div className="flex items-center gap-2">
              <div className="w-8 h-8 bg-blue-600 text-white rounded-full flex items-center justify-center font-semibold text-sm">
                3
              </div>
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                重启 Claude Code
              </h3>
            </div>
            <p className="text-gray-600 dark:text-gray-400 ml-10">
              保存配置文件后，需要重启 Claude Code 才能使配置生效。
            </p>
          </div>

          {/* Step 4 */}
          <div className="space-y-3">
            <div className="flex items-center gap-2">
              <div className="w-8 h-8 bg-blue-600 text-white rounded-full flex items-center justify-center font-semibold text-sm">
                4
              </div>
              <h3 className="text-lg font-semibold text-gray-900 dark:text-white">
                验证配置
              </h3>
            </div>
            <p className="text-gray-600 dark:text-gray-400 ml-10">
              配置成功后，在本应用的"日志"页面中可以看到 Claude Code 发送的请求记录。
            </p>
          </div>

          {/* Troubleshooting */}
          <div className="border-t border-gray-200 dark:border-gray-700 pt-6 space-y-3">
            <h3 className="text-lg font-semibold text-gray-900 dark:text-white flex items-center gap-2">
              <svg className="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              遇到问题？
            </h3>
            <ul className="ml-10 space-y-2 text-sm text-gray-600 dark:text-gray-400">
              <li className="flex items-start gap-2">
                <span className="text-blue-600 mt-0.5">•</span>
                <span>确保代理服务已启动（查看仪表盘中的"服务状态"）</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-blue-600 mt-0.5">•</span>
                <span>确保 API 密钥已正确复制（不要包含多余的空格）</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-blue-600 mt-0.5">•</span>
                <span>确保配置文件的 JSON 格式正确（可以使用 JSON 验证工具检查）</span>
              </li>
              <li className="flex items-start gap-2">
                <span className="text-blue-600 mt-0.5">•</span>
                <span>如果仍然无法连接，尝试重启本应用和 Claude Code</span>
              </li>
            </ul>
          </div>
        </div>

        {/* Footer */}
        <div className="sticky bottom-0 bg-gray-50 dark:bg-gray-900 border-t border-gray-200 dark:border-gray-700 px-6 py-4 flex justify-end">
          <button
            onClick={onClose}
            className="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg transition-colors font-medium"
          >
            我知道了
          </button>
        </div>
      </div>
    </div>
  )
}
