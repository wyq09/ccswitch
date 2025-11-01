# 开发指南

## 项目结构

```
ccswitch-app/
├── src/                      # 前端源代码
│   ├── App.vue              # 主应用组件
│   ├── main.ts              # 应用入口
│   ├── style.css            # 全局样式
│   ├── types/               # TypeScript 类型定义
│   │   └── index.ts
│   ├── config/              # 配置文件
│   │   └── templates.ts     # 内置模板配置
│   ├── stores/              # Pinia 状态管理
│   │   └── provider.ts
│   ├── router/              # Vue Router 路由配置
│   │   └── index.ts
│   └── pages/               # 页面组件
│       └── providers/
│           ├── index.vue    # 供应商列表页
│           ├── add.vue      # 添加供应商页
│           └── edit.vue     # 编辑供应商页
├── src-tauri/               # Tauri 后端
│   ├── src/
│   │   ├── main.rs         # 主程序入口
│   │   └── lib.rs          # 核心逻辑和命令
│   ├── Cargo.toml          # Rust 依赖配置
│   └── tauri.conf.json     # Tauri 配置
└── package.json            # Node.js 依赖配置
```

## 核心功能实现

### 1. 数据存储

应用使用以下位置存储数据：

- **供应商配置**: `{APP_DATA_DIR}/providers.json`
- **自定义模板**: `{APP_DATA_DIR}/templates.json`
- **Claude 配置**: `~/.claude/settings.json`

其中 `{APP_DATA_DIR}` 在不同平台的位置：
- macOS: `~/Library/Application Support/com.yiqunwu.ccswitch/`
- Windows: `%APPDATA%\com.yiqunwu.ccswitch\`

### 2. Tauri 命令

后端实现了以下命令（在 `src-tauri/src/lib.rs`）：

- `load_providers()`: 加载所有供应商
- `save_provider(provider)`: 保存/更新供应商
- `delete_provider(id)`: 删除供应商
- `switch_provider(id)`: 切换激活的供应商
- `load_templates()`: 加载自定义模板
- `save_template(template)`: 保存自定义模板
- `delete_template(id)`: 删除自定义模板
- `get_claude_settings_path_cmd()`: 获取 Claude 配置文件路径

### 3. 供应商切换流程

1. 用户点击供应商卡片
2. 前端调用 `switchProvider(id)`
3. 后端执行：
   - 更新 `providers.json` 中的 `isActive` 状态
   - 读取 `~/.claude/settings.json`
   - 更新 `env` 对象中的环境变量：
     - `ANTHROPIC_BASE_URL`
     - `ANTHROPIC_API_KEY`
     - `ANTHROPIC_MODEL`
     - `ANTHROPIC_DEFAULT_OPUS_MODEL`
     - `ANTHROPIC_DEFAULT_SONNET_MODEL`
     - `ANTHROPIC_DEFAULT_HAIKU_MODEL`
     - 等
   - 写回 `~/.claude/settings.json`
4. 前端显示成功提示

### 4. 内置模板

在 `src/config/templates.ts` 中定义了 6 个内置模板：

1. Claude 默认
2. 智谱 AI
3. 月之暗面
4. DeepSeek
5. 七牛云
6. gaccode

用户可以点击模板快速填充表单。

## 开发工作流

### 启动开发服务器

```bash
npm run tauri dev
```

这会：
1. 启动 Vite 开发服务器（热重载）
2. 启动 Tauri 应用窗口
3. 监听文件变化自动刷新

### 前端开发

修改 `src/` 目录下的文件会自动热重载。

### 后端开发

修改 `src-tauri/src/` 目录下的 Rust 文件需要重启 `npm run tauri dev`。

### 调试

- **前端**: 使用浏览器开发者工具（在 Tauri 窗口中右键 -> Inspect）
- **后端**: 使用 `println!()` 或 Rust 调试器

### 构建生产版本

```bash
# macOS
npm run tauri build -- --target universal-apple-darwin

# Windows
npm run tauri build -- --target x86_64-pc-windows-msvc

# 或使用构建脚本
./build.sh
```

## 常见问题

### Q: 如何添加新的 Tauri 命令？

1. 在 `src-tauri/src/lib.rs` 中定义命令函数
2. 添加 `#[tauri::command]` 属性
3. 在 `run()` 函数的 `invoke_handler` 中注册命令
4. 在前端使用 `invoke('command_name', { params })` 调用

### Q: 如何添加新的路由页面？

1. 在 `src/pages/` 创建新的 `.vue` 文件
2. 在 `src/router/index.ts` 中添加路由配置
3. 在需要的地方使用 `router.push()` 导航

### Q: 如何修改主题颜色？

在 `tailwind.config.js` 中修改 `theme.extend.colors`。

### Q: 构建失败怎么办？

1. 确保安装了所有依赖：`npm install`
2. 清理构建缓存：`rm -rf dist src-tauri/target`
3. 重新构建：`npm run tauri build`

## 测试

### 手动测试清单

- [ ] 添加供应商
- [ ] 编辑供应商
- [ ] 删除供应商
- [ ] 切换供应商
- [ ] 使用快捷模板
- [ ] 标签添加/删除
- [ ] 验证 Claude 配置文件更新
- [ ] 测试退出按钮

### 测试 Claude 配置更新

```bash
# 切换供应商后，检查配置文件
cat ~/.claude/settings.json | jq .env
```

## 发布流程

1. 更新版本号：修改 `package.json` 和 `src-tauri/tauri.conf.json`
2. 构建应用：`./build.sh`
3. 测试安装包
4. 发布到 GitHub Releases 或其他平台

## 贡献指南

1. Fork 项目
2. 创建特性分支：`git checkout -b feature/amazing-feature`
3. 提交更改：`git commit -m 'Add amazing feature'`
4. 推送分支：`git push origin feature/amazing-feature`
5. 创建 Pull Request

