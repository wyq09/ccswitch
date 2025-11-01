# GitHub Actions 修复说明

## 🔧 修复内容

### 问题描述
GitHub Actions 在运行时报错：
```
Error: Some specified paths were not resolved, unable to cache dependencies.
```

### 根本原因
1. **Node.js 版本过低**: 使用的是 Node.js 18，建议升级到 20
2. **缓存路径配置问题**: `cache-dependency-path` 配置可能导致缓存失败
3. **工作目录问题**: 需要明确指定工作目录

### 已修复的问题

#### 1. 升级 Node.js 版本
```yaml
# 之前
node-version: '18'

# 现在
node-version: '20'
```

#### 2. 移除 npm 缓存配置
```yaml
# 之前
- name: Setup Node.js
  uses: actions/setup-node@v4
  with:
    node-version: '18'
    cache: 'npm'
    cache-dependency-path: ccswitch-app/package-lock.json

# 现在（更简单、更稳定）
- name: Setup Node.js
  uses: actions/setup-node@v4
  with:
    node-version: '20'
```

**原因**: npm 缓存在某些情况下可能导致路径解析问题，移除后让 GitHub Actions 自动处理。

#### 3. 使用 working-directory
```yaml
# 之前
- name: Install frontend dependencies
  run: |
    cd ccswitch-app
    npm ci

# 现在
- name: Install frontend dependencies
  working-directory: ccswitch-app
  run: npm ci
```

**原因**: 使用 `working-directory` 更清晰，避免路径问题。

---

## 🚀 如何重新触发构建

### 方案 1: 删除并重新创建标签（推荐）

```bash
# 1. 删除本地标签
git tag -d v1.0.0

# 2. 删除远程标签
git push origin --delete v1.0.0

# 3. 提交修复后的工作流文件
git add .github/workflows/release.yml
git commit -m "fix: update GitHub Actions workflow - upgrade Node.js to v20"
git push origin main

# 4. 重新创建标签
git tag -a v1.0.0 -m "Release v1.0.0"

# 5. 推送标签（触发新的构建）
git push origin v1.0.0
```

### 方案 2: 创建新版本标签

如果不想删除旧标签，可以创建新版本：

```bash
# 1. 提交修复
git add .github/workflows/release.yml
git commit -m "fix: update GitHub Actions workflow"
git push origin main

# 2. 创建新标签
git tag -a v1.0.1 -m "Release v1.0.1 - Fix GitHub Actions"
git push origin v1.0.1
```

---

## ✅ 验证修复

### 1. 检查工作流运行
访问: https://github.com/wyq09/ccswitch/actions

查看新的工作流运行是否成功。

### 2. 检查构建步骤
确认以下步骤都成功：
- ✅ Checkout repository
- ✅ Setup Node.js (应该显示 v20.x.x)
- ✅ Install Rust stable
- ✅ Rust cache
- ✅ Install frontend dependencies
- ✅ Build Tauri app

### 3. 检查 Release
访问: https://github.com/wyq09/ccswitch/releases

确认 Release 已创建并且包含所有平台的安装包。

---

## 📋 完整的修复后工作流

```yaml
name: Release

on:
  push:
    tags:
      - 'v*.*.*'

jobs:
  build-and-release:
    strategy:
      fail-fast: false
      matrix:
        include:
          - platform: 'macos-latest'
            target: 'universal-apple-darwin'
            args: '--target universal-apple-darwin'
          - platform: 'macos-latest'
            target: 'aarch64-apple-darwin'
            args: '--target aarch64-apple-darwin'
          - platform: 'macos-latest'
            target: 'x86_64-apple-darwin'
            args: '--target x86_64-apple-darwin'
          - platform: 'windows-latest'
            target: 'x86_64-pc-windows-msvc'
            args: '--target x86_64-pc-windows-msvc'

    runs-on: ${{ matrix.platform }}
    
    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'  # ✅ 升级到 Node.js 20

      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Rust cache
        uses: swatinem/rust-cache@v2
        with:
          workspaces: './ccswitch-app/src-tauri -> target'

      - name: Install frontend dependencies
        working-directory: ccswitch-app  # ✅ 使用 working-directory
        run: npm ci

      - name: Build Tauri app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          projectPath: './ccswitch-app'
          tagName: ${{ github.ref_name }}
          releaseName: 'CCSwitch ${{ github.ref_name }}'
          releaseBody: |
            ## CCSwitch ${{ github.ref_name }}
            
            ### 🎉 What's New
            
            Please see [CHANGELOG.md](https://github.com/wyq09/ccswitch/blob/main/CHANGELOG.md) for details.
            
            ### 📦 Downloads
            
            Choose the appropriate installer for your platform below.
            
            ### 📖 Documentation
            
            - [README](https://github.com/wyq09/ccswitch/blob/main/README.md)
            - [Quick Start Guide](https://github.com/wyq09/ccswitch/blob/main/docs/QUICK_START.md)
            
            ### ⚠️ Important Notes
            
            - After switching providers, you must restart Claude Code for changes to take effect
            - Backup your `~/.claude/settings.json` before using CCSwitch
          releaseDraft: false
          prerelease: false
          args: ${{ matrix.args }}
```

---

## 🔍 其他可能的问题

### 如果仍然失败，检查以下内容：

#### 1. package-lock.json 存在
```bash
ls -la ccswitch-app/package-lock.json
```

如果不存在，运行：
```bash
cd ccswitch-app
npm install
git add package-lock.json
git commit -m "chore: add package-lock.json"
git push origin main
```

#### 2. 检查 package.json
确保 `ccswitch-app/package.json` 中的依赖都是有效的。

#### 3. 本地测试构建
```bash
cd ccswitch-app
npm ci
npm run tauri build
```

如果本地构建失败，先修复本地问题。

#### 4. 查看详细日志
在 GitHub Actions 页面点击失败的工作流，查看详细错误信息。

---

## 📞 需要帮助？

如果问题仍然存在：

1. 查看 [GitHub Actions 日志](https://github.com/wyq09/ccswitch/actions)
2. 在 [Issues](https://github.com/wyq09/ccswitch/issues) 中报告问题
3. 提供完整的错误日志

---

## ✨ 修复总结

| 项目 | 之前 | 现在 | 原因 |
|------|------|------|------|
| Node.js 版本 | 18 | 20 | 更新、更稳定 |
| npm 缓存 | 启用 | 禁用 | 避免路径问题 |
| 工作目录 | cd 命令 | working-directory | 更清晰 |

---

**修复完成！准备重新触发构建。** 🚀

