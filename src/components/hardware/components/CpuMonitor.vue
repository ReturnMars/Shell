<template>
  <n-card
    size="small"
    class="cpu-monitor"
    :content-style="{ padding: '8px' }"
    :header-style="{ padding: '8px 8px 4px 8px' }"
  >
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <n-icon size="16" color="#1890ff">
            <DesktopOutlined />
          </n-icon>
          <span class="text-sm font-medium">CPU</span>
        </div>
        <n-tag size="small" :type="getCpuStatusType(cpuInfo?.usage || 0)">
          {{ cpuInfo?.usage?.toFixed(2) || 0 }}%
        </n-tag>
      </div>
    </template>

    <div class="cpu-content">
      <!-- CPU 使用率环形图 -->
      <div class="cpu-chart">
        <RingChart
          :data="{ name: 'CPU', value: cpuInfo?.usage || 0 }"
          :width="80"
          :height="80"
          :size="70"
          :stroke-width="6"
          label="使用率"
        />
      </div>

      <!-- CPU 详细信息 -->
      <div class="cpu-details">
        <div class="detail-item">
          <span class="detail-label">型号:</span>
          <span class="detail-value">
            <n-ellipsis :line-clamp="1">{{
              cpuInfo?.model || "未知"
            }}</n-ellipsis>
          </span>
        </div>
        <div class="detail-item">
          <span class="detail-label">核心数:</span>
          <span class="detail-value">{{ cpuInfo?.cores || 0 }} 核</span>
        </div>
        <div class="detail-item" v-if="cpuInfo?.temperature">
          <span class="detail-label">温度:</span>
          <span
            class="detail-value"
            :class="getTemperatureClass(cpuInfo.temperature)"
          >
            {{ cpuInfo.temperature }}°C
          </span>
        </div>
        <div class="detail-item" v-if="cpuInfo?.frequency">
          <span class="detail-label">频率:</span>
          <span class="detail-value">{{
            formatFrequency(cpuInfo.frequency)
          }}</span>
        </div>
      </div>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { DesktopOutlined } from "@vicons/antd";
import RingChart from "./charts/RingChart.vue";
import type { CpuInfo } from "../types";

/**
 * CPU监控组件
 * 
 * 数据单位说明（后端返回）：
 * - model: String（CPU型号）
 * - cores: Number（核心数）
 * - usage: 百分比（0-100，CPU使用率）
 * - frequency: MHz（兆赫兹，使用 formatFrequency() 格式化显示）
 * - temperature: 摄氏度（使用 getTemperatureClass() 根据温度显示不同颜色）
 */

interface Props {
  cpuInfo?: CpuInfo | null;
}

defineProps<Props>();

const getCpuStatusType = (usage: number) => {
  if (usage >= 90) return "error";
  if (usage >= 70) return "warning";
  return "info";
};

const getTemperatureClass = (temperature: number) => {
  if (temperature >= 80) return "text-red-500";
  if (temperature >= 70) return "text-orange-500";
  return "text-green-500";
};

const formatFrequency = (frequency: number) => {
  if (frequency >= 1000) {
    return `${(frequency / 1000).toFixed(1)} GHz`;
  }
  return `${frequency} MHz`;
};
</script>

<style scoped>
.cpu-monitor {
  margin-bottom: 2px;
}

.cpu-content {
  display: flex;
  gap: 8px;
  align-items: flex-start;
}

.cpu-chart {
  flex-shrink: 0;
}

.cpu-details {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.detail-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 10px;
  padding: 4px 0;
}

.detail-label {
  color: #666;
  font-weight: 500;
  line-height: 1;
}

.detail-value {
  flex: 1;
  color: #333;
  font-weight: 600;
  line-height: 50%;
  text-align: right;
}

.text-red-500 {
  color: #ef4444;
}

.text-orange-500 {
  color: #f97316;
}

.text-green-500 {
  color: #22c55e;
}
</style>
