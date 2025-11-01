# GitHub Actions 路径修复文档

## 问题描述

在 GitHub Actions 中运行 release workflow 时出现以下错误：

```
Error: An error occurred trying to start process '/bin/bash' with working directory '/Users/runner/work/ccswitch/ccswitch/ccswitch-app'. No such file or directory
```

## 根本原因

workflow 配置文件中的路径设置不正确。问题在于：

1. **仓库结构误解**：
   - 仓库名称：`ccswitch`
   - 项目根目录：就是仓库根目录（不是嵌套的 `ccswitch-app` 目录）
   - GitHub Actions checkout 后的路径：`/Users/runner/work/ccswitch/ccswitch/`

2. **错误的路径配置**：
   ```yaml
   # 错误配置
   - name: Rust cache
     uses: swatinem/rust-cache@v2
     with:
       workspaces: './ccswitch-app/src-tauri -> target'  # ❌ 错误

   - name: Install frontend dependencies
     working-directory: ccswitch-app  # ❌ 错误
     run: npm ci

   - name: Build Tauri app
     with:
       projectPath: './ccswitch-app'  # ❌ 错误
   ```

## 解决方案

### 修复内容

修改 `.github/workflows/release.yml` 文件中的三个路径配置：

```yaml
# 正确配置
- name: Rust cache
  uses: swatinem/rust-cache@v2
  with:
    workspaces: './src-tauri -> target'  # ✅ 正确

- name: Install frontend dependencies
  run: npm ci  # ✅ 正确（移除 working-directory）

- name: Build Tauri app
  with:
    projectPath: './'  # ✅ 正确
```

### 修改说明

1. **Rust cache 路径**：
   - 从 `'./ccswitch-app/src-tauri -> target'` 改为 `'./src-tauri -> target'`
   - 因为 checkout 后的当前目录就是项目根目录

2. **npm ci 工作目录**：
   - 移除 `working-directory: ccswitch-app`
   - 直接在根目录执行 `npm ci`

3. **Tauri action 项目路径**：
   - 从 `projectPath: './ccswitch-app'` 改为 `projectPath: './'`
   - 指向当前目录（项目根目录）

## 验证方法

### 本地验证

在项目根目录运行以下命令确认结构正确：

```bash
# 检查目录结构
ls -la
# 应该能看到：
# - src-tauri/
# - src/
# - package.json
# - tauri.conf.json (在 src-tauri/ 中)

# 验证 npm 依赖安装
npm ci

# 验证 Rust 项目路径
cd src-tauri
cargo check
cd ..
```

### GitHub Actions 验证

1. 提交修复后的 workflow 文件
2. 推送到 main 分支
3. 创建并推送新的版本标签
4. 在 GitHub Actions 页面查看构建日志

预期结果：
- ✅ npm ci 步骤成功执行
- ✅ Rust cache 正确配置
- ✅ Tauri 构建成功完成
- ✅ Release 自动创建并上传构建产物

## 相关文件

- `.github/workflows/release.yml` - GitHub Actions workflow 配置
- `fix-and-release.sh` - 自动化修复和发布脚本
- `package.json` - 前端依赖配置（位于根目录）
- `src-tauri/Cargo.toml` - Rust 依赖配置

## 项目结构说明

```
ccswitch/                          # 仓库根目录
├── .github/
│   └── workflows/
│       └── release.yml           # GitHub Actions workflow
├── src/                          # Vue 前端源码
├── src-tauri/                    # Tauri Rust 后端
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── src/
├── package.json                  # 前端依赖
├── package-lock.json
├── vite.config.ts
└── target/                       # Rust 构建输出
```

## 经验总结

1. **GitHub Actions 路径规则**：
   - checkout action 后，当前工作目录是仓库根目录
   - 路径应该相对于仓库根目录，而不是假设的子目录

2. **Tauri 项目结构**：
   - 前端代码在根目录
   - Rust 代码在 `src-tauri/` 子目录
   - 构建输出在 `target/` 目录

3. **调试技巧**：
   - 在 workflow 中添加 `pwd` 和 `ls -la` 命令查看当前目录
   - 检查 GitHub Actions 日志中的完整路径
   - 对比本地项目结构和 CI 环境中的结构

## 参考资料

- [Tauri GitHub Actions Guide](https://tauri.app/v1/guides/building/cross-platform)
- [rust-cache Action](https://github.com/Swatinem/rust-cache)
- [tauri-action](https://github.com/tauri-apps/tauri-action)

---

**修复日期**: 2025-11-01  
**版本**: v1.0.0  
**状态**: ✅ 已解决

