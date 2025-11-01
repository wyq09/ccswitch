# Contributing to CCSwitch

[English](#english) | [中文](#中文)

---

<a name="english"></a>

## 🤝 English - Contributing Guide

Thank you for your interest in contributing to CCSwitch! This document provides guidelines for contributing to the project.

### Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for everyone.

### How Can I Contribute?

#### 🐛 Reporting Bugs

Before creating a bug report:
1. Check the [existing issues](https://github.com/wyq09/ccswitch/issues) to avoid duplicates
2. Collect information about the bug:
   - Steps to reproduce
   - Expected behavior
   - Actual behavior
   - Screenshots (if applicable)
   - Your environment (OS, version, etc.)

Create an issue with the `bug` label and include all relevant information.

#### 💡 Suggesting Features

Feature suggestions are welcome! Please:
1. Check if the feature has already been suggested
2. Clearly describe the feature and its benefits
3. Provide examples of how it would work
4. Consider the scope and feasibility

Create an issue with the `enhancement` label.

#### 📝 Improving Documentation

Documentation improvements are always appreciated:
- Fix typos or unclear explanations
- Add missing information
- Improve examples
- Translate documentation

#### 💻 Code Contributions

##### Development Setup

1. **Fork and Clone**
   ```bash
   git clone https://github.com/YOUR_USERNAME/ccswitch.git
   cd ccswitch/ccswitch-app
   ```

2. **Install Dependencies**
   ```bash
   npm install
   ```

3. **Create a Branch**
   ```bash
   git checkout -b feature/your-feature-name
   # or
   git checkout -b fix/your-bug-fix
   ```

4. **Make Changes**
   - Write clean, readable code
   - Follow the existing code style
   - Add comments for complex logic
   - Update documentation if needed

5. **Test Your Changes**
   ```bash
   npm run tauri:dev
   ```

6. **Commit Your Changes**
   ```bash
   git add .
   git commit -m "feat: add amazing feature"
   # or
   git commit -m "fix: resolve issue with provider switching"
   ```

   **Commit Message Format**:
   - `feat:` - New feature
   - `fix:` - Bug fix
   - `docs:` - Documentation changes
   - `style:` - Code style changes (formatting, etc.)
   - `refactor:` - Code refactoring
   - `test:` - Adding or updating tests
   - `chore:` - Maintenance tasks

7. **Push and Create Pull Request**
   ```bash
   git push origin feature/your-feature-name
   ```
   
   Then create a Pull Request on GitHub.

##### Code Style Guidelines

- **TypeScript**: Use TypeScript for all new code
- **Vue 3**: Use Composition API with `<script setup>`
- **Naming**:
  - Components: PascalCase (e.g., `ProviderCard.vue`)
  - Functions: camelCase (e.g., `handleSubmit`)
  - Constants: UPPER_SNAKE_CASE (e.g., `API_TIMEOUT`)
- **Formatting**: Code will be automatically formatted (if configured)

##### Project Structure

```
ccswitch-app/
├── src/
│   ├── assets/          # Static assets
│   ├── components/      # Vue components
│   ├── composables/     # Vue composables
│   ├── config/          # Configuration files
│   ├── locales/         # i18n translations
│   ├── pages/           # Page components
│   ├── plugins/         # Vue plugins
│   ├── router/          # Vue Router config
│   ├── stores/          # Pinia stores
│   └── types/           # TypeScript types
├── src-tauri/           # Tauri backend
└── docs/                # Documentation
```

### Pull Request Process

1. **Update Documentation**: If your changes affect user-facing features
2. **Add Tests**: If applicable (we're working on test coverage)
3. **Update CHANGELOG**: Add your changes to the unreleased section
4. **Request Review**: Wait for maintainer review
5. **Address Feedback**: Make requested changes
6. **Merge**: Once approved, your PR will be merged

### Questions?

Feel free to ask questions by:
- Opening an issue with the `question` label
- Starting a discussion in [GitHub Discussions](https://github.com/wyq09/ccswitch/discussions)

---

<a name="中文"></a>

## 🤝 中文 - 贡献指南

感谢你有兴趣为 CCSwitch 做出贡献！本文档提供了为项目做出贡献的指南。

### 行为准则

参与本项目即表示你同意为每个人维护一个尊重和包容的环境。

### 我可以如何贡献？

#### 🐛 报告 Bug

在创建 bug 报告之前：
1. 检查[现有问题](https://github.com/wyq09/ccswitch/issues)以避免重复
2. 收集有关 bug 的信息：
   - 重现步骤
   - 预期行为
   - 实际行为
   - 截图（如适用）
   - 你的环境（操作系统、版本等）

使用 `bug` 标签创建问题并包含所有相关信息。

#### 💡 建议功能

欢迎功能建议！请：
1. 检查该功能是否已被建议
2. 清楚地描述功能及其好处
3. 提供其工作方式的示例
4. 考虑范围和可行性

使用 `enhancement` 标签创建问题。

#### 📝 改进文档

文档改进总是受欢迎的：
- 修复拼写错误或不清楚的解释
- 添加缺失的信息
- 改进示例
- 翻译文档

#### 💻 代码贡献

##### 开发设置

1. **Fork 和克隆**
   ```bash
   git clone https://github.com/YOUR_USERNAME/ccswitch.git
   cd ccswitch/ccswitch-app
   ```

2. **安装依赖**
   ```bash
   npm install
   ```

3. **创建分支**
   ```bash
   git checkout -b feature/your-feature-name
   # 或
   git checkout -b fix/your-bug-fix
   ```

4. **进行更改**
   - 编写干净、可读的代码
   - 遵循现有的代码风格
   - 为复杂逻辑添加注释
   - 如需要，更新文档

5. **测试你的更改**
   ```bash
   npm run tauri:dev
   ```

6. **提交你的更改**
   ```bash
   git add .
   git commit -m "feat: 添加惊人的功能"
   # 或
   git commit -m "fix: 解决供应商切换问题"
   ```

   **提交消息格式**：
   - `feat:` - 新功能
   - `fix:` - Bug 修复
   - `docs:` - 文档更改
   - `style:` - 代码样式更改（格式化等）
   - `refactor:` - 代码重构
   - `test:` - 添加或更新测试
   - `chore:` - 维护任务

7. **推送并创建 Pull Request**
   ```bash
   git push origin feature/your-feature-name
   ```
   
   然后在 GitHub 上创建 Pull Request。

##### 代码风格指南

- **TypeScript**：所有新代码使用 TypeScript
- **Vue 3**：使用 Composition API 和 `<script setup>`
- **命名**：
  - 组件：PascalCase（例如 `ProviderCard.vue`）
  - 函数：camelCase（例如 `handleSubmit`）
  - 常量：UPPER_SNAKE_CASE（例如 `API_TIMEOUT`）
- **格式化**：代码将自动格式化（如已配置）

##### 项目结构

```
ccswitch-app/
├── src/
│   ├── assets/          # 静态资源
│   ├── components/      # Vue 组件
│   ├── composables/     # Vue 组合式函数
│   ├── config/          # 配置文件
│   ├── locales/         # 国际化翻译
│   ├── pages/           # 页面组件
│   ├── plugins/         # Vue 插件
│   ├── router/          # Vue Router 配置
│   ├── stores/          # Pinia 状态管理
│   └── types/           # TypeScript 类型
├── src-tauri/           # Tauri 后端
└── docs/                # 文档
```

### Pull Request 流程

1. **更新文档**：如果你的更改影响面向用户的功能
2. **添加测试**：如适用（我们正在努力提高测试覆盖率）
3. **更新 CHANGELOG**：将你的更改添加到未发布部分
4. **请求审查**：等待维护者审查
5. **处理反馈**：进行请求的更改
6. **合并**：一旦批准，你的 PR 将被合并

### 有问题？

随时通过以下方式提问：
- 使用 `question` 标签开启问题
- 在 [GitHub Discussions](https://github.com/wyq09/ccswitch/discussions) 中开始讨论

---

## 🙏 Thank You / 谢谢

Thank you for contributing to CCSwitch! Every contribution, no matter how small, is valued and appreciated.

感谢你为 CCSwitch 做出贡献！每一个贡献，无论大小，都是有价值和受赞赏的。

---

**Happy Contributing! / 贡献愉快！** 🎉

