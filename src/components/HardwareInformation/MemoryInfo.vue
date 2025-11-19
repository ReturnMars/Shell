<template>
  <div class="hardware-card">
    <div class="flex items-start gap-1.5">
      <n-icon :size="14" class="text-green-500 flex-shrink-0 mt-0.5">
        <component :is="MemoryIcon" />
      </n-icon>
      <div class="flex-1 min-w-0">
        <div class="flex items-center gap-1 mb-0.5">
          <span class="text-xs text-gray-500 whitespace-nowrap">内存</span>
          <n-progress
            type="line"
            :percentage="memoryUsage"
            :show-indicator="false"
            :height="3"
            class="flex-1 min-w-0"
          />
          <span class="text-xs font-medium whitespace-nowrap ml-1">{{ memoryUsage }}%</span>
        </div>
        <div class="text-xs text-gray-400 truncate leading-tight">{{ formatBytes(used) }} / {{ formatBytes(total) }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { ServerOutline } from "@vicons/ionicons5";
import { invoke } from "@tauri-apps/api/core";
import { formatBytes } from "../../utils/formatData";

const MemoryIcon = ServerOutline;
const memoryUsage = ref(0);
const used = ref(0);
const total = ref(0);

let intervalId: number | null = null;

const fetchMemoryInfo = async () => {
  try {
    const info = await invoke<{ usage: number; used: number; total: number }>("get_memory_info");
    memoryUsage.value = Math.round(info.usage);
    used.value = info.used;
    total.value = info.total;
  } catch (error) {
    const mockTotal = 16 * 1024 * 1024 * 1024;
    const mockUsed = mockTotal * (Math.random() * 0.3 + 0.4);
    memoryUsage.value = Math.round((mockUsed / mockTotal) * 100);
    used.value = mockUsed;
    total.value = mockTotal;
  }
};

onMounted(() => {
  fetchMemoryInfo();
  intervalId = window.setInterval(fetchMemoryInfo, 2000);
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

