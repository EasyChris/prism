import { cn } from "@/lib/utils"

interface Tab {
  id: string
  label: string
}

interface TabNavigationProps {
  tabs: Tab[]
  activeTab: string
  onTabChange: (tabId: string) => void
}

export function TabNavigation({ tabs, activeTab, onTabChange }: TabNavigationProps) {
  return (
    <div className="flex items-center gap-2">
      {tabs.map((tab) => (
        <button
          key={tab.id}
          onClick={() => onTabChange(tab.id)}
          className={cn(
            "px-6 py-2 rounded-full text-sm font-medium transition-all",
            activeTab === tab.id
              ? "bg-gray-900 dark:bg-gray-600 text-white"
              : "bg-transparent text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
          )}
        >
          {tab.label}
        </button>
      ))}
    </div>
  )
}
