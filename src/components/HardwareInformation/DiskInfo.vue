<template>
  <div class="hardware-card">
    <div class="flex items-start gap-1.5">
      <n-icon :size="14" class="text-orange-500 flex-shrink-0 mt-0.5">
        <component :is="DiskIcon" />
      </n-icon>
      <div class="flex-1 min-w-0">
        <div class="flex items-center gap-1 mb-0.5">
          <span class="text-xs text-gray-500 whitespace-nowrap">硬盘</span>
          <n-progress
            type="line"
            :percentage="diskUsage"
            :show-indicator="false"
            :height="3"
            class="flex-1 min-w-0"
          />
          <span class="text-xs font-medium whitespace-nowrap ml-1">{{ diskUsage }}%</span>
        </div>
        <div class="text-xs text-gray-400 truncate leading-tight">{{ formatBytes(used) }} / {{ formatBytes(total) }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { ArchiveOutline } from "@vicons/ionicons5";
import { formatBytes } from "../../utils/formatData";

const DiskIcon = ArchiveOutline;
const diskUsage = ref(0);
const used = ref(0);
const total = ref(500 * 1024 * 1024 * 1024);

let intervalId: number | null = null;

const fetchDiskInfo = () => {
  const mockUsed = total.value * (Math.random() * 0.2 + 0.5);
  diskUsage.value = Math.round((mockUsed / total.value) * 100);
  used.value = mockUsed;
};

onMounted(() => {
  fetchDiskInfo();
  intervalId = window.setInterval(fetchDiskInfo, 3000);
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

