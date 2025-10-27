<template>
  <n-card
    size="small"
    class="network-monitor"
    :content-style="{ padding: '8px' }"
    :header-style="{ padding: '8px 8px 4px 8px' }"
  >
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <n-icon size="16" color="#13c2c2">
            <WifiOutlined />
          </n-icon>
          <span class="text-sm font-medium">网络</span>
        </div>
        <n-tag size="small" type="info">
          {{ networkInfo.interfaces.length }} 个接口
        </n-tag>
      </div>
    </template>

    <div class="network-content">
      <!-- 网络总览 -->
      <div class="network-overview">
        <div class="overview-item">
          <div class="overview-label">总接收</div>
          <div class="overview-value">
            {{ formatBytes(networkInfo.total_rx * 1024 * 1024) }}
          </div>
        </div>
        <div class="overview-item">
          <div class="overview-label">总发送</div>
          <div class="overview-value">
            {{ formatBytes(networkInfo.total_tx * 1024 * 1024) }}
          </div>
        </div>
        <div class="overview-item">
          <div class="overview-label">接收速度</div>
          <div class="overview-value">
            {{ (networkInfo.rx_speed || 0).toFixed(1) }} MB/s
          </div>
        </div>
        <div class="overview-item">
          <div class="overview-label">发送速度</div>
          <div class="overview-value">
            {{ (networkInfo.tx_speed || 0).toFixed(1) }} MB/s
          </div>
        </div>
      </div>

      <!-- 网络接口列表 -->
      <div class="network-interfaces">
        <div
          class="interface-item"
          v-for="(iface, index) in networkInfo.interfaces"
          :key="index"
        >
          <div class="interface-header">
            <div class="interface-name">
              <n-icon
                size="14"
                :color="iface.status === 'up' ? '#52c41a' : '#ff4d4f'"
              >
                <component
                  :is="
                    iface.status === 'up' ? WifiOutlined : DisconnectOutlined
                  "
                />
              </n-icon>
              <span class="interface-label">{{ iface.name }}</span>
            </div>
          </div>

          <div class="interface-stats">
            <div class="stat-item">
              <span class="stat-label">接收:</span>
              <span class="stat-value">{{
                formatBytes(iface.rx * 1024 * 1024)
              }}</span>
              <span class="stat-speed"
                >{{ (iface.rx_speed || 0).toFixed(1) }} MB/s</span
              >
            </div>
            <div class="stat-item">
              <span class="stat-label">发送:</span>
              <span class="stat-value">{{
                formatBytes(iface.tx * 1024 * 1024)
              }}</span>
              <span class="stat-speed"
                >{{ (iface.tx_speed || 0).toFixed(1) }} MB/s</span
              >
            </div>
          </div>
        </div>
      </div>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { WifiOutlined, DisconnectOutlined } from "@vicons/antd";
import type { NetworkInfo } from "../types";
import { formatBytes } from "@/utils/format";

interface Props {
  networkInfo: NetworkInfo;
}

defineProps<Props>();
</script>

<style scoped>
.network-monitor {
  margin-bottom: 2px;
}

.network-content {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.network-overview {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 4px;
}

.overview-item {
  text-align: center;
  padding: 4px;
  background-color: #f8f9fa;
  border-radius: 4px;
}

.overview-label {
  font-size: 9px;
  color: #666;
  margin-bottom: 1px;
}

.overview-value {
  font-size: 10px;
  font-weight: 600;
  color: #333;
}

.network-interfaces {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.interface-item {
  padding: 4px;
  border: 1px solid #f0f0f0;
  border-radius: 4px;
  background-color: #fafafa;
}

.interface-header {
  margin-bottom: 3px;
}

.interface-name {
  display: flex;
  align-items: center;
  gap: 4px;
}

.interface-label {
  font-size: 12px;
  font-weight: 600;
  color: #333;
}

.interface-stats {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 9px;
  padding: 1px 0;
}

.stat-label {
  color: #666;
  font-weight: 500;
}

.stat-value {
  color: #333;
  font-weight: 600;
}

.stat-speed {
  color: #1890ff;
  font-weight: 500;
  font-size: 9px;
}
</style>
