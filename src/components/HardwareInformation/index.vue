<template>
  <div class="hardware-info-container">
    <div class="hardware-info-title">
      <n-text>系统信息</n-text>
    </div>
    <div class="hardware-info-content">
      <CpuInfo />
      <MemoryInfo />
      <DiskInfo />
      <NetworkInfo />
      <GpuInfo />
    </div>
  </div>
</template>

<script setup lang="ts">
import CpuInfo from "./CpuInfo.vue";
import MemoryInfo from "./MemoryInfo.vue";
import DiskInfo from "./DiskInfo.vue";
import NetworkInfo from "./NetworkInfo.vue";
import GpuInfo from "./GpuInfo.vue";
</script>

<style scoped lang="scss">
.hardware-info-container {
  width: 100%;
  box-sizing: border-box;
  overflow-x: hidden;
  .hardware-info-title {
    border-bottom: 1px dashed var(--n-border-color);
    padding: 8px 8px;
    margin-bottom: 4px;
  }
}
</style>
