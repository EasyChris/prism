#!/bin/bash

# Prism Hub 版本准备脚本
# 用途：本地更新版本号、创建 Git tag（不推送）

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

# 检查是否有未提交的更改
if ! git diff-index --quiet HEAD --; then
    print_warning "检测到未提交的更改"
    git status --short
    echo ""
    read -p "是否继续？(y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_info "已取消"
        exit 0
    fi
fi

# 获取当前版本号
CURRENT_VERSION=$(grep -m 1 '"version"' package.json | sed 's/.*"version": "\(.*\)".*/\1/')
print_info "当前版本: ${CURRENT_VERSION}"

# 输入新版本号
echo ""
read -p "请输入新版本号 (例如: 0.2.0): " NEW_VERSION

# 验证版本号格式
if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    print_error "版本号格式错误，必须是 x.y.z 格式 (例如: 0.2.0)"
    exit 1
fi

print_info "新版本: ${NEW_VERSION}"
echo ""

# 确认
read -p "确认更新版本号？(y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "已取消"
    exit 0
fi

echo ""
print_info "开始更新版本号..."

# 更新 package.json
print_info "更新 package.json..."
sed -i.bak "s/\"version\": \".*\"/\"version\": \"${NEW_VERSION}\"/" package.json && rm package.json.bak
print_success "package.json 已更新"

# 更新 src-tauri/Cargo.toml
print_info "更新 src-tauri/Cargo.toml..."
sed -i.bak "s/^version = \".*\"/version = \"${NEW_VERSION}\"/" src-tauri/Cargo.toml && rm src-tauri/Cargo.toml.bak
print_success "src-tauri/Cargo.toml 已更新"

# 更新 src-tauri/tauri.conf.json
print_info "更新 src-tauri/tauri.conf.json..."
sed -i.bak "s/\"version\": \".*\"/\"version\": \"${NEW_VERSION}\"/" src-tauri/tauri.conf.json && rm src-tauri/tauri.conf.json.bak
print_success "src-tauri/tauri.conf.json 已更新"

echo ""
print_info "提交更改..."
git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json
git commit -m "chore: bump version to ${NEW_VERSION}"
print_success "版本更新已提交"

echo ""
print_info "创建 Git tag..."
git tag "v${NEW_VERSION}"
print_success "Git tag v${NEW_VERSION} 已创建（仅在本地）"

echo ""
print_success "🎉 版本准备完成！"
echo ""
print_info "版本号已更新为: ${NEW_VERSION}"
print_info "Git tag v${NEW_VERSION} 已在本地创建"
echo ""
print_warning "注意：更改尚未推送到 GitHub"
echo ""
print_info "接下来："
echo "  1. 如果需要发布此版本，运行: ./scripts/publish-release.sh"
echo "  2. 如果需要继续开发，可以继续提交代码"
echo "  3. 如果需要撤销，运行: git reset --hard HEAD~1 && git tag -d v${NEW_VERSION}"
echo ""
