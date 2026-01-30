<script lang="ts">
  import { worldStore } from '../lib/stores';
</script>

<div class="info-panel">
  <h3>World Info</h3>
  {#if $worldStore.world}
    <div class="info-content">
      <div class="info-row">
        <span class="info-label">Name:</span>
        <span class="info-value">{$worldStore.world.name}</span>
      </div>
      <div class="info-row">
        <span class="info-label">Width:</span>
        <span class="info-value">{$worldStore.world.width}</span>
      </div>
      <div class="info-row">
        <span class="info-label">Height:</span>
        <span class="info-value">{$worldStore.world.height}</span>
      </div>
    </div>
  {:else}
    <p class="info-placeholder">No world loaded</p>
  {/if}
</div>

<style>
  .info-panel {
    padding: 15px;
    background-color: #f9f9f9;
    border-left: 1px solid #cccccc;
    overflow-y: auto;
  }

  h3 {
    margin: 0 0 15px 0;
    font-size: 16px;
    color: #333333;
    border-bottom: 1px solid #cccccc;
    padding-bottom: 10px;
  }

  .info-content {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .info-row {
    display: flex;
    justify-content: space-between;
    padding: 5px 0;
  }

  .info-label {
    font-weight: bold;
    color: #555555;
  }

  .info-value {
    color: #333333;
  }

  .info-placeholder {
    color: #999999;
    font-style: italic;
    text-align: center;
    margin: 20px 0;
  }
</style>