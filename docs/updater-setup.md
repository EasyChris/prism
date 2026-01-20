# Tauri Updater 配置指南

> 本指南适用于 **Public（公开）仓库**，配置简单且完全免费。

## 前置要求

- ✅ GitHub 账号
- ✅ 项目已推送到 GitHub（Public 仓库）
- ✅ 本地已安装 pnpm 和 Tauri CLI

---

## 配置步骤

### 步骤 1: 生成签名密钥对

更新功能需要使用签名密钥对来确保更新包的安全性。

#### 1.1 在终端执行以下命令

```bash
pnpm tauri signer generate -w ~/.tauri/prism-hub.key
```

#### 1.2 按提示操作

- 提示输入密码时，**直接按回车跳过**（不设置密码更简单）
- 命令执行后会显示公钥，类似这样：

```
Your keypair was generated successfully
Private: /Users/你的用户名/.tauri/prism-hub.key (Keep this secret!)
Public: dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IDkyQjkxN0Q5MEY4OTgxNUIKUldSYmdZa1AyUmU1a29mWHRtTTk1N01xUW1xOFMxY1FpWituSHRmODNnV3NvRFNmYXl6L0tsNVkK
```

#### 1.3 复制公钥

**重要**：复制 `Public:` 后面的整个字符串（以 `dW50cnVzdGVk` 开头的长字符串）

---

### 步骤 2: 配置公钥到项目

公钥已经在 `src-tauri/tauri.conf.json` 中配置好了，但你需要确认它是否是你刚才生成的公钥。

#### 2.1 打开配置文件

```bash
# 使用你喜欢的编辑器打开
code src-tauri/tauri.conf.json
# 或
vim src-tauri/tauri.conf.json
```

#### 2.2 找到 updater 配置

找到这一段：
```json
"plugins": {
  "updater": {
    "active": true,
    "endpoints": [...],
    "dialog": true,
    "pubkey": "dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk6IDkyQjkxN0Q5MEY4OTgxNUIKUldSYmdZa1AyUmU1a29mWHRtTTk1N01xUW1xOFMxY1FpWituSHRmODNnV3NvRFNmYXl6L0tsNVkK"
  }
}
```

#### 2.3 替换公钥

将 `pubkey` 的值替换为你在步骤 1.3 中复制的公钥。

**注意**：如果你看到的公钥已经是你刚才生成的，就不需要修改。

---

### 步骤 3: 配置 GitHub Secrets

需要将私钥添加到 GitHub Secrets 中，供 GitHub Actions 使用。

#### 3.1 读取私钥内容

在终端执行：
```bash
cat ~/.tauri/prism-hub.key
```

#### 3.2 复制私钥内容

复制**整个输出内容**，包括开头和结尾的注释行，类似这样：
```
untrusted comment: rsign secret key
RWRTSwAAABQAAAAA...（很长的字符串）
```

#### 3.3 添加到 GitHub Secrets

1. 打开你的 GitHub 仓库页面
2. 点击 **Settings**（设置）
3. 左侧菜单找到 **Secrets and variables** → **Actions**
4. 点击 **New repository secret** 按钮
5. 填写信息：
   - **Name**: `TAURI_PRIVATE_KEY`
   - **Value**: 粘贴步骤 3.2 复制的私钥内容
6. 点击 **Add secret** 保存

**注意**：因为你没有设置密码，所以不需要添加 `TAURI_KEY_PASSWORD`。

---

### 步骤 4: 更新 GitHub 仓库地址

需要将配置文件中的仓库地址改为你的实际仓库地址。

#### 4.1 打开配置文件

```bash
code src-tauri/tauri.conf.json
```

#### 4.2 找到 endpoints 配置

找到这一行：
```json
"endpoints": [
  "https://github.com/your-username/prism/releases/latest/download/latest.json"
]
```

#### 4.3 替换用户名

将 `your-username` 替换为你的 GitHub 用户名。

例如，如果你的 GitHub 用户名是 `zhangsan`，改为：
```json
"endpoints": [
  "https://github.com/zhangsan/prism/releases/latest/download/latest.json"
]
```

---

## 发布新版本

配置完成后，你就可以发布新版本了。

**推荐方式**：使用自动化脚本（灵活且安全）
**手动方式**：按步骤 5-6 手动操作

---

### 方式一：使用自动化脚本（推荐）

我们提供了两个脚本，让你可以灵活控制发布流程：

#### 脚本说明

1. **`bump-version.sh`** - 版本准备脚本（本地操作）
   - 更新三个文件的版本号
   - 提交更改到本地
   - 创建 Git tag（仅在本地）
   - **不会推送到 GitHub**

2. **`publish-release.sh`** - 版本发布脚本（推送到 GitHub）
   - 推送代码到 GitHub
   - 推送 tag 到 GitHub
   - 触发 GitHub Actions 自动构建

#### 使用场景

**场景 1：日常开发（不发布版本）**

正常提交代码，不会触发版本发布：

```bash
git add .
git commit -m "feat: 添加新功能"
git push origin main
```

**场景 2：准备发布新版本**

分两步执行，更加灵活和安全：

```bash
# 步骤 1: 准备版本（本地操作）
./scripts/bump-version.sh

# 此时可以：
# - 检查版本号是否正确
# - 继续修改代码
# - 运行测试
# - 如果有问题，可以撤销：git reset --hard HEAD~1 && git tag -d v0.2.0

# 步骤 2: 确认无误后，发布到 GitHub
./scripts/publish-release.sh
```

#### 脚本执行流程

**bump-version.sh 会做什么：**

1. ✅ 检查 Git 状态
2. ✅ 提示输入新版本号
3. ✅ 验证版本号格式
4. ✅ 更新三个文件的版本号
5. ✅ 提交更改到本地
6. ✅ 创建 Git tag（仅在本地）
7. ✅ 显示下一步操作提示

**publish-release.sh 会做什么：**

1. ✅ 检查是否存在对应的 tag
2. ✅ 确认是否发布
3. ✅ 推送代码到 GitHub
4. ✅ 推送 tag 到 GitHub
5. ✅ 触发 GitHub Actions 构建

**优势：**
- 🔒 更安全：版本准备和发布分离，可以在发布前检查
- 🎯 更灵活：日常开发不会误触发版本发布
- 🔄 可撤销：发布前可以随时撤销版本准备

---

### 方式二：手动操作

如果你想完全手动控制每一步，可以按照以下步骤操作。

### 步骤 5: 更新版本号

发布前需要同步更新三个文件中的版本号。

#### 5.1 更新 Cargo.toml

打开 `src-tauri/Cargo.toml`，找到：
```toml
[package]
version = "0.1.0"
```

改为新版本号，例如：
```toml
[package]
version = "0.2.0"
```

#### 5.2 更新 tauri.conf.json

打开 `src-tauri/tauri.conf.json`，找到：
```json
{
  "version": "0.1.0"
}
```

改为：
```json
{
  "version": "0.2.0"
}
```

#### 5.3 更新 package.json

打开 `package.json`，找到：
```json
{
  "version": "0.1.0"
}
```

改为：
```json
{
  "version": "0.2.0"
}
```

**重要**：三个文件的版本号必须完全一致！

---

### 步骤 6: 创建 Git Tag 并推送

#### 6.1 提交版本更新

```bash
git add .
git commit -m "chore: bump version to 0.2.0"
```

#### 6.2 创建 Git Tag

```bash
git tag v0.2.0
```

**注意**：tag 必须以 `v` 开头，后面跟版本号。

#### 6.3 推送到 GitHub

```bash
git push origin main
git push origin v0.2.0
```

推送 tag 后，GitHub Actions 会自动触发构建流程。

---

### 步骤 7: 查看构建进度

#### 7.1 进入 Actions 页面

1. 打开你的 GitHub 仓库
2. 点击顶部的 **Actions** 标签
3. 你会看到一个名为 "Release" 的工作流正在运行

#### 7.2 查看构建日志

点击工作流可以查看详细的构建日志。构建过程大约需要 5-10 分钟。

#### 7.3 等待构建完成

构建成功后，会在 **Releases** 页面自动创建一个草稿版本。

---

### 步骤 8: 发布 Release

#### 8.1 进入 Releases 页面

1. 打开你的 GitHub 仓库
2. 点击右侧的 **Releases**
3. 你会看到一个草稿版本（Draft）

#### 8.2 编辑 Release 说明

1. 点击草稿版本的 **Edit** 按钮
2. 在描述框中添加更新内容，例如：

```markdown
## 新功能
- 添加了自动更新功能
- 优化了界面显示

## 修复
- 修复了某个 bug

## 其他
- 更新了依赖版本
```

#### 8.3 发布 Release

1. 确认信息无误后，点击 **Publish release** 按钮
2. Release 发布后，用户就可以通过应用内的"检查更新"功能获取新版本了

---

## 注意事项

### 版本号格式

- ✅ 必须使用语义化版本号：`major.minor.patch`（例如：`0.2.0`）
- ✅ Git tag 必须以 `v` 开头（例如：`v0.2.0`）
- ✅ 三个文件中的版本号必须完全一致

### macOS 代码签名

- 生产环境建议配置 Apple 开发者证书
- 未签名的应用可能被 macOS Gatekeeper 阻止
- 测试环境可以暂时跳过签名

### 安全性

- ⚠️ 私钥文件 (`~/.tauri/prism-hub.key`) 必须妥善保管
- ⚠️ 不要将私钥提交到 Git 仓库
- ✅ GitHub Secrets 中的私钥只有仓库管理员可见

---

## 故障排查

### 问题 1: 检查更新失败

**现象**：点击"检查更新"后提示失败

**可能原因**：
1. GitHub 仓库地址配置错误
2. 还没有发布任何 Release
3. 网络连接问题

**解决方案**：
1. 检查 `tauri.conf.json` 中的 `endpoints` 地址是否正确
2. 确认至少发布了一个 Release（不能是草稿）
3. 检查网络连接是否正常

---

### 问题 2: GitHub Actions 构建失败

**现象**：推送 tag 后构建失败

**可能原因**：
1. `TAURI_PRIVATE_KEY` 配置错误
2. 私钥格式不完整
3. 依赖安装失败

**解决方案**：
1. 检查 GitHub Secrets 中的 `TAURI_PRIVATE_KEY` 是否正确
2. 确认私钥包含开头和结尾的注释行
3. 查看 Actions 日志获取详细错误信息

---

### 问题 3: 签名验证失败

**现象**：下载更新时提示签名验证失败

**可能原因**：
1. 公钥和私钥不匹配
2. 构建时使用了错误的私钥

**解决方案**：
1. 确认 `tauri.conf.json` 中的公钥与私钥匹配
2. 重新生成密钥对并更新配置
3. 确保 GitHub Secrets 中的私钥是正确的

---

## 快速检查清单

在发布前，请确认以下事项：

- [ ] 已生成签名密钥对
- [ ] 公钥已配置到 `tauri.conf.json`
- [ ] 私钥已添加到 GitHub Secrets
- [ ] GitHub 仓库地址已更新
- [ ] 三个文件的版本号已同步更新
- [ ] Git tag 已创建并推送
- [ ] GitHub Actions 构建成功
- [ ] Release 已发布（不是草稿）
- [ ] 在应用中测试更新功能

---

## 总结

恭喜！你已经完成了 Tauri 自动更新功能的配置。

现在你可以：
1. 在应用的设置页面点击"检查更新"
2. 查看是否有新版本
3. 一键下载并安装更新

每次发布新版本时，只需要：
1. 运行 `./scripts/bump-version.sh` 准备版本
2. 运行 `./scripts/publish-release.sh` 发布到 GitHub
3. 在 GitHub Releases 页面编辑并发布

日常开发时：
- 正常提交代码不会触发版本发布
- 只有推送 tag 时才会触发 GitHub Actions 构建

就这么简单！🎉
