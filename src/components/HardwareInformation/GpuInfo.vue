<template>
  <div v-if="hasGpu" class="hardware-card">
    <div class="flex items-start gap-1.5">
      <n-icon :size="14" class="text-red-500 flex-shrink-0 mt-0.5">
        <component :is="GpuIcon" />
      </n-icon>
      <div class="flex-1 min-w-0">
        <div class="flex items-center gap-1 mb-0.5">
          <span class="text-xs text-gray-500 whitespace-nowrap">GPU</span>
          <n-progress
            type="line"
            :percentage="gpuUsage"
            :show-indicator="false"
            :height="3"
            class="flex-1 min-w-0"
          />
          <span class="text-xs font-medium whitespace-nowrap ml-1">{{ gpuUsage }}%</span>
        </div>
        <div class="text-xs text-gray-400 truncate leading-tight" :title="gpuName">{{ gpuName }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { DesktopOutline } from "@vicons/ionicons5";

const GpuIcon = DesktopOutline;
const gpuUsage = ref(0);
const gpuName = ref("NVIDIA GeForce RTX 3070");
const hasGpu = ref(true);

let intervalId: number | null = null;

const fetchGpuInfo = () => {
  gpuUsage.value = Math.floor(Math.random() * 40 + 10);
};

onMounted(() => {
  fetchGpuInfo();
  intervalId = window.setInterval(fetchGpuInfo, 2000);
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

