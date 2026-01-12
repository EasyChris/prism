export function Settings() {
  return (
    <div className="space-y-6">
      {/* Header */}
      <h2 className="text-2xl font-semibold text-gray-900">设置</h2>

      {/* Settings Sections */}
      <div className="space-y-4">
        {/* Proxy Settings */}
        <div className="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">代理服务</h3>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                监听端口
              </label>
              <input
                type="number"
                defaultValue={3000}
                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              />
            </div>
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm font-medium text-gray-900">服务状态</div>
                <div className="text-xs text-gray-500 mt-1">代理服务当前运行在 127.0.0.1:3000</div>
              </div>
              <button className="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors">
                运行中
              </button>
            </div>
          </div>
        </div>

        {/* Application Settings */}
        <div className="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">应用设置</h3>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm font-medium text-gray-900">开机自启动</div>
                <div className="text-xs text-gray-500 mt-1">系统启动时自动运行应用</div>
              </div>
              <button className="relative inline-flex h-6 w-11 items-center rounded-full bg-gray-200">
                <span className="inline-block h-4 w-4 transform rounded-full bg-white transition translate-x-1" />
              </button>
            </div>
            <div className="flex items-center justify-between">
              <div>
                <div className="text-sm font-medium text-gray-900">主题</div>
                <div className="text-xs text-gray-500 mt-1">选择应用主题</div>
              </div>
              <select className="px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500">
                <option>浅色</option>
                <option>深色</option>
                <option>跟随系统</option>
              </select>
            </div>
          </div>
        </div>

        {/* Data Management */}
        <div className="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">数据管理</h3>
          <div className="space-y-3">
            <button className="w-full px-4 py-2 bg-blue-50 text-blue-700 rounded-lg hover:bg-blue-100 transition-colors text-left">
              导出配置
            </button>
            <button className="w-full px-4 py-2 bg-blue-50 text-blue-700 rounded-lg hover:bg-blue-100 transition-colors text-left">
              导入配置
            </button>
            <button className="w-full px-4 py-2 bg-red-50 text-red-700 rounded-lg hover:bg-red-100 transition-colors text-left">
              清除所有日志
            </button>
          </div>
        </div>

        {/* About */}
        <div className="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
          <h3 className="text-lg font-semibold text-gray-900 mb-4">关于</h3>
          <div className="space-y-2 text-sm text-gray-600">
            <div>版本：0.1.0</div>
            <div>Claude Code Proxy Hub - 专为 Claude Code 打造的动态路由网关</div>
          </div>
        </div>
      </div>
    </div>
  )
}
