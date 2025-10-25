<template>
  <div class="bar-chart" :style="{ width: width + 'px', height: height + 'px' }">
    <div class="bar-container">
      <div class="bar-item" v-for="(item, index) in data" :key="index">
        <div class="bar-label">{{ item.name }}</div>
        <div class="bar-wrapper">
          <div class="bar-track">
            <div 
              class="bar-fill" 
              :style="{ 
                width: getBarWidth(item.value) + '%',
                backgroundColor: item.color || getBarColor(item.value)
              }"
            ></div>
          </div>
          <div class="bar-value">{{ item.value }}%</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { BarChartData } from '../../types';

interface Props {
  data: BarChartData[];
  width?: number;
  height?: number;
  maxValue?: number;
  showValues?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  width: 200,
  height: 120,
  maxValue: 100,
  showValues: true
});

const getBarWidth = (value: number) => {
  return Math.min((value / props.maxValue) * 100, 100);
};

const getBarColor = (value: number) => {
  if (value >= 90) return '#ff4d4f'; // 红色 - 危险
  if (value >= 70) return '#faad14'; // 橙色 - 警告
  if (value >= 50) return '#52c41a'; // 绿色 - 正常
  return '#1890ff'; // 蓝色 - 良好
};
</script>

<style scoped>
.bar-chart {
  display: flex;
  align-items: center;
  justify-content: center;
}

.bar-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.bar-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.bar-label {
  font-size: 12px;
  color: #666;
  font-weight: 500;
  margin-bottom: 2px;
}

.bar-wrapper {
  display: flex;
  align-items: center;
  gap: 8px;
}

.bar-track {
  flex: 1;
  height: 16px;
  background-color: #f0f0f0;
  border-radius: 8px;
  overflow: hidden;
  position: relative;
}

.bar-fill {
  height: 100%;
  border-radius: 8px;
  transition: width 0.3s ease;
  position: relative;
}

.bar-fill::after {
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

.bar-value {
  font-size: 11px;
  color: #333;
  font-weight: 600;
  min-width: 30px;
  text-align: right;
}
</style>
