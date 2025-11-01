#!/bin/bash

# CCSwitch Build Script

echo "🔨 Building CCSwitch..."

# Detect platform
OS="$(uname -s)"
case "${OS}" in
    Darwin*)    PLATFORM="macOS";;
    Linux*)     PLATFORM="Linux";;
    MINGW*|MSYS*|CYGWIN*)    PLATFORM="Windows";;
    *)          PLATFORM="Unknown";;
esac

echo "📦 Platform detected: ${PLATFORM}"

# Build based on platform
if [ "${PLATFORM}" = "macOS" ]; then
    echo "🍎 Building for macOS (Universal Binary)..."
    npm run tauri build -- --target universal-apple-darwin
elif [ "${PLATFORM}" = "Windows" ]; then
    echo "🪟 Building for Windows..."
    npm run tauri build -- --target x86_64-pc-windows-msvc
else
    echo "🐧 Building for current platform..."
    npm run tauri build
fi

echo "✅ Build complete!"
echo "📂 Output files are in: src-tauri/target/release/bundle/"

