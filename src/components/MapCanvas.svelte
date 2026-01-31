<script lang="ts">
  import { onMount } from 'svelte';
  import { worldStore, highlightStore, npcStore, viewStore } from '../lib/stores';

  let canvasElement: HTMLCanvasElement;
  let wasmModule: any = null;
  let renderer: any = null;
  let isDragging = false;
  let lastX = 0;
  let lastY = 0;

  async function initWasm() {
    try {
      wasmModule = await import('@terra-map-wasm/core/terra_map_wasm.js');
      await wasmModule.default();
    } catch (error) {
      console.error('Failed to load WASM module:', error);
    }
  }

  function initRenderer() {
    if (!wasmModule || !canvasElement) return;
    try {
      renderer = new wasmModule.Renderer(canvasElement);
    } catch (error) {
      console.error('Failed to create renderer:', error);
    }
  }

  function handleWheel(e: WheelEvent) {
    e.preventDefault();
    const delta = e.deltaY > 0 ? 0.9 : 1.1;
    const newScale = Math.max(0.1, Math.min(10, $viewStore.scale * delta));
    viewStore.setScale(newScale);
    if (renderer) {
      renderer.set_scale(newScale);
    }
    render();
  }

  function handleMouseDown(e: MouseEvent) {
    isDragging = true;
    lastX = e.clientX;
    lastY = e.clientY;
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isDragging) return;
    const dx = e.clientX - lastX;
    const dy = e.clientY - lastY;
    viewStore.setOffset($viewStore.offsetX + dx, $viewStore.offsetY + dy);
    lastX = e.clientX;
    lastY = e.clientY;
    render();
  }

  function handleMouseUp() {
    isDragging = false;
  }

  function render() {
    if (!renderer) return;

    const ctx = canvasElement.getContext('2d');
    if (!ctx) return;

    // 清空画布
    ctx.clearRect(0, 0, canvasElement.width, canvasElement.height);
    ctx.save();
    ctx.translate($viewStore.offsetX, $viewStore.offsetY);
    ctx.scale($viewStore.scale, $viewStore.scale);

    // 渲染世界
    if ($worldStore.world) {
      try {
        const worldJs = $worldStore.world;

        // 计算可见区域
        const canvasWidth = canvasElement.width / $viewStore.scale;
        const canvasHeight = canvasElement.height / $viewStore.scale;
        const startX = Math.floor(-$viewStore.offsetX / $viewStore.scale);
        const startY = Math.floor(-$viewStore.offsetY / $viewStore.scale);
        const visibleArea = [startX, startY, Math.ceil(canvasWidth), Math.ceil(canvasHeight)];

        // 如果有高亮位置，使用高亮渲染
        if ($highlightStore.foundPositions.length > 0) {
          const highlightPositions = $highlightStore.foundPositions.flatMap(p => [p.x, p.y]);
          renderer.render_world_visible_js(
            worldJs,
            visibleArea,
            highlightPositions,
            $highlightStore.highlightAll
          );
        } else {
          renderer.render_world_visible_js(
            worldJs,
            visibleArea,
            [],
            false
          );
        }

        // 渲染 NPC 高亮
        if ($npcStore.npcPosition) {
          const { x, y } = $npcStore.npcPosition;

          // 绘制 NPC 位置标记（红色圆圈）
          ctx.strokeStyle = '#ff0000';
          ctx.lineWidth = 2 / $viewStore.scale;
          ctx.beginPath();
          ctx.arc(x + 0.5, y + 0.5, 1.5, 0, Math.PI * 2);
          ctx.stroke();

          // 绘制 NPC 位置标记（填充圆圈）
          ctx.fillStyle = 'rgba(255, 0, 0, 0.5)';
          ctx.beginPath();
          ctx.arc(x + 0.5, y + 0.5, 1, 0, Math.PI * 2);
          ctx.fill();
        }
      } catch (error) {
        console.error('Failed to render world:', error);
      }
    } else {
      // 显示提示文字
      ctx.fillStyle = '#666666';
      ctx.font = '24px Arial';
      ctx.textAlign = 'center';
      ctx.fillText('Please select a Terraria world file (.wld)', canvasElement.width / 2, canvasElement.height / 2);
    }

    ctx.restore();
  }

  onMount(() => {
    initWasm().then(() => {
      initRenderer();
      render();
    });

    canvasElement.addEventListener('wheel', handleWheel, { passive: false });
    canvasElement.addEventListener('mousedown', handleMouseDown);
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);

    return () => {
      canvasElement.removeEventListener('wheel', handleWheel);
      canvasElement.removeEventListener('mousedown', handleMouseDown);
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    };
  });

  // 响应式更新
  $: if ($worldStore.world || $highlightStore.foundPositions.length > 0 || $npcStore.npcPosition) {
    render();
  }

  // 同步 renderer 的缩放状态
  $: if (renderer && $viewStore.scale !== renderer.get_scale()) {
    renderer.set_scale($viewStore.scale);
    render();
  }
</script>

<canvas bind:this={canvasElement} class="map-canvas"></canvas>

<style>
  .map-canvas {
    width: 100%;
    height: 100%;
    background-color: #000000;
    cursor: grab;
  }

  .map-canvas:active {
    cursor: grabbing;
  }
</style>