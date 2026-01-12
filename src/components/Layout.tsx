import { ReactNode } from "react"
import { TabNavigation } from "./TabNavigation"

interface LayoutProps {
  children: ReactNode
  activeTab: string
  onTabChange: (tabId: string) => void
}

const tabs = [
  { id: "dashboard", label: "仪表盘" },
  { id: "profiles", label: "配置管理" },
  { id: "logs", label: "请求日志" },
  { id: "settings", label: "设置" },
]

export function Layout({ children, activeTab, onTabChange }: LayoutProps) {
  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <header className="bg-white border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-8 py-6">
          <div className="flex items-center justify-between">
            {/* Logo */}
            <h1 className="text-xl font-semibold text-gray-900">
              Claude Code Proxy Hub
            </h1>

            {/* Tab Navigation */}
            <TabNavigation
              tabs={tabs}
              activeTab={activeTab}
              onTabChange={onTabChange}
            />

            {/* Right Actions */}
            <div className="flex items-center gap-4">
              {/* Theme Toggle Placeholder */}
              <button className="p-2 rounded-lg hover:bg-gray-100">
                <svg
                  className="w-5 h-5 text-gray-600"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
                  />
                </svg>
              </button>
            </div>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto px-8 py-8">
        {children}
      </main>
    </div>
  )
}
