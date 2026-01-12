#!/bin/bash

# 测试代理转发功能
# 使用方法: ./test-proxy.sh

echo "🧪 开始测试代理转发功能..."
echo ""

# 代理服务器地址
PROXY_URL="http://127.0.0.1:3000"

# 测试 1: 检查代理服务器是否运行
echo "📡 测试 1: 检查代理服务器状态..."
if curl -s --max-time 2 "$PROXY_URL" > /dev/null 2>&1; then
    echo "✅ 代理服务器正在运行"
else
    echo "❌ 代理服务器未运行，请先启动: pnpm tauri dev"
    exit 1
fi

echo ""

# 测试 2: 发送测试请求
echo "📡 测试 2: 发送 API 请求到代理..."
echo "请求地址: $PROXY_URL/v1/messages"
echo ""

RESPONSE=$(curl -s -w "\n%{http_code}" -X POST "$PROXY_URL/v1/messages" \
  -H "Content-Type: application/json" \
  -H "anthropic-version: 2023-06-01" \
  -d '{
    "model": "claude-3-5-sonnet-20241022",
    "max_tokens": 100,
    "messages": [
      {
        "role": "user",
        "content": "Hello, this is a test message. Please respond with: TEST_SUCCESS"
      }
    ]
  }')

HTTP_CODE=$(echo "$RESPONSE" | tail -n1)
BODY=$(echo "$RESPONSE" | sed '$d')

echo "HTTP 状态码: $HTTP_CODE"
echo ""

if [ "$HTTP_CODE" = "200" ]; then
    echo "✅ 请求成功转发"
    echo ""
    echo "响应内容:"
    echo "$BODY" | jq '.' 2>/dev/null || echo "$BODY"
else
    echo "❌ 请求失败"
    echo ""
    echo "响应内容:"
    echo "$BODY"
fi

echo ""
echo "🎉 测试完成！"
