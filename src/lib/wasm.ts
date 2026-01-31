// WASM 模块封装
// 支持懒加载和缓存

let wasmModule: any = null;
let wasmInitPromise: Promise<void> | null = null;
let isLoading = false;
let loadError: Error | null = null;

// WASM 加载状态
export type WasmLoadStatus = 'uninitialized' | 'loading' | 'loaded' | 'error';

// 获取当前 WASM 加载状态
export function getWasmStatus(): WasmLoadStatus {
  if (loadError) return 'error';
  if (wasmModule) return 'loaded';
  if (isLoading) return 'loading';
  return 'uninitialized';
}

// 获取加载错误（如果有）
export function getWasmError(): Error | null {
  return loadError;
}

// 初始化 WASM 模块（带缓存，避免重复加载）
export async function initWasm(): Promise<void> {
  // 如果已经加载，直接返回
  if (wasmModule) {
    return;
  }

  // 如果正在加载，等待加载完成
  if (wasmInitPromise) {
    return wasmInitPromise;
  }

  // 开始加载
  isLoading = true;
  wasmInitPromise = (async () => {
    try {
      // 动态导入 WASM 模块（懒加载）
      wasmModule = await import('@terra-map-wasm/core/terra_map_wasm.js');
      
      // 初始化 WASM（如果模块有 default 函数）
      if (typeof wasmModule.default === 'function') {
        await wasmModule.default();
      }

      isLoading = false;
      loadError = null;
    } catch (error) {
      isLoading = false;
      loadError = error instanceof Error ? error : new Error(String(error));
      throw loadError;
    }
  })();

  return wasmInitPromise;
}

// 获取 WASM 模块（确保已初始化）
export function getWasmModule() {
  if (!wasmModule) {
    throw new Error('WASM module not initialized. Call initWasm() first.');
  }
  return wasmModule;
}

// 预加载 WASM 模块（可选，用于提前加载）
export async function preloadWasm(): Promise<void> {
  return initWasm();
}

// 重置 WASM 模块（主要用于测试）
export function resetWasmModule() {
  wasmModule = null;
  wasmInitPromise = null;
  isLoading = false;
  loadError = null;
}