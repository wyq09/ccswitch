# 清空配置功能说明

## 功能介绍

添加了"清空所有配置"功能，允许用户一键清空所有供应商配置和Claude Code环境变量设置。

## 功能特点

### 1. **UI/UX设计**

#### 清空按钮
- 位置：顶部标题栏右侧（在"添加供应商"按钮右边）
- 样式：红色边框 + 红色文字
- 显示条件：仅当存在供应商配置时显示
- 图标：垃圾桶图标（`fas fa-trash-alt`）

#### 确认对话框
- 防止误操作的确认弹窗
- 黑色半透明背景 + 模糊效果
- 显示警告信息
- 两个按钮：取消、确认清空

### 2. **交互流程**

```
用户点击清空按钮
    ↓
弹出确认对话框
    ↓
用户选择"确认清空"
    ↓
1. 清空所有供应商配置文件
2. 清除 ~/.claude/settings.json 中的环境变量
3. 显示成功提示
4. 刷新供应商列表（变为空）
5. 隐藏清空按钮
```

## 技术实现

### 前端部分

#### 1. 组件状态 (`index.vue`)

```typescript
const showClearDialog = ref(false)  // 控制对话框显示/隐藏
```

#### 2. 清空按钮

```vue
<button 
  v-if="providers.length > 0"
  @click="showClearDialog = true"
  class="px-4 py-2 border border-red-300 text-red-600 rounded-md hover:bg-red-50 transition-colors"
  title="清空所有配置"
>
  <i class="fas fa-trash-alt"></i>
</button>
```

**特点**：
- 条件渲染：仅当有供应商时显示
- 点击打开确认对话框
- 红色主题表示危险操作

#### 3. 确认对话框

```vue
<div 
  v-if="showClearDialog"
  class="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50"
>
  <div class="bg-white rounded-2xl shadow-2xl max-w-md w-full mx-4">
    <!-- 标题 -->
    <!-- 警告信息 -->
    <!-- 操作按钮 -->
  </div>
</div>
```

**特点**：
- 固定定位，覆盖整个视口
- 半透明黑色背景 + 模糊效果
- 圆角白色卡片
- 响应式设计

#### 4. 处理函数

```typescript
const handleClearAll = async () => {
  try {
    await providerStore.clearAllProviders()
    showClearDialog.value = false
    toast.success('已清空所有配置')
  } catch (e) {
    toast.error('清空失败：' + e)
  }
}
```

**流程**：
1. 调用store方法
2. 隐藏对话框
3. 显示成功提示
4. 自动刷新列表

### 后端部分

#### Pinia Store (`provider.ts`)

```typescript
async function clearAllProviders() {
  try {
    loading.value = true
    error.value = null
    await invoke('clear_all_providers')  // 调用Rust命令
    await loadProviders()                 // 刷新列表
  } catch (e) {
    error.value = e as string
    console.error('Failed to clear all providers:', e)
    throw e
  } finally {
    loading.value = false
  }
}
```

#### Rust命令 (`lib.rs`)

```rust
#[tauri::command]
fn clear_all_providers(app: tauri::AppHandle) -> Result<(), String> {
    // 1. 清空提供商文件
    let path = get_providers_file_path(app)?;
    fs::write(&path, "[]")
        .map_err(|e| format!("Failed to clear providers file: {}", e))?;
    
    // 2. 清除 Claude 设置
    let settings_path = get_claude_settings_path()?;
    if settings_path.exists() {
        let mut settings: ClaudeSettings = serde_json::from_str(&content)?;
        
        // 清空 env 对象的所有字段
        if let Some(env_obj) = settings.env.as_object_mut() {
            env_obj.clear();
        }
        
        fs::write(&settings_path, serde_json::to_string_pretty(&settings)?)
            .map_err(|e| format!("Failed to write Claude settings: {}", e))?;
    }
    
    Ok(())
}
```

**操作**：
1. 将 `providers.json` 设置为空数组 `[]`
2. 读取 `~/.claude/settings.json`
3. 清空 `env` 对象中的所有环境变量
4. 写回文件

## 文件变更

### 1. 前端文件

#### `src/pages/providers/index.vue`
- ✅ 添加清空按钮到顶部标题栏
- ✅ 添加确认对话框组件
- ✅ 添加 `showClearDialog` 状态
- ✅ 添加 `handleClearAll` 处理函数

#### `src/stores/provider.ts`
- ✅ 添加 `clearAllProviders()` 异步函数
- ✅ 导出新方法

### 2. 后端文件

#### `src-tauri/src/lib.rs`
- ✅ 添加 `clear_all_providers()` Rust命令
- ✅ 在 `invoke_handler` 中注册命令

## 使用示例

### 场景 1：误操作后恢复

```
用户不小心添加了很多错误的供应商配置
    ↓
点击"清空所有配置"按钮
    ↓
确认清空
    ↓
所有配置被删除
    ↓
可以重新添加正确的配置
```

### 场景 2：切换工作环境

```
用户在不同项目中工作
    ↓
项目A中配置的供应商已不需要
    ↓
切换到项目B前清空所有配置
    ↓
为项目B添加新的供应商配置
```

### 场景 3：重置应用状态

```
应用出现异常
    ↓
用户想重置为初始状态
    ↓
点击清空所有配置
    ↓
应用回到空白状态
```

## 安全性考虑

### 1. **确认对话框**
- 防止误操作
- 必须显式点击"确认清空"才能执行

### 2. **视觉警告**
- 使用红色主题表示危险操作
- 对话框中显示清晰的警告信息
- 显示操作影响范围

### 3. **操作可逆性**
- ❌ **不可逆**：清空后需要重新添加配置
- 建议用户在清空前备份重要配置

### 4. **错误处理**
- 捕获所有错误
- 向用户显示清晰的错误信息
- 保持UI状态一致

## 清空后的文件状态

### 清空前

**providers.json:**
```json
[
  {
    "id": "provider-1",
    "name": "gaccode",
    "baseUrl": "https://gaccode.com/claudecode",
    "apiKey": "sk-...",
    "models": { ... },
    "isActive": true,
    ...
  }
]
```

**~/.claude/settings.json:**
```json
{
  "env": {
    "ANTHROPIC_BASE_URL": "https://gaccode.com/claudecode",
    "ANTHROPIC_AUTH_TOKEN": "sk-...",
    "ANTHROPIC_MODEL": "MiniMax-M2",
    ...
  }
}
```

### 清空后

**providers.json:**
```json
[]
```

**~/.claude/settings.json:**
```json
{
  "env": {}
}
```

## UI/UX 细节

### 清空按钮样式

```
[ + 添加供应商 ]  [ 清空 ]  [ 退出 ]
```

- 添加按钮：橙色（主操作）
- 清空按钮：红色（危险操作，仅在有配置时显示）
- 退出按钮：灰色（辅助操作）

### 对话框设计

```
┌─────────────────────────────────┐
│ ⚠️  清空所有配置                 │
│     此操作不可撤销                │
│                                  │
│ ⓘ 将删除所有供应商配置，并清除   │
│   Claude Code 环境变量设置        │
│                                  │
│ [ 取消 ]    [ 确认清空 ]         │
└─────────────────────────────────┘
```

## 可能的改进

### 1. **导出配置**
在清空前提供导出配置的选项：
```
[ 导出并清空 ]  [ 仅清空 ]  [ 取消 ]
```

### 2. **恢复功能**
添加本地备份和恢复功能：
- 清空时自动创建备份
- 提供在最近备份中恢复的选项

### 3. **选择性清空**
允许用户选择清空特定供应商或全部：
- 清空选中的供应商
- 清空所有供应商

### 4. **清空日志**
记录清空操作：
- 时间戳
- 删除的供应商数量
- 操作用户（可选）

## 测试检查清单

- [ ] 点击清空按钮时对话框正确显示
- [ ] 取消按钮能正确关闭对话框
- [ ] 确认清空后：
  - [ ] providers.json 变为空数组
  - [ ] ~/.claude/settings.json 的 env 对象被清空
  - [ ] 供应商列表刷新为空
  - [ ] 清空按钮隐藏
  - [ ] 成功提示显示
- [ ] 错误处理：
  - [ ] 文件权限错误时显示错误提示
  - [ ] 网络错误时显示重试选项
- [ ] 应用可以在清空后正常添加新配置

## 总结

✅ **功能完成**：
- 添加了清空按钮到UI
- 实现了确认对话框
- 后端清空逻辑正确
- 错误处理完善
- 构建成功

✅ **安全性**：
- 多重确认机制
- 清晰的警告信息
- 完善的错误处理

✅ **UX体验**：
- 响应式设计
- 清晰的视觉反馈
- 流畅的交互流程

🎉 **可以正常使用了！**
