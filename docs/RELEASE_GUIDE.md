# Release Guide / 发布指南

[English](#english) | [中文](#中文)

---

<a name="english"></a>

## 🚀 English - How to Release

This guide explains how to create a new release of CCSwitch using GitHub Actions.

### Prerequisites

1. **Permissions**: You must have write access to the repository
2. **Clean State**: Ensure all changes are committed and pushed
3. **Version Number**: Decide on the version number following [Semantic Versioning](https://semver.org/)

### Release Process

#### Step 1: Update Version Numbers

Update the version in the following files:

**1. `ccswitch-app/package.json`**
```json
{
  "version": "1.0.0"  // Update this
}
```

**2. `ccswitch-app/src-tauri/tauri.conf.json`**
```json
{
  "version": "1.0.0"  // Update this
}
```

**3. `ccswitch-app/src-tauri/Cargo.toml`**
```toml
[package]
version = "1.0.0"  # Update this
```

#### Step 2: Update CHANGELOG

Edit `CHANGELOG.md` and move items from `[Unreleased]` to a new version section:

```markdown
## [1.0.0] - 2025-01-15

### Added
- Feature 1
- Feature 2

### Fixed
- Bug fix 1
```

#### Step 3: Commit Changes

```bash
git add .
git commit -m "chore: bump version to 1.0.0"
git push origin main
```

#### Step 4: Create and Push Tag

```bash
# Create a tag
git tag -a v1.0.0 -m "Release v1.0.0"

# Push the tag
git push origin v1.0.0
```

**Important**: The tag must follow the format `v*.*.*` (e.g., `v1.0.0`, `v2.1.3`)

#### Step 5: Monitor GitHub Actions

1. Go to [GitHub Actions](https://github.com/wyq09/ccswitch/actions)
2. Watch the "Release" workflow run
3. The workflow will:
   - Create a GitHub Release
   - Build for all platforms (macOS Universal, Intel, ARM, Windows 64-bit, 32-bit)
   - Upload all installers to the release

#### Step 6: Verify Release

1. Go to [Releases](https://github.com/wyq09/ccswitch/releases)
2. Check that the new release is created
3. Verify all platform installers are uploaded
4. Test download and installation

### What Gets Built

The GitHub Actions workflow builds:

#### macOS
- **Universal Binary** (`.dmg`) - Works on both Intel and Apple Silicon
- **Apple Silicon** (`.dmg`) - Optimized for M1/M2/M3
- **Intel** (`.dmg`) - For Intel-based Macs

#### Windows
- **64-bit** (`.msi`, `.exe`) - For 64-bit Windows
- **32-bit** (`.msi`, `.exe`) - For 32-bit Windows

### Troubleshooting

#### Build Fails

1. Check the [Actions logs](https://github.com/wyq09/ccswitch/actions)
2. Common issues:
   - **Rust compilation error**: Check `src-tauri/` code
   - **Node.js error**: Check frontend dependencies
   - **Permission error**: Ensure `GITHUB_TOKEN` has correct permissions

#### Release Not Created

- Ensure tag format is correct (`v*.*.*`)
- Check that the tag was pushed: `git push origin --tags`
- Verify GitHub Actions is enabled in repository settings

#### Assets Not Uploaded

- Check the workflow logs for upload errors
- Verify file paths in `.github/workflows/release.yml`
- Ensure builds completed successfully

### Manual Release (Fallback)

If GitHub Actions fails, you can build and release manually:

```bash
# Build for your platform
cd ccswitch-app
npm run tauri:build:mac  # or tauri:build:win

# Find the installers in:
# src-tauri/target/[target]/release/bundle/
```

Then manually create a release on GitHub and upload the files.

---

<a name="中文"></a>

## 🚀 中文 - 如何发布

本指南说明如何使用 GitHub Actions 创建 CCSwitch 的新版本。

### 前提条件

1. **权限**：你必须对仓库有写入权限
2. **干净状态**：确保所有更改已提交并推送
3. **版本号**：遵循[语义化版本](https://semver.org/)决定版本号

### 发布流程

#### 步骤 1：更新版本号

在以下文件中更新版本：

**1. `ccswitch-app/package.json`**
```json
{
  "version": "1.0.0"  // 更新这里
}
```

**2. `ccswitch-app/src-tauri/tauri.conf.json`**
```json
{
  "version": "1.0.0"  // 更新这里
}
```

**3. `ccswitch-app/src-tauri/Cargo.toml`**
```toml
[package]
version = "1.0.0"  # 更新这里
```

#### 步骤 2：更新 CHANGELOG

编辑 `CHANGELOG.md`，将 `[Unreleased]` 中的项目移到新版本部分：

```markdown
## [1.0.0] - 2025-01-15

### Added
- 功能 1
- 功能 2

### Fixed
- Bug 修复 1
```

#### 步骤 3：提交更改

```bash
git add .
git commit -m "chore: bump version to 1.0.0"
git push origin main
```

#### 步骤 4：创建并推送标签

```bash
# 创建标签
git tag -a v1.0.0 -m "Release v1.0.0"

# 推送标签
git push origin v1.0.0
```

**重要**：标签必须遵循 `v*.*.*` 格式（例如 `v1.0.0`、`v2.1.3`）

#### 步骤 5：监控 GitHub Actions

1. 访问 [GitHub Actions](https://github.com/wyq09/ccswitch/actions)
2. 观察"Release"工作流运行
3. 工作流将：
   - 创建 GitHub Release
   - 为所有平台构建（macOS 通用版、Intel、ARM、Windows 64位、32位）
   - 将所有安装程序上传到 release

#### 步骤 6：验证发布

1. 访问 [Releases](https://github.com/wyq09/ccswitch/releases)
2. 检查新版本是否已创建
3. 验证所有平台安装程序是否已上传
4. 测试下载和安装

### 构建内容

GitHub Actions 工作流构建：

#### macOS
- **通用二进制**（`.dmg`）- 同时支持 Intel 和 Apple Silicon
- **Apple Silicon**（`.dmg`）- 为 M1/M2/M3 优化
- **Intel**（`.dmg`）- 用于基于 Intel 的 Mac

#### Windows
- **64位**（`.msi`、`.exe`）- 用于 64 位 Windows
- **32位**（`.msi`、`.exe`）- 用于 32 位 Windows

### 故障排除

#### 构建失败

1. 检查 [Actions 日志](https://github.com/wyq09/ccswitch/actions)
2. 常见问题：
   - **Rust 编译错误**：检查 `src-tauri/` 代码
   - **Node.js 错误**：检查前端依赖
   - **权限错误**：确保 `GITHUB_TOKEN` 有正确权限

#### Release 未创建

- 确保标签格式正确（`v*.*.*`）
- 检查标签是否已推送：`git push origin --tags`
- 验证仓库设置中是否启用了 GitHub Actions

#### 资产未上传

- 检查工作流日志中的上传错误
- 验证 `.github/workflows/release.yml` 中的文件路径
- 确保构建成功完成

### 手动发布（备用方案）

如果 GitHub Actions 失败，可以手动构建和发布：

```bash
# 为你的平台构建
cd ccswitch-app
npm run tauri:build:mac  # 或 tauri:build:win

# 在以下位置找到安装程序：
# src-tauri/target/[target]/release/bundle/
```

然后在 GitHub 上手动创建 release 并上传文件。

---

## 📋 Release Checklist / 发布检查清单

### Before Release / 发布前

- [ ] All tests pass / 所有测试通过
- [ ] Documentation updated / 文档已更新
- [ ] CHANGELOG.md updated / CHANGELOG.md 已更新
- [ ] Version numbers updated in all files / 所有文件中的版本号已更新
- [ ] All changes committed and pushed / 所有更改已提交并推送

### During Release / 发布中

- [ ] Tag created with correct format / 标签以正确格式创建
- [ ] Tag pushed to GitHub / 标签已推送到 GitHub
- [ ] GitHub Actions workflow triggered / GitHub Actions 工作流已触发
- [ ] All builds completed successfully / 所有构建成功完成

### After Release / 发布后

- [ ] Release created on GitHub / 在 GitHub 上创建了 release
- [ ] All installers uploaded / 所有安装程序已上传
- [ ] Release notes accurate / Release 说明准确
- [ ] Downloads tested / 下载已测试
- [ ] Installation tested on target platforms / 在目标平台上测试了安装

---

## 🔄 Version Numbering / 版本编号

Follow [Semantic Versioning](https://semver.org/):

- **MAJOR** (1.x.x): Breaking changes / 破坏性更改
- **MINOR** (x.1.x): New features, backward compatible / 新功能，向后兼容
- **PATCH** (x.x.1): Bug fixes, backward compatible / Bug 修复，向后兼容

### Examples / 示例

- `v1.0.0` - Initial release / 初始版本
- `v1.1.0` - Added new provider template / 添加新供应商模板
- `v1.1.1` - Fixed provider switching bug / 修复供应商切换 bug
- `v2.0.0` - Changed configuration file format / 更改配置文件格式

---

**Happy Releasing! / 发布愉快！** 🎉

