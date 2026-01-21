import { useState, useEffect } from 'react'
import { ThemeProvider } from './contexts/ThemeContext'
import { LanguageProvider } from './contexts/LanguageContext'
import { Layout } from './components/Layout'
import { Dashboard } from './pages/Dashboard'
import { Profiles } from './pages/Profiles'
import { Logs } from './pages/Logs'
import { Settings } from './pages/Settings'

function App() {
  const [activeTab, setActiveTab] = useState('dashboard')

  // 监听来自子组件的 tab 切换事件
  useEffect(() => {
    const handleSwitchTab = (event: CustomEvent) => {
      setActiveTab(event.detail)
    }

    window.addEventListener('switchTab', handleSwitchTab as EventListener)
    return () => {
      window.removeEventListener('switchTab', handleSwitchTab as EventListener)
    }
  }, [])

  const renderContent = () => {
    switch (activeTab) {
      case 'dashboard':
        return <Dashboard />
      case 'profiles':
        return <Profiles />
      case 'logs':
        return <Logs />
      case 'settings':
        return <Settings />
      default:
        return <Dashboard />
    }
  }

  return (
    <ThemeProvider>
      <LanguageProvider>
        <Layout activeTab={activeTab} onTabChange={setActiveTab}>
          {renderContent()}
        </Layout>
      </LanguageProvider>
    </ThemeProvider>
  )
}

export default App
