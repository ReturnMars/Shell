<template>
  <div class="hardware-test">
    <h2>硬件监控测试页面</h2>
    <div class="test-controls">
      <n-button @click="testFetchData" :loading="loading">
        获取测试数据
      </n-button>
      <n-button @click="clearData">
        清除数据
      </n-button>
      <n-button @click="toggleAutoRefresh">
        {{ autoRefresh ? '停止自动刷新' : '开始自动刷新' }}
      </n-button>
    </div>
    
    <div class="test-content">
      <HardwareMonitor />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useHardwareStore } from '@/stores/hardware';

const hardwareStore = useHardwareStore();

const loading = ref(false);

const testFetchData = async () => {
  loading.value = true;
  try {
    await hardwareStore.fetchHardwareInfo('test-connection');
  } finally {
    loading.value = false;
  }
};

const clearData = () => {
  hardwareStore.clearHardwareInfo();
};

const toggleAutoRefresh = () => {
  hardwareStore.setAutoRefresh(!hardwareStore.autoRefresh);
};

const autoRefresh = ref(hardwareStore.autoRefresh);
</script>

<style scoped>
.hardware-test {
  padding: 20px;
  max-width: 400px;
  margin: 0 auto;
}

.test-controls {
  display: flex;
  gap: 8px;
  margin-bottom: 20px;
  flex-wrap: wrap;
}

.test-content {
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  padding: 16px;
  background: white;
}
</style>
