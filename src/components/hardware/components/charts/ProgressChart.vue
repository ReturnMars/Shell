<template>
  <div class="progress-chart" :style="{ width: width + 'px', height: height + 'px' }">
    <div class="progress-container">
      <div class="progress-bar">
        <div 
          class="progress-fill" 
          :style="{ 
            width: percentage + '%',
            backgroundColor: color
          }"
        ></div>
      </div>
      <div class="progress-text">
        <span class="progress-label">{{ data.label }}</span>
        <span class="progress-value">{{ displayValue }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { ProgressChartData } from '../../types';

interface Props {
  data: ProgressChartData;
  width?: number;
  height?: number;
  showPercentage?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  width: 200,
  height: 40,
  showPercentage: true
});

const percentage = computed(() => {
  if (props.data.max === 0) return 0;
  return Math.min((props.data.value / props.data.max) * 100, 100);
});

const displayValue = computed(() => {
  if (props.showPercentage) {
    return `${Math.round(percentage.value)}%`;
  }
  return `${props.data.value}${props.data.unit}/${props.data.max}${props.data.unit}`;
});

const color = computed(() => {
  if (props.data.color) return props.data.color;
  
  const percent = percentage.value;
  if (percent >= 90) return '#ff4d4f'; // 红色 - 危险
  if (percent >= 70) return '#faad14'; // 橙色 - 警告
  if (percent >= 50) return '#52c41a'; // 绿色 - 正常
  return '#1890ff'; // 蓝色 - 良好
});
</script>

<style scoped>
.progress-chart {
  display: flex;
  align-items: center;
  justify-content: center;
}

.progress-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background-color: #f0f0f0;
  border-radius: 4px;
  overflow: hidden;
  position: relative;
}

.progress-fill {
  height: 100%;
  border-radius: 4px;
  transition: width 0.3s ease;
  position: relative;
}

.progress-fill::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
  animation: shimmer 2s infinite;
}

@keyframes shimmer {
  0% { transform: translateX(-100%); }
  100% { transform: translateX(100%); }
}

.progress-text {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 12px;
}

.progress-label {
  color: #666;
  font-weight: 500;
}

.progress-value {
  color: #333;
  font-weight: 600;
}
</style>
