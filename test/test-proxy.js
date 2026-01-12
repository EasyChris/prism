// 测试代理转发功能
// 使用方法: node test-proxy.js

const PROXY_URL = "http://127.0.0.1:3000";

async function testProxy() {
  console.log("🧪 开始测试代理转发功能...\n");

  // 测试 1: 检查代理服务器是否运行
  console.log("📡 测试 1: 检查代理服务器状态...");
  try {
    const healthCheck = await fetch(PROXY_URL, {
      method: "GET",
      signal: AbortSignal.timeout(2000)
    });
    console.log("✅ 代理服务器正在运行\n");
  } catch (error) {
    console.log("❌ 代理服务器未运行，请先启动: pnpm tauri dev");
    process.exit(1);
  }

  // 测试 2: 发送测试请求
  console.log("📡 测试 2: 发送 API 请求到代理...");
  console.log(`请求地址: ${PROXY_URL}/v1/messages\n`);

  try {
    const response = await fetch(`${PROXY_URL}/v1/messages`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "anthropic-version": "2023-06-01",
        "x-api-key": "test-key-placeholder", // 占位符，实际会被代理替换
      },
      body: JSON.stringify({
        model: "claude-opus-4-5-20251101",
        max_tokens: 100,
        messages: [
          {
            role: "user",
            content: "Hello, this is a test. Please respond with: TEST_SUCCESS"
          }
        ]
      }),
      signal: AbortSignal.timeout(30000) // 30秒超时
    });

    console.log(`HTTP 状态码: ${response.status}\n`);

    if (response.ok) {
      console.log("✅ 请求成功转发\n");

      // 获取响应文本
      const text = await response.text();
      console.log("响应内容:");

      try {
        // 尝试解析为 JSON
        const data = JSON.parse(text);
        console.log(JSON.stringify(data, null, 2));

        // 检查是否包含预期的响应
        if (data.content && data.content.length > 0) {
          console.log("\n✅ 收到有效的 API 响应");
        }
      } catch (e) {
        // 如果不是 JSON，直接显示文本
        console.log(text);
      }
    } else {
      console.log("❌ 请求失败\n");
      const text = await response.text();
      console.log("响应内容:");
      console.log(text);
    }
  } catch (error) {
    console.log("❌ 请求出错:", error.message);
    if (error.cause) {
      console.log("错误详情:", error.cause);
    }
  }

  console.log("\n🎉 测试完成！");
}

testProxy();
