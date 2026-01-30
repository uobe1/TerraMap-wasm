<script lang="ts">
  import { onMount } from 'svelte';
  import MapCanvas from './components/MapCanvas.svelte';
  import Toolbar from './components/Toolbar.svelte';
  import BlockSelector from './components/BlockSelector.svelte';
  import NPCTracker from './components/NPCTracker.svelte';
  import InfoPanel from './components/InfoPanel.svelte';
  import { worldStore } from './lib/stores';

  onMount(async () => {
    // 初始化 WASM 模块
    try {
      const wasm = await import('/pkg/terra_map_wasm.js');
      console.log('WASM module loaded:', wasm);
      // 初始化 panic hook
      if (wasm.set_once) {
        wasm.set_once();
      }
    } catch (error) {
      console.error('Failed to load WASM module:', error);
    }
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