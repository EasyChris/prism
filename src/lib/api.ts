// Tauri API 调用封装

import { invoke } from "@tauri-apps/api/core"

export type ModelMappingMode = "passthrough" | "override" | "map"

export interface Profile {
  id: string
  name: string
  apiBaseUrl: string
  apiKey: string
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
  requestId: string
  timestamp: number
  profileId: string
  profileName: string
  provider: string
  originalModel: string
  modelMode: string
  forwardedModel: string
  inputTokens: number
  outputTokens: number
  durationMs: number
  upstreamDurationMs?: number
  statusCode: number
  errorMessage?: string
  isStream: boolean
  requestSizeBytes?: number
  responseSizeBytes?: number
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

// 统计数据接口
export interface DashboardStats {
  todayRequests: number
  todayTokens: number
  totalRequests: number
  totalTokens: number
}

// 获取仪表盘统计数据
export async function getDashboardStats(): Promise<DashboardStats> {
  console.log("[API] Calling get_dashboard_stats...")
  try {
    const result = await invoke<DashboardStats>("get_dashboard_stats")
    console.log("[API] get_dashboard_stats result:", result)
    return result
  } catch (error) {
    console.error("[API] get_dashboard_stats error:", error)
    throw error
  }
}

// Token 统计数据接口
export interface TokenDataPoint {
  label: string
  tokens: number
  cacheReadTokens: number  // 缓存命中的 token 数
}

export type TimeRange = 'hour' | 'day' | 'week' | 'month'

// 获取 Token 使用量统计数据
export async function getTokenStats(timeRange: TimeRange): Promise<TokenDataPoint[]> {
  console.log("[API] Calling get_token_stats...", { timeRange })
  try {
    const result = await invoke<TokenDataPoint[]>("get_token_stats", { timeRange })
    console.log("[API] get_token_stats result:", result)
    return result
  } catch (error) {
    console.error("[API] get_token_stats error:", error)
    throw error
  }
}

// API Key 管理相关接口

// 获取代理服务 API Key
export async function getProxyApiKey(): Promise<string | null> {
  console.log("[API] Calling get_proxy_api_key...")
  try {
    const result = await invoke<string | null>("get_proxy_api_key")
    console.log("[API] get_proxy_api_key result:", result)
    return result
  } catch (error) {
    console.error("[API] get_proxy_api_key error:", error)
    throw error
  }
}
// 刷新代理服务 API Key
export async function refreshProxyApiKey(): Promise<string> {
  console.log("[API] Calling refresh_proxy_api_key...")
  try {
    const result = await invoke<string>("refresh_proxy_api_key")
    console.log("[API] refresh_proxy_api_key result:", result)
    return result
  } catch (error) {
    console.error("[API] refresh_proxy_api_key error:", error)
    throw error
  }
}

// 获取访问授权开关状态
export async function getAuthEnabled(): Promise<boolean> {
  console.log("[API] Calling get_auth_enabled...")
  try {
    const result = await invoke<boolean>("get_auth_enabled")
    console.log("[API] get_auth_enabled result:", result)
    return result
  } catch (error) {
    console.error("[API] get_auth_enabled error:", error)
    throw error
  }
}

// 设置访问授权开关
export async function setAuthEnabled(enabled: boolean): Promise<void> {
  console.log("[API] Calling set_auth_enabled...", { enabled })
  try {
    await invoke("set_auth_enabled", { enabled })
    console.log("[API] set_auth_enabled success")
  } catch (error) {
    console.error("[API] set_auth_enabled error:", error)
    throw error
  }
}

// 获取代理服务器地址
export async function getProxyServerUrl(): Promise<string> {
  console.log("[API] Calling get_proxy_server_url...")
  try {
    const result = await invoke<string>("get_proxy_server_url")
    console.log("[API] get_proxy_server_url result:", result)
    return result
  } catch (error) {
    console.error("[API] get_proxy_server_url error:", error)
    throw error
  }
}

// ==================== 窗口控制相关接口 ====================

// 显示主窗口
export async function showMainWindow(): Promise<void> {
  console.log("[API] Calling show_main_window...")
  try {
    await invoke("show_main_window")
    console.log("[API] show_main_window success")
  } catch (error) {
    console.error("[API] show_main_window error:", error)
    throw error
  }
}

// 更新托盘菜单
export async function updateTrayMenu(): Promise<void> {
  console.log("[API] Calling update_tray_menu...")
  try {
    await invoke("update_tray_menu")
    console.log("[API] update_tray_menu success")
  } catch (error) {
    console.error("[API] update_tray_menu error:", error)
    throw error
  }
}
