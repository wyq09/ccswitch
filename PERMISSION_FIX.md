# GitHub Actions 权限修复

## 🐛 新问题

在路径修复后，构建成功但创建 Release 时出现新错误：

```
Error: Resource not accessible by integration
https://docs.github.com/rest/releases/releases#create-a-release
```

## 🔍 根本原因

GitHub Actions 默认的 `GITHUB_TOKEN` 权限不足以创建 Release。从 GitHub 的安全更新后，需要显式声明 workflow 所需的权限。

## ✅ 解决方案

在 `.github/workflows/release.yml` 中添加 `permissions` 配置：

```yaml
name: Release

on:
  push:
    tags:
      - 'v*.*.*'

# ✅ 添加这个配置
permissions:
  contents: write

jobs:
  build-and-release:
    # ... 其他配置
```

### 权限说明

- `contents: write` - 允许创建、更新和删除仓库内容，包括：
  - 创建 Release
  - 上传 Release 资产（构建产物）
  - 更新 Release 信息

## 📋 完整修复列表

### 修复 1: 路径配置（已完成）
- ✅ Rust cache workspaces 路径
- ✅ npm ci working-directory
- ✅ Tauri projectPath

### 修复 2: 权限配置（新增）
- ✅ 添加 `permissions: contents: write`

## 🚀 重新发布

由于之前的发布失败，需要删除标签并重新发布：

```bash
# 1. 删除本地标签
git tag -d v1.0.0

# 2. 删除远程标签
git push origin --delete v1.0.0

# 3. 提交权限修复
git add .github/workflows/release.yml
git add PERMISSION_FIX.md
git commit -m "fix: add contents write permission to workflow

- Add permissions.contents: write to allow Release creation
- Fix 'Resource not accessible by integration' error
- Required for creating and uploading Release assets"

# 4. 推送到 main
git push origin main

# 5. 创建新标签
git tag -a v1.0.0 -m "Release v1.0.0"

# 6. 推送标签（触发构建）
git push origin v1.0.0
```

## 📊 预期结果

修复后，GitHub Actions 应该能够：
1. ✅ 成功构建所有平台
2. ✅ 创建 Release v1.0.0
3. ✅ 上传所有构建产物
4. ✅ 完成发布流程

## 🔗 参考资料

- [GitHub Actions Permissions](https://docs.github.com/en/actions/using-jobs/assigning-permissions-to-jobs)
- [Automatic token authentication](https://docs.github.com/en/actions/security-guides/automatic-token-authentication)
- [Permissions for the GITHUB_TOKEN](https://docs.github.com/en/actions/security-guides/automatic-token-authentication#permissions-for-the-github_token)

---

**修复日期**: 2025-11-01  
**状态**: ✅ 已修复  
**下一步**: 重新发布

