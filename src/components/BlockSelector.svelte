<script lang="ts">
  // 方块选择器组件
  // 将在后续阶段实现完整功能
</script>

<div class="block-selector">
  <h3>Blocks...</h3>
  <select class="form-control" disabled>
    <option>Coming soon...</option>
  </select>
</div>

<style>
  .block-selector {
    padding: 15px;
    background-color: #f9f9f9;
    border-right: 1px solid #cccccc;
  }

  h3 {
    margin: 0 0 10px 0;
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
  }

  .form-control:focus {
    border-color: #66afe9;
    outline: 0;
    box-shadow: inset 0 1px 1px rgba(0, 0, 0, 0.075), 0 0 8px rgba(102, 175, 233, 0.6);
  }
</style>