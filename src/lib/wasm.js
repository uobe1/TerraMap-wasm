// WASM 模块封装
let wasmModule = null;
export async function initWasm() {
    if (wasmModule)
        return;
    try {
        wasmModule = await import('@terra-map-wasm/core/terra_map_wasm.js');
        await wasmModule.default();
        console.log('WASM module initialized:', wasmModule);
    }
    catch (error) {
        console.error('Failed to initialize WASM module:', error);
        throw error;
    }
}
export function getWasmModule() {
    if (!wasmModule) {
        throw new Error('WASM module not initialized. Call initWasm() first.');
    }
    return wasmModule;
}
//# sourceMappingURL=wasm.js.map