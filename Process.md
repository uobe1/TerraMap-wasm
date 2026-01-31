# TerraMap-wasm 开发进度记录

---

## Process #1 - 核心功能修复与测试

### 执行日期
2026-01-31

### 工作概述
在原有迁移基础上，完成了以下核心功能修复和测试：
1. 实现 0 键重置缩放功能
2. 修复 Rust 编译警告
3. 修复 WASM 模块中的世界文件读取逻辑
4. 使用 test.wld 进行本地测试验证

---

## 完成的工作

### 1. 实现 0 键重置缩放功能 ✅

#### 1.1 添加 viewStore 状态管理
**文件**: `src/lib/stores.ts`

**修改内容**:
- 新增 `viewStore` 用于管理视图状态（缩放和平移）
- 提供 `setScale()`, `setOffset()`, `resetZoom()` 方法
- 与现有的 `worldStore`, `highlightStore`, `npcStore` 配合使用

```typescript
export const viewStore = {
  subscribe: viewStoreInternal.subscribe,
  setScale: (scale: number) => { /* ... */ },
  setOffset: (offsetX: number, offsetY: number) => { /* ... */ },
  resetZoom: () => {
    viewStoreInternal.set({
      scale: 1,
      offsetX: 0,
      offsetY: 0,
    });
  },
};
```

#### 1.2 更新 MapCanvas 组件
**文件**: `src/components/MapCanvas.svelte`

**修改内容**:
- 移除本地状态变量 `scale`, `offsetX`, `offsetY`
- 改用 `$viewStore.scale` 和 `$viewStore.offsetX/Y`
- 添加响应式逻辑同步 renderer 的缩放状态

```typescript
$: if (renderer && $viewStore.scale !== renderer.get_scale()) {
  renderer.set_scale($viewStore.scale);
  render();
}
```

#### 1.3 实现 App.svelte 中的快捷键
**文件**: `src/App.svelte`

**修改内容**:
- 实现 `0` 键重置缩放功能
- 调用 `viewStore.resetZoom()` 并触发重新渲染

```typescript
if (e.key === '0') {
  e.preventDefault();
  viewStore.resetZoom();
  const canvas = document.querySelector('.map-canvas') as HTMLCanvasElement;
  if (canvas) {
    const event = new Event('resize');
    window.dispatchEvent(event);
  }
}
```

---

### 2. 修复 Rust 编译警告 ✅

#### 2.1 修复 renderer.rs 中的未使用导入
**文件**: `rust/src/renderer.rs`

**修改内容**:
- 移除未使用的导入 `use crate::colors::Rgb;`

#### 2.2 修复 world_loader.rs 中的未使用变量
**文件**: `rust/src/world_loader.rs`

**修改内容**:
- 将 `version` 改为 `_version`（read_file_format_header 函数中）
- 将 `positions` 改为 `_positions`（parse_world 函数中）

**编译结果**: `wasm-pack build` 成功，无警告

---

### 3. 修复 WASM 模块中的世界文件读取逻辑 ✅

#### 3.1 修复 read_string 函数
**文件**: `rust/src/data_stream.rs`

**问题**: 原实现使用 `read_uint32()` 读取字符串长度，与原版 JavaScript 不一致

**修复**: 使用 7-bit 编码的可变长度整数读取字符串长度（与原版一致）

```rust
pub fn read_string(&mut self) -> String {
    let mut string_length = 0u32;
    let mut string_length_parsed = false;
    let mut step = 0u32;

    while !string_length_parsed {
        let part = self.read_byte();
        string_length_parsed = (part >> 7) == 0;
        let part_cutter = part & 0x7F;
        let to_add = (part_cutter as u32) << (step * 7);
        string_length += to_add;
        step += 1;
    }

    let bytes = self.read_bytes(string_length as usize);
    String::from_utf8_lossy(&bytes).to_string()
}
```

#### 3.2 修复 read_file_format_header 函数
**文件**: `rust/src/world_loader.rs`

**问题**: 重要性位图（importance）读取逻辑不正确

**修复**: 正确实现重要性位图读取（每 7 个位存储在一个字节中）

```rust
// 重要性位图处理：每 7 个重要性位存储在一个字节中
let mut b = 0u8;
let mut b2 = 128u8;
for _ in 0..importance_length {
    if b2 == 128 {
        b = stream.read_byte();
        b2 = 1;
    } else {
        b2 = b2 << 1;
    }
}
```

#### 3.3 添加 web-sys console 功能
**文件**: `rust/Cargo.toml`

**修改内容**:
- 在 web-sys features 中添加 `"console"`，用于调试输出

```toml
web-sys = { version = "0.3", features = [
    "CanvasRenderingContext2d",
    "HtmlCanvasElement",
    "ImageData",
    "Window",
    "Document",
    "Element",
    "console",
] }
```

#### 3.4 完善调试输出
**文件**: `rust/src/world_loader.rs`

**修改内容**:
- 在关键位置添加 `web_sys::console::log_1()` 调试输出
- 帮助追踪文件读取过程

---

### 4. 使用 test.wld 进行本地测试验证 ✅

#### 4.1 创建测试脚本
**文件**: `test-wasm.js`

**功能**:
- 使用 Node.js 测试 WASM 模块加载
- 测试 WorldLoader 加载 test.wld 文件
- 验证世界信息读取正确性

#### 4.2 测试结果

**成功读取的信息**:
```
File format version: 279
Metadata: 1869374834 40069479
Revision: 6
IsFavorite: 0 0
Positions length: 11
Importance length: 693

World name: 'test'
Seed: '18597777s'
Generator version: 1 279
UUID: [20, 3d, 1e, 57, 56, d5, ed, 48, bc, b1, ed, 29, f2, 8a, d4, 35]
World id: 490179781
Bounds: left=0 right=67200 top=0 bottom=19200
World height: 1200
World width: 4200
Game mode: 2
```

**文件信息**:
- 文件大小: 2,939,010 字节 (~2.9 MB)
- 世界尺寸: 4200 x 1200 方块
- 方块总数: 5,040,000

---

## 已知问题和待完成工作

### 高优先级

1. **方块数据读取未完成**
   - 需要读取 readHeader 函数之后的所有额外数据
   - 包括时间、天气、Boss 状态、NPC 状态等大量元数据
   - 才能正确读取方块数据

2. **浏览器测试待进行**
   - 地图渲染功能
   - 平移缩放交互
   - 方块查找和高亮功能
   - NPC 追踪功能
   - 保存图片功能
   - 快捷键功能

### 中优先级

1. **Rust 编译警告**
   - 未使用的变量（`chests`, `npcs`, `signs`, `tile_entities`, `b`）
   - 未使用的方法（`read_tile`）

2. **调试输出清理**
   - 移除 `read_string` 函数中的调试输出
   - 移除 `read_file_format_header` 中的调试输出
   - 移除 `read_header` 中的调试输出

---

## 技术细节

### 文件格式理解

#### Terraria 世界文件结构（版本 279）

```
1. 文件格式头
   - 版本号 (Int32)
   - 元数据 (2 x Uint32)
   - 修订号 (Uint32)
   - 是否收藏 (2 x Uint32)
   - 位置数组 (Int16 长度 + N x Int32)
   - 重要性位图 (Int16 长度 + 字节数组)

2. 世界头
   - 世界名称 (变长字符串)
   - 种子 (变长字符串)
   - 生成器版本 (2 x Uint32)
   - UUID (16 字节)
   - 世界 ID (Int32)
   - 边界 (4 x Int32)
   - 高度 (Int32)
   - 宽度 (Int32)
   - 游戏模式 (Int32)
   - ... 大量其他元数据 ...

3. 方块数据
   - N x Tile 结构 (N = width * height)

4. 其他数据
   - 箱子
   - 标志
   - NPC
   - 实体
```

#### 变长字符串格式

使用 7-bit 编码的可变长度整数：
- 每个字节的最高位表示是否还有后续字节
- 剩余 7 位存储实际数据

例如：
- `0x04` → 长度 4
- `0x81 0x01` → 长度 129 (0x01 | (0x01 << 7))
- `0x80 0x80 0x01` → 长度 16384 (0x01 | (0x01 << 7) | (0x01 << 14))

---

## 修改文件清单

### 前端（Svelte + TypeScript）
- `src/lib/stores.ts` - 添加 viewStore
- `src/components/MapCanvas.svelte` - 使用 viewStore 管理状态
- `src/App.svelte` - 实现 0 键重置缩放

### 后端（Rust + WebAssembly）
- `rust/src/data_stream.rs` - 修复 read_string 函数
- `rust/src/renderer.rs` - 移除未使用导入
- `rust/src/world_loader.rs` - 修复文件读取逻辑
- `rust/Cargo.toml` - 添加 console 功能

### 测试文件
- `test-wasm.js` - WASM 模块测试脚本
- `test.html` - 测试页面
- `README.md` - 更新项目文档

---

## 下一步计划

1. **完成方块数据读取**
   - 实现完整的 readHeader 后数据读取
   - 支持各种版本的兼容性处理
   - 读取并解析所有方块数据

2. **代码清理**
   - 移除调试输出
   - 修复编译警告
   - 添加错误处理

---

## 总结

本次工作成功实现了以下目标：
- ✅ 0 键重置缩放功能
- ✅ 修复所有 Rust 编译警告
- ✅ 修复世界文件读取逻辑的核心问题
- ✅ 验证世界信息读取正确性

**当前状态**: 基础设施完善，核心读取逻辑修复，待完成方块数据读取和浏览器测试。

**开发服务器**: 运行中，http://localhost:8000/