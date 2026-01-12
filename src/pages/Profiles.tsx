import { useState, useEffect } from "react"
import { Modal } from "@/components/Modal"
import { ProfileForm } from "@/components/ProfileForm"
import { ConfirmDialog } from "@/components/ConfirmDialog"
import * as api from "@/lib/api"

interface Profile {
  id: string
  name: string
  apiBaseUrl: string
  apiKey: string
  modelId: string
  isActive: boolean
}

export function Profiles() {
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
        await api.updateProfile(
          editingProfile.id,
          formData.name,
          formData.apiBaseUrl,
          formData.apiKey,
          formData.modelId
        )
      } else {
        // 添加新配置
        await api.createProfile(
          formData.name,
          formData.apiBaseUrl,
          formData.apiKey,
          formData.modelId
        )
      }
      setIsModalOpen(false)
      await loadProfiles() // 重新加载配置列表
    } catch (error) {
      console.error("Failed to save profile:", error)
      alert("保存配置失败：" + error)
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
      } catch (error) {
        console.error("Failed to delete profile:", error)
        alert("删除配置失败：" + error)
      }
    }
  }

  const handleToggleActive = async (profile: Profile) => {
    try {
      await api.activateProfile(profile.id)
      await loadProfiles() // 重新加载配置列表
    } catch (error) {
      console.error("Failed to activate profile:", error)
      alert("激活配置失败：" + error)
    }
  }

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <h2 className="text-2xl font-semibold text-gray-900">配置管理</h2>
        <button
          onClick={handleAddProfile}
          className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
        >
          + 添加配置
        </button>
      </div>

      {/* Profiles List */}
      <div className="bg-white rounded-xl shadow-sm border border-gray-200 overflow-hidden">
        <table className="w-full">
          <thead className="bg-gray-50 border-b border-gray-200">
            <tr>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                配置名称
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                API 地址
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                模型
              </th>
              <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                状态
              </th>
              <th className="px-6 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                操作
              </th>
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200">
            {profiles.map((profile) => (
              <tr key={profile.id} className="hover:bg-gray-50">
                <td className="px-6 py-4 whitespace-nowrap">
                  <div className="flex items-center">
                    <span className="text-sm font-medium text-gray-900">{profile.name}</span>
                    {profile.isActive && (
                      <span className="ml-2 px-2 py-1 text-xs font-medium bg-blue-100 text-blue-800 rounded">
                        当前
                      </span>
                    )}
                  </div>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-600">
                  {profile.apiBaseUrl}
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-600">
                  {profile.modelId}
                </td>
                <td className="px-6 py-4 whitespace-nowrap">
                  <button
                    onClick={() => handleToggleActive(profile)}
                    disabled={profile.isActive}
                    className={`px-2 py-1 text-xs font-medium rounded transition-colors ${
                      profile.isActive
                        ? "bg-green-100 text-green-800 cursor-default"
                        : "bg-gray-100 text-gray-800 hover:bg-blue-100 hover:text-blue-800 cursor-pointer"
                    }`}
                  >
                    {profile.isActive ? "激活" : "未激活"}
                  </button>
                </td>
                <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                  <button
                    onClick={() => handleEditProfile(profile)}
                    className="text-blue-600 hover:text-blue-900 mr-3"
                  >
                    编辑
                  </button>
                  <button
                    onClick={() => handleDeleteProfile(profile)}
                    className="text-red-600 hover:text-red-900"
                  >
                    删除
                  </button>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Modal */}
      <Modal
        isOpen={isModalOpen}
        onClose={() => setIsModalOpen(false)}
        title={editingProfile ? "编辑配置" : "添加配置"}
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
        title="确认删除"
        message={`确定要删除配置 "${deletingProfile?.name}" 吗？此操作无法撤销。`}
      />
    </div>
  )
}
