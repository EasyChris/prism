#!/bin/bash

# Prism Hub 版本发布脚本
# 用途：推送代码和 tag 到 GitHub，触发自动构建

set -e  # 遇到错误立即退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}ℹ ${NC}$1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

# 检查是否在 git 仓库中
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_error "当前目录不是 Git 仓库"
    exit 1
fi

# 获取当前版本号
CURRENT_VERSION=$(grep -m 1 '"version"' package.json | sed 's/.*"version": "\(.*\)".*/\1/')

# 检查是否存在对应的 tag
if ! git tag | grep -q "^v${CURRENT_VERSION}$"; then
    print_error "未找到 tag v${CURRENT_VERSION}"
    print_info "请先运行 ./scripts/bump-version.sh 准备版本"
    exit 1
fi

# 检查 tag 是否已经推送
if git ls-remote --tags origin | grep -q "refs/tags/v${CURRENT_VERSION}"; then
    print_warning "Tag v${CURRENT_VERSION} 已经存在于远程仓库"
    read -p "是否继续推送？(y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "已取消"
        exit 0
    fi
fi

print_info "当前版本: ${CURRENT_VERSION}"
print_info "准备推送到 GitHub..."
echo ""

# 确认
read -p "确认发布版本 v${CURRENT_VERSION}？(y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "已取消"
    exit 0
fi

echo ""
print_info "推送代码到 GitHub..."
git push origin main
print_success "代码已推送到 main 分支"

echo ""
print_info "推送 tag 到 GitHub..."
git push origin "v${CURRENT_VERSION}"
print_success "Tag v${CURRENT_VERSION} 已推送"

echo ""
print_success "🎉 版本发布完成！"
echo ""
print_info "GitHub Actions 将自动开始构建"
echo ""
print_info "接下来："
echo "  1. 访问 GitHub Actions 查看构建进度"
echo "     https://github.com/EasyChris/prism/actions"
echo "  2. 构建完成后，在 Releases 页面编辑并发布 Release"
echo "     https://github.com/EasyChris/prism/releases"
echo ""
