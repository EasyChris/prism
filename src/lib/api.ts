// Tauri API 调用封装

import { invoke } from "@tauri-apps/api/core"

export type ModelMappingMode = "passthrough" | "override" | "map"

export interface Profile {
  id: string
  name: string
  apiBaseUrl: string
  apiKey: string
  modelId: string
  isActive: boolean
  modelMappingMode: ModelMappingMode
  overrideModel?: string
  modelMappings: Record<string, string>
}

// 获取所有配置
export async function getAllProfiles(): Promise<Profile[]> {
  console.log("[API] Calling get_all_profiles...")
  try {
    const result = await invoke<Profile[]>("get_all_profiles")
    console.log("[API] get_all_profiles result:", result)
    return result
  } catch (error) {
    console.error("[API] get_all_profiles error:", error)
    throw error
  }
}

// 创建配置
export async function createProfile(profile: Omit<Profile, "id" | "isActive">): Promise<string> {
  console.log("[API] Calling create_profile...", profile)
  try {
    const result = await invoke<string>("create_profile", { profile })
    console.log("[API] create_profile result:", result)
    return result
  } catch (error) {
    console.error("[API] create_profile error:", error)
    throw error
  }
}

// 更新配置
export async function updateProfile(id: string, profile: Omit<Profile, "id" | "isActive">): Promise<void> {
  console.log("[API] Calling update_profile...", { id, profile })
  try {
    // 将 id 和 isActive 合并到 profile 对象中
    const fullProfile = {
      ...profile,
      id,
      isActive: false  // 后端会忽略这个字段，保持原有的 isActive 状态
    }
    await invoke("update_profile", { id, profile: fullProfile })
    console.log("[API] update_profile success")
  } catch (error) {
    console.error("[API] update_profile error:", error)
    throw error
  }
}

// 删除配置
export async function deleteProfile(id: string): Promise<void> {
  console.log("[API] Calling delete_profile...", { id })
  try {
    await invoke("delete_profile", { id })
    console.log("[API] delete_profile success")
  } catch (error) {
    console.error("[API] delete_profile error:", error)
    throw error
  }
}

// 激活配置
export async function activateProfile(id: string): Promise<void> {
  console.log("[API] Calling activate_profile...", { id })
  try {
    await invoke("activate_profile", { id })
    console.log("[API] activate_profile success")
  } catch (error) {
    console.error("[API] activate_profile error:", error)
    throw error
  }
}

// 获取当前激活的配置
export async function getActiveProfile(): Promise<Profile | null> {
  console.log("[API] Calling get_active_profile...")
  try {
    const profiles = await getAllProfiles()
    const activeProfile = profiles.find(p => p.isActive) || null
    console.log("[API] get_active_profile result:", activeProfile)
    return activeProfile
  } catch (error) {
    console.error("[API] get_active_profile error:", error)
    throw error
  }
}

// 日志相关接口
export interface RequestLog {
  id?: number
  timestamp: number
  profileId: string
  profileName: string
  model: string
  provider: string
  inputTokens: number
  outputTokens: number
  durationMs: number
  statusCode: number
  errorMessage?: string
  isStream: boolean
}

// 获取日志列表
export async function getLogs(limit?: number, offset?: number): Promise<RequestLog[]> {
  console.log("[API] Calling get_logs...", { limit, offset })
  try {
    const result = await invoke<RequestLog[]>("get_logs", { limit, offset })
    console.log("[API] get_logs result:", result)
    return result
  } catch (error) {
    console.error("[API] get_logs error:", error)
    throw error
  }
}
