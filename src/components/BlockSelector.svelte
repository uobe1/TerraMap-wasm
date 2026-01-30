<script lang="ts">
  import { worldStore, highlightStore } from '../lib/stores';
  import {
    tiles,
    tilesByCategory,
    type TileData,
  } from '../lib/tileData';

  let selectedCategory = '';
  let selectedTile: TileData | null = null;
  let searchQuery = '';

  // 过滤后的方块列表
  $: filteredTiles = tiles.filter(tile => {
    const matchesCategory = !selectedCategory || tile.category === selectedCategory;
    const matchesSearch = !searchQuery ||
      tile.name.toLowerCase().includes(searchQuery.toLowerCase());
    return matchesCategory && matchesSearch;
  });

  // 获取分类列表
  const categories = Object.keys(tilesByCategory);

  // 选择方块
  function selectTile(tile: TileData) {
    selectedTile = tile;
    highlightStore.setSelectedTile(tile.id, tile.name);
    highlightStore.setHighlightAll(false);

    // 查找所有匹配的方块位置
    if ($worldStore.world) {
      const positions: Array<{ x: number; y: number }> = [];
      for (let y = 0; y < $worldStore.world.height; y++) {
        for (let x = 0; x < $worldStore.world.width; x++) {
          const idx = y * $worldStore.world.width + x;
          if (idx < $worldStore.world.tiles.length) {
            const tileData = $worldStore.world.tiles[idx];
            if (tileData.tile_id === tile.id && tileData.is_active) {
              positions.push({ x, y });
            }
          }
        }
      }
      highlightStore.setFoundPositions(positions);
    }
  }

  // 高亮所有
  function highlightAll() {
    if (selectedTile) {
      highlightStore.setHighlightAll(true);
      selectTile(selectedTile);
    }
  }

  // 清除高亮
  function clearHighlight() {
    selectedTile = null;
    selectedCategory = '';
    searchQuery = '';
    highlightStore.clear();
  }

  // 获取当前选中的方块数量
  $: foundCount = $highlightStore.foundPositions.length;
</script>

<div class="block-selector">
  <h3>Blocks...</h3>

  <!-- 搜索框 -->
  <input
    type="text"
    class="form-control"
    placeholder="Search blocks..."
    bind:value={searchQuery}
    disabled={!$worldStore.world}
  />

  <!-- 分类选择 -->
  <select
    class="form-control"
    bind:value={selectedCategory}
    disabled={!$worldStore.world}
  >
    <option value="">All Categories</option>
    {#each categories as category}
      <option value={category}>{category}</option>
    {/each}
  </select>

  <!-- 方块列表 -->
  <div class="tile-list">
    {#each filteredTiles as tile}
      <button
        class="tile-item"
        class:selected={selectedTile?.id === tile.id}
        on:click={() => selectTile(tile)}
        disabled={!$worldStore.world}
      >
        {tile.name}
      </button>
    {/each}
  </div>

  <!-- 操作按钮 -->
  <div class="action-buttons">
    <button
      class="btn btn-primary"
      on:click={highlightAll}
      disabled={!selectedTile}
    >
      Highlight All ({foundCount})
    </button>
    <button
      class="btn btn-default"
      on:click={clearHighlight}
      disabled={!selectedTile && !$highlightStore.highlightAll}
    >
      Clear Highlight
    </button>
  </div>
</div>

<style>
  .block-selector {
    padding: 15px;
    background-color: #f9f9f9;
    border-right: 1px solid #cccccc;
    display: flex;
    flex-direction: column;
    gap: 10px;
    max-height: 100%;
    overflow: hidden;
  }

  h3 {
    margin: 0 0 5px 0;
    font-size: 16px;
    color: #333333;
  }

  .form-control {
    width: 100%;
    padding: 6px 12px;
    font-size: 14px;
    line-height: 1.42857143;
    color: #555555;
    background-color: #ffffff;
    background-image: none;
    border: 1px solid #cccccc;
    border-radius: 4px;
    transition: border-color ease-in-out 0.15s, box-shadow ease-in-out 0.15s;
    box-sizing: border-box;
  }

  .form-control:focus {
    border-color: #66afe9;
    outline: 0;
    box-shadow: inset 0 1px 1px rgba(0, 0, 0, 0.075), 0 0 8px rgba(102, 175, 233, 0.6);
  }

  .form-control:disabled {
    background-color: #eeeeee;
    cursor: not-allowed;
  }

  .tile-list {
    flex: 1;
    overflow-y: auto;
    border: 1px solid #cccccc;
    border-radius: 4px;
    background-color: #ffffff;
    min-height: 200px;
  }

  .tile-item {
    width: 100%;
    padding: 6px 12px;
    font-size: 13px;
    line-height: 1.42857143;
    color: #333333;
    background-color: #ffffff;
    border: none;
    border-bottom: 1px solid #eeeeee;
    text-align: left;
    cursor: pointer;
    transition: background-color 0.15s;
    box-sizing: border-box;
  }

  .tile-item:hover:not(:disabled) {
    background-color: #f5f5f5;
  }

  .tile-item.selected {
    background-color: #337ab7;
    color: #ffffff;
  }

  .tile-item.selected:hover {
    background-color: #286090;
  }

  .tile-item:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }

  .action-buttons {
    display: flex;
    gap: 10px;
    flex-shrink: 0;
  }

  .btn {
    flex: 1;
    padding: 6px 12px;
    font-size: 14px;
    font-weight: normal;
    line-height: 1.42857143;
    text-align: center;
    white-space: nowrap;
    vertical-align: middle;
    touch-action: manipulation;
    cursor: pointer;
    user-select: none;
    background-image: none;
    border: 1px solid transparent;
    border-radius: 4px;
    transition: all 0.15s ease-in-out;
    box-sizing: border-box;
  }

  .btn-primary {
    color: #ffffff;
    background-color: #337ab7;
    border-color: #2e6da4;
  }

  .btn-primary:hover:not(:disabled) {
    color: #ffffff;
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