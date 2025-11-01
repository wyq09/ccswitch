# CCSwitch v0.1.0 发布说明

## 概述

CCSwitch 是一个桌面应用程序，用于管理和切换 Claude Code 的 API 供应商配置。

## 功能特性

### ✨ 核心功能

- **供应商管理**: 添加、编辑、删除多个 API 供应商配置
- **一键切换**: 快速切换不同的供应商，自动更新 Claude Code 配置
- **快捷模板**: 内置 6 个常用供应商模板，快速填充配置
- **标签系统**: 为供应商添加标签，方便分类管理
- **安全存储**: API 密钥安全存储在本地

### 🎨 用户界面

- 现代化的 UI 设计
- 直观的操作流程
- 实时状态反馈
- 响应式布局

### 🔧 技术特性

- 跨平台支持（macOS、Windows）
- 原生性能（基于 Tauri）
- 自动配置同步
- 数据本地存储

## 内置模板

1. **Claude 默认** - Anthropic 官方 API
2. **智谱 AI** - 智谱 GLM 系列模型
3. **月之暗面** - Moonshot AI
4. **DeepSeek** - DeepSeek 模型
5. **七牛云** - 七牛云 API
6. **gaccode** - gaccode 服务

## 安装

### macOS

1. 下载 `CCSwitch_0.1.0_aarch64.dmg`（Apple Silicon）或 `CCSwitch_0.1.0_x64.dmg`（Intel）
2. 打开 DMG 文件
3. 将 CCSwitch 拖拽到应用程序文件夹
4. 首次运行时，右键点击应用选择"打开"（因为应用未经过 Apple 公证）

### Windows

1. 下载 `CCSwitch_0.1.0_x64.msi` 或 `CCSwitch_0.1.0_x64.exe`
2. 运行安装程序
3. 按照安装向导完成安装

## 使用指南

### 首次使用

1. 启动 CCSwitch
2. 点击右上角的 "+" 按钮添加供应商
3. 选择快捷模板或手动填写配置信息
4. 填写 API 密钥（必填）
5. 点击保存

### 切换供应商

1. 在供应商列表中点击要使用的供应商卡片
2. 等待切换完成提示
3. **重启 Claude Code** 使配置生效

### 编辑供应商

1. 点击供应商卡片右侧的信息图标
2. 修改配置信息
3. 点击保存

### 删除供应商

1. 进入供应商编辑页面
2. 点击右上角的删除按钮
3. 确认删除操作

## 配置文件位置

### 应用数据

- **macOS**: `~/Library/Application Support/com.yiqunwu.ccswitch/`
- **Windows**: `%APPDATA%\com.yiqunwu.ccswitch\`

包含文件：
- `providers.json` - 供应商配置
- `templates.json` - 自定义模板

### Claude 配置

- **所有平台**: `~/.claude/settings.json`

CCSwitch 会自动修改此文件的 `env` 部分。

## 注意事项

⚠️ **重要提示**

1. **切换后需重启**: 切换供应商后，必须重启 Claude Code 才能使新配置生效
2. **备份配置**: 建议在首次使用前备份 `~/.claude/settings.json`
3. **API 密钥安全**: API 密钥存储在本地，请妥善保管应用数据目录
4. **网络连接**: 确保能够访问所选供应商的 API 端点

## 环境变量说明

CCSwitch 会在 Claude 配置中设置以下环境变量：

- `ANTHROPIC_BASE_URL` - API 基础 URL
- `ANTHROPIC_API_KEY` - API 密钥
- `API_TIMEOUT_MS` - 超时时间（默认 3000000ms）
- `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` - 禁用非必要流量（默认 1）
- `ANTHROPIC_MODEL` - 默认模型（Sonnet）
- `ANTHROPIC_SMALL_FAST_MODEL` - 快速模型（Haiku）
- `ANTHROPIC_DEFAULT_SONNET_MODEL` - Sonnet 模型
- `ANTHROPIC_DEFAULT_OPUS_MODEL` - Opus 模型
- `ANTHROPIC_DEFAULT_HAIKU_MODEL` - Haiku 模型

## 故障排除

### 应用无法启动

- **macOS**: 检查是否允许了应用权限（系统设置 > 隐私与安全性）
- **Windows**: 检查是否被防病毒软件拦截

### 切换后配置未生效

1. 确认已重启 Claude Code
2. 检查 `~/.claude/settings.json` 是否正确更新
3. 验证 API 密钥是否正确

### 无法保存供应商

1. 检查应用是否有写入权限
2. 确认应用数据目录是否存在
3. 查看应用日志（开发者工具）

## 已知问题

- 首次启动可能需要几秒钟加载
- macOS 版本需要手动允许运行（未公证）

## 反馈与支持

如遇到问题或有功能建议，请通过以下方式反馈：

- GitHub Issues: [项目地址]
- Email: [联系邮箱]

## 更新日志

### v0.1.0 (2025-10-28)

**首次发布**

- ✨ 供应商管理功能
- ✨ 一键切换供应商
- ✨ 6 个内置快捷模板
- ✨ 标签系统
- ✨ Toast 通知系统
- 🎨 现代化 UI 设计
- 🔧 macOS 和 Windows 支持

## 技术栈

- **前端**: Vue 3 + TypeScript + Tailwind CSS
- **桌面框架**: Tauri 2.x
- **状态管理**: Pinia
- **路由**: Vue Router 4

## 开源协议

MIT License

---

感谢使用 CCSwitch！🎉

