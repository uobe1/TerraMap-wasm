# TerraMap-wasm 迁移方案

## 项目概述

将 TerraMap-wasm 从纯 JavaScript + Bootstrap 3 技术栈迁移到现代技术栈，使用 Rust WebAssembly 实现核心功能，Svelte + TypeScript 作为前端框架，同时保持界面风格完全一致，并增加响应式布局和移动端适配。

## 技术栈

### 后端（Rust + WebAssembly）
- **Rust** - 用于所有核心功能
- **wasm-bindgen** - Rust 与 JavaScript 的互操作
- **wasm-pack** - Rust WebAssembly 打包工具
- **js-sys / web-sys** - 浏览器 API 绑定

### 前端（Svelte + TypeScript）
- **Svelte 5** - 现代响应式框架
- **TypeScript** - 类型安全
- **Vite** - 构建工具

### UI 组件库
- **Bootstrap 5.3** - 升级到最新版本
- **自定义 CSS** - 通过覆盖样式保持与 Bootstrap 3 完全一致的视觉效果

### 开发工具
- ESLint + Prettier - 代码规范
- TypeScript 严格模式 - 类型检查
- Vite 插件：`@wasm-tool/vite-plugin-rust`

## 新架构

```
TerraMap-wasm/
├── rust/                    # Rust WebAssembly 模块
│   ├── src/
│   │   ├── lib.rs          # 主入口，导出 WASM 接口
│   │   ├── data_stream.rs  # 二进制数据流解析
│   │   ├── world_loader.rs # 世界文件加载
│   │   ├── map_helper.rs   # 地图渲染辅助
│   │   ├── renderer.rs     # Canvas 渲染逻辑
│   │   └── search.rs       # 数据查找和筛选
│   ├── Cargo.toml
│   └── pkg/                # wasm-pack 输出目录
│
├── src/                     # Svelte + TypeScript 前端
│   ├── components/         # Svelte 组件
│   │   ├── MapCanvas.svelte      # 地图 Canvas 组件
│   │   ├── Toolbar.svelte        # 工具栏
│   │   ├── BlockSelector.svelte  # 方块选择器
│   │   ├── NPCTracker.svelte     # NPC 追踪
│   │   └── InfoPanel.svelte      # 信息面板
│   ├── lib/
│   │   ├── wasm.ts         # Rust WASM 模块封装
│   │   ├── stores.ts       # Svelte stores（状态管理）
│   │   └── types.ts        # TypeScript 类型定义
│   ├── App.svelte          # 主应用组件
│   ├── main.ts             # 入口文件
│   └── global.css          # 全局样式（复刻 Bootstrap 3 风格）
│
├── static/                  # 静态资源
│   ├── css/
│   │   └── bootstrap.css   # Bootstrap 5（定制版本）
│   ├── js/
│   │   └── legacy/         # 保留的原有数据文件
│   │       ├── settings.js
│   │       ├── names.js
│   │       ├── sets.js
│   │       ├── tileIds.js
│   │       ├── itemIds.js
│   │       └── wallIds.js
│   └── images/
│
├── index.html
├── vite.config.ts
├── tsconfig.json
├── svelte.config.js
└── package.json
```

## 迁移阶段

### 阶段 1：基础设施搭建（1-2 周）

**目标**：搭建完整的开发环境和项目结构

**任务清单**：
- [ ] 初始化 Svelte + Vite + TypeScript 项目
- [ ] 配置 Rust + wasm-pack 环境
- [ ] 集成 Vite Rust 插件
- [ ] 设置 Bootstrap 5 + 自定义样式复刻原有视觉
- [ ] 配置 ESLint、Prettier、TypeScript 严格模式
- [ ] 创建基础项目结构（目录、配置文件）
- [ ] 设置 Git 忽略规则

**技术细节**：
```bash
# 初始化项目
npm create vite@latest terra-map-svelte -- --template svelte-ts
cd terra-map-svelte
npm install

# 安装依赖
npm install -D @wasm-tool/vite-plugin-rust
npm install bootstrap

# Rust 初始化
cd rust
cargo init --lib
```

**配置文件**：
- `vite.config.ts` - 配置 Rust 插件
- `tsconfig.json` - TypeScript 严格模式
- `.eslintrc.cjs` - ESLint 规则
- `.prettierrc` - Prettier 格式化规则
- `svelte.config.js` - Svelte 配置

---

### 阶段 2：数据层迁移（2-3 周）

**目标**：将 JavaScript 数据解析逻辑迁移到 Rust

**任务清单**：
- [ ] 将 `DataStream.js` 迁移到 `data_stream.rs`
- [ ] 将 `WorldLoader.js` 迁移到 `world_loader.rs`
- [ ] 实现 `wasm-bindgen` 接口，暴露给前端
- [ ] 编写 TypeScript 类型定义
- [ ] 单元测试验证数据解析正确性

**技术细节**：

**data_stream.rs**：
```rust
// 实现二进制数据流读取
pub struct DataStream {
    buffer: Vec<u8>,
    position: usize,
}

impl DataStream {
    pub fn new(buffer: Vec<u8>) -> Self { ... }
    pub fn read_byte(&mut self) -> u8 { ... }
    pub fn read_int16(&mut self) -> i16 { ... }
    pub fn read_int32(&mut self) -> i32 { ... }
    pub fn read_string(&mut self) -> String { ... }
    pub fn read_bool(&mut self) -> bool { ... }
    // ... 其他方法
}
```

**world_loader.rs**：
```rust
pub struct World {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
    pub chests: Vec<Chest>,
    pub npcs: Vec<NPC>,
    // ... 其他字段
}

pub fn load_world(data: &[u8]) -> Result<World, String> {
    let mut stream = DataStream::new(data.to_vec());
    // 解析世界文件格式
    // ...
}
```

**WASM 接口**：
```rust
#[wasm_bindgen]
pub struct WorldLoader {
    // ...
}

#[wasm_bindgen]
impl WorldLoader {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self { ... }

    #[wasm_bindgen]
    pub async fn load_from_file(&self, file: &JsValue) -> Result<JsValue, JsValue> {
        // 加载并解析世界文件
    }
}
```

**测试验证**：
- 使用已知的世界文件测试
- 对比解析结果与原版 JavaScript
- 验证所有字段正确解析

---

### 阶段 3：渲染层迁移（2-3 周）

**目标**：将 JavaScript 渲染逻辑迁移到 Rust

**任务清单**：
- [ ] 将 `MapHelper.js` 颜色定义迁移到 Rust
- [ ] 将 `MapHelper.js` 渲染函数迁移到 `renderer.rs`
- [ ] 使用 `web-sys` Canvas API 进行渲染
- [ ] 优化渲染性能（批量绘制、离屏 Canvas）
- [ ] 对比验证渲染结果与原版一致

**技术细节**：

**颜色定义**：
```rust
// map_helper.rs
pub struct TileColors {
    pub dirt: (u8, u8, u8),
    pub grass: (u8, u8, u8),
    pub stone: (u8, u8, u8),
    // ... 44,000+ 颜色定义
}

lazy_static! {
    static ref TILE_COLORS: TileColors = TileColors { ... };
}
```

**渲染器**：
```rust
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    scale: f64,
}

impl Renderer {
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, JsValue> {
        let ctx = canvas.get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        ctx.set_image_smoothing_enabled(false);
        Ok(Self { ctx, scale: 1.0 })
    }

    pub fn render_tile(&self, x: i32, y: i32, tile: &Tile) {
        let color = TILE_COLORS.get_color(tile.tile_id);
        self.ctx.set_fill_style(&format!("rgb({}, {}, {})", color.0, color.1, color.2));
        self.ctx.fill_rect(x as f64, y as f64, 1.0, 1.0);
    }

    pub fn render_world(&self, world: &World) {
        // 批量渲染优化
        for y in 0..world.height {
            for x in 0..world.width {
                self.render_tile(x, y, &world.tiles[(y * world.width + x) as usize]);
            }
        }
    }
}
```

**性能优化**：
- 批量绘制相同颜色的方块
- 使用 Offscreen Canvas（如果支持）
- 按需渲染（只渲染可见区域）

---

### 阶段 4：查找功能迁移（1-2 周）

**目标**：将 JavaScript 查找逻辑迁移到 Rust

**任务清单**：
- [ ] 将方块、物品、NPC 查找逻辑迁移到 `search.rs`
- [ ] 实现高效的搜索算法
- [ ] 暴露查询接口给前端
- [ ] 性能测试和优化

**技术细节**：

**search.rs**：
```rust
pub struct Searcher {
    world: World,
}

impl Searcher {
    pub fn new(world: World) -> Self {
        Self { world }
    }

    pub fn find_tiles(&self, tile_ids: &[i32]) -> Vec<(i32, i32)> {
        let mut results = Vec::new();
        let id_set: HashSet<i32> = tile_ids.iter().cloned().collect();

        for (idx, tile) in self.world.tiles.iter().enumerate() {
            if id_set.contains(&tile.tile_id) {
                let x = idx as i32 % self.world.width;
                let y = idx as i32 / self.world.width;
                results.push((x, y));
            }
        }
        results
    }

    pub fn find_chests_with_item(&self, item_id: i32) -> Vec<Chest> {
        self.world.chests.iter()
            .filter(|chest| chest.items.iter().any(|item| item.id == item_id))
            .cloned()
            .collect()
    }

    pub fn find_npcs(&self, npc_name: &str) -> Vec<NPC> {
        self.world.npcs.iter()
            .filter(|npc| npc.name.contains(npc_name))
            .cloned()
            .collect()
    }
}
```

**WASM 接口**：
```rust
#[wasm_bindgen]
impl Searcher {
    #[wasm_bindgen]
    pub fn find_tiles(&self, tile_ids: JsValue) -> JsValue {
        // 转换参数并返回结果
    }
}
```

---

### 阶段 5：前端 UI 重构（3-4 周）

**目标**：将 JavaScript UI 逻辑迁移到 Svelte 组件

**任务清单**：
- [ ] 将 `main.js` UI 逻辑迁移到 Svelte 组件
- [ ] 实现响应式布局（移动优先设计）
- [ ] 保持原有交互逻辑和快捷键
- [ ] 集成 Panzoom 替代方案

**组件结构**：

**App.svelte** - 主应用
```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { worldStore } from './lib/stores';
  import MapCanvas from './components/MapCanvas.svelte';
  import Toolbar from './components/Toolbar.svelte';
  import BlockSelector from './components/BlockSelector.svelte';
  import NPCTracker from './components/NPCTracker.svelte';
  import InfoPanel from './components/InfoPanel.svelte';
  import { initWasm } from './lib/wasm';

  onMount(async () => {
    await initWasm();
  });
</script>

<div class="app-container">
  <Toolbar />
  <div class="main-content">
    <MapCanvas />
    <BlockSelector />
    <NPCTracker />
    <InfoPanel />
  </div>
</div>
```

**MapCanvas.svelte** - 地图 Canvas
```svelte
<script lang="ts">
  import { onMount } from 'svelte';
  import { worldStore } from '../lib/stores';
  import { renderer } from '../lib/wasm';

  let canvasElement: HTMLCanvasElement;
  let scale = 1;
  let offsetX = 0;
  let offsetY = 0;

  // 缩放和平移逻辑
  function handleWheel(e: WheelEvent) {
    e.preventDefault();
    // 实现缩放
  }

  function handleDrag(e: MouseEvent) {
    // 实现拖动
  }

  // 渲染地图
  function render() {
    if ($worldStore.world) {
      renderer.render_world(canvasElement, $worldStore.world);
    }
  }

  onMount(() => {
    canvasElement.addEventListener('wheel', handleWheel);
    canvasElement.addEventListener('mousedown', handleDrag);
  });
</script>

<canvas bind:this={canvasElement} />
```

**响应式布局**：
```css
/* global.css */
.app-container {
  display: grid;
  grid-template-columns: 250px 1fr;
  height: 100vh;
}

@media (max-width: 768px) {
  .app-container {
    grid-template-columns: 1fr;
    grid-template-rows: auto 1fr auto;
  }

  .sidebar {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    /* 底部导航 */
  }
}
```

---

### 阶段 6：集成与优化（2-3 周）

**目标**：连接 Rust WASM 模块与 Svelte 前端，优化性能

**任务清单**：
- [ ] 连接 Rust WASM 模块与 Svelte 前端
- [ ] 性能优化（Wasm 模块懒加载、渲染优化）
- [ ] 兼容性测试（各浏览器、移动端）
- [ ] 修复 Bug 和边缘情况

**性能优化**：
- WASM 模块懒加载
- 虚拟滚动（对于大型列表）
- 渲染优化（只渲染可见区域）
- 内存管理优化

**兼容性测试**：
- Chrome、Firefox、Safari、Edge
- iOS Safari、Android Chrome
- 不同屏幕尺寸

---

### 阶段 7：测试与发布（1-2 周）

**目标**：完整测试和部署

**任务清单**：
- [ ] 完整功能测试
- [ ] 性能基准测试（对比原版）
- [ ] 文档更新
- [ ] 部署到 GitHub Pages

**功能测试**：
- [ ] 文件加载
- [ ] 地图渲染
- [ ] 平移缩放
- [ ] 方块查找
- [ ] 高亮显示
- [ ] NPC 追踪
- [ ] 保存图片
- [ ] 移动端触摸操作

**性能基准**：
- 世界文件加载时间
- 地图渲染 FPS
- 搜索响应时间

---

## 关键技术决策

### 1. Rust 渲染方案
**选择**：使用 `web-sys` Canvas API
**原因**：
- 原项目使用 Canvas 2D，保持一致性
- WebGL 过度复杂，迁移成本高
- Rust Canvas 2D 性能已经足够快

### 2. 状态管理
**选择**：Svelte 5 Runes（原生响应式）
**原因**：
- Svelte 5 内置响应式，无需额外库
- 性能优于 Redux/MobX
- 代码简洁

### 3. Panzoom 替代方案
**选择**：使用原生 API 或轻量库（如 `panzoom` npm 包）
**原因**：
- jQuery Panzoom 依赖 jQuery，增加包体积
- 现代 API（Transform API）已足够

### 4. 数据文件处理
**选择**：保留 `settings.js`、`names.js` 等为 JSON 格式，由 Rust 加载
**原因**：
- 这些是静态数据，不需要编译到 WASM
- 减小 WASM 模块体积
- 便于后续更新

---

## 响应式设计策略

### 桌面端
- 保持原有布局
- 侧边栏 + 主地图区域

### 移动端
- 隐藏侧边栏，改为底部导航
- 触摸手势支持（捏合缩放、拖动）
- 工具栏折叠为汉堡菜单
- 弹窗全屏显示

### 断点
- 平板：768px
- 手机：480px

---

## 保持界面风格一致的方法

1. **CSS 变量映射**：提取原版 Bootstrap 3 的颜色值，映射到 Bootstrap 5
2. **像素级对比**：使用截图对比工具验证 UI 一致性
3. **保留原版图标**：使用相同的图标资源
4. **字体匹配**：使用相同的字体栈

### Bootstrap 3 → Bootstrap 5 样式映射
```css
/* global.css */
:root {
  /* Bootstrap 3 颜色 */
  --bs-primary: #337ab7;
  --bs-success: #5cb85c;
  --bs-info: #5bc0de;
  --bs-warning: #f0ad4e;
  --bs-danger: #d9534f;
}

/* 按钮样式 */
.btn-primary {
  background-color: var(--bs-primary);
  border-color: #2e6da4;
}

/* 表单样式 */
.form-control {
  border: 1px solid #ccc;
  border-radius: 4px;
}

/* 其他组件样式... */
```

---

## 风险与挑战

| 风险 | 缓解措施 |
|------|---------|
| Rust WASM 性能未达预期 | 性能基准测试，必要时使用 Rust 优化技巧 |
| 移动端兼容性问题 | 浏览器兼容性测试，Polyfill 处理 |
| UI 样式差异 | 像素级对比，CSS 细致调整 |
| 迁移周期过长 | 分阶段交付，先完成核心功能 |
| WASM 模块体积过大 | 代码分割，懒加载 |

---

## 预估时间线

| 阶段 | 任务 | 时间 |
|------|------|------|
| 阶段 1 | 基础设施搭建 | 1-2 周 |
| 阶段 2 | 数据层迁移 | 2-3 周 |
| 阶段 3 | 渲染层迁移 | 2-3 周 |
| 阶段 4 | 查找功能迁移 | 1-2 周 |
| 阶段 5 | 前端 UI 重构 | 3-4 周 |
| 阶段 6 | 集成与优化 | 2-3 周 |
| 阶段 7 | 测试与发布 | 1-2 周 |
| **总计** | | **12-19 周（约 3-5 个月）** |

---

## 参考资料

- [Rust WASM 官方文档](https://rustwasm.github.io/)
- [Svelte 官方文档](https://svelte.dev/)
- [Bootstrap 5 文档](https://getbootstrap.com/)
- [Vite 文档](https://vitejs.dev/)
- [TerraMap 原项目](https://github.com/uobe1/TerraMap-wasm)