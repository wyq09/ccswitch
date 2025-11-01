# Quick Start Guide / 快速开始指南

[English](#english) | [中文](#中文)

---

<a name="english"></a>

## 🚀 English - Quick Start

### For Users (Just Want to Use the App)

1. **Download the installer** from the [Releases](https://github.com/wyq09/ccswitch/releases) page
   - macOS: Download `.dmg` file
   - Windows: Download `.exe` or `.msi` file

2. **Install the application**
   - macOS: Open `.dmg` and drag to Applications folder
   - Windows: Run the installer and follow the wizard

3. **Launch CCSwitch** from your Applications folder

4. **Add your first provider**
   - Click "Add Provider" button
   - Select a template or enter custom configuration
   - Enter your API key
   - Click "Save"

5. **Switch providers**
   - Click on any provider card to activate it
   - Restart Claude Code

That's it! 🎉

---

### For Developers (Want to Build from Source)

#### Prerequisites Check

Before starting, verify you have:

```bash
# Check Node.js (should be 18+)
node --version

# Check npm
npm --version

# Check Rust (should be 1.70+)
rustc --version

# Check Cargo
cargo --version
```

If any command fails, install the missing tool first.

#### 5-Minute Setup

```bash
# 1. Clone the repository
git clone https://github.com/wyq09/ccswitch.git
cd ccswitch/ccswitch-app

# 2. Install dependencies (this may take a few minutes)
npm install

# 3. Run in development mode
npm run tauri:dev
```

The app will launch automatically! 🚀

#### Build for Production

**macOS (Universal Binary)**:
```bash
npm run tauri:build:mac
```

**Windows (64-bit)**:
```bash
npm run tauri:build:win
```

**Find your build**:
- macOS: `src-tauri/target/universal-apple-darwin/release/bundle/`
- Windows: `src-tauri/target/x86_64-pc-windows-msvc/release/bundle/`

---

<a name="中文"></a>

## 🚀 中文 - 快速开始

### 普通用户（只想使用应用）

1. **下载安装包** 从 [Releases](https://github.com/wyq09/ccswitch/releases) 页面
   - macOS：下载 `.dmg` 文件
   - Windows：下载 `.exe` 或 `.msi` 文件

2. **安装应用程序**
   - macOS：打开 `.dmg` 并拖动到应用程序文件夹
   - Windows：运行安装程序并按照向导操作

3. **启动 CCSwitch** 从应用程序文件夹

4. **添加第一个供应商**
   - 点击"添加供应商"按钮
   - 选择模板或输入自定义配置
   - 输入你的 API 密钥
   - 点击"保存"

5. **切换供应商**
   - 点击任意供应商卡片以激活
   - 重启 Claude Code

完成！🎉

---

### 开发者（想要从源码构建）

#### 环境检查

开始之前，验证你已安装：

```bash
# 检查 Node.js（应该是 18+）
node --version

# 检查 npm
npm --version

# 检查 Rust（应该是 1.70+）
rustc --version

# 检查 Cargo
cargo --version
```

如果任何命令失败，请先安装缺失的工具。

#### 5 分钟设置

```bash
# 1. 克隆仓库
git clone https://github.com/wyq09/ccswitch.git
cd ccswitch/ccswitch-app

# 2. 安装依赖（可能需要几分钟）
npm install

# 3. 以开发模式运行
npm run tauri:dev
```

应用会自动启动！🚀

#### 生产环境构建

**macOS（通用二进制）**：
```bash
npm run tauri:build:mac
```

**Windows（64 位）**：
```bash
npm run tauri:build:win
```

**查找构建文件**：
- macOS：`src-tauri/target/universal-apple-darwin/release/bundle/`
- Windows：`src-tauri/target/x86_64-pc-windows-msvc/release/bundle/`

---

## 📝 Available npm Scripts / 可用的 npm 脚本

| Script | Description (EN) | 说明 (中文) |
|--------|------------------|------------|
| `npm install` | Install dependencies | 安装依赖 |
| `npm run dev` | Start Vite dev server | 启动 Vite 开发服务器 |
| `npm run tauri:dev` | Start Tauri in dev mode | 启动 Tauri 开发模式 |
| `npm run build` | Build frontend | 构建前端 |
| `npm run tauri:build` | Build for current platform | 为当前平台构建 |
| `npm run tauri:build:mac` | Build macOS universal | 构建 macOS 通用版本 |
| `npm run tauri:build:mac:intel` | Build macOS Intel | 构建 macOS Intel 版本 |
| `npm run tauri:build:mac:arm` | Build macOS Apple Silicon | 构建 macOS Apple Silicon 版本 |
| `npm run tauri:build:win` | Build Windows 64-bit | 构建 Windows 64 位版本 |
| `npm run tauri:build:win32` | Build Windows 32-bit | 构建 Windows 32 位版本 |

---

## 🆘 Need Help? / 需要帮助？

- 📖 [Full Documentation](../README.md) / [完整文档](../README.md)
- 🐛 [Report Issues](https://github.com/wyq09/ccswitch/issues) / [报告问题](https://github.com/wyq09/ccswitch/issues)
- 💬 [Discussions](https://github.com/wyq09/ccswitch/discussions) / [讨论区](https://github.com/wyq09/ccswitch/discussions)

---

**Happy Coding! / 编码愉快！** 🎉

