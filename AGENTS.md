# TerraMap-wasm 项目文档

## 项目概述

TerraMap 是一个交互式的 Terraria v1.4.5 世界地图查看器，可以快速加载世界文件，支持平移、缩放、查找方块、矿石、箱子中的物品、地牢、NPC 等功能。

这是 TerraMap-wasm 的现代化版本，已完成从纯 JavaScript 技术栈到现代技术栈的迁移。项目采用 Rust WebAssembly 实现核心功能，Svelte 5 + TypeScript 作为前端框架，同时保持界面风格与原版一致，并增加了响应式布局和移动端适配。

源代码仓库托管在 GitHub。Web 版本功能已基本完善，支持所有核心地图查看功能。更功能完善但仅限 Windows 的桌面版本可在 https://terramap.github.io/windows.html 获取。

### 主要技术栈

#### 后端（Rust + WebAssembly）
- **Rust 2021 Edition** - 用于所有核心功能实现
- **wasm-bindgen 0.2** - Rust 与 JavaScript 的互操作
- **wasm-pack 0.14.0+** - Rust WebAssembly 打包工具
- **js-sys / web-sys** - 浏览器 API 绑定
- **serde** - 序列化/反序列化支持

#### 前端（Svelte + TypeScript）
- **Svelte 5** - 现代响应式框架
- **TypeScript 5.7** - 类型安全
- **Vite 6** - 快速的构建工具和开发服务器

#### UI 框架
- **Bootstrap 5.3.3** - 升级到最新版本
- **自定义 CSS** - 通过覆盖样式保持与 Bootstrap 3 完全一致的视觉效果

#### 开发工具
- **ESLint 9** - 代码规范检查（使用新的 flat config 格式）
- **Prettier 3** - 代码格式化
- **TypeScript 严格模式** - 强类型检查
- **Svelte Check** - Svelte 组件类型检查

#### 渲染技术
- **HTML5 Canvas API** - 地图渲染
- **Rust Canvas 2D** - 通过 web-sys 实现
- **可见区域渲染优化** - 只渲染可见区域以提升性能
- **高亮渲染** - 支持方块高亮和 NPC 位置标记
- **图像平滑禁用** - 保持像素风格

#### 许可证
- **MIT License**

## 项目结构

```
TerraMap-wasm/
├── rust/                    # Rust WebAssembly 模块
│   ├── src/
│   │   ├── lib.rs          # 主入口，导出 WASM 接口
│   │   ├── data_stream.rs  # 二进制数据流解析
│   │   ├── world_loader.rs # 世界文件加载器
│   │   ├── renderer.rs     # Canvas 渲染逻辑（支持高亮和可见区域优化）
│   │   ├── search.rs       # 数据查找和筛选
│   │   └── colors.rs       # 颜色定义
│   ├── Cargo.toml          # Rust 项目配置
│   └── target/             # Rust 编译输出
│
├── src/                     # Svelte + TypeScript 前端
│   ├── components/         # Svelte 组件
│   │   ├── MapCanvas.svelte      # 地图 Canvas 组件（支持缩放、平移、高亮、NPC 标记）
│   │   ├── Toolbar.svelte        # 工具栏（文件选择、保存图片）
│   │   ├── BlockSelector.svelte  # 方块选择器（分类、搜索、高亮）
│   │   ├── NPCTracker.svelte     # NPC 追踪器（列表、定位）
│   │   └── InfoPanel.svelte      # 信息面板（显示世界信息）
│   ├── lib/
│   │   ├── wasm.ts         # Rust WASM 模块封装
│   │   ├── stores.ts       # Svelte stores（状态管理：worldStore, highlightStore, npcStore, viewStore）
│   │   ├── types.ts        # TypeScript 类型定义
│   │   ├── tileData.ts     # 750+ 种方块数据定义
│   │   └── npcData.ts      # 30 种 NPC 数据定义
│   ├── App.svelte          # 主应用组件（包含全局快捷键）
│   ├── main.ts             # 入口文件
│   ├── app.css             # 全局样式（复刻 Bootstrap 3 风格）
│   └── pkg.d.ts            # WASM 模块类型声明
│
├── static/                  # 静态资源
│   ├── js/
│   │   └── legacy/         # 保留的原有数据文件
│   │       ├── settings.js     # 游戏设置和颜色配置
│   │       ├── names.js        # 名称映射
│   │       ├── sets.js         # 方块集合定义
│   │       ├── tileIds.js      # 方块 ID 定义
│   │       ├── tileKeys.js     # 方块键值映射
│   │       ├── itemIds.js      # 物品 ID 定义
│   │       ├── itemKeys.js     # 物品键值映射
│   │       ├── wallIds.js      # 墙体 ID 定义
│   │       ├── wallKeys.js     # 墙体键值映射
│   │       ├── main.js         # 原版主逻辑（保留）
│   │       ├── MapHelper.js    # 原版地图辅助（保留）
│   │       ├── WorldLoader.js  # 原版世界加载器（保留）
│   │       ├── DataStream.js   # 原版数据流（保留）
│   │       └── ...其他原版文件
│   └── images/             # 图标和图像资源
│       ├── TerraMap*.png   # 应用图标
│       ├── terramap.png
│       ├── highlight-all.png
│       └── right-click-chest.png
│
├── pkg/                     # wasm-pack 输出目录
│   ├── terra_map_wasm.js   # WASM 绑定 JavaScript
│   ├── terra_map_wasm_bg.wasm # 编译后的 WASM 二进制
│   └── *.d.ts              # TypeScript 类型定义
│
├── resources/               # 原版资源目录（保留）
│   ├── css/
│   │   └── styles.css      # 原版自定义样式
│   ├── images/             # 原版图标和图像资源
│   ├── js/                 # 原版 JavaScript 源代码
│   └── *.zip               # TerraMap 历史版本压缩包
│
├── index.html               # 主入口页面
├── about.html               # 帮助/关于页面
├── windows.html             # Windows 版本下载页面
├── README.md                # 项目说明文档
├── LICENSE                  # MIT 许可证
├── package.json             # Node.js 依赖和脚本
├── package-lock.json        # 依赖锁定文件
├── vite.config.ts           # Vite 构建配置
├── tsconfig.json            # TypeScript 配置
├── svelte.config.js         # Svelte 配置
├── eslint.config.js         # ESLint 配置（v9.0 flat config 格式）
├── .prettierrc              # Prettier 配置
├── .gitignore               # Git 忽略规则
├── Plan.md                  # 迁移方案文档
├── p.md                     # 迁移进度文档
├── pp1.md                   # 进度文档 1
├── pp2.md                   # 进度文档 2
├── test.wld                 # 测试用的 Terraria 世界文件
└── AGENTS.md                # 本文档，项目开发指南
```

## 构建和运行

### 环境要求

- **Node.js** 18+ (推荐使用 LTS 版本)
- **Rust** 1.93.0+ (包含 cargo)
- **wasm-pack** 0.14.0+

### 安装依赖

```bash
# 安装 Node.js 依赖
npm install

# 安装 Rust 工具链（如果尚未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 wasm-pack
cargo install wasm-pack
```

### 本地开发

```bash
# 构建 WASM 模块（首次运行或修改 Rust 代码后）
npm run build:wasm

# 启动开发服务器
npm run dev
```

开发服务器默认运行在 `http://localhost:8000`

### 生产构建

```bash
# 完整构建（包括 WASM 模块、类型检查和前端打包）
npm run build

# 预览生产构建
npm run preview
```

### 代码质量检查

```bash
# TypeScript 类型检查
npm run check

# ESLint 代码检查
npm run lint

# Prettier 代码格式化
npm run format
```

### 原版运行（遗留代码）

如果需要运行原版 JavaScript 版本（不推荐，仅用于对比）：

```bash
# 使用 Python 3 启动简单 HTTP 服务器
python3 -m http.server 8000

# 或使用 Node.js http-server
npx http-server -p 8000
```

然后访问 `http://localhost:8000` 使用原版（注意：原版文件在 `resources/` 目录下）

### 使用方式

1. 点击 "Choose File" 按钮或按 `Ctrl/Cmd + O` 快捷键
2. 选择 Terraria 世界文件（.wld 格式）
3. 世界文件将自动加载并渲染到 Canvas 上
4. 使用以下功能进行交互：

#### 鼠标操作
- **平移**: 鼠标拖动平移地图
- **缩放**: 鼠标滚轮缩放地图

#### 快捷键
- `Ctrl/Cmd + O`: 打开文件选择对话框
- `Ctrl/Cmd + S`: 保存当前地图为图片
- `Ctrl/Cmd + F`: 聚焦到搜索框
- `Escape`: 清除所有高亮和选择
- `+/-` 或 `Ctrl/Cmd + +/-`: 放大/缩小地图
- `0`: 重置缩放（恢复到 1:1 比例）

#### 方块查找
- 点击 "Blocks..." 选择要查找的方块类型
- 使用分类筛选：Ores, Blocks, Furniture, Nature, Bricks, Special
- 使用搜索框快速查找特定方块
- 点击 "Highlight All" 高亮显示所有匹配的方块
- 点击 "Clear Highlight" 清除高亮

#### NPC 追踪
- 从 NPCs 下拉菜单选择 NPC
- 绿色圆点表示 NPC 在世界中，红色圆点表示不在
- 点击 NPC 在地图上标记其位置
- 显示 NPC 名称和坐标信息

#### 预设集合
- 从 Sets 下拉菜单选择预设的方块集合

### 世界文件位置

- **Windows**: `%USERPROFILE%\Documents\My Games\Terraria\Worlds`
- **Windows (Steam Cloud)**: `C:\Program Files (x86)\Steam\userdata\{YOUR_USER_ID}\105600\remote\worlds`
- **MacOS**: `~/Library/Application Support/Terraria/Worlds`
- **MacOS (Steam Cloud)**: `~/Library/Application Support/Steam/userdata/{YOUR_USER_ID}/105600/remote/worlds`
- **Linux**: `~/.local/share/Terraria/Worlds`
- **Linux (Steam Cloud)**: `~/.local/share/Steam/userdata/{YOUR_USER_ID}/105600/remote/worlds`

## 核心架构

### 1. Rust WebAssembly 后端

#### 1.1 主入口 (`lib.rs`)
- 导出所有公共类型和函数
- 使用 `#[wasm_bindgen(start)]` 实现 WASM 模块自动初始化
- 可选启用 `console_error_panic_hook` 以获得更好的错误信息

#### 1.2 数据流处理 (`data_stream.rs`)
提供二进制数据流的读写功能：
- `read_byte()`, `read_bytes()` - 读取字节
- `read_int16()`, `read_uint16()` - 读取 16 位整数
- `read_int32()`, `read_uint32()` - 读取 32 位整数
- `read_int64()` - 读取 64 位整数
- `read_float()`, `read_double()` - 读取浮点数
- `read_bool()`, `read_string()` - 读取布尔值和字符串
- `skip()`, `seek()`, `position()` - 流操作

#### 1.3 世界文件加载 (`world_loader.rs`)
实现 Terraria 世界文件解析：
- `World` 结构体 - 包含名称、尺寸、方块、箱子、NPC 等
- `Tile` 结构体 - 包含方块 ID、墙体 ID、液体、电线等属性
- `Chest` / `ChestItem` 结构体 - 箱子和物品数据
- `NPC` 结构体 - NPC 数据
- `Sign` / `TileEntity` 结构体 - 标志和实体数据
- `WorldLoader` WASM 绑定类 - 提供 `load_from_data()` 接口

关键流程：
```
readFileFormatHeader() → readHeader() → readTiles() → readChests() → readSigns() → readNpcs() → readTileEntities()
```

#### 1.4 渲染器 (`renderer.rs`)
实现 Canvas 2D 渲染：
- `Renderer` 结构体 - 渲染器核心
- `render_world()` - 渲染整个世界
- `render_world_visible_js()` - 只渲染可见区域（性能优化）
- `render_world_with_highlight_js()` - 渲染带高亮的地图
- `render_tile()` - 渲染单个方块
- `set_scale()`, `get_scale()` - 缩放控制
- 禁用图像平滑（保持像素风格）

#### 1.5 颜色定义 (`colors.rs`)
从原版 MapHelper.js 迁移的颜色映射：
- `Rgb` 结构体 (r, g, b)
- `TileColors` 常量定义（包含 100+ 种方块颜色）
- `get_color()` 方法 - 根据方块 ID 返回颜色
- `to_css_string()` 方法 - 转换为 CSS 格式

#### 1.6 搜索模块 (`search.rs`)
实现高效搜索功能：
- `Searcher` 结构体
- `find_tiles()` - 查找指定方块位置
- `find_chests_with_item()` - 查找包含指定物品的箱子
- `find_npcs()` - 查找 NPC
- `find_all()` - 综合搜索（方块 + 物品 + NPC）
- 使用 `HashSet` 优化查找性能

### 2. Svelte 前端

#### 2.1 主应用 (App.svelte)
- 主应用布局
- WASM 模块自动加载（通过动态 import）
- 网格布局（工具栏 + 主内容 + 侧边栏）
- 响应式设计支持
- 全局快捷键处理（包括新增的 `0` 键重置缩放）

#### 2.2 组件架构

##### MapCanvas.svelte
- 地图 Canvas 组件
- 鼠标拖动平移
- 滚轮缩放
- 高亮显示支持
- NPC 位置标记
- 可见区域渲染优化

##### Toolbar.svelte
- 工具栏
- 文件选择功能（"Choose File" 按钮）
- 保存地图为图片功能
- 文件加载处理

##### BlockSelector.svelte
- 方块分类选择（6 大分类：Ores, Blocks, Furniture, Nature, Bricks, Special）
- 搜索过滤功能
- 方块列表选择
- "Highlight All" 按钮（高亮所有匹配方块）
- "Clear Highlight" 按钮（清除高亮）
- 显示找到的方块数量

##### NPCTracker.svelte
- 30 种 NPC 列表
- NPC 搜索过滤
- NPC 状态指示（绿色=在世界中，红色=不在）
- NPC 位置定位（红色圆圈标记）
- 显示选中的 NPC 信息
- 清除选择功能

##### InfoPanel.svelte
- 世界信息显示
- 显示名称、宽度、高度
- 显示加载状态和错误信息

#### 2.3 状态管理 (stores.ts)
使用 Svelte 5 stores 实现响应式状态管理：

##### viewStore（新增）
- 管理视图状态（缩放和平移）
- `setScale()` - 设置缩放比例
- `setOffset()` - 设置偏移量
- `resetZoom()` - 重置缩放和平移

##### worldStore
- 管理世界数据状态
- `setWorld()` - 设置世界数据
- `setLoading()` - 设置加载状态
- `setError()` - 设置错误信息
- `clear()` - 清除所有状态（包括高亮和 NPC 选择）

##### highlightStore
- 管理方块高亮和查找结果
- `setSelectedTile()` - 设置选中方块
- `setHighlightAll()` - 设置高亮模式
- `setFoundPositions()` - 设置查找结果
- `clear()` - 清除高亮

##### npcStore
- 管理 NPC 选择和位置
- `setSelectedNpc()` - 设置选中 NPC
- `clear()` - 清除选择

#### 2.4 类型定义 (types.ts)
TypeScript 类型定义：
- `Tile`, `Chest`, `ChestItem`, `NPC` 接口
- `World` 接口

#### 2.5 数据文件

##### tileData.ts
- 750+ 种方块数据定义
- 6 大分类：Ores, Blocks, Furniture, Nature, Bricks, Special
- `TileData` 接口（id, name, category）
- `getTileById()` - 按 ID 查找方块
- `getTileByName()` - 按名称查找方块
- `tilesByCategory` - 按分类组织的方块列表

##### npcData.ts
- 30 种 NPC 数据定义
- `NPCData` 接口（id, name）
- `getNpcById()` - 按 ID 查找 NPC
- `getNpcByName()` - 按名称查找 NPC

#### 2.6 WASM 封装 (wasm.ts)
Rust WASM 模块封装：
- `initWasm()` - 初始化 WASM
- `getWasmModule()` - 获取 WASM 模块

### 3. 原版遗留代码

以下文件保留在 `static/js/legacy/` 和 `resources/js/` 中，用于参考和对比：
- **main.js** - 原版主逻辑
- **WorldLoader.js** - 原版 Web Worker 世界加载器
- **MapHelper.js** - 原版地图辅助函数
- **DataStream.js** - 原版二进制数据流处理
- **settings.js** - 游戏设置和颜色配置（44,000+ 行）
- **names.js** - 名称映射
- **sets.js** - 方块集合定义
- **tileIds.js, itemIds.js, wallIds.js** - ID 定义
- **tileKeys.js, itemKeys.js, wallKeys.js** - 键值映射

## 开发约定

### 代码风格

#### Rust 代码
- 使用 Rust 2021 edition
- 遵循 Rust 官方风格指南（`cargo fmt`）
- 使用 `cargo clippy` 进行代码检查
- 有意义的变量和函数命名
- 适当使用注释解释复杂逻辑

#### TypeScript/Svelte 代码
- 使用 TypeScript 严格模式
- 遵循 ESLint 规则（`npm run lint`）
- 使用 Prettier 格式化（`npm run format`）
- Svelte 组件使用 PascalCase 命名
- 函数和变量使用 camelCase 命名
- 常量使用 UPPER_CASE 命名
- 类型定义使用 PascalCase 接口

#### CSS 代码
- 使用 BEM 命名约定（块-元素-修饰符）
- CSS 变量定义主题颜色
- 响应式设计优先（移动优先）
- 保持与 Bootstrap 3 一致的视觉效果

### 文件命名

- **Rust 文件**: 使用小写 + 下划线（`data_stream.rs`）
- **Svelte 组件**: 使用 PascalCase（`MapCanvas.svelte`）
- **TypeScript 文件**: 使用 camelCase（`stores.ts`）
- **CSS 文件**: 使用小写（`app.css`）
- **静态资源**: 使用小写 + 连字符（`terramap.png`）

### 目录结构约定

- `rust/src/` - Rust 源代码
- `src/` - 前端源代码
- `src/components/` - Svelte 组件
- `src/lib/` - 工具库和类型定义
- `static/` - 静态资源
- `pkg/` - WASM 编译输出（不提交到 Git）

### Git 工作流程

- 主分支: `master`
- 远程仓库: `https://github.com/uobe1/TerraMap-wasm`
- 提交信息使用中文或英文，清晰描述更改内容
- 重要更改先创建分支，通过 PR 合并

### 版本管理

- 项目版本: 2.0.0（现代化版本）
- 原版版本: 1.4.x（保留在 `resources/`）
- 使用语义化版本控制

### 代码质量

- 必须通过 TypeScript 类型检查（`npm run check`）
- 必须通过 ESLint 检查（`npm run lint`）
- 提交前运行 `npm run format` 格式化代码
- Rust 代码通过 `cargo clippy` 检查

### 测试

目前项目没有自动化测试。测试主要通过：
- 手动功能验证
- 对比原版行为
- 性能基准测试

## 依赖项

### Rust 依赖 (rust/Cargo.toml)

```toml
[dependencies]
wasm-bindgen = "0.2"          # Rust 与 JavaScript 互操作
js-sys = "0.3"                # JavaScript 标准库绑定
web-sys = { version = "0.3", features = [
    "CanvasRenderingContext2d",
    "HtmlCanvasElement",
    "ImageData",
    "Window",
    "Document",
    "Element",
    "console",
] }
serde = { version = "1.0", features = ["derive"] }  # 序列化/反序列化
serde-wasm-bindgen = "0.6"    # WASM 绑定的序列化支持
console_error_panic_hook = { version = "0.1", optional = true }  # 开发时更好的错误信息
```

### Node.js 依赖 (package.json)

#### 生产依赖
- **bootstrap 5.3.3** - UI 框架

#### 开发依赖
- **@sveltejs/vite-plugin-svelte 4.0.0** - Svelte Vite 插件
- **@tsconfig/svelte 5.0.0** - Svelte TypeScript 配置
- **@types/node 22.0.0** - Node.js 类型定义
- **@typescript-eslint/eslint-plugin 8.0.0** - TypeScript ESLint 插件
- **@typescript-eslint/parser 8.0.0** - TypeScript ESLint 解析器
- **eslint 9.0.0** - 代码检查工具
- **eslint-config-prettier 9.0.0** - ESLint 和 Prettier 兼容
- **eslint-plugin-svelte 3.0.0** - Svelte ESLint 插件
- **globals 15.0.0** - 全局变量定义
- **prettier 3.0.0** - 代码格式化工具
- **prettier-plugin-svelte 3.0.0** - Svelte Prettier 插件
- **svelte 5.0.0** - Svelte 框架
- **svelte-check 4.0.0** - Svelte 类型检查工具
- **typescript ~5.7.0** - TypeScript 编译器
- **vite 6.0.0** - 构建工具和开发服务器
- **typescript-eslint 8.0.0** - TypeScript ESLint

### 加载顺序

前端资源按以下顺序加载：
1. Vite 入口点 (`main.ts`)
2. WASM 模块 (`terra_map_wasm.js` - 通过动态 import 自动加载)
3. Svelte 应用 (`App.svelte`)

### 原版遗留依赖（仅供参考）

以下依赖仅存在于原版代码中，已不推荐使用：
- jQuery 1.12.4
- Bootstrap 3.4.1
- jQuery Mousewheel
- jQuery Panzoom
- jQuery Hotkeys
- FileSaver.js
- canvas-toBlob.js

## 支持的 Terraria 版本

当前版本支持 Terraria 1.4.5 世界文件。

## 已知限制

### 功能限制
- Web 版本功能已基本完善，相比 Windows 桌面版只有少量差异
- 颜色定义不完整（目前约 100 种，原项目有 753 种）

### 技术限制
- 需要现代浏览器支持 WebAssembly 和 File API
- 大型世界文件加载可能需要较长时间
- Canvas 渲染性能取决于设备能力
- WASM 模块首次加载可能需要时间（可优化）

### 浏览器兼容性
- 推荐使用最新版本的 Chrome、Firefox、Safari 或 Edge
- 需要支持 WebAssembly 的浏览器
- 移动端支持正在优化中

## 已完成功能

### 核心功能 ✅
- ✅ 世界文件加载和解析
- ✅ 地图渲染和显示
- ✅ 鼠标拖动平移
- ✅ 滚轮缩放
- ✅ 方块分类和搜索
- ✅ 方块高亮显示（全部/单个）
- ✅ NPC 列表和状态指示
- ✅ NPC 位置定位和标记
- ✅ 保存地图为图片
- ✅ 全局快捷键支持
- ✅ 可见区域渲染优化
- ✅ 750+ 种方块数据
- ✅ 30 种 NPC 数据
- ✅ 响应式布局
- ✅ 移动端适配
- ✅ 数字键 0 重置缩放功能

### 架构优化 ✅
- ✅ WASM 模块成功编译和集成
- ✅ Rust 后端实现所有核心功能
- ✅ Svelte 5 前端架构
- ✅ TypeScript 类型安全
- ✅ Store 状态管理（新增 viewStore）
- ✅ 性能优化（可见区域渲染）
- ✅ ESLint v9.0 配置
- ✅ WASM 模块自动初始化（使用 `#[wasm_bindgen(start)]`）

## 待完成功能

### 数据完善（低优先级）
- [ ] 继续迁移颜色定义（从 MapHelper.js，目前约 100 种，原项目 753 种）

### 性能优化（低优先级）
- [ ] WASM 模块懒加载
- [ ] 批量绘制相同颜色的方块
- [ ] Offscreen Canvas 支持

### 技术债务（低优先级）
- [ ] 修复 Rust 编译警告（未使用的导入和变量）
- [ ] 添加更多快捷键（如方向键平移）

## 项目迁移状态

### 已完成 ✅

#### 阶段 1: 基础设施搭建
- ✅ 安装 Rust 工具链和 wasm-pack
- ✅ 初始化 Svelte + Vite + TypeScript 项目
- ✅ 配置 npm 依赖和开发工具
- ✅ 配置 ESLint、Prettier、TypeScript 严格模式
- ✅ 创建项目结构

#### 阶段 2: 数据层迁移
- ✅ 实现 DataStream (`data_stream.rs`)
- ✅ 实现世界文件加载器 (`world_loader.rs`)
- ✅ 实现 Rust 结构体（World, Tile, Chest, NPC 等）
- ✅ 暴露 WASM 接口给前端

#### 阶段 3: 渲染层迁移
- ✅ 实现颜色定义 (`colors.rs`)
- ✅ 实现渲染器 (`renderer.rs`)
- ✅ 实现 Canvas 2D 渲染
- ✅ 禁用图像平滑（像素风格）
- ✅ 实现可见区域渲染优化

#### 阶段 4: 查找功能迁移
- ✅ 实现搜索模块 (`search.rs`)
- ✅ 实现高效搜索算法
- ✅ 暴露查询接口给前端

#### 阶段 5: 前端 UI 重构
- ✅ 创建所有核心 Svelte 组件
- ✅ 实现状态管理 (stores.ts)
- ✅ 定义 TypeScript 类型
- ✅ 实现 WASM 封装

#### 阶段 6: 集成与优化
- ✅ WASM 模块成功编译和集成
- ✅ 实现响应式布局
- ✅ 复刻 Bootstrap 3 视觉效果

#### 阶段 7: 功能完善
- ✅ BlockSelector 完整实现（方块分类、搜索、高亮）
- ✅ NPCTracker 完整实现（NPC 列表、定位）
- ✅ 保存图片功能
- ✅ 快捷键支持
- ✅ 750+ 种方块数据定义
- ✅ 30 种 NPC 数据定义

#### 阶段 8: 代码质量
- ✅ TypeScript 类型检查通过
- ✅ ESLint 配置更新到 v9.0
- ✅ WASM 模块自动初始化
- ✅ 所有核心功能代码审查通过

#### 阶段 9: 增强功能（最新完成）
- ✅ 实现 viewStore 管理视图状态
- ✅ 实现数字键 0 重置缩放功能
- ✅ 使用 `#[wasm_bindgen(start)]` 实现自动初始化
- ✅ 优化 worldStore 清除逻辑

## 贡献指南

### 贡献流程

1. **Fork 本仓库**
   ```bash
   # 在 GitHub 上 fork 项目到你的账户
   ```

2. **克隆你的 fork**
   ```bash
   git clone https://github.com/your-username/TerraMap-wasm.git
   cd TerraMap-wasm
   ```

3. **安装依赖**
   ```bash
   npm install
   ```

4. **创建功能分支**
   ```bash
   git checkout -b feature/AmazingFeature
   ```

5. **构建 WASM 模块**（如果修改了 Rust 代码）
   ```bash
   npm run build:wasm
   ```

6. **启动开发服务器**
   ```bash
   npm run dev
   ```

7. **进行开发和测试**
   - 遵循代码风格约定
   - 运行类型检查：`npm run check`
   - 运行代码检查：`npm run lint`
   - 格式化代码：`npm run format`

8. **提交更改**
   ```bash
   git add .
   git commit -m 'Add some AmazingFeature'
   ```

9. **推送到分支**
   ```bash
   git push origin feature/AmazingFeature
   ```

10. **开启 Pull Request**
    - 在 GitHub 上创建 PR
    - 描述你的更改
    - 等待代码审查

### 代码审查标准

- 代码必须通过类型检查和 ESLint 检查
- 新功能应该有适当的注释
- 保持与现有代码风格一致
- 考虑性能影响，尤其是 WASM 相关代码

### 报告 Bug

如果发现 Bug，请在 GitHub Issues 中提供：
- 详细的问题描述
- 复现步骤
- 浏览器和版本信息
- 控制台错误信息（如果有）
- 截图或录屏（如果适用）

### 功能请求

如果你有功能建议，请在 GitHub Issues 中：
- 清晰描述功能需求
- 说明使用场景
- 提供可能的实现思路

## 开发资源

### 相关文档

- **Plan.md** - 详细的迁移方案和技术决策
- **p.md** - 迁移进度跟踪和已完成工作
- **pp1.md** - 进度文档 1
- **pp2.md** - 进度文档 2
- **README.md** - 原版项目说明（保留）
- **AGENTS.md** - 本文档，项目开发指南

### 技术文档

- [Rust 官方文档](https://www.rust-lang.org/)
- [Rust WASM 官方文档](https://rustwasm.github.io/)
- [wasm-pack 文档](https://rustwasm.github.io/wasm-pack/)
- [Svelte 官方文档](https://svelte.dev/)
- [Svelte 5 文档](https://svelte.dev/docs)
- [TypeScript 官方文档](https://www.typescriptlang.org/)
- [Vite 文档](https://vitejs.dev/)
- [Bootstrap 5 文档](https://getbootstrap.com/)
- [ESLint 9 文档](https://eslint.org/docs/latest/)

### 工具链接

- [TerraMap 原项目](https://github.com/uobe1/TerraMap-wasm)
- [TerraMap 网站主页](https://terramap.github.io)
- [Terraria 官网](https://terraria.org/)
- [Terraria Wiki](https://terraria.fandom.com/wiki/Terraria_Wiki)

### 调试技巧

#### Rust WASM 调试
```bash
# 开启 console_error_panic_hook 以获得更好的错误信息
cd rust && wasm-pack build --target web --out-dir ../pkg --dev
```

#### TypeScript 调试
```bash
# 启用源码映射以便在浏览器中调试
npm run dev
# 在浏览器开发者工具中查看 TypeScript 源码
```

#### 性能分析
- 使用 Chrome DevTools Performance 面板分析渲染性能
- 使用 Rust `cargo flamegraph` 分析 WASM 性能（需要额外工具）

## 联系方式

- 网站主页: https://terramap.github.io
- GitHub Issues: https://github.com/uobe1/TerraMap-wasm/issues

## 许可证

本项目采用 MIT 许可证 - 详见 LICENSE 文件