// 测试 Tauri API 调用
// 在浏览器控制台运行这个脚本来测试

import { invoke } from "@tauri-apps/api/core"

// 测试获取所有配置
async function testGetProfiles() {
  try {
    console.log("Testing get_all_profiles...")
    const profiles = await invoke("get_all_profiles")
    console.log("Profiles:", profiles)
  } catch (error) {
    console.error("Error:", error)
  }
}

// 测试激活配置
async function testActivateProfile(id) {
  try {
    console.log("Testing activate_profile with id:", id)
    await invoke("activate_profile", { id })
    console.log("Profile activated successfully")
  } catch (error) {
    console.error("Error:", error)
  }
}

// 运行测试
testGetProfiles()
