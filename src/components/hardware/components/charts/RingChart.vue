<template>
  <div class="ring-chart" :style="{ width: width + 'px', height: height + 'px' }">
    <div class="ring-container">
      <div class="ring-circle" :style="{ width: size + 'px', height: size + 'px' }">
        <svg :width="size" :height="size" class="ring-svg">
          <!-- 背景圆环 -->
          <circle
            :cx="center"
            :cy="center"
            :r="radius"
            fill="none"
            :stroke="backgroundColor"
            :stroke-width="strokeWidth"
            class="ring-background"
          />
          <!-- 进度圆环 -->
          <circle
            :cx="center"
            :cy="center"
            :r="radius"
            fill="none"
            :stroke="progressColor"
            :stroke-width="strokeWidth"
            :stroke-dasharray="circumference"
            :stroke-dashoffset="dashOffset"
            stroke-linecap="round"
            class="ring-progress"
            :style="{ transform: 'rotate(-90deg)', transformOrigin: 'center' }"
          />
        </svg>
        <!-- 中心文字 -->
        <div class="ring-center">
          <div class="ring-percentage">{{ Math.round(percentage) }}%</div>
          <div class="ring-label">{{ label }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { RingChartData } from '../../types';

interface Props {
  data: RingChartData;
  width?: number;
  height?: number;
  size?: number;
  strokeWidth?: number;
  label?: string;
}

const props = withDefaults(defineProps<Props>(), {
  width: 120,
  height: 120,
  size: 100,
  strokeWidth: 8,
  label: '使用率'
});

const center = computed(() => props.size / 2);
const radius = computed(() => (props.size - props.strokeWidth) / 2);
const circumference = computed(() => 2 * Math.PI * radius.value);

const percentage = computed(() => {
  return Math.min(Math.max(props.data.value, 0), 100);
});

const dashOffset = computed(() => {
  return circumference.value - (percentage.value / 100) * circumference.value;
});

const progressColor = computed(() => {
  if (props.data.color) return props.data.color;
  
  const percent = percentage.value;
  if (percent >= 90) return '#ff4d4f'; // 红色 - 危险
  if (percent >= 70) return '#faad14'; // 橙色 - 警告
  if (percent >= 50) return '#52c41a'; // 绿色 - 正常
  return '#1890ff'; // 蓝色 - 良好
});

const backgroundColor = computed(() => '#f0f0f0');
</script>

<style scoped>
.ring-chart {
  display: flex;
  align-items: center;
  justify-content: center;
}

.ring-container {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.ring-circle {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.ring-svg {
  position: absolute;
  top: 0;
  left: 0;
}

.ring-background {
  opacity: 0.3;
}

.ring-progress {
  transition: stroke-dashoffset 0.5s ease;
}

.ring-center {
  position: relative;
  z-index: 1;
  text-align: center;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.ring-percentage {
  font-size: 20px;
  font-weight: 700;
  color: #333;
  line-height: 1;
  margin-bottom: 4px;
}

.ring-label {
  font-size: 12px;
  color: #666;
  font-weight: 500;
}
</style>
