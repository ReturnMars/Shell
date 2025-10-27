<template>
  <div class="hardware-monitor">
    <!-- 标题栏 -->
    <div class="hardware-header" :class="{ scrolled: isScrolled }">
      <div class="flex items-center gap-2 p-2 pt-0">
        <span class="text-medium font-medium text-gray-600 tracking-wider">
          硬件监控
        </span>
      </div>
      <div class="flex items-center gap-2">
        <!-- 刷新模式切换 -->
        <n-tooltip trigger="hover" :disabled="!isCurrentConnectionConnected">
          <template #trigger>
            <n-button
              quaternary
              circle
              size="tiny"
              :type="autoRefresh ? 'primary' : 'default'"
              @click="toggleAutoRefresh"
              class="refresh-mode-btn"
              :disabled="!isCurrentConnectionConnected"
            >
              <template #icon>
                <n-icon size="16" :color="autoRefresh ? '#18a058' : '#9ca4ae'">
                  <span
                    :class="{
                      'cloud-sync-icon': !autoRefresh,
                    }"
                  >
                    <CloudSyncOutlined />
                  </span>
                </n-icon>
              </template>
            </n-button>
          </template>
          {{ autoRefresh ? "自动刷新中" : "手动模式" }}
        </n-tooltip>

        <!-- 手动刷新按钮 -->
        <n-tooltip trigger="hover">
          <template #trigger>
            <n-button
              quaternary
              circle
              size="tiny"
              :loading="manualLoading"
              @click="handleRefresh"
              :disabled="!isCurrentConnectionConnected"
            >
              <template #icon>
                <n-icon>
                  <SyncOutlined />
                </n-icon>
              </template>
            </n-button>
          </template>
          立即刷新
        </n-tooltip>
      </div>
    </div>

    <!-- 硬件信息内容 -->
    <div class="hardware-content" ref="contentRef" @scroll="handleScroll">
      <!-- 加载状态 -->
      <div v-if="loading && !hardwareInfo" class="hardware-loading">
        <n-spin size="small">
          <div class="hardware-loading-text">正在获取硬件信息...</div>
        </n-spin>
      </div>

      <!-- 错误状态 -->
      <div v-else-if="error" class="error-container">
        <n-alert type="error" size="small" :show-icon="false">
          <div class="error-content">
            <n-icon size="14" color="#ff4d4f">
              <ExclamationCircleOutlined />
            </n-icon>
            <span class="error-text">{{ error }}</span>
          </div>
        </n-alert>
      </div>

      <!-- 硬件信息展示 -->
      <div v-else-if="hardwareInfo" class="hardware-info">
        <!-- CPU 监控 -->
        <CpuMonitor :cpu-info="hardwareInfo.cpu" />
        <!-- 内存监控 -->
        <MemoryMonitor :memory-info="hardwareInfo.memory" />

        <!-- 存储监控 -->
        <StorageMonitor :storage-list="hardwareInfo.storage" />

        <!-- 网络信息（可选） -->
        <NetworkMonitor
          v-if="hardwareInfo.network"
          :network-info="hardwareInfo.network"
        />
      </div>

      <!-- 无连接状态 -->
      <div v-else class="no-connection">
        <n-empty size="small" description="未连接到服务器">
          <template #icon>
            <n-icon size="24" color="#d9d9d9">
              <DisconnectOutlined />
            </n-icon>
          </template>
        </n-empty>
      </div>
    </div>

    <!-- 底部状态信息 -->
    <div v-if="hardwareInfo" class="hardware-footer">
      <div class="status-info">
        <!-- 连接状态详情 -->
        <n-tooltip trigger="hover" v-if="currentConnectionState">
          <template #trigger>
            <n-icon
              size="14"
              :color="isCurrentConnectionConnected ? '#18a058' : '#d03050'"
            >
              <ExclamationCircleOutlined />
            </n-icon>
          </template>
          <div class="text-xs">
            <div>状态: {{ currentConnectionState.status }}</div>
            <div v-if="currentConnectionState.error">
              错误: {{ currentConnectionState.error }}
            </div>
            <div>
              重试次数: {{ currentConnectionState.retryCount }}/{{
                currentConnectionState.maxRetries
              }}
            </div>
            <div>
              最后检查:
              {{ new Date(currentConnectionState.lastCheck).toLocaleString() }}
            </div>
          </div>
        </n-tooltip>
        <span class="status-text">
          最后更新: {{ formatLastUpdate(lastUpdate) }}
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted, watch, ref } from "vue";
import {
  CloudSyncOutlined,
  SyncOutlined,
  ExclamationCircleOutlined,
  DisconnectOutlined,
} from "@vicons/antd";
import { useHardwareStore } from "@/stores/hardware";
import { useConnectionStore } from "@/stores/connection";
import CpuMonitor from "./components/CpuMonitor.vue";
import MemoryMonitor from "./components/MemoryMonitor.vue";
import StorageMonitor from "./components/StorageMonitor.vue";
import NetworkMonitor from "./components/NetworkMonitor.vue";

/**
 * 硬件监控主组件
 * 
 * 硬件信息数据说明：
 * 1. CPU: 
 *    - usage: 百分比 (0-100)
 *    - frequency: MHz（兆赫兹）
 *    - temperature: 摄氏度
 * 
 * 2. Memory（内存）:
 *    - total/used/free: MB
 *    - swap.total/used/free: MB
 * 
 * 3. Storage（存储）:
 *    - total/used/free: MB
 * 
 * 4. Network（网络）:
 *    - total_rx/total_tx: bytes（字节）
 *    - rx_speed/tx_speed: bytes/s（字节每秒）
 *    - interface.rx/tx: bytes（字节）
 *    - interface.rx_speed/tx_speed: bytes/s（字节每秒）
 * 
 * 格式化函数说明：
 * - formatFromMB(): 用于内存和存储，输入MB，自动格式化为MB/GB/TB
 * - formatFromBytes(): 用于网络流量和速度，输入bytes，自动格式化为B/KB/MB/GB
 * - formatFrequency(): 用于CPU频率，输入MHz，自动格式化为MHz/GHz
 */

const hardwareStore = useHardwareStore();
const connectionStore = useConnectionStore();

// 滚动状态
const contentRef = ref<HTMLElement>();
const isScrolled = ref(false);

// 响应式数据
const loading = computed(() => hardwareStore.loading);
const error = computed(() => hardwareStore.error);
const hardwareInfo = computed(() => hardwareStore.hardwareInfo);
const lastUpdate = computed(() => hardwareStore.lastUpdate);
const autoRefresh = computed(() => hardwareStore.autoRefresh);

// 当前连接
const currentConnection = computed(() => connectionStore.currentConnection);

// 当前连接ID
const currentConnectionId = computed(() => currentConnection.value?.id);

// 当前连接是否已连接 - 使用新的状态管理器
const isCurrentConnectionConnected = computed(
  () => connectionStore.isCurrentConnectionConnected
);

// 当前连接状态
const currentConnectionState = computed(
  () => connectionStore.currentConnectionState
);

// 手动刷新状态
const manualLoading = ref(false);

// 方法
const handleRefresh = async () => {
  if (currentConnectionId.value && isCurrentConnectionConnected.value) {
    manualLoading.value = true;
    await hardwareStore.refreshHardwareInfo(currentConnectionId.value);
    manualLoading.value = false;
  }
};

const toggleAutoRefresh = () => {
  hardwareStore.setAutoRefresh(!autoRefresh.value);
};

// 处理滚动事件
const handleScroll = () => {
  if (contentRef.value) {
    isScrolled.value = contentRef.value.scrollTop > 0;
  }
};

const formatLastUpdate = (timestamp: number | null) => {
  if (!timestamp) return "从未更新";

  const now = Date.now();
  const diff = now - timestamp;

  if (diff < 60000) {
    // 1分钟内
    return "刚刚";
  } else if (diff < 3600000) {
    // 1小时内
    return `${Math.floor(diff / 60000)}分钟前`;
  } else if (diff < 86400000) {
    // 1天内
    return `${Math.floor(diff / 3600000)}小时前`;
  } else {
    return new Date(timestamp).toLocaleString();
  }
};

// 监听连接变化
watch(
  [currentConnectionId, isCurrentConnectionConnected],
  async ([newConnectionId, isConnected], [oldConnectionId, wasConnected]) => {

    // 如果连接ID变化了，或者连接状态变化了
    if (newConnectionId !== oldConnectionId || isConnected !== wasConnected) {
      if (newConnectionId && isConnected) {
        // 设置当前连接ID（会立即从池中获取缓存的硬件信息）
        hardwareStore.setCurrentConnectionId(newConnectionId);
        
        // 如果池中没有缓存，则获取硬件信息
        if (!hardwareStore.hardwareInfo) {
          await hardwareStore.fetchHardwareInfo(newConnectionId);
        }
        
        // 开始自动刷新
        hardwareStore.startAutoRefresh(newConnectionId);
      } else {
        // 清空当前连接ID并停止自动刷新
        hardwareStore.setCurrentConnectionId(null);
        hardwareStore.stopAutoRefresh();
      }
    }
  },
  { immediate: true }
);

// 监听连接状态变化（处理错误和重连）
watch(currentConnectionState, (newState, oldState) => {
  if (newState && oldState && newState.status !== oldState.status) {
    // 如果连接状态变为错误或断开，清除硬件信息
    if (newState.status === "error" || newState.status === "disconnected") {
      hardwareStore.clearHardwareInfo();
      hardwareStore.stopAutoRefresh();
    }
  }
});


// 组件挂载时初始化
onMounted(async () => {
  // 初始化连接状态管理器
  connectionStore.initializeStateManager();

  // 同步后端连接状态
  await connectionStore.connectionStateManager.syncBackendConnections();

  // 如果有已连接，则获取硬件信息
  if (currentConnectionId.value && isCurrentConnectionConnected.value) {
    await hardwareStore.fetchHardwareInfo(currentConnectionId.value);
    hardwareStore.startAutoRefresh(currentConnectionId.value);
  }
});

// 组件卸载时清理
onUnmounted(() => {
  hardwareStore.cleanup();
  connectionStore.cleanupStateManager();
});
</script>

<style scoped>
.hardware-monitor {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.hardware-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
  padding: 0 2px;
  position: relative;
  transition: box-shadow 0.2s ease;
  z-index: 10;
}

.hardware-header.scrolled {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  background-color: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(8px);
}

.hardware-content {
  flex: 1;
  overflow-y: auto;
  padding: 0 2px;
  min-height: 0; /* 确保 flex 子元素可以收缩 */
}

.hardware-loading {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 80px;
  padding: 20px;
  margin: 8px 0;
}

.loading-text {
  font-size: 12px;
  color: #666;
  margin-left: 8px;
}

.error-container {
  margin-bottom: 8px;
}

.error-content {
  display: flex;
  align-items: center;
  gap: 6px;
}

.error-text {
  font-size: 11px;
}

.hardware-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-height: 0; /* 确保可以滚动 */
  padding: 4px 8px;
}

.no-connection {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 120px;
}

.hardware-footer {
  margin-top: 4px;
  padding: 2px 4px;
  border-top: 1px solid #f0f0f0;
  background-color: #fafafa;
}

.status-info {
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.status-text {
  font-size: 10px;
  color: #666;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .hardware-header {
    flex-direction: column;
    gap: 4px;
    align-items: flex-start;
  }

  .hardware-content {
    padding: 0 2px;
  }

  .hardware-footer {
    padding: 2px 4px;
  }

  .status-info {
    flex-direction: column;
    gap: 2px;
    align-items: flex-start;
  }
}

@media (max-width: 480px) {
  .hardware-content {
    padding: 0 1px;
  }

  .hardware-info {
    gap: 4px;
  }
}

/* 自定义滚动条 */
.hardware-content::-webkit-scrollbar {
  width: 3px;
}

.hardware-content::-webkit-scrollbar-track {
  background: transparent;
}

.hardware-content::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 2px;
}

.hardware-content::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}

/* 云同步图标斜杠效果 */
.cloud-sync-icon {
  position: relative;
}

.cloud-sync-icon::after {
  content: "";
  position: absolute;
  top: 30%;
  left: 50%;
  width: 15px;
  height: 1px;
  background-color: #9ca4ae;
  transform: translate(-50%, -50%) rotate(45deg);
  z-index: 1;
}
</style>
