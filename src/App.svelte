<script lang="ts">
  import { onMount } from 'svelte';
  import MapCanvas from './components/MapCanvas.svelte';
  import Toolbar from './components/Toolbar.svelte';
  import BlockSelector from './components/BlockSelector.svelte';
  import NPCTracker from './components/NPCTracker.svelte';
  import InfoPanel from './components/InfoPanel.svelte';
  import { highlightStore, npcStore, viewStore } from './lib/stores';

  onMount(() => {
    // 初始化 WASM 模块
    (async () => {
      try {
        const wasm = await import('@terra-map-wasm/core/terra_map_wasm.js');
        console.log('WASM module loaded:', wasm);
      } catch (error) {
        console.error('Failed to load WASM module:', error);
      }
    })();

    // 添加快捷键支持
    window.addEventListener('keydown', handleKeyDown);

    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  });

  function handleKeyDown(e: KeyboardEvent) {
    // Ctrl/Cmd + O: 打开文件选择
    if ((e.ctrlKey || e.metaKey) && e.key === 'o') {
      e.preventDefault();
      const fileInput = document.querySelector('input[type="file"]') as HTMLInputElement;
      if (fileInput) {
        fileInput.click();
      }
    }

    // Ctrl/Cmd + S: 保存地图图片
    if ((e.ctrlKey || e.metaKey) && e.key === 's') {
      e.preventDefault();
      const saveButton = document.querySelector('button:has-text("Save Map Image")') as HTMLButtonElement;
      if (saveButton && !saveButton.disabled) {
        saveButton.click();
      }
    }

    // Ctrl/Cmd + F: 聚焦搜索框
    if ((e.ctrlKey || e.metaKey) && e.key === 'f') {
      e.preventDefault();
      const searchInput = document.querySelector('input[type="text"]') as HTMLInputElement;
      if (searchInput && !searchInput.disabled) {
        searchInput.focus();
      }
    }

    // Escape: 清除高亮和选择
    if (e.key === 'Escape') {
      highlightStore.clear();
      npcStore.clear();
    }

    // +/- 或 Ctrl/Cmd + +/-: 缩放
    if (e.key === '+' || e.key === '=' || (e.ctrlKey && e.key === '=')) {
      e.preventDefault();
      const canvas = document.querySelector('.map-canvas') as HTMLCanvasElement;
      if (canvas) {
        const event = new WheelEvent('wheel', {
          deltaY: -10,
          bubbles: true,
          cancelable: true,
        });
        canvas.dispatchEvent(event);
      }
    }

    if (e.key === '-' || e.key === '_' || (e.ctrlKey && e.key === '-')) {
      e.preventDefault();
      const canvas = document.querySelector('.map-canvas') as HTMLCanvasElement;
      if (canvas) {
        const event = new WheelEvent('wheel', {
          deltaY: 10,
          bubbles: true,
          cancelable: true,
        });
        canvas.dispatchEvent(event);
      }
    }

    // 数字键 0: 重置缩放
    if (e.key === '0') {
      e.preventDefault();
      viewStore.resetZoom();
      // 更新 renderer 的缩放
      const canvas = document.querySelector('.map-canvas') as HTMLCanvasElement;
      if (canvas) {
        // 触发重新渲染
        const event = new Event('resize');
        window.dispatchEvent(event);
      }
    }
  }
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

<style>
  .app-container {
    display: grid;
    grid-template-columns: 250px 1fr;
    grid-template-rows: auto 1fr;
    height: 100vh;
    width: 100%;
  }

  .main-content {
    display: grid;
    grid-template-columns: 1fr 300px;
    grid-template-rows: 1fr;
    overflow: hidden;
  }

  @media (max-width: 768px) {
    .app-container {
      grid-template-columns: 1fr;
      grid-template-rows: auto 1fr auto;
    }

    .main-content {
      grid-template-columns: 1fr;
      grid-template-rows: 1fr auto;
    }
  }
</style>