import { useEffect, useState } from "react"
import { useTranslation } from "react-i18next"
import { getProxyConfig, setProxyConfig, getProxyStatus, getAppVersion, type ProxyConfig, type ProxyServerStatus } from "../lib/api"
import { UpdateDialog } from "../components/UpdateDialog"

export function Settings() {
  const { t } = useTranslation('settings')
  const [proxyConfig, setProxyConfigState] = useState<ProxyConfig>({ host: "127.0.0.1", port: 15288 })
  const [proxyStatus, setProxyStatus] = useState<ProxyServerStatus | null>(null)
  const [appVersion, setAppVersion] = useState<string>("0.1.0")
  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)
  const [message, setMessage] = useState<{ type: 'success' | 'error', text: string } | null>(null)
  const [showUpdateDialog, setShowUpdateDialog] = useState(false)

  // 加载配置和状态
  useEffect(() => {
    async function loadConfig() {
      try {
        const [config, status, version] = await Promise.all([
          getProxyConfig(),
          getProxyStatus(),
          getAppVersion()
        ])
        setProxyConfigState(config)
        setProxyStatus(status)
        setAppVersion(version)
      } catch (error) {
        console.error("Failed to load proxy config:", error)
        showMessage("error", t('messages.loadConfigFailed'))
      } finally {
        setLoading(false)
      }
    }
    loadConfig()
  }, [])

  // 保存配置并自动重启
  async function handleSaveConfig() {
    setSaving(true)
    try {
      await setProxyConfig(proxyConfig)
      showMessage("success", t('messages.configSaved'))

      // 等待一下后刷新状态
      setTimeout(async () => {
        try {
          const status = await getProxyStatus()
          setProxyStatus(status)
        } catch (error) {
          console.error("Failed to refresh status:", error)
        }
      }, 1000)
    } catch (error: any) {
      console.error("Failed to save proxy config:", error)
      showMessage("error", error?.toString() || t('messages.saveConfigFailed'))
    } finally {
      setSaving(false)
    }
  }

  // 显示消息
  function showMessage(type: 'success' | 'error', text: string) {
    setMessage({ type, text })
    setTimeout(() => setMessage(null), 3000)
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <h2 className="text-2xl font-semibold text-gray-900 dark:text-white">{t('title')}</h2>

      {/* Message Toast */}
      {message && (
        <div className={`fixed top-4 right-4 px-4 py-3 rounded-lg shadow-lg z-50 ${
          message.type === 'success' ? 'bg-green-500' : 'bg-red-500'
        } text-white`}>
          {message.text}
        </div>
      )}

      {/* Settings Sections */}
      <div className="space-y-4">
        {/* Proxy Settings */}
        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">{t('sections.proxyService')}</h3>
          <div className="space-y-4">
            {loading ? (
              <div className="text-center py-4 text-gray-500 dark:text-gray-400">
                {t('common:actions.loading')}
              </div>
            ) : (
              <>
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                      {t('proxyService.listenAddress')}
                    </label>
                    <input
                      type="text"
                      value={proxyConfig.host}
                      onChange={(e) => setProxyConfigState({ ...proxyConfig, host: e.target.value })}
                      className="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
                      placeholder="127.0.0.1"
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                      {t('proxyService.listenPort')}
                    </label>
                    <input
                      type="number"
                      value={proxyConfig.port}
                      onChange={(e) => setProxyConfigState({ ...proxyConfig, port: parseInt(e.target.value) || 15288 })}
                      className="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
                      placeholder="15288"
                    />
                  </div>
                </div>
                <div className="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-900 rounded-lg">
                  <div>
                    <div className="text-sm font-medium text-gray-900 dark:text-white">{t('proxyService.serviceStatus')}</div>
                    <div className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                      {proxyStatus?.isRunning ? (
                        <>{t('proxyService.runningAt', { addr: proxyStatus.addr })}</>
                      ) : (
                        t('proxyService.notRunning')
                      )}
                    </div>
                    {proxyStatus?.startedAt && (
                      <div className="text-xs text-gray-400 dark:text-gray-500 mt-1">
                        {t('proxyService.startTime', { date: new Date(proxyStatus.startedAt * 1000).toLocaleString() })}
                      </div>
                    )}
                  </div>
                  <div className={`px-4 py-2 rounded-lg ${
                    proxyStatus?.isRunning
                      ? 'bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400'
                      : 'bg-gray-100 dark:bg-gray-900/30 text-gray-700 dark:text-gray-400'
                  }`}>
                    {proxyStatus?.isRunning ? t('common:status.running') : t('common:status.stopped')}
                  </div>
                </div>
                <button
                  onClick={handleSaveConfig}
                  disabled={saving}
                  className="w-full px-4 py-2 bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                >
                  {saving ? t('proxyService.savingAndRestarting') : t('proxyService.saveConfig')}
                </button>
              </>
            )}
          </div>
        </div>

        {/* Application Settings */}
        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">{t('sections.appSettings')}</h3>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm font-medium text-gray-900 dark:text-white">{t('appSettings.autoStart')}</div>
                <div className="text-xs text-gray-500 dark:text-gray-400 mt-1">{t('appSettings.autoStartDesc')}</div>
              </div>
              <button className="relative inline-flex h-6 w-11 items-center rounded-full bg-gray-200 dark:bg-gray-700">
                <span className="inline-block h-4 w-4 transform rounded-full bg-white transition translate-x-1" />
              </button>
            </div>
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm font-medium text-gray-900 dark:text-white">{t('appSettings.theme')}</div>
                <div className="text-xs text-gray-500 dark:text-gray-400 mt-1">{t('appSettings.themeDesc')}</div>
              </div>
              <div className="relative">
                <select className="px-4 py-2.5 pr-10 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-700 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500 dark:focus:ring-blue-400 min-w-[140px] appearance-none cursor-pointer">
                  <option value="light">{t('common:theme.light')}</option>
                  <option value="dark">{t('common:theme.dark')}</option>
                  <option value="system">{t('common:theme.system')}</option>
                </select>
                <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-3 text-gray-500 dark:text-gray-400">
                  <svg className="h-4 w-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
                  </svg>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* Data Management */}
        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">{t('sections.dataManagement')}</h3>
          <div className="space-y-3">
            <button className="w-full px-4 py-2 bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-400 rounded-lg hover:bg-blue-100 dark:hover:bg-blue-900/30 transition-colors text-left">
              {t('dataManagement.exportConfig')}
            </button>
            <button className="w-full px-4 py-2 bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-400 rounded-lg hover:bg-blue-100 dark:hover:bg-blue-900/30 transition-colors text-left">
              {t('dataManagement.importConfig')}
            </button>
            <button className="w-full px-4 py-2 bg-red-50 dark:bg-red-900/20 text-red-700 dark:text-red-400 rounded-lg hover:bg-red-100 dark:hover:bg-red-900/30 transition-colors text-left">
              {t('dataManagement.clearLogs')}
            </button>
          </div>
        </div>

        {/* About */}
        <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
          <h3 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">{t('sections.about')}</h3>
          <div className="space-y-4">
            <div className="space-y-2 text-sm text-gray-600 dark:text-gray-300">
              <div>{t('about.version')} {appVersion}</div>
              <div>{t('about.description')}</div>
            </div>
            <button
              onClick={() => setShowUpdateDialog(true)}
              className="w-full px-4 py-2 bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-400 rounded-lg hover:bg-blue-100 dark:hover:bg-blue-900/30 transition-colors"
            >
              {t('about.checkUpdate')}
            </button>
          </div>
        </div>
      </div>

      {/* Update Dialog */}
      <UpdateDialog isOpen={showUpdateDialog} onClose={() => setShowUpdateDialog(false)} />
    </div>
  )
}
