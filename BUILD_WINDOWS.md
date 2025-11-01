# Windows 构建指南

由于我们当前在 macOS 环境，无法直接构建 Windows 版本。以下是在 Windows 系统上构建应用的步骤。

## 前置要求

### 1. 安装 Node.js

从 [Node.js 官网](https://nodejs.org/) 下载并安装 LTS 版本（18.x 或更高）。

### 2. 安装 Rust

1. 从 [Rust 官网](https://www.rust-lang.org/tools/install) 下载 `rustup-init.exe`
2. 运行安装程序，选择默认安装
3. 安装完成后，打开新的命令提示符验证：
   ```cmd
   rustc --version
   cargo --version
   ```

### 3. 安装 Microsoft Visual Studio C++ Build Tools

1. 下载 [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/)
2. 运行安装程序
3. 选择 "使用 C++ 的桌面开发" 工作负载
4. 确保勾选以下组件：
   - MSVC v143 - VS 2022 C++ x64/x86 生成工具
   - Windows 10 SDK 或 Windows 11 SDK

### 4. 安装 WebView2

Windows 10/11 通常已预装 WebView2。如果没有，从 [Microsoft 官网](https://developer.microsoft.com/microsoft-edge/webview2/) 下载安装。

## 构建步骤

### 1. 克隆或复制项目

将整个 `ccswitch-app` 目录复制到 Windows 系统。

### 2. 安装依赖

打开命令提示符或 PowerShell，进入项目目录：

```cmd
cd path\to\ccswitch-app
npm install
```

### 3. 构建应用

```cmd
npm run tauri build -- --target x86_64-pc-windows-msvc
```

构建过程可能需要 5-15 分钟，取决于系统性能。

### 4. 查找构建产物

构建完成后，安装包位于：

```
src-tauri\target\release\bundle\msi\CCSwitch_0.1.0_x64_en-US.msi
src-tauri\target\release\bundle\nsis\CCSwitch_0.1.0_x64-setup.exe
```

## 构建选项

### 仅构建 MSI 安装包

```cmd
npm run tauri build -- --target x86_64-pc-windows-msvc --bundles msi
```

### 仅构建 NSIS 安装包

```cmd
npm run tauri build -- --target x86_64-pc-windows-msvc --bundles nsis
```

### 开发模式测试

在构建前，可以先在开发模式下测试：

```cmd
npm run tauri dev
```

这会启动开发服务器并打开应用窗口。

## 常见问题

### Q: 构建时出现 "link.exe not found" 错误

**A**: 确保已正确安装 Visual Studio Build Tools，并重启命令提示符。

### Q: 构建时出现 WebView2 相关错误

**A**: 安装 [WebView2 Runtime](https://developer.microsoft.com/microsoft-edge/webview2/)。

### Q: 构建时间过长

**A**: 首次构建需要下载和编译大量依赖，后续构建会快很多。可以使用 `--release` 标志进行优化构建。

### Q: 如何减小安装包体积

**A**: Tauri 已经做了很多优化。如需进一步减小，可以：
1. 在 `tauri.conf.json` 中禁用不需要的功能
2. 使用 `strip` 移除调试符号（已默认启用）

## 测试安装包

1. 双击 `.msi` 或 `.exe` 文件
2. 按照安装向导完成安装
3. 从开始菜单启动 CCSwitch
4. 测试所有功能：
   - 添加供应商
   - 编辑供应商
   - 切换供应商
   - 删除供应商
   - 验证 Claude 配置更新

## 签名（可选）

为了避免 Windows SmartScreen 警告，建议对应用进行代码签名：

1. 获取代码签名证书
2. 使用 `signtool` 签名：
   ```cmd
   signtool sign /f certificate.pfx /p password /t http://timestamp.digicert.com CCSwitch_0.1.0_x64-setup.exe
   ```

## 分发

构建完成后，可以通过以下方式分发：

1. **直接分发**: 将 `.msi` 或 `.exe` 文件上传到文件托管服务
2. **GitHub Releases**: 创建 Release 并上传安装包
3. **自建服务器**: 托管在自己的服务器上

## 自动化构建（GitHub Actions）

可以使用 GitHub Actions 自动构建 Windows 版本。创建 `.github/workflows/build-windows.yml`：

```yaml
name: Build Windows

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '18'
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Install dependencies
        run: npm install
      
      - name: Build
        run: npm run tauri build -- --target x86_64-pc-windows-msvc
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: windows-installer
          path: src-tauri/target/release/bundle/**/*
```

## 更新应用

如需更新应用，可以：

1. 修改 `package.json` 和 `src-tauri/tauri.conf.json` 中的版本号
2. 重新构建
3. 用户下载新版本安装包覆盖安装

未来可以集成 Tauri 的自动更新功能。

## 技术支持

如在 Windows 构建过程中遇到问题，请：

1. 检查所有前置要求是否正确安装
2. 查看构建日志中的错误信息
3. 在项目 GitHub Issues 中搜索类似问题
4. 提交新的 Issue 并附上详细的错误信息

---

祝构建顺利！🚀

