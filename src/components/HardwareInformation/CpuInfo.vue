<template>
  <div class="hardware-card">
    <div class="flex items-start gap-1.5">
      <n-icon :size="14" class="text-blue-500 flex-shrink-0 mt-0.5">
        <component :is="CpuIcon" />
      </n-icon>
      <div class="flex-1 min-w-0">
        <div class="flex items-center gap-1 mb-0.5">
          <span class="text-xs text-gray-500 whitespace-nowrap">CPU</span>
          <n-progress
            type="line"
            :percentage="cpuUsage"
            :show-indicator="false"
            :height="3"
            class="flex-1 min-w-0"
          />
          <span class="text-xs font-medium whitespace-nowrap ml-1">{{ cpuUsage }}%</span>
        </div>
        <div class="text-xs text-gray-400 truncate leading-tight" :title="cpuName">{{ cpuName }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { HardwareChipOutline } from "@vicons/ionicons5";

const CpuIcon = HardwareChipOutline;
const cpuUsage = ref(0);
const cpuName = ref("Intel Core i7-12700K");

let intervalId: number | null = null;

const fetchCpuInfo = () => {
  cpuUsage.value = Math.floor(Math.random() * 30 + 20);
};

onMounted(() => {
  fetchCpuInfo();
  intervalId = window.setInterval(fetchCpuInfo, 2000);
});

onUnmounted(() => {
  if (intervalId) {
    clearInterval(intervalId);
  }
});
</script>

<style scoped>
.hardware-card {
  padding: 4px 6px;
  margin-bottom: 3px;
  border-radius: 3px;
  background: var(--n-card-color);
  width: 100%;
  box-sizing: border-box;
}
</style>

