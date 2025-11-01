# Git Setup and Push Guide / Git 设置和推送指南

[English](#english) | [中文](#中文)

---

<a name="english"></a>

## 🚀 English - Git Setup

### First Time Setup

#### Step 1: Initialize Git (if not already done)

```bash
cd /Users/yiqunwu/wuyiqun/power_project/ai-x/ai_coding/ccswitch/ccswitch-app
git init
```

#### Step 2: Configure Git

```bash
# Set your name and email
git config user.name "Your Name"
git config user.email "your.email@example.com"

# Optional: Set default branch name to main
git config init.defaultBranch main
```

#### Step 3: Add Remote Repository

```bash
# Add the GitHub repository as remote
git remote add origin https://github.com/wyq09/ccswitch.git

# Verify the remote
git remote -v
```

### Pushing Code to GitHub

#### Option 1: Push Everything (Recommended for First Push)

```bash
# Check current status
git status

# Add all files
git add .

# Commit with a message
git commit -m "feat: initial commit - CCSwitch v1.0.0"

# Push to main branch
git push -u origin main
```

#### Option 2: Push Specific Files

```bash
# Add specific files
git add README.md
git add package.json
git add .github/workflows/release.yml

# Commit
git commit -m "docs: update README and add CI/CD workflow"

# Push
git push origin main
```

### Creating Your First Release

After pushing the code:

```bash
# Create a tag for version 1.0.0
git tag -a v1.0.0 -m "Release v1.0.0 - Initial public release"

# Push the tag
git push origin v1.0.0
```

This will trigger the GitHub Actions workflow to build and create a release automatically!

### Common Git Commands

```bash
# Check status
git status

# View commit history
git log --oneline

# View remote repositories
git remote -v

# Pull latest changes
git pull origin main

# Create a new branch
git checkout -b feature/new-feature

# Switch branches
git checkout main

# View all tags
git tag -l

# Delete a tag (local)
git tag -d v1.0.0

# Delete a tag (remote)
git push origin --delete v1.0.0
```

### Troubleshooting

#### "Repository not found" Error

```bash
# Check if remote is correct
git remote -v

# If wrong, update it
git remote set-url origin https://github.com/wyq09/ccswitch.git
```

#### "Permission denied" Error

```bash
# Use SSH instead of HTTPS
git remote set-url origin git@github.com:wyq09/ccswitch.git

# Or use GitHub CLI
gh auth login
```

#### "Rejected - non-fast-forward" Error

```bash
# Pull first, then push
git pull origin main --rebase
git push origin main
```

---

<a name="中文"></a>

## 🚀 中文 - Git 设置

### 首次设置

#### 步骤 1：初始化 Git（如果尚未完成）

```bash
cd /Users/yiqunwu/wuyiqun/power_project/ai-x/ai_coding/ccswitch/ccswitch-app
git init
```

#### 步骤 2：配置 Git

```bash
# 设置你的姓名和邮箱
git config user.name "你的姓名"
git config user.email "your.email@example.com"

# 可选：将默认分支名称设置为 main
git config init.defaultBranch main
```

#### 步骤 3：添加远程仓库

```bash
# 将 GitHub 仓库添加为远程仓库
git remote add origin https://github.com/wyq09/ccswitch.git

# 验证远程仓库
git remote -v
```

### 推送代码到 GitHub

#### 方案 1：推送所有内容（首次推送推荐）

```bash
# 检查当前状态
git status

# 添加所有文件
git add .

# 提交并附上消息
git commit -m "feat: initial commit - CCSwitch v1.0.0"

# 推送到 main 分支
git push -u origin main
```

#### 方案 2：推送特定文件

```bash
# 添加特定文件
git add README.md
git add package.json
git add .github/workflows/release.yml

# 提交
git commit -m "docs: update README and add CI/CD workflow"

# 推送
git push origin main
```

### 创建首个发布版本

推送代码后：

```bash
# 为版本 1.0.0 创建标签
git tag -a v1.0.0 -m "Release v1.0.0 - Initial public release"

# 推送标签
git push origin v1.0.0
```

这将自动触发 GitHub Actions 工作流来构建并创建发布版本！

### 常用 Git 命令

```bash
# 检查状态
git status

# 查看提交历史
git log --oneline

# 查看远程仓库
git remote -v

# 拉取最新更改
git pull origin main

# 创建新分支
git checkout -b feature/new-feature

# 切换分支
git checkout main

# 查看所有标签
git tag -l

# 删除标签（本地）
git tag -d v1.0.0

# 删除标签（远程）
git push origin --delete v1.0.0
```

### 故障排除

#### "Repository not found" 错误

```bash
# 检查远程仓库是否正确
git remote -v

# 如果错误，更新它
git remote set-url origin https://github.com/wyq09/ccswitch.git
```

#### "Permission denied" 错误

```bash
# 使用 SSH 而不是 HTTPS
git remote set-url origin git@github.com:wyq09/ccswitch.git

# 或使用 GitHub CLI
gh auth login
```

#### "Rejected - non-fast-forward" 错误

```bash
# 先拉取，然后推送
git pull origin main --rebase
git push origin main
```

---

## 📋 Complete Setup Checklist / 完整设置检查清单

### Before First Push / 首次推送前

- [ ] Git initialized / Git 已初始化
- [ ] Git configured (name, email) / Git 已配置（姓名、邮箱）
- [ ] Remote repository added / 远程仓库已添加
- [ ] All files ready to commit / 所有文件准备好提交

### Files to Include / 要包含的文件

- [ ] Source code (`src/`, `src-tauri/`) / 源代码
- [ ] Configuration files / 配置文件
- [ ] Documentation (`README.md`, `docs/`) / 文档
- [ ] GitHub Actions (`.github/workflows/`) / GitHub Actions
- [ ] Package files (`package.json`, `Cargo.toml`) / 包文件
- [ ] `.gitignore` file / `.gitignore` 文件

### Files to Exclude / 要排除的文件

- [ ] `node_modules/` (in .gitignore) / `node_modules/`（在 .gitignore 中）
- [ ] `target/` (in .gitignore) / `target/`（在 .gitignore 中）
- [ ] `dist/` (in .gitignore) / `dist/`（在 .gitignore 中）
- [ ] `.env` files with secrets / 包含密钥的 `.env` 文件
- [ ] Build artifacts / 构建产物

---

## 🔐 Security Tips / 安全提示

### Never Commit / 永远不要提交

- API keys / API 密钥
- Passwords / 密码
- Private keys / 私钥
- `.env` files with secrets / 包含密钥的 `.env` 文件
- Personal configuration / 个人配置

### Use .gitignore / 使用 .gitignore

Make sure your `.gitignore` includes:

```
# Dependencies
node_modules/
target/

# Build outputs
dist/
dist-ssr/
*.local

# Environment files
.env
.env.local
.env.*.local

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db
```

---

## 🎯 Quick Reference / 快速参考

### First Time Push / 首次推送

```bash
git init
git add .
git commit -m "feat: initial commit"
git remote add origin https://github.com/wyq09/ccswitch.git
git push -u origin main
```

### Create Release / 创建发布

```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

### Update and Push / 更新并推送

```bash
git add .
git commit -m "feat: add new feature"
git push origin main
```

---

**Happy Coding! / 编码愉快！** 🎉

