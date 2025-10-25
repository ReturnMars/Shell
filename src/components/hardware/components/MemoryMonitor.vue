<template>
  <n-card size="small" class="memory-monitor" :content-style="{ padding: '8px' }" :header-style="{ padding: '8px 8px 4px 8px' }">
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <n-icon size="16" color="#52c41a">
            <DatabaseOutlined />
          </n-icon>
          <span class="text-sm font-medium">内存</span>
        </div>
        <n-tag size="small" :type="getMemoryStatusType(memoryInfo?.usage || 0)">
          {{ memoryInfo?.usage || 0 }}%
        </n-tag>
      </div>
    </template>

    <div class="memory-content">
      <!-- 内存使用率环形图 -->
      <div class="memory-chart">
        <RingChart
          :data="{ name: '内存', value: memoryInfo?.usage || 0 }"
          :width="80"
          :height="80"
          :size="70"
          :stroke-width="6"
          label="使用率"
        />
      </div>

      <!-- 内存详细信息 -->
      <div class="memory-details">
        <div class="memory-info">
          <div class="detail-item">
            <span class="detail-label">总容量:</span>
            <span class="detail-value">{{ formatBytes(memoryInfo?.total || 0) }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">已使用:</span>
            <span class="detail-value text-orange-500">{{ formatBytes(memoryInfo?.used || 0) }}</span>
          </div>
          <div class="detail-item">
            <span class="detail-label">空闲:</span>
            <span class="detail-value text-green-500">{{ formatBytes(memoryInfo?.free || 0) }}</span>
          </div>
        </div>

        <!-- 交换分区信息 -->
        <div class="swap-info" v-if="memoryInfo?.swap">
          <div class="swap-header">
            <span class="swap-title">交换分区</span>
            <span class="swap-usage">{{ memoryInfo.swap.usage }}%</span>
          </div>
          <ProgressChart
            :data="{
              value: memoryInfo.swap.used,
              max: memoryInfo.swap.total,
              label: 'Swap',
              unit: 'MB'
            }"
            :width="120"
            :height="20"
            :show-percentage="false"
          />
        </div>
      </div>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { DatabaseOutlined } from '@vicons/antd';
import RingChart from './charts/RingChart.vue';
import ProgressChart from './charts/ProgressChart.vue';
import type { MemoryInfo } from '../types';

interface Props {
  memoryInfo?: MemoryInfo | null;
}

defineProps<Props>();

const getMemoryStatusType = (usage: number) => {
  if (usage >= 90) return 'error';
  if (usage >= 80) return 'warning';
  return 'info';
};

const formatBytes = (bytes: number) => {
  if (bytes === 0) return '0 MB';
  
  const mb = bytes / 1024;
  if (mb >= 1024) {
    return `${(mb / 1024).toFixed(1)} GB`;
  }
  return `${mb.toFixed(0)} MB`;
};
</script>

<style scoped>
.memory-monitor {
  margin-bottom: 2px;
}

.memory-content {
  display: flex;
  gap: 8px;
  align-items: flex-start;
}

.memory-chart {
  flex-shrink: 0;
}

.memory-details {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.memory-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.detail-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 10px;
  padding: 1px 0;
}

.detail-label {
  color: #666;
  font-weight: 500;
}

.detail-value {
  color: #333;
  font-weight: 600;
}

.text-orange-500 {
  color: #f97316;
}

.text-green-500 {
  color: #22c55e;
}

.swap-info {
  border-top: 1px solid #f0f0f0;
  padding-top: 4px;
}

.swap-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2px;
}

.swap-title {
  font-size: 10px;
  color: #666;
  font-weight: 500;
}

.swap-usage {
  font-size: 10px;
  color: #333;
  font-weight: 600;
}
</style>
