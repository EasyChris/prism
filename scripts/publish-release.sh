#!/bin/bash

# Prism Hub 版本发布脚本
# 用途：自动递增版本号（或手动指定），创建 tag 并推送到 GitHub，触发自动构建
# 使用方法：
#   ./scripts/publish-release.sh           # 自动递增 patch 版本
#   ./scripts/publish-release.sh 1.0.0     # 手动指定版本号

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

# 自动递增版本号函数
auto_increment_version() {
    local version=$1
    local major minor patch

    IFS='.' read -r major minor patch <<< "$version"
    patch=$((patch + 1))
    echo "${major}.${minor}.${patch}"
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

# 确定新版本号
if [ -n "$1" ]; then
    # 使用命令行参数指定的版本号
    NEW_VERSION="$1"
    print_info "使用手动指定的版本号: ${NEW_VERSION}"
else
    # 自动递增 patch 版本
    NEW_VERSION=$(auto_increment_version "$CURRENT_VERSION")
    print_info "自动递增版本号: ${NEW_VERSION}"
    echo ""
    read -p "使用自动版本号 ${NEW_VERSION}？(Y/n) 或输入自定义版本号: " -r
    echo
    if [[ $REPLY =~ ^[Nn]$ ]]; then
        print_info "已取消"
        exit 0
    elif [[ ! -z "$REPLY" ]] && [[ ! $REPLY =~ ^[Yy]$ ]]; then
        # 用户输入了自定义版本号
        NEW_VERSION="$REPLY"
        print_info "使用自定义版本号: ${NEW_VERSION}"
    fi
fi

# 验证版本号格式
if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    print_error "版本号格式错误，必须是 x.y.z 格式 (例如: 0.2.0)"
    exit 1
fi

# 检查版本号是否已存在
if git tag | grep -q "^v${NEW_VERSION}$"; then
    print_error "Tag v${NEW_VERSION} 已经存在"
    print_info "请使用不同的版本号"
    exit 1
fi

# 检查远程是否已存在该版本
if git ls-remote --tags origin | grep -q "refs/tags/v${NEW_VERSION}"; then
    print_error "Tag v${NEW_VERSION} 已经存在于远程仓库"
    print_info "请使用不同的版本号"
    exit 1
fi

echo ""
print_info "版本变更: ${CURRENT_VERSION} → ${NEW_VERSION}"
echo ""

# 确认发布
read -p "确认发布版本 v${NEW_VERSION}？(y/N) " -n 1 -r
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
print_info "提交版本更新..."
git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json
git commit -m "chore: bump version to ${NEW_VERSION}"
print_success "版本更新已提交"

echo ""
print_info "创建 Git tag..."
git tag "v${NEW_VERSION}"
print_success "Git tag v${NEW_VERSION} 已创建"

echo ""
print_info "推送代码到 GitHub..."
git push origin main
print_success "代码已推送到 main 分支"

echo ""
print_info "推送 tag 到 GitHub..."
git push origin "v${NEW_VERSION}"
print_success "Tag v${NEW_VERSION} 已推送"

echo ""
print_success "版本发布完成！"
echo ""
print_info "版本号已更新为: ${NEW_VERSION}"
print_info "GitHub Actions 将自动开始构建"
echo ""
print_info "接下来："
echo "  1. 访问 GitHub Actions 查看构建进度"
echo "     https://github.com/EasyChris/prism/actions"
echo "  2. 构建完成后，在 Releases 页面编辑并发布 Release"
echo "     https://github.com/EasyChris/prism/releases"
echo ""
