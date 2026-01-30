<script lang="ts">
  import { onMount } from 'svelte';
  import { worldStore } from '../lib/stores';

  let canvasElement: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;
  let scale = 1;
  let offsetX = 0;
  let offsetY = 0;
  let isDragging = false;
  let lastX = 0;
  let lastY = 0;

  function handleWheel(e: WheelEvent) {
    e.preventDefault();
    const delta = e.deltaY > 0 ? 0.9 : 1.1;
    const newScale = Math.max(0.1, Math.min(10, scale * delta));
    scale = newScale;
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
    offsetX += dx;
    offsetY += dy;
    lastX = e.clientX;
    lastY = e.clientY;
    render();
  }

  function handleMouseUp() {
    isDragging = false;
  }

  function render() {
    if (!ctx) return;
    ctx.clearRect(0, 0, canvasElement.width, canvasElement.height);
    ctx.save();
    ctx.translate(offsetX, offsetY);
    ctx.scale(scale, scale);

    // 这里将来会从 WASM 模块调用渲染逻辑
    if ($worldStore.world) {
      // 临时渲染占位
      ctx.fillStyle = '#ffffff';
      ctx.fillRect(0, 0, $worldStore.world.width, $worldStore.world.height);
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
    ctx = canvasElement.getContext('2d')!;
    if (ctx) {
      ctx.imageSmoothingEnabled = false;
    }

    canvasElement.addEventListener('wheel', handleWheel, { passive: false });
    canvasElement.addEventListener('mousedown', handleMouseDown);
    window.addEventListener('mousemove', handleMouseMove);
    window.addEventListener('mouseup', handleMouseUp);

    render();

    return () => {
      canvasElement.removeEventListener('wheel', handleWheel);
      canvasElement.removeEventListener('mousedown', handleMouseDown);
      window.removeEventListener('mousemove', handleMouseMove);
      window.removeEventListener('mouseup', handleMouseUp);
    };
  });

  // 响应式更新
  $: render();
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