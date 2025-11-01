#!/bin/bash

# CCSwitch 构建脚本
# 用于构建 macOS 和 Windows 可执行文件

set -e

echo "🚀 CCSwitch 构建脚本"
echo "===================="

# 检查环境
echo "📋 检查环境..."
command -v node >/dev/null 2>&1 || { echo "❌ Node.js 未安装"; exit 1; }
command -v cargo >/dev/null 2>&1 || { echo "❌ Rust 未安装"; exit 1; }

echo "✅ Node.js 版本: $(node -v)"
echo "✅ Rust 版本: $(rustc -V)"

# 安装依赖
echo ""
echo "📦 安装依赖..."
npm install

# 构建前端
echo ""
echo "🎨 构建前端..."
npm run build

# 构建 Tauri 应用
echo ""
echo "🔨 构建 Tauri 应用..."
npm run tauri build

# 显示构建结果
echo ""
echo "✅ 构建完成！"
echo ""
echo "📦 构建输出："
if [ -f "src-tauri/target/release/bundle/macos/CCSwitch.app" ]; then
    echo "   ✅ macOS: src-tauri/target/release/bundle/macos/CCSwitch.app"
else
    echo "   ⏳ macOS 构建中... (需要更多时间)"
fi

if [ -f "src-tauri/target/release/bundle/msi/CCSwitch.msi" ]; then
    echo "   ✅ Windows: src-tauri/target/release/bundle/msi/CCSwitch.msi"
else
    echo "   ℹ️  Windows 需要在 Windows 系统上构建"
fi

echo ""
echo "📝 后续步骤："
echo "   1. macOS: 打开 src-tauri/target/release/bundle/macos/"
echo "   2. Windows: 在 Windows 系统上运行此脚本"
echo "   3. 分发: 复制生成的可执行文件"

