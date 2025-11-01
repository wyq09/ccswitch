# 多语言支持指南

## 概述

本应用已集成 Vue I18n，支持中文（简体）和英文两种语言。

## 功能特性

- ✅ 自动检测浏览器语言
- ✅ 语言选择持久化存储
- ✅ 实时切换语言
- ✅ 全局语言切换组件

## 使用方法

### 在组件中使用翻译

```vue
<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>

<template>
  <div>
    <h1>{{ t('providers.title') }}</h1>
    <button>{{ t('common.save') }}</button>
  </div>
</template>
```

### 添加新的翻译

1. 在 `src/locales/zh-CN.ts` 中添加中文翻译
2. 在 `src/locales/en-US.ts` 中添加对应的英文翻译

示例：

```typescript
// zh-CN.ts
export default {
  myFeature: {
    title: '我的功能',
    description: '这是描述'
  }
}

// en-US.ts
export default {
  myFeature: {
    title: 'My Feature',
    description: 'This is description'
  }
}
```

### 使用语言切换组件

语言切换组件已添加到供应商列表页面的顶部工具栏。用户可以点击地球图标切换语言。

```vue
<template>
  <LanguageSwitcher />
</template>

<script setup lang="ts">
import LanguageSwitcher from '@/components/LanguageSwitcher.vue'
</script>
```

## 文件结构

```
src/
├── locales/              # 语言文件
│   ├── zh-CN.ts         # 中文翻译
│   ├── en-US.ts         # 英文翻译
│   └── index.ts         # 导出配置
├── plugins/
│   └── i18n.ts          # i18n 插件配置
└── components/
    └── LanguageSwitcher.vue  # 语言切换组件
```

## 支持的语言

- 🇨🇳 简体中文 (zh-CN)
- 🇺🇸 English (en-US)

## 默认语言

应用会按以下优先级确定默认语言：

1. localStorage 中保存的语言设置
2. 浏览器语言（如果是中文则使用 zh-CN）
3. 默认使用 zh-CN

## API

### setLocale(locale: string)

手动切换语言

```typescript
import { setLocale } from '@/plugins/i18n'

setLocale('en-US')  // 切换到英文
setLocale('zh-CN')  // 切换到中文
```

### useI18n()

Vue I18n 的 Composition API

```typescript
import { useI18n } from 'vue-i18n'

const { t, locale } = useI18n()

// 获取翻译
const title = t('providers.title')

// 获取当前语言
console.log(locale.value)  // 'zh-CN' 或 'en-US'
```

## 翻译键命名规范

- 使用小驼峰命名法
- 按功能模块分组
- 保持层级清晰

示例：
```
common.save          // 通用保存按钮
providers.title      // 供应商页面标题
providerForm.apiKey  // 供应商表单的 API 密钥字段
```

