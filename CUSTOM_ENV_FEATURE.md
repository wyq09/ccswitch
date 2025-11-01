# 自定义环境变量功能

## 功能概述

在供应商配置页面中添加了自定义环境变量功能，允许用户为每个供应商配置额外的环境变量。这些环境变量会在切换供应商时自动添加到 `~/.claude/settings.json` 的 `env` 对象中。

## 功能场景

### 场景 1：添加自定义超时设置
用户想为某个供应商配置特定的 API 超时时间：
- **环境变量名**: `API_TIMEOUT_MS`
- **值**: `5000000`

### 场景 2：启用/禁用非必要流量
用户想为某个供应商禁用非必要流量：
- **环境变量名**: `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC`
- **值**: `1`

### 场景 3：添加自定义 headers
用户想为 API 请求添加自定义头：
- **环境变量名**: `CUSTOM_HEADERS`
- **值**: `Authorization: Bearer token123`

## 技术实现

### 1. 数据模型更新

#### TypeScript 类型 (`src/types/index.ts`)
```typescript
export type Provider = {
  // ... 其他字段
  customEnv?: Record<string, string | number>  // 新增自定义环境变量字段
  // ... 其他字段
}
```

#### Rust 结构体 (`src-tauri/src/lib.rs`)
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Provider {
    // ... 其他字段
    #[serde(rename = "customEnv")]
    #[serde(default)]
    custom_env: serde_json::Map<String, serde_json::Value>,
    // ... 其他字段
}
```

### 2. 前端 UI 实现

#### 添加供应商页面 (`src/pages/providers/add.vue`)

**模板部分**:
```vue
<!-- Custom Environment Variables -->
<div>
  <div class="flex items-center justify-between mb-3">
    <label class="flex items-center gap-2 text-sm font-medium text-gray-700">
      <svg class="w-4 h-4"><!-- 齿轮图标 --></svg>
      自定义环境变量
    </label>
    <span class="text-xs text-gray-500">可选</span>
  </div>
  
  <!-- 已添加的环境变量列表 -->
  <div class="space-y-3 mb-3">
    <div v-for="(value, key, index) in form.customEnv" :key="index" class="flex gap-2 items-end">
      <input
        :value="key"
        @input="updateCustomEnvKey(index, ($event.target as HTMLInputElement).value)"
        type="text"
        placeholder="环境变量名"
        class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent text-sm"
      />
      <input
        :value="value"
        @input="updateCustomEnvValue(index, ($event.target as HTMLInputElement).value)"
        type="text"
        placeholder="值"
        class="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-primary focus:border-transparent text-sm"
      />
      <button
        @click="removeCustomEnv(index)"
        type="button"
        class="px-3 py-2 text-red-600 hover:bg-red-50 rounded-lg transition-colors"
      >
        <svg class="w-5 h-5"><!-- 删除图标 --></svg>
      </button>
    </div>
  </div>
  
  <!-- 添加新环境变量按钮 -->
  <button
    @click="addCustomEnv"
    type="button"
    class="w-full px-4 py-2 border-2 border-dashed border-gray-300 text-gray-600 hover:border-primary hover:text-primary rounded-lg transition-colors text-sm font-medium"
  >
    <svg class="w-4 h-4 inline-block mr-2"><!-- 加号图标 --></svg>
    添加环境变量
  </button>
</div>
```

**Script 部分**:
```typescript
const form = ref({
  // ... 其他字段
  customEnv: {} as Record<string, string | number>,
})

const addCustomEnv = () => {
  const key = tagInput.value.trim()
  if (key) {
    form.value.customEnv[key] = ''
    tagInput.value = ''
  }
}

const updateCustomEnvKey = (index: number, key: string) => {
  const newKey = key.trim()
  if (newKey) {
    const [oldKey] = Object.keys(form.value.customEnv).splice(index, 1)
    form.value.customEnv[newKey] = form.value.customEnv[oldKey]
    delete form.value.customEnv[oldKey]
  }
}

const updateCustomEnvValue = (index: number, value: string | number) => {
  const newKey = Object.keys(form.value.customEnv)[index]
  if (newKey) {
    form.value.customEnv[newKey] = String(value).trim()
  }
}

const removeCustomEnv = (index: number) => {
  const [key] = Object.keys(form.value.customEnv).splice(index, 1)
  delete form.value.customEnv[key]
}
```

#### 编辑供应商页面 (`src/pages/providers/edit.vue`)

类似的 UI 和逻辑，在页面加载时从现有供应商数据恢复自定义环境变量：

```typescript
onMounted(async () => {
  await providerStore.loadProviders()
  if (provider.value) {
    form.value = {
      // ... 其他字段
      customEnv: { ...(provider.value.customEnv || {}) },
    }
  }
  loading.value = false
})
```

### 3. 后端处理

#### Claude 设置更新 (`src-tauri/src/lib.rs`)

在 `update_claude_settings` 函数中添加自定义环境变量处理：

```rust
fn update_claude_settings(provider: &Provider) -> Result<(), String> {
    // ... 读取现有设置 ...
    
    let env_obj = settings.env.as_object_mut()
        .ok_or("env is not an object")?;
    
    // 添加基础环境变量
    env_obj.insert("ANTHROPIC_BASE_URL".to_string(), serde_json::json!(provider.base_url));
    env_obj.insert("ANTHROPIC_AUTH_TOKEN".to_string(), serde_json::json!(provider.api_key));
    env_obj.insert("ANTHROPIC_MODEL".to_string(), serde_json::json!(&provider.models.default));
    // ... 其他模型字段 ...
    
    // 添加自定义环境变量
    for (key, value) in provider.custom_env.iter() {
        env_obj.insert(key.clone(), value.clone());
    }
    
    // ... 保存设置 ...
}
```

## JSON 格式示例

### 保存到 `providers.json` 的格式

```json
{
  "id": "provider-1",
  "name": "智谱 AI",
  "baseUrl": "https://open.bigmodel.cn/api/anthropic",
  "apiKey": "sk-...",
  "models": {
    "default": "glm-4.6",
    "smallFast": "glm-4.5-air",
    "opus": "glm-4.6",
    "sonnet": "glm-4.6",
    "haiku": "glm-4.5-air"
  },
  "customEnv": {
    "API_TIMEOUT_MS": "5000000",
    "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": 1,
    "CUSTOM_HEADER": "Authorization: Bearer token"
  },
  "isActive": true,
  "tags": [],
  "createdAt": 1234567890,
  "updatedAt": 1234567890
}
```

### 写入 `~/.claude/settings.json` 的格式

```json
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://open.bigmodel.cn/api/anthropic",
    "ANTHROPIC_AUTH_TOKEN": "sk-...",
    "API_TIMEOUT_MS": "5000000",
    "CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC": 1,
    "CUSTOM_HEADER": "Authorization: Bearer token",
    "ANTHROPIC_MODEL": "glm-4.6",
    "ANTHROPIC_SMALL_FAST_MODEL": "glm-4.5-air",
    "ANTHROPIC_DEFAULT_SONNET_MODEL": "glm-4.6",
    "ANTHROPIC_DEFAULT_OPUS_MODEL": "glm-4.6",
    "ANTHROPIC_DEFAULT_HAIKU_MODEL": "glm-4.5-air"
  }
}
```

## 工作流程

### 1. 添加供应商时配置自定义环境变量

```
用户点击 "添加供应商" 
  ↓
输入供应商信息（名称、API Key、Base URL、模型等）
  ↓
点击 "添加环境变量"
  ↓
输入环境变量名和值
  ↓
点击 "保存"
  ↓
供应商和自定义环境变量保存到 providers.json
```

### 2. 编辑供应商时修改自定义环境变量

```
用户点击 "编辑" 按钮
  ↓
页面加载时自动恢复已有的自定义环境变量
  ↓
用户可以：
  - 修改环境变量的值
  - 添加新的环境变量
  - 删除环境变量
  ↓
点击 "保存"
  ↓
供应商和自定义环境变量更新到 providers.json
```

### 3. 切换供应商时应用自定义环境变量

```
用户点击某个供应商的 "切换" 按钮
  ↓
后端读取供应商配置和自定义环境变量
  ↓
生成 ~/.claude/settings.json
  ↓
基础环境变量 + 模型配置 + 自定义环境变量 合并写入
  ↓
弹出提示：已切换供应商，请重启 Claude Code
```

## 特点

✅ **灵活性**: 支持任意数量的自定义环境变量

✅ **可编辑**: 可以随时修改环境变量名和值

✅ **可删除**: 可以删除不需要的环境变量

✅ **可选**: 自定义环境变量完全可选，不配置也能使用

✅ **类型安全**: 支持 string 和 number 类型的值

✅ **持久化**: 自定义环境变量保存在供应商配置中

✅ **自动应用**: 切换供应商时自动应用所有自定义环境变量

## 使用示例

### 示例 1：配置超时和禁用非必要流量

1. 添加供应商 "Moonshot"
2. 在自定义环境变量部分添加：
   - `API_TIMEOUT_MS` = `3000000`
   - `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` = `1`
3. 保存供应商
4. 切换到 "Moonshot" 时，这些环境变量会自动添加到 Claude Code 的配置中

### 示例 2：配置多个供应商的不同设置

- **供应商 A**: 超时 3 秒，禁用非必要流量
- **供应商 B**: 超时 5 秒，启用非必要流量，添加自定义 token
- **供应商 C**: 默认配置，无自定义环境变量

用户可以根据不同供应商的需求灵活配置。

## 技术细节

### 数据结构

- **customEnv**: `Record<string, string | number>`
  - 键：环境变量名
  - 值：环境变量值（可以是字符串或数字）
  - 可选字段（默认为空对象）

### 序列化

- 前端：使用 TypeScript 的 `Record<string, string | number>` 类型
- 后端：使用 `serde_json::Map<String, serde_json::Value>` 类型
- JSON：自动在 `customEnv` 和 `custom_env` 之间转换

### 验证

- 环境变量名：非空、去除空格
- 环境变量值：可以为空（删除之前需要确认）

## 未来改进

1. **预设模板**: 为常见环境变量提供预设值
2. **变量验证**: 对特定环境变量进行格式验证
3. **变量提示**: 提供常见环境变量的自动完成
4. **变量说明**: 为每个环境变量提供简短说明
5. **导入/导出**: 支持从文件导入或导出环境变量配置

## 相关文件

- `src/types/index.ts` - TypeScript 类型定义
- `src/pages/providers/add.vue` - 添加供应商页面
- `src/pages/providers/edit.vue` - 编辑供应商页面
- `src-tauri/src/lib.rs` - Rust 后端逻辑
- `src/stores/provider.ts` - Pinia 状态管理

## 完成情况

✅ 前端 UI 实现（添加、编辑、删除）
✅ 类型定义更新
✅ 后端逻辑实现
✅ JSON 序列化/反序列化
✅ 环境变量合并到 Claude 设置
✅ 编译通过，无错误
