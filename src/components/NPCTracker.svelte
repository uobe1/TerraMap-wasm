<script lang="ts">
  import { worldStore, npcStore } from '../lib/stores';
  import { npcData, type NPCData } from '../lib/npcData';

  let selectedNpc: NPCData | null = null;
  let searchQuery = '';

  // 过滤后的 NPC 列表
  $: filteredNpcs = npcData.filter(npc =>
    !searchQuery || npc.name.toLowerCase().includes(searchQuery.toLowerCase())
  );

  // 获取世界中的 NPC 列表
  $: worldNpcs = $worldStore.world?.npcs || [];

  // 选择 NPC
  function selectNpc(npc: NPCData) {
    selectedNpc = npc;

    // 在世界中查找该 NPC
    const foundNpc = worldNpcs.find(n => n.id === npc.id);
    if (foundNpc) {
      npcStore.setSelectedNpc(npc.id, npc.name, {
        x: Math.floor(foundNpc.position_x),
        y: Math.floor(foundNpc.position_y),
      });
    } else {
      // NPC 不在世界中
      npcStore.setSelectedNpc(npc.id, npc.name, null);
    }
  }

  // 清除选择
  function clearSelection() {
    selectedNpc = null;
    searchQuery = '';
    npcStore.clear();
  }

  // 检查 NPC 是否在世界中
  function isNpcInWorld(npcId: number): boolean {
    return worldNpcs.some(n => n.id === npcId);
  }
</script>

<div class="npc-tracker">
  <h3>NPCs</h3>

  <!-- 搜索框 -->
  <input
    type="text"
    class="form-control"
    placeholder="Search NPCs..."
    bind:value={searchQuery}
    disabled={!$worldStore.world}
  />

  <!-- NPC 列表 -->
  <div class="npc-list">
    {#each filteredNpcs as npc}
      <button
        class="npc-item"
        class:selected={selectedNpc?.id === npc.id}
        class:in-world={isNpcInWorld(npc.id)}
        on:click={() => selectNpc(npc)}
        disabled={!$worldStore.world}
      >
        <span class="npc-status">
          {#if isNpcInWorld(npc.id)}
            <span class="status-dot status-active"></span>
          {:else}
            <span class="status-dot status-inactive"></span>
          {/if}
        </span>
        {npc.name}
      </button>
    {/each}
  </div>

  <!-- 清除按钮 -->
  <button
    class="btn btn-default"
    on:click={clearSelection}
    disabled={!selectedNpc}
  >
    Clear Selection
  </button>

  <!-- NPC 信息 -->
  {#if $npcStore.selectedNpcName}
    <div class="npc-info">
      <strong>Selected:</strong> {$npcStore.selectedNpcName}
      {#if $npcStore.npcPosition}
        <div>Position: ({$npcStore.npcPosition.x}, {$npcStore.npcPosition.y})</div>
      {:else}
        <div class="npc-not-found">Not in world</div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .npc-tracker {
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

  .npc-list {
    flex: 1;
    overflow-y: auto;
    border: 1px solid #cccccc;
    border-radius: 4px;
    background-color: #ffffff;
    min-height: 200px;
  }

  .npc-item {
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
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .npc-item:hover:not(:disabled) {
    background-color: #f5f5f5;
  }

  .npc-item.selected {
    background-color: #337ab7;
    color: #ffffff;
  }

  .npc-item.selected:hover {
    background-color: #286090;
  }

  .npc-item.in-world {
    font-weight: bold;
  }

  .npc-item:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }

  .npc-status {
    display: flex;
    align-items: center;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    display: inline-block;
  }

  .status-active {
    background-color: #5cb85c;
  }

  .status-inactive {
    background-color: #d9534f;
  }

  .npc-item.selected .status-active,
  .npc-item.selected .status-inactive {
    border: 1px solid rgba(255, 255, 255, 0.5);
  }

  .btn {
    width: 100%;
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

  .npc-info {
    padding: 8px;
    background-color: #ffffff;
    border: 1px solid #cccccc;
    border-radius: 4px;
    font-size: 13px;
  }

  .npc-not-found {
    color: #d9534f;
    font-style: italic;
  }
</style>