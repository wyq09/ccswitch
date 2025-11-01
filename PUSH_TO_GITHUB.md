# 🚀 Push to GitHub Guide / 推送到 GitHub 指南

## 📋 Quick Commands / 快速命令

### 完整推送流程（首次推送）

```bash
# 1. 进入项目目录
cd /Users/yiqunwu/wuyiqun/power_project/ai-x/ai_coding/ccswitch

# 2. 初始化 Git（如果还没有）
git init

# 3. 添加远程仓库
git remote add origin https://github.com/wyq09/ccswitch.git

# 4. 检查状态
git status

# 5. 添加所有文件
git add .

# 6. 提交
git commit -m "feat: initial commit - CCSwitch v1.0.0

- Add provider management functionality
- Add multi-language support (EN/CN)
- Add built-in templates for popular providers
- Add GitHub Actions for auto-release
- Add comprehensive documentation"

# 7. 推送到 main 分支
git push -u origin main
```

### 创建首个发布版本

```bash
# 1. 创建标签
git tag -a v1.0.0 -m "Release v1.0.0

Initial public release of CCSwitch
- Provider management
- Multi-language support
- Auto-release workflow"

# 2. 推送标签（这将触发 GitHub Actions 自动构建和发布）
git push origin v1.0.0
```

---

## 📝 详细步骤

### Step 1: 检查 Git 状态

```bash
cd /Users/yiqunwu/wuyiqun/power_project/ai-x/ai_coding/ccswitch

# 检查是否已初始化 Git
git status

# 如果显示 "not a git repository"，则需要初始化
git init
```

### Step 2: 配置 Git（如果还没配置）

```bash
# 设置用户名和邮箱
git config user.name "wyq09"
git config user.email "your.email@example.com"

# 验证配置
git config --list | grep user
```

### Step 3: 添加远程仓库

```bash
# 添加 GitHub 仓库
git remote add origin https://github.com/wyq09/ccswitch.git

# 验证远程仓库
git remote -v
# 应该显示：
# origin  https://github.com/wyq09/ccswitch.git (fetch)
# origin  https://github.com/wyq09/ccswitch.git (push)
```

### Step 4: 准备提交

```bash
# 查看将要提交的文件
git status

# 查看更改详情
git diff

# 添加所有文件
git add .

# 或者选择性添加
git add ccswitch-app/
git add README.md
git add CHANGELOG.md
git add .github/
```

### Step 5: 提交代码

```bash
# 提交并附上详细说明
git commit -m "feat: initial commit - CCSwitch v1.0.0

Features:
- Provider management system
- One-click provider switching
- Built-in templates (Claude, Zhipu AI, Moonshot, DeepSeek, etc.)
- Multi-language support (English and Simplified Chinese)
- Modern glass morphism UI
- Cross-platform support (macOS and Windows)
- GitHub Actions auto-release workflow

Documentation:
- Comprehensive README with bilingual support
- Quick start guide
- Release guide
- Contributing guide
- Git setup guide

Infrastructure:
- GitHub Actions workflow for automated releases
- Build scripts for all platforms
- CHANGELOG for version tracking"
```

### Step 6: 推送到 GitHub

```bash
# 首次推送（设置上游分支）
git push -u origin main

# 如果遇到错误 "failed to push some refs"
# 可能需要先拉取远程更改
git pull origin main --allow-unrelated-histories
git push -u origin main
```

### Step 7: 创建发布标签

```bash
# 创建带注释的标签
git tag -a v1.0.0 -m "Release v1.0.0 - Initial Public Release

🎉 Features:
- Provider management for Claude Code
- Multi-language support (EN/CN)
- Built-in templates for 9 popular providers
- Modern UI with glass morphism design
- Auto-release via GitHub Actions

📦 Platforms:
- macOS (Universal, Intel, Apple Silicon)
- Windows (64-bit)

📖 Documentation:
- Complete bilingual documentation
- Quick start guide
- Contributing guidelines"

# 推送标签到 GitHub
git push origin v1.0.0

# 推送所有标签
git push origin --tags
```

---

## 🔍 验证推送

### 检查 GitHub 仓库

1. 访问 https://github.com/wyq09/ccswitch
2. 确认代码已上传
3. 检查文件结构是否完整

### 检查 GitHub Actions

1. 访问 https://github.com/wyq09/ccswitch/actions
2. 查看 "Release" 工作流是否触发
3. 等待构建完成（可能需要 10-20 分钟）

### 检查 Release

1. 访问 https://github.com/wyq09/ccswitch/releases
2. 确认 v1.0.0 release 已创建
3. 检查所有平台的安装包是否已上传

---

## ⚠️ 常见问题

### 问题 1: "Permission denied"

```bash
# 解决方案 1: 使用 HTTPS 并输入 GitHub 凭据
git remote set-url origin https://github.com/wyq09/ccswitch.git

# 解决方案 2: 使用 SSH
git remote set-url origin git@github.com:wyq09/ccswitch.git

# 解决方案 3: 使用 GitHub CLI
gh auth login
```

### 问题 2: "Repository not found"

```bash
# 检查远程仓库 URL
git remote -v

# 更新远程仓库 URL
git remote set-url origin https://github.com/wyq09/ccswitch.git
```

### 问题 3: "Failed to push some refs"

```bash
# 拉取远程更改
git pull origin main --rebase

# 解决冲突（如果有）
# 然后推送
git push origin main
```

### 问题 4: GitHub Actions 未触发

```bash
# 确认标签格式正确（必须是 v*.*.* 格式）
git tag -l

# 删除错误的标签
git tag -d v1.0.0
git push origin --delete v1.0.0

# 重新创建正确的标签
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

---

## 📊 推送后检查清单

- [ ] 代码已推送到 GitHub
- [ ] 所有文件都在仓库中
- [ ] README.md 正确显示
- [ ] 文档链接正常工作
- [ ] 标签已创建并推送
- [ ] GitHub Actions 工作流已触发
- [ ] Release 已自动创建
- [ ] 所有平台的安装包已上传

---

## 🎯 后续更新流程

### 日常代码更新

```bash
# 1. 修改代码
# 2. 查看更改
git status
git diff

# 3. 添加更改
git add .

# 4. 提交
git commit -m "feat: add new feature"

# 5. 推送
git push origin main
```

### 发布新版本

```bash
# 1. 更新版本号（package.json, tauri.conf.json, Cargo.toml）
# 2. 更新 CHANGELOG.md
# 3. 提交更改
git add .
git commit -m "chore: bump version to 1.1.0"
git push origin main

# 4. 创建新标签
git tag -a v1.1.0 -m "Release v1.1.0"
git push origin v1.1.0
```

---

## 🔗 相关链接

- **GitHub 仓库**: https://github.com/wyq09/ccswitch
- **Actions**: https://github.com/wyq09/ccswitch/actions
- **Releases**: https://github.com/wyq09/ccswitch/releases
- **Issues**: https://github.com/wyq09/ccswitch/issues

---

## 📚 更多文档

- [README.md](README.md) - 项目主文档
- [docs/QUICK_START.md](docs/QUICK_START.md) - 快速开始指南
- [docs/RELEASE_GUIDE.md](docs/RELEASE_GUIDE.md) - 发布指南
- [docs/GIT_SETUP.md](docs/GIT_SETUP.md) - Git 详细设置
- [CONTRIBUTING.md](CONTRIBUTING.md) - 贡献指南
- [CHANGELOG.md](CHANGELOG.md) - 更新日志

---

**准备好了吗？开始推送吧！🚀**

