<template>
  <div class="hardware-info-container">
    <CpuInfo />
    <MemoryInfo />
    <DiskInfo />
    <NetworkInfo />
    <GpuInfo />
  </div>
</template>

<script setup lang="ts">
import CpuInfo from "./CpuInfo.vue";
import MemoryInfo from "./MemoryInfo.vue";
import DiskInfo from "./DiskInfo.vue";
import NetworkInfo from "./NetworkInfo.vue";
import GpuInfo from "./GpuInfo.vue";
</script>

<style scoped>
.hardware-info-container {
  width: 100%;
  padding: 4px;
  box-sizing: border-box;
  overflow-x: hidden;
}
</style>
