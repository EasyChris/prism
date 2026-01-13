import { ReactNode } from "react"
import { TabNavigation } from "./TabNavigation"
import { useTheme } from "@/contexts/ThemeContext"

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
  const { theme, toggleTheme } = useTheme()

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors">
      {/* Header */}
      <header className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 transition-colors">
        <div className="max-w-7xl mx-auto px-8 py-6">
          <div className="flex items-center justify-between">
            {/* Logo */}
            <h1 className="text-xl font-semibold text-gray-900 dark:text-white">
              Prism Hub
            </h1>

            {/* Tab Navigation */}
            <TabNavigation
              tabs={tabs}
              activeTab={activeTab}
              onTabChange={onTabChange}
            />

            {/* Right Actions */}
            <div className="flex items-center gap-4">
              {/* Theme Toggle Button */}
              <button
                onClick={toggleTheme}
                className="p-2 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors"
                title={theme === 'light' ? '切换到暗色模式' : '切换到亮色模式'}
              >
                {theme === 'light' ? (
                  // 月亮图标 (暗色模式)
                  <svg
                    className="w-5 h-5 text-gray-600 dark:text-gray-300"
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
                ) : (
                  // 太阳图标 (亮色模式)
                  <svg
                    className="w-5 h-5 text-gray-300"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={2}
                      d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z"
                    />
                  </svg>
                )}
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
