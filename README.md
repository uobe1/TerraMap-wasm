# TerraMap-wasm

TerraMap is an interactive Terraria v1.4.5 world map viewer that loads quickly and lets you pan, zoom, find blocks, ores, items in chests, dungeons, NPCs, etc.

This is the **modern WebAssembly version** of TerraMap, featuring:

- **Rust + WebAssembly backend** for high-performance world file parsing and rendering
- **Svelte 5 + TypeScript frontend** for a modern, reactive user interface
- **Responsive design** with mobile support
- **All core features**: world loading, map rendering, pan/zoom, block search, NPC tracking, and image export

## Technology Stack

- **Backend**: Rust 1.93.0+ with WebAssembly
  - wasm-bindgen for JavaScript interop
  - web-sys for browser API bindings
  - High-performance binary data parsing and canvas rendering

- **Frontend**: Svelte 5 + TypeScript 5.7
  - Vite 6 for fast development and building
  - Bootstrap 5.3 with custom styling (replicates Bootstrap 3 look)
  - Reactive state management with Svelte stores

## Features

- ✅ World file loading and parsing (Terraria 1.4.5+)
- ✅ Map rendering with visible-area optimization
- ✅ Pan and zoom with mouse and keyboard
- ✅ Block search with 750+ block types
- ✅ Block highlighting (single or all matches)
- ✅ NPC tracking with 30 NPC types
- ✅ Save map as image (PNG)
- ✅ Keyboard shortcuts (Ctrl+O, Ctrl+S, Ctrl+F, etc.)
- ✅ Responsive design (mobile and desktop)

## Development

### Prerequisites

- Node.js 18+
- Rust 1.93.0+
- wasm-pack 0.14.0+

### Installation

```bash
# Clone the repository
git clone https://github.com/uobe1/TerraMap-wasm.git
cd TerraMap-wasm

# Install Node.js dependencies
npm install

# Install Rust toolchain (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install wasm-pack
cargo install wasm-pack
```

### Building

```bash
# Build WASM module
npm run build:wasm

# Start development server
npm run dev

# Production build
npm run build

# Preview production build
npm run preview
```

### Code Quality

```bash
# TypeScript type checking
npm run check

# ESLint code checking
npm run lint

# Prettier code formatting
npm run format
```

## Usage

1. Start the development server: `npm run dev`
2. Open `http://localhost:8000` in your browser
3. Click "Choose File" or press `Ctrl+O` to select a Terraria world file (.wld)
4. Use mouse to pan and zoom, or keyboard shortcuts:
   - `Ctrl+O`: Open file
   - `Ctrl+S`: Save map image
   - `Ctrl+F`: Focus search
   - `Escape`: Clear highlights
   - `+/-`: Zoom in/out
   - `0`: Reset zoom

## World File Locations

- **Windows**: `%USERPROFILE%\Documents\My Games\Terraria\Worlds`
- **Windows (Steam)**: `C:\Program Files (x86)\Steam\userdata\{YOUR_USER_ID}\105600\remote\worlds`
- **macOS**: `~/Library/Application Support/Terraria/Worlds`
- **Linux**: `~/.local/share/Terraria/Worlds`
- **Linux (Steam)**: `~/.local/share/Steam/userdata/{YOUR_USER_ID}\105600\remote/worlds`

## License

MIT License - see LICENSE file for details

## Links

- [Original TerraMap](https://terramap.github.io)
- [Terraria Wiki](https://terraria.fandom.com/wiki/Terraria_Wiki)

## Migration Status

This project is a modern rewrite of the original TerraMap, migrating from JavaScript + Bootstrap 3 to Rust + WebAssembly + Svelte 5. See `p.md` and `pp2.md` for detailed migration progress.
