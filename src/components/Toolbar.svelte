<script lang="ts">
  import { worldStore } from '../lib/stores';

  let fileInput: HTMLInputElement;

  async function handleFileSelect(event: Event) {
    const target = event.target as HTMLInputElement;
    const file = target.files?.[0];
    if (!file) return;

    try {
      const wasm = await import('/pkg/terra_map_wasm.js');
      const arrayBuffer = await file.arrayBuffer();
      const data = new Uint8Array(arrayBuffer);

      // 使用 WASM 模块加载世界
      const worldLoader = new wasm.WorldLoader();
      const worldJson = worldLoader.load_from_data(data);
      const world = JSON.parse(worldJson);

      worldStore.setWorld(world);
    } catch (error) {
      console.error('Failed to load world:', error);
      alert('Failed to load world file: ' + error);
    }
  }

  function triggerFileInput() {
    fileInput.click();
  }
</script>

<div class="toolbar">
  <div class="toolbar-section">
    <h1>TerraMap</h1>
  </div>
  <div class="toolbar-section">
    <button class="btn btn-primary" on:click={triggerFileInput}>
      Choose File
    </button>
    <input type="file" bind:this={fileInput} accept=".wld" hidden on:change={handleFileSelect} />
  </div>
</div>

<style>
  .toolbar {
    background-color: #f5f5f5;
    border-bottom: 1px solid #cccccc;
    padding: 10px 20px;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .toolbar-section {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  h1 {
    margin: 0;
    font-size: 24px;
    color: #333333;
  }

  .btn {
    padding: 8px 12px;
    border: 1px solid transparent;
    border-radius: 4px;
    font-size: 14px;
    cursor: pointer;
    transition: all 0.15s ease-in-out;
  }

  .btn-primary {
    background-color: var(--bs-primary);
    color: #ffffff;
    border-color: #2e6da4;
  }

  .btn-primary:hover {
    background-color: #286090;
    border-color: #204d74;
  }
</style>