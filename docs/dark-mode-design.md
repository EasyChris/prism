# 黑夜模式设计规范

## 概述
本文档记录 Claude Code Proxy Hub 的黑夜模式设计规范，基于参考截图分析得出。

---

## 整体配色方案

### 背景色系
- **主背景色**: `#1a1d24` - 深灰蓝色，作为整个应用的底色
- **卡片/面板背景**: `#252932` - 比主背景稍亮的深灰色
- **输入框背景**: `#2d3139` - 更深的灰色，用于表单输入区域
- **悬停状态**: `#2f3440` - 交互元素悬停时的背景色

### 文字色系
- **主标题文字**: `#ffffff` - 纯白色，用于主要标题（如 "Antigravity Tools"）
- **正文文字**: `#e5e7eb` - 浅灰白色，用于常规文本
- **次要文字**: `#9ca3af` - 中灰色，用于辅助说明文字
- **禁用文字**: `#6b7280` - 深灰色，用于禁用状态

### 强调色系
- **主题色（绿色）**: `#10b981` - 用于状态指示（如"服务运行中"）
- **警告色（橙色）**: `#f59e0b` - 用于警告提示
- **危险色（红色）**: `#ef4444` - 用于停止按钮等危险操作
- **信息色（蓝色）**: `#3b82f6` - 用于信息提示和链接

---

## 组件样式规范

### 1. 顶部导航栏
```css
{
  background: #1a1d24;
  border-bottom: 1px solid #2d3139;
  height: 60px;
}
```

**导航标签**:
- 未选中: `color: #9ca3af`
- 选中: `background: #ffffff, color: #1a1d24, border-radius: 20px`
- 悬停: `color: #e5e7eb`

### 2. 状态指示器
**服务运行状态**:
```css
{
  color: #10b981; /* 绿色 */
  display: flex;
  align-items: center;
  gap: 8px;
}
```
- 使用圆点图标 `●` 或 `<span class="status-dot">`
- 文字说明紧随其后

### 3. 按钮样式

**主要按钮（停止服务）**:
```css
{
  background: transparent;
  border: 1px solid #ef4444;
  color: #ef4444;
  border-radius: 8px;
  padding: 8px 16px;
}
```

**次要按钮（打开监控）**:
```css
{
  background: transparent;
  border: 1px solid #6b7280;
  color: #e5e7eb;
  border-radius: 8px;
  padding: 8px 16px;
}
```

**悬停效果**:
- 主要按钮: `background: rgba(239, 68, 68, 0.1)`
- 次要按钮: `background: rgba(107, 114, 128, 0.1)`

### 4. 输入框
```css
{
  background: #2d3139;
  border: 1px solid #3f4451;
  color: #e5e7eb;
  border-radius: 8px;
  padding: 10px 14px;
}

/* 聚焦状态 */
:focus {
  border-color: #3b82f6;
  outline: none;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}
```

### 5. 开关组件（Toggle）
```css
{
  /* 关闭状态 */
  background: #3f4451;
  border-radius: 12px;
  width: 44px;
  height: 24px;
}

/* 开启状态 */
.toggle-on {
  background: #10b981;
}

/* 滑块 */
.toggle-thumb {
  background: #ffffff;
  width: 20px;
  height: 20px;
  border-radius: 50%;
}
```

### 6. 卡片/面板
```css
{
  background: #252932;
  border: 1px solid #2d3139;
  border-radius: 12px;
  padding: 20px;
}

/* 悬停效果 */
:hover {
  border-color: #3f4451;
}
```

### 7. 折叠面板（Accordion）
**标题栏**:
```css
{
  background: #252932;
  border: 1px solid #2d3139;
  border-radius: 12px;
  padding: 16px 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}
```

**展开图标**:
- 使用 `chevron-down` 图标
- 颜色: `#9ca3af`
- 展开时旋转 180 度

### 8. 标签（Badge）
**已启用标签**:
```css
{
  background: rgba(16, 185, 129, 0.1);
  color: #10b981;
  border: 1px solid rgba(16, 185, 129, 0.2);
  border-radius: 6px;
  padding: 4px 12px;
  font-size: 12px;
}
```

### 9. 提示信息
**警告提示**:
```css
{
  background: rgba(245, 158, 11, 0.1);
  border-left: 3px solid #f59e0b;
  color: #fbbf24;
  padding: 12px 16px;
  border-radius: 6px;
}
```

**信息提示**:
```css
{
  background: rgba(59, 130, 246, 0.1);
  border-left: 3px solid #3b82f6;
  color: #60a5fa;
  padding: 12px 16px;
  border-radius: 6px;
}
```

### 10. 图标
- **尺寸**: 16px - 20px（常规）
- **颜色**:
  - 默认: `#9ca3af`
  - 激活: `#e5e7eb`
  - 强调: 跟随主题色

---

## 特殊元素

### 1. 帮助图标（?）
```css
{
  width: 16px;
  height: 16px;
  border: 1px solid #6b7280;
  border-radius: 50%;
  color: #9ca3af;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
}

:hover {
  border-color: #9ca3af;
  color: #e5e7eb;
}
```

### 2. 复制/刷新按钮
```css
{
  background: transparent;
  border: none;
  color: #9ca3af;
  padding: 6px;
  border-radius: 6px;
  cursor: pointer;
}

:hover {
  background: #2d3139;
  color: #e5e7eb;
}
```

### 3. 下拉选择框
```css
{
  background: #2d3139;
  border: 1px solid #3f4451;
  color: #e5e7eb;
  border-radius: 8px;
  padding: 10px 14px;
  appearance: none;
}

/* 下拉箭头 */
.select-arrow {
  color: #9ca3af;
  pointer-events: none;
}
```

---

## 阴影效果

### 卡片阴影
```css
box-shadow: 0 1px 3px rgba(0, 0, 0, 0.3);
```

### 悬浮阴影
```css
box-shadow: 0 4px 6px rgba(0, 0, 0, 0.4);
```

### 模态框阴影
```css
box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.5);
```

---

## 间距规范

### 内边距（Padding）
- **小**: 8px
- **中**: 16px
- **大**: 24px
- **特大**: 32px

### 外边距（Margin）
- **小**: 8px
- **中**: 16px
- **大**: 24px
- **特大**: 32px

### 组件间距（Gap）
- **紧凑**: 8px
- **常规**: 16px
- **宽松**: 24px

---

## 圆角规范

- **小圆角**: 6px - 用于小按钮、标签
- **中圆角**: 8px - 用于输入框、常规按钮
- **大圆角**: 12px - 用于卡片、面板
- **圆形**: 50% - 用于头像、状态点

---

## 过渡动画

### 通用过渡
```css
transition: all 0.2s ease-in-out;
```

### 颜色过渡
```css
transition: color 0.15s ease-in-out,
            background-color 0.15s ease-in-out,
            border-color 0.15s ease-in-out;
```

### 变换过渡
```css
transition: transform 0.2s ease-in-out;
```

---

## Tailwind CSS v4 配置建议

```css
@theme {
  /* 颜色定义 */
  --color-dark-bg-primary: #1a1d24;
  --color-dark-bg-secondary: #252932;
  --color-dark-bg-tertiary: #2d3139;
  --color-dark-bg-hover: #2f3440;

  --color-dark-text-primary: #ffffff;
  --color-dark-text-secondary: #e5e7eb;
  --color-dark-text-tertiary: #9ca3af;
  --color-dark-text-disabled: #6b7280;

  --color-dark-border-primary: #2d3139;
  --color-dark-border-secondary: #3f4451;

  --color-success: #10b981;
  --color-warning: #f59e0b;
  --color-danger: #ef4444;
  --color-info: #3b82f6;

  /* 圆角 */
  --radius-sm: 6px;
  --radius-md: 8px;
  --radius-lg: 12px;
  --radius-full: 50%;

  /* 间距 */
  --spacing-xs: 8px;
  --spacing-sm: 12px;
  --spacing-md: 16px;
  --spacing-lg: 24px;
  --spacing-xl: 32px;
}
```

---

## 实现注意事项

1. **对比度**: 确保文字与背景的对比度符合 WCAG AA 标准（至少 4.5:1）
2. **一致性**: 所有交互元素保持统一的视觉反馈
3. **可访问性**:
   - 所有交互元素支持键盘导航
   - 提供清晰的焦点指示器
   - 使用语义化的 HTML 标签
4. **性能**:
   - 使用 CSS 变量便于主题切换
   - 避免过度使用阴影和模糊效果
   - 优化动画性能（使用 transform 和 opacity）

---

## 参考资源

- [Tailwind CSS v4 文档](https://tailwindcss.com/docs)
- [macOS Human Interface Guidelines - Dark Mode](https://developer.apple.com/design/human-interface-guidelines/dark-mode)
- [Material Design - Dark Theme](https://m3.material.io/styles/color/dark-theme/overview)

---

**最后更新**: 2026-01-14
**维护者**: Claude Code Proxy Hub Team
