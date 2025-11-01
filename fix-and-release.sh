#!/bin/bash

# CCSwitch - 修复并重新发布脚本
# Fix and Re-release Script

set -e  # 遇到错误立即退出

echo "🔧 CCSwitch - GitHub Actions 修复和重新发布"
echo "============================================"
echo ""

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 版本号
VERSION="v1.0.0"

echo -e "${YELLOW}步骤 1: 检查当前状态${NC}"
git status

echo ""
echo -e "${YELLOW}步骤 2: 删除旧标签${NC}"
echo "删除本地标签 $VERSION..."
git tag -d $VERSION 2>/dev/null || echo "本地标签不存在"

echo "删除远程标签 $VERSION..."
git push origin --delete $VERSION 2>/dev/null || echo "远程标签不存在"

echo ""
echo -e "${YELLOW}步骤 3: 提交修复${NC}"
git add .github/workflows/release.yml
git add docs/GITHUB_ACTIONS_PATH_FIX.md
git add fix-and-release.sh

echo "提交更改..."
git commit -m "fix: correct GitHub Actions workflow paths

- Fix rust-cache workspaces path from './ccswitch-app/src-tauri' to './src-tauri'
- Remove incorrect working-directory from npm ci step
- Update tauri-action projectPath from './ccswitch-app' to './'
- Fix 'No such file or directory' error in GitHub Actions

The issue was that the workflow assumed a nested ccswitch-app directory,
but the repository root is already the project root."

echo ""
echo -e "${YELLOW}步骤 4: 推送到 main 分支${NC}"
git push origin main

echo ""
echo -e "${YELLOW}步骤 5: 创建新标签${NC}"
git tag -a $VERSION -m "Release $VERSION

Initial public release of CCSwitch

Features:
- Provider management for Claude Code
- Multi-language support (EN/CN)
- Built-in templates for 9 popular providers
- Modern UI with glass morphism design
- Auto-release via GitHub Actions

Platforms:
- macOS (Universal, Intel, Apple Silicon)
- Windows (64-bit)

Documentation:
- Complete bilingual documentation
- Quick start guide
- Contributing guidelines"

echo ""
echo -e "${YELLOW}步骤 6: 推送标签（触发 GitHub Actions）${NC}"
git push origin $VERSION

echo ""
echo -e "${GREEN}✅ 完成！${NC}"
echo ""
echo "📊 查看构建进度:"
echo "   https://github.com/wyq09/ccswitch/actions"
echo ""
echo "📦 查看 Release:"
echo "   https://github.com/wyq09/ccswitch/releases"
echo ""
echo "⏳ 构建通常需要 10-20 分钟完成"
echo ""

