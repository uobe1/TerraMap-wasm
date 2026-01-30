// WASM 模块类型声明
declare module '/pkg/terra_map_wasm.js' {
  export function set_once(): void;
  export function default(): Promise<void>;

  export class WorldLoader {
    constructor();
    load_from_data(data: Uint8Array): any;
  }

  export class Renderer {
    constructor(canvas: HTMLCanvasElement);
    set_scale(scale: number): void;
    get_scale(): number;
    render_world_js(world: any): void;
    render_world_with_highlight_js(world: any, highlightPositions: number[], highlightAll: boolean): void;
    render_world_visible_js(world: any, visibleArea: number[], highlightPositions: number[], highlightAll: boolean): void;
  }
}