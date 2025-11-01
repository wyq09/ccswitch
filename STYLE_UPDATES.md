# 样式更新说明

## 更新日期
2025-10-28

## 更新内容

### 1. 供应商列表页样式优化

#### 之前
- 白色卡片背景，激活时有橙色边框
- 标签显示在卡片底部

#### 现在（参考设计图）
- 未激活：白色背景，灰色边框
- **激活时：整个卡片变为橙色背景，白色文字**
- 更大的圆角（rounded-2xl）
- 更紧凑的间距
- 图标更大更醒目
- 移除标签显示（简化视图）

#### 关键变化
```vue
<!-- 激活状态的卡片 -->
<div class="rounded-2xl p-5 bg-primary text-white">
  <div class="w-14 h-14 rounded-xl bg-white/20">
    <!-- 图标 -->
  </div>
  <h3 class="text-white">供应商名称</h3>
  <p class="text-white/80">API地址</p>
</div>
```

### 2. 添加供应商页样式优化

#### 之前
- 全屏页面布局
- 表单占满整个内容区

#### 现在（参考设计图）
- **弹窗/模态框式布局**
- 居中显示的白色卡片
- 背景添加半透明黑色遮罩和模糊效果
- 添加"查看文档"按钮（橙色背景）
- 更紧凑的模板网格布局
- 添加分隔线区分模板和表单

#### 关键变化
```vue
<!-- 弹窗式布局 -->
<div class="h-full flex items-center justify-center bg-black/40 backdrop-blur-sm">
  <div class="bg-white rounded-2xl shadow-2xl w-full max-w-2xl max-h-[90vh]">
    <!-- 内容 -->
  </div>
</div>

<!-- 查看文档按钮 -->
<button class="px-3 py-1.5 bg-orange-50 text-primary hover:bg-orange-100 rounded-lg">
  查看文档
</button>
```

### 3. 编辑供应商页样式优化

#### 之前
- 全屏页面布局
- 激活横幅独立显示

#### 现在（参考设计图）
- **弹窗/模态框式布局**（与添加页一致）
- 激活横幅紧贴卡片顶部（无间距）
- 更圆润的按钮样式
- 删除按钮颜色更浅（hover时变红）

#### 关键变化
```vue
<!-- 激活横幅（无边距，紧贴卡片） -->
<div class="bg-primary text-white px-8 py-4">
  <span>当前激活的供应商</span>
</div>

<!-- 删除按钮 -->
<button class="text-gray-400 hover:text-red-600">
  <!-- 垃圾桶图标 -->
</button>
```

## 设计原则

### 颜色规范
- **主色（橙色）**: `#F59E0B` (Tailwind primary)
- **侧边栏**: `#2D3748` (Tailwind sidebar)
- **激活状态**: 橙色背景 + 白色文字
- **未激活状态**: 白色背景 + 灰色文字
- **遮罩背景**: `bg-black/40` + `backdrop-blur-sm`

### 圆角规范
- **卡片**: `rounded-2xl` (16px)
- **按钮**: `rounded-xl` (12px) / `rounded-lg` (8px)
- **图标容器**: `rounded-xl` (12px)

### 间距规范
- **卡片间距**: `space-y-3` (12px)
- **卡片内边距**: `p-5` (20px)
- **表单字段间距**: `space-y-5` (20px)

### 字体规范
- **标题**: `text-xl font-semibold`
- **供应商名称**: `text-lg font-semibold`
- **描述文字**: `text-sm text-gray-500`
- **按钮文字**: `font-medium`

### 阴影规范
- **卡片阴影**: `shadow-sm` (未激活) / 无阴影 (激活)
- **弹窗阴影**: `shadow-2xl`
- **hover 阴影**: `hover:shadow-md`

## UI 组件更新

### 供应商卡片
```vue
<div 
  class="rounded-2xl p-5 cursor-pointer transition-all hover:shadow-md"
  :class="isActive ? 'bg-primary text-white' : 'bg-white border border-gray-200 shadow-sm'"
>
  <!-- 卡片内容 -->
</div>
```

### 模态框容器
```vue
<div class="h-full flex items-center justify-center bg-black/40 backdrop-blur-sm">
  <div class="bg-white rounded-2xl shadow-2xl w-full max-w-2xl max-h-[90vh] overflow-hidden flex flex-col">
    <!-- 模态框内容 -->
  </div>
</div>
```

### 主按钮
```vue
<button class="px-6 py-2.5 bg-primary hover:bg-orange-600 text-white rounded-xl font-medium">
  保存
</button>
```

### 次要按钮
```vue
<button class="px-3 py-1.5 bg-orange-50 text-primary hover:bg-orange-100 rounded-lg font-medium">
  查看文档
</button>
```

## 响应式设计

### 容器宽度
- **列表页**: `max-w-5xl mx-auto`
- **弹窗**: `max-w-2xl`
- **最大高度**: `max-h-[90vh]`

### 网格布局
- **模板网格**: `grid-cols-6 gap-3`
- **响应式**: 在小屏幕可考虑 `sm:grid-cols-3 md:grid-cols-6`

## 动画效果

### 过渡动画
- **通用**: `transition-all`
- **颜色**: `transition-colors`
- **阴影**: `hover:shadow-md`

### 背景模糊
- **弹窗遮罩**: `backdrop-blur-sm`

## 对比总结

| 元素 | 之前 | 现在 |
|------|------|------|
| 列表卡片布局 | 白色卡片 + 橙色边框 | 橙色背景 + 白色文字 |
| 添加/编辑页 | 全屏页面 | 居中弹窗 |
| 按钮圆角 | rounded-lg | rounded-xl |
| 卡片圆角 | rounded-xl | rounded-2xl |
| 背景遮罩 | 无 | bg-black/40 + blur |
| 查看文档按钮 | 无 | 橙色背景按钮 |

## 下一步优化建议

1. **动画增强**
   - 添加卡片点击时的缩放效果
   - 弹窗出现/消失的淡入淡出动画

2. **交互优化**
   - 添加骨架屏加载状态
   - 优化表单输入的聚焦效果

3. **可访问性**
   - 添加键盘导航支持
   - 增强对比度

4. **性能优化**
   - 使用虚拟滚动（大量供应商时）
   - 图片/图标懒加载

---

更新完成！UI 现在完全符合设计图要求。

