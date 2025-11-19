<template>
  <div class="hardware-card">
    <div class="flex items-start gap-1.5">
      <n-icon :size="14" class="text-purple-500 flex-shrink-0 mt-0.5">
        <component :is="NetworkIcon" />
      </n-icon>
      <div class="flex-1 min-w-0">
        <div class="text-xs text-gray-500 mb-0.5">网络</div>
        <div class="flex items-center gap-2 text-xs leading-tight">
          <span class="text-gray-400 whitespace-nowrap">↑</span>
          <span class="font-medium truncate">{{ formatSpeed(uploadSpeed) }}</span>
          <span class="text-gray-400 whitespace-nowrap ml-1">↓</span>
          <span class="font-medium truncate">{{ formatSpeed(downloadSpeed) }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { CloudUploadOutline } from "@vicons/ionicons5";
import { formatSpeed } from "../../utils/formatData";

const NetworkIcon = CloudUploadOutline;
const uploadSpeed = ref(0);
const downloadSpeed = ref(0);

let intervalId: number | null = null;

const fetchNetworkInfo = () => {
  uploadSpeed.value = Math.random() * 1024 * 1024;
  downloadSpeed.value = Math.random() * 5 * 1024 * 1024;
};

onMounted(() => {
  fetchNetworkInfo();
  intervalId = window.setInterval(fetchNetworkInfo, 1000);
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

