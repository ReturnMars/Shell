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
          <div class="overview-label">接收速度</div>
          <div class="overview-value">
            {{ `${formatFromBytes(networkInfo.rx_speed)} /s` }}
          </div>
        </div>
        <div class="overview-item">
          <div class="overview-label">发送速度</div>
          <div class="overview-value">
            {{ `${formatFromBytes(networkInfo.tx_speed)} /s` }}
          </div>
        </div>
        <div class="overview-item">
          <div class="overview-label">总接收</div>
          <div class="overview-value">
            {{ formatFromBytes(networkInfo.total_rx) }}
          </div>
        </div>
        <div class="overview-item">
          <div class="overview-label">总发送</div>
          <div class="overview-value">
            {{ formatFromBytes(networkInfo.total_tx) }}
          </div>
        </div>
      </div>

      <!-- 网络接口列表 -->
      <div class="network-interfaces">
        <n-collapse size="tiny">
          <n-collapse-item name="network-interfaces">
            <template #arrow>
              <n-icon size="12" color="#13c2c2">
                <CaretRightOutlined />
              </n-icon>
            </template>
            <template #header>
              <div class="interface-header-title text-xs font-medium">
                网络接口
              </div>
            </template>
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
                        iface.status === 'up'
                          ? WifiOutlined
                          : DisconnectOutlined
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
                    formatFromBytes(iface.rx)
                  }}</span>
                  <span class="stat-speed"
                    >{{ formatFromBytes(iface.rx_speed) }} /s</span
                  >
                </div>
                <div class="stat-item">
                  <span class="stat-label">发送:</span>
                  <span class="stat-value">{{
                    formatFromBytes(iface.tx)
                  }}</span>
                  <span class="stat-speed"
                    >{{ formatFromBytes(iface.tx_speed) }} /s</span
                  >
                </div>
              </div>
            </div>
          </n-collapse-item>
        </n-collapse>
      </div>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import {
  WifiOutlined,
  DisconnectOutlined,
  CaretRightOutlined,
} from "@vicons/antd";
import type { NetworkInfo } from "../types";
import { formatFromBytes } from "@/utils/format";

/**
 * 网络监控组件
 *
 * 数据单位说明（后端返回）：
 * - networkInfo.total_rx/total_tx: bytes（字节数），累计接收/发送的总字节数，使用 formatFromBytes() 格式化
 * - networkInfo.rx_speed/tx_speed: bytes/s（字节/秒），总接收/发送速度，使用 formatFromBytes() 格式化
 * - interface.rx/tx: bytes（字节数），接口累计接收/发送的字节数，使用 formatFromBytes() 格式化
 * - interface.rx_speed/tx_speed: bytes/s（字节/秒），接口接收/发送速度，使用 formatFromBytes() 格式化
 *
 * formatFromBytes() 函数会自动转换为合适的单位显示（B/KB/MB/GB等）
 */

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
