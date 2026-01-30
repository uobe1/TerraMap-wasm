<script lang="ts">
  import { worldStore } from '../lib/stores';

  let fileInput: HTMLInputElement;

  async function handleFileSelect(event: Event) {
    const target = event.target as HTMLInputElement;
    const file = target.files?.[0];
    if (!file) return;

    try {
      worldStore.setLoading(true);
      worldStore.setError('');

      const wasm = await import('@terra-map-wasm/core/terra_map_wasm.js');
      const arrayBuffer = await file.arrayBuffer();
      const data = new Uint8Array(arrayBuffer);

      // 使用 WASM 模块加载世界
      const worldLoader = new wasm.WorldLoader();
      const world = worldLoader.load_from_data(data);

      worldStore.setWorld(world);
    } catch (error) {
      console.error('Failed to load world:', error);
      worldStore.setError('Failed to load world file: ' + error);
      alert('Failed to load world file: ' + error);
    }
  }

  function triggerFileInput() {
    fileInput.click();
  }

  function saveMapImage() {
    // 获取 MapCanvas 组件中的 canvas 元素
    const canvas = document.querySelector('.map-canvas') as HTMLCanvasElement;
    if (!canvas) {
      alert('No map to save');
      return;
    }

    // 创建临时 canvas 以渲染完整世界
    const tempCanvas = document.createElement('canvas');
    const ctx = tempCanvas.getContext('2d');
    if (!ctx) {
      alert('Failed to create canvas context');
      return;
    }

    // 设置临时 canvas 大小为完整世界大小
    if ($worldStore.world) {
      const width = $worldStore.world.width;
      const height = $worldStore.world.height;
      tempCanvas.width = width;
      tempCanvas.height = height;

      // 绘制当前 canvas 的内容到临时 canvas
      // 注意：这需要获取当前渲染器并重新渲染完整世界
      // 由于缩放和平移状态，这里需要特殊处理

      // 简化方案：直接保存当前可见区域
      tempCanvas.width = canvas.width;
      tempCanvas.height = canvas.height;
      ctx.drawImage(canvas, 0, 0);

      // 导出为图片
      const worldName = $worldStore.world.name.replace(/[^a-zA-Z0-9]/g, '_');
      const dataUrl = tempCanvas.toDataURL('image/png');

      // 创建下载链接
      const link = document.createElement('a');
      link.download = `${worldName}_map.png`;
      link.href = dataUrl;
      link.click();
    }
  }
</script>

<div class="toolbar">
  <div class="toolbar-section">
    <h1>TerraMap</h1>
  </div>
  <div class="toolbar-section">
    <button
      class="btn btn-default"
      on:click={saveMapImage}
      disabled={!$worldStore.world}
    >
      Save Map Image
    </button>
    <button class="btn btn-primary" on:click={triggerFileInput} disabled={$worldStore.loading}>
      {$worldStore.loading ? 'Loading...' : 'Choose File'}
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

  .btn-primary:hover:not(:disabled) {
    background-color: #286090;
    border-color: #204d74;
  }

  .btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-default {
    color: #333333;
    background-color: #ffffff;
    border-color: #cccccc;
  }

  .btn-default:hover:not(:disabled) {
    color: #333333;
    background-color: #e6e6e6;
    border-color: #adadad;
  }

  .btn-default:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
</style>