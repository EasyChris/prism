import { useState, useEffect } from "react"
import { useTranslation } from "react-i18next"
import { Modal } from "@/components/Modal"
import { ProfileForm } from "@/components/ProfileForm"
import { ConfirmDialog } from "@/components/ConfirmDialog"
import { Edit2, Trash2, CheckCircle2, Circle, Copy } from "lucide-react"
import * as api from "@/lib/api"
import type { Profile } from "@/lib/api"

export function Profiles() {
  const { t } = useTranslation('profiles')
  const [isModalOpen, setIsModalOpen] = useState(false)
  const [editingProfile, setEditingProfile] = useState<Profile | null>(null)
  const [isDeleteDialogOpen, setIsDeleteDialogOpen] = useState(false)
  const [deletingProfile, setDeletingProfile] = useState<Profile | null>(null)
  const [profiles, setProfiles] = useState<Profile[]>([])

  // 加载配置列表
  const loadProfiles = async () => {
    console.log("[Profiles] Loading profiles...")
    try {
      const data = await api.getAllProfiles()
      console.log("[Profiles] Loaded profiles:", data)
      setProfiles(data)
    } catch (error) {
      console.error("[Profiles] Failed to load profiles:", error)
    }
  }

  // 组件挂载时加载配置
  useEffect(() => {
    loadProfiles()
  }, [])

  const handleAddProfile = () => {
    setEditingProfile(null)
    setIsModalOpen(true)
  }

  const handleEditProfile = (profile: Profile) => {
    setEditingProfile(profile)
    setIsModalOpen(true)
  }

  const handleSubmit = async (formData: Omit<Profile, "id" | "isActive">) => {
    try {
      if (editingProfile) {
        // 编辑现有配置
        await api.updateProfile(editingProfile.id, formData)
      } else {
        // 添加新配置
        await api.createProfile(formData)
      }
      setIsModalOpen(false)
      await loadProfiles() // 重新加载配置列表

      // 更新托盘菜单
      try {
        await api.updateTrayMenu()
      } catch (error) {
        console.error("Failed to update tray menu:", error)
      }
    } catch (error) {
      console.error("Failed to save profile:", error)
      alert(t('errors.saveFailed') + error)
    }
  }

  const handleDuplicateProfile = async (profile: Profile) => {
    try {
      // 生成新的配置名称
      let newName = `${profile.name}${t('actions.duplicateSuffix')}`
      let counter = 1

      // 检查名称是否重复，如果重复则添加数字后缀
      while (profiles.some(p => p.name === newName)) {
        counter++
        newName = `${profile.name}${t('actions.duplicateSuffix')} ${counter}`
      }

      // 创建新配置（深拷贝原配置数据，但不包含 id 和 isActive）
      const newProfile: Omit<Profile, "id" | "isActive"> = {
        name: newName,
        apiBaseUrl: profile.apiBaseUrl,
        apiKey: profile.apiKey,
        modelMappingMode: profile.modelMappingMode,
        overrideModel: profile.overrideModel,
        modelMappings: profile.modelMappings.map(rule => ({ ...rule })) // 深拷贝映射规则
      }

      await api.createProfile(newProfile)
      await loadProfiles() // 重新加载配置列表

      // 更新托盘菜单
      try {
        await api.updateTrayMenu()
      } catch (error) {
        console.error("Failed to update tray menu:", error)
      }
    } catch (error) {
      console.error("Failed to duplicate profile:", error)
      alert(t('errors.duplicateFailed') + error)
    }
  }

  const handleDeleteProfile = (profile: Profile) => {
    setDeletingProfile(profile)
    setIsDeleteDialogOpen(true)
  }

  const confirmDelete = async () => {
    if (deletingProfile) {
      try {
        await api.deleteProfile(deletingProfile.id)
        setIsDeleteDialogOpen(false)
        setDeletingProfile(null)
        await loadProfiles() // 重新加载配置列表

        // 更新托盘菜单
        try {
          await api.updateTrayMenu()
        } catch (error) {
          console.error("Failed to update tray menu:", error)
        }
      } catch (error) {
        console.error("Failed to delete profile:", error)
        alert(t('errors.deleteFailed') + error)
      }
    }
  }

  const handleToggleActive = async (profile: Profile) => {
    try {
      await api.activateProfile(profile.id)
      await loadProfiles() // 重新加载配置列表

      // 更新托盘菜单
      try {
        await api.updateTrayMenu()
      } catch (error) {
        console.error("Failed to update tray menu:", error)
      }
    } catch (error) {
      console.error("Failed to activate profile:", error)
      alert(t('errors.activateFailed') + error)
    }
  }

  // 从 API Base URL 提取 Provider 名称
  const extractProvider = (apiBaseUrl: string): string => {
    try {
      const url = new URL(apiBaseUrl)
      const hostname = url.hostname.toLowerCase()

      if (hostname.includes('anthropic')) return 'Anthropic'
      if (hostname.includes('openai')) return 'OpenAI'
      if (hostname.includes('google')) return 'Google'
      if (hostname.includes('azure')) return 'Azure'
      if (hostname.includes('deepseek')) return 'DeepSeek'
      if (hostname.includes('zhipu') || hostname.includes('glm')) return 'GLM'

      // 返回域名的主要部分
      const parts = hostname.split('.')
      if (parts.length >= 2) {
        return parts[parts.length - 2].charAt(0).toUpperCase() + parts[parts.length - 2].slice(1)
      }

      return hostname
    } catch {
      return 'Custom'
    }
  }

  // 获取模型映射模式的显示文本
  const getMappingModeText = (profile: Profile): string => {
    switch (profile.modelMappingMode) {
      case 'passthrough':
        return t('mappingMode.passthrough')
      case 'override':
        return t('mappingMode.overrideLabel', { model: profile.overrideModel || t('mappingMode.unknown') })
      case 'map':
        const count = profile.modelMappings.length
        return t('mappingMode.mapLabel', { count })
      default:
        return t('mappingMode.unknown')
    }
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-semibold text-gray-900 dark:text-white">{t('title')}</h2>
        <button
          onClick={handleAddProfile}
          className="px-4 py-2 bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors"
        >
          + {t('addProfile')}
        </button>
      </div>

      {/* Profiles Grid - 卡片布局 */}
      {profiles.length === 0 ? (
        <div className="text-center py-12 bg-white dark:bg-gray-800 rounded-xl border-2 border-dashed border-gray-300 dark:border-gray-600">
          <p className="text-gray-500 dark:text-gray-400 mb-4">{t('noProfiles')}</p>
          <button
            onClick={handleAddProfile}
            className="px-4 py-2 bg-blue-600 dark:bg-blue-500 text-white rounded-lg hover:bg-blue-700 dark:hover:bg-blue-600 transition-colors"
          >
            {t('addFirstProfile')}
          </button>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {profiles.map((profile) => (
            <div
              key={profile.id}
              className={`relative bg-white dark:bg-gray-800 rounded-lg border-2 transition-all duration-200 hover:shadow-lg hover:-translate-y-1 ${
                profile.isActive
                  ? 'border-blue-500 dark:border-blue-400 shadow-md'
                  : 'border-gray-200 dark:border-gray-700 shadow-sm'
              }`}
            >
              {/* 卡片内容 - 紧凑布局 */}
              <div className="p-4">
                <div className="flex items-start justify-between mb-3">
                  <div className="flex-1 min-w-0">
                    <h3 className="text-base font-semibold text-gray-900 dark:text-white truncate mb-1">
                      {profile.name}
                    </h3>
                    <div className="flex items-center gap-2">
                      <span className="px-2 py-0.5 bg-gray-100 dark:bg-gray-700 rounded text-xs font-medium text-gray-600 dark:text-gray-400">
                        {extractProvider(profile.apiBaseUrl)}
                      </span>
                      <span className="text-xs text-gray-500 dark:text-gray-400">
                        {getMappingModeText(profile)}
                      </span>
                    </div>
                  </div>
                  {profile.isActive && (
                    <span className="ml-2 flex items-center gap-1 px-2 py-0.5 text-xs font-medium bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-400 rounded-full flex-shrink-0">
                      <CheckCircle2 size={12} />
                      {t('common:status.current')}
                    </span>
                  )}
                </div>

                {/* 操作按钮 */}
                <div className="flex items-center justify-between gap-2 pt-3 border-t border-gray-100 dark:border-gray-700">
                  {!profile.isActive ? (
                    <button
                      onClick={() => handleToggleActive(profile)}
                      className="flex items-center gap-1 px-2.5 py-1 text-xs font-medium text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded transition-colors"
                    >
                      <Circle size={14} />
                      {t('common:actions.activate')}
                    </button>
                  ) : (
                    <div className="flex items-center gap-1 px-2.5 py-1 text-xs font-medium text-green-600 dark:text-green-400">
                      <CheckCircle2 size={14} />
                      {t('actions.activated')}
                    </div>
                  )}
                  <div className="flex items-center gap-1">
                    <button
                      onClick={() => handleEditProfile(profile)}
                      className="p-1.5 text-gray-600 dark:text-gray-400 hover:text-blue-600 dark:hover:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded transition-colors"
                      title={t('common:actions.edit')}
                    >
                      <Edit2 size={16} />
                    </button>
                    <button
                      onClick={() => handleDuplicateProfile(profile)}
                      className="p-1.5 text-gray-600 dark:text-gray-400 hover:text-green-600 dark:hover:text-green-400 hover:bg-green-50 dark:hover:bg-green-900/20 rounded transition-colors"
                      title={t('common:actions.duplicate')}
                    >
                      <Copy size={16} />
                    </button>
                    <button
                      onClick={() => handleDeleteProfile(profile)}
                      className="p-1.5 text-gray-600 dark:text-gray-400 hover:text-red-600 dark:hover:text-red-400 hover:bg-red-50 dark:hover:bg-red-900/20 rounded transition-colors"
                      title={t('common:actions.delete')}
                    >
                      <Trash2 size={16} />
                    </button>
                  </div>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Modal */}
      <Modal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        title={editingProfile ? t('editProfile') : t('addProfile')}
      >
        <ProfileForm
          profile={editingProfile || undefined}
          onSubmit={handleSubmit}
          onCancel={() => setIsModalOpen(false)}
        />
      </Modal>

      {/* Delete Confirmation Dialog */}
      <ConfirmDialog
        isOpen={isDeleteDialogOpen}
        onClose={() => setIsDeleteDialogOpen(false)}
        onConfirm={confirmDelete}
        title={t('deleteConfirm.title')}
        message={t('deleteConfirm.message', { name: deletingProfile?.name })}
      />
    </div>
  )
}
