# TerraMap-wasm 项目文档

## 项目概述

TerraMap 是一个交互式的 Terraria v1.4.5 世界地图查看器，可以快速加载世界文件，支持平移、缩放、查找方块、矿石、箱子中的物品、地牢、NPC 等功能。

这是一个基于 Web 的 TerraMap 项目，源代码仓库托管在 GitHub。Web 版本仍处于实验阶段，功能不完整。更功能完善但仅限 Windows 的桌面版本可在 https://terramap.github.io/windows.html 获取。

### 主要技术栈

- **前端框架**: 纯 HTML5/CSS3/JavaScript
- **UI 框架**: Bootstrap 3.4.1
- **JavaScript 库**:
  - jQuery 1.12.4
  - jQuery Mousewheel (鼠标滚轮支持)
  - jQuery Panzoom (平移缩放)
  - jQuery Hotkeys (键盘快捷键)
  - FileSaver.js (文件保存)
  - canvas-toBlob.js (Canvas 转换)
- **渲染技术**: HTML5 Canvas API
- **并发处理**: Web Workers (用于世界文件加载)
- **许可证**: MIT License

## 项目结构

```
TerraMap-wasm/
├── index.html              # 主入口页面
├── about.html              # 帮助/关于页面
├── windows.html            # Windows 版本下载页面
├── README.md               # 项目说明文档
├── LICENSE                 # MIT 许可证
├── resources/              # 资源目录
│   ├── css/
│   │   └── styles.css      # 自定义样式
│   ├── images/             # 图标和图像资源
│   └── js/                 # JavaScript 源代码
│       ├── main.js         # 主逻辑，处理 UI 和 Canvas 渲染
│       ├── WorldLoader.js  # Web Worker，处理世界文件加载
│       ├── MapHelper.js    # 地图辅助函数，包含颜色定义
│       ├── DataStream.js   # 二进制数据流处理
│       ├── settings.js     # 游戏设置和颜色配置
│       ├── names.js        # 名称映射
│       ├── sets.js         # 方块集合定义
│       ├── tileIds.js      # 方块 ID 定义
│       ├── tileKeys.js     # 方块键值映射
│       ├── itemIds.js      # 物品 ID 定义
│       ├── itemKeys.js     # 物品键值映射
│       ├── wallIds.js      # 墙体 ID 定义
│       └── wallKeys.js     # 墙体键值映射
└── resources/*.zip         # TerraMap 历史版本压缩包
```

## 构建和运行

### 本地开发

这是一个纯前端项目，不需要构建过程。只需使用本地 Web 服务器即可运行：

```bash
# 使用 Python 3 启动简单 HTTP 服务器
python3 -m http.server 8000

# 或使用 Python 2
python -m SimpleHTTPServer 8000

# 或使用 Node.js http-server
npx http-server -p 8000
```

然后在浏览器中访问: `http://localhost:8000`

### 使用方式

1. 点击 "Choose File" 按钮
2. 选择 Terraria 世界文件（.wld 格式）
3. 世界文件将自动加载并渲染到 Canvas 上
4. 使用以下功能进行交互：
   - **平移/缩放**: 鼠标拖动平移，滚轮缩放
   - **查找方块**: 点击 "Blocks..." 选择要查找的方块类型
   - **高亮显示**: 点击 "Highlight All" 高亮显示所有匹配的方块
   - **清除高亮**: 点击 "Clear Highlight" 清除高亮
   - **保存图片**: 点击 "Save Map Image" 保存当前地图截图
   - **NPC 定位**: 从 NPCs 下拉菜单选择 NPC
   - **预设集合**: 从 Sets 下拉菜单选择预设的方块集合

### 世界文件位置

- **Windows**: `%USERPROFILE%\Documents\My Games\Terraria\Worlds`
- **Windows (Steam Cloud)**: `C:\Program Files (x86)\Steam\userdata\{YOUR_USER_ID}\105600\remote\worlds`
- **MacOS**: `~/Library/Application Support/Terraria/Worlds`
- **MacOS (Steam Cloud)**: `~/Library/Application Support/Steam/userdata/{YOUR_USER_ID}/105600/remote/worlds`
- **Linux**: `~/.local/share/Terraria/Worlds`
- **Linux (Steam Cloud)**: `~/.local/share/Steam/userdata/{YOUR_USER_ID}/105600/remote/worlds`

## 核心架构

### 1. 主流程 (main.js)

`main.js` 是应用的核心控制器，负责：

- **Canvas 初始化**: 设置三个 Canvas 层（主地图层、覆盖层、选择层）
- **UI 事件处理**: 处理文件选择、按钮点击、下拉菜单等
- **PanZoom 集成**: 使用 jQuery Panzoom 插件实现地图的平移和缩放
- **方块高亮**: 查找并高亮显示指定的方块类型
- **地图渲染**: 将世界数据渲染到 Canvas 上
- **快捷键支持**: 使用 jQuery Hotkeys 支持键盘快捷键

### 2. 世界文件加载 (WorldLoader.js)

`WorldLoader.js` 作为 Web Worker 运行，负责：

- **异步读取**: 使用 FileReaderSync 同步读取世界文件（在 Worker 线程中）
- **二进制解析**: 使用 DataStream 解析 Terraria 世界文件格式
- **数据提取**: 提取世界头信息、方块数据、箱子、NPC、标志等
- **消息通信**: 通过 postMessage 向主线程发送加载进度和结果

关键流程：
```javascript
readFileFormatHeader() → readHeader() → readTiles() → readChests() → readSigns() → readNpcs() → readTileEntities()
```

### 3. 地图辅助 (MapHelper.js)

`MapHelper.js` 包含：

- **颜色定义**: 从 Terraria 源代码反编译的方块、液体、墙体颜色
- **渲染函数**: 处理方块的像素级渲染
- **数据查找**: 提供方块、物品、墙体的信息查询功能

### 4. 设置和数据 (settings.js, names.js, sets.js)

- **settings.js**: 包含全局颜色、方块、物品、墙体的详细配置（44,000+ 行）
- **names.js**: 提供方块、物品、墙体的名称映射
- **sets.js**: 定义预设的方块集合（如矿石、家具等）

### 5. 数据处理 (DataStream.js, tileIds.js, itemIds.js, wallIds.js)

- **DataStream.js**: 提供二进制数据流的读写功能
- **tileIds.js, itemIds.js, wallIds.js**: 定义游戏内各类对象的 ID 常量
- **tileKeys.js, itemKeys.js, wallKeys.js**: 提供 ID 到名称的键值映射

## 开发约定

### 代码风格

- 使用 JavaScript ES5+ 语法（兼容性考虑）
- 采用 jQuery 进行 DOM 操作和事件处理
- 使用模块化函数组织代码，但未使用现代模块系统（ES6 Modules）
- Canvas 渲染禁用图像平滑（pixelated 风格）

### 文件命名

- HTML 文件使用小写（index.html, about.html）
- JavaScript 文件使用驼峰命名（main.js, WorldLoader.js）
- CSS 文件使用小写（styles.css）

### 版本控制

- 项目使用 Git 进行版本控制
- 主分支: `master`
- 远程仓库: `git@github.com:uobe1/TerraMap-wasm.git`

### 测试

目前项目没有自动化测试。测试主要通过手动验证功能完成。

## 依赖项

### 外部依赖

- jQuery 1.12.4 (CDN)
- Bootstrap 3.4.1 CSS/JS (CDN)
- Google Analytics (用于网站分析)
- Google AdSense (用于广告)

### 内部依赖

所有 JavaScript 文件按以下顺序加载（在 index.html 中）：
1. jQuery 核心库
2. Bootstrap
3. jQuery 插件（mousewheel, panzoom, hotkeys）
4. 工具库（canvas-toBlob, FileSaver）
5. 游戏数据（settings, names, sets）
6. ID 映射（tileKeys, itemKeys, wallKeys）
7. 核心功能（MapHelper, main）
8. Web Worker（WorldLoader - 异步加载）

## 支持的 Terraria 版本

当前版本支持 Terraria 1.4.5 世界文件。

## 已知限制

- Web 版本功能不完整，相比 Windows 桌面版缺少一些功能
- 需要现代浏览器支持 File API 和 Web Workers
- 大型世界文件加载可能需要较长时间
- Canvas 渲染性能取决于设备能力

## 贡献指南

1. Fork 本仓库
2. 创建功能分支（`git checkout -b feature/AmazingFeature`）
3. 提交更改（`git commit -m 'Add some AmazingFeature'`）
4. 推送到分支（`git push origin feature/AmazingFeature`）
5. 开启 Pull Request

## 联系方式

- 网站主页: https://terramap.github.io
- GitHub Issues: https://github.com/uobe1/TerraMap-wasm/issues

## 许可证

本项目采用 MIT 许可证 - 详见 LICENSE 文件