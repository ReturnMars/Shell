// 硬件信息状态管理
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { 
  HardwareInfo
} from '@/components/hardware/types';

export const useHardwareStore = defineStore('hardware', () => {
  // 状态
  const hardwareInfo = ref<HardwareInfo | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const lastUpdate = ref<number | null>(null);
  const autoRefresh = ref(true);
  const refreshInterval = ref(5000); // 5秒刷新一次
  const refreshTimer = ref<NodeJS.Timeout | null>(null);

  // 计算属性
  const isConnected = computed(() => hardwareInfo.value !== null);
  const cpuUsage = computed(() => hardwareInfo.value?.cpu.usage || 0);
  const memoryUsage = computed(() => hardwareInfo.value?.memory.usage || 0);
  const storageUsage = computed(() => {
    if (!hardwareInfo.value?.storage) return [];
    return hardwareInfo.value.storage.map(storage => ({
      device: storage.device,
      mount_point: storage.mount_point,
      filesystem: storage.filesystem,
      usage: storage.usage,
      total: storage.total,
      used: storage.used,
      free: storage.free,
      type: storage.type
    }));
  });

  // 获取硬件信息
  const fetchHardwareInfo = async (connectionId?: string) => {
    console.log('硬件Store - fetchHardwareInfo 被调用:', connectionId);
    
    if (!connectionId) {
      error.value = '未指定连接ID';
      console.log('硬件Store - 错误: 未指定连接ID');
      return;
    }

    // 在获取硬件信息前先检查连接状态
    try {
      const isConnected = await invoke<boolean>('check_connection_status', { connectionId });
      if (!isConnected) {
        console.log('硬件Store - 连接不存在，停止获取硬件信息');
        hardwareInfo.value = null;
        lastUpdate.value = null;
        error.value = '连接不存在';
        return;
      }
    } catch (checkError) {
      console.warn('硬件Store - 检查连接状态失败:', checkError);
      // 如果检查失败，继续尝试获取硬件信息
    }

    loading.value = true;
    error.value = null;

    try {
      console.log('硬件Store - 调用 Tauri API get_hardware_info:', connectionId);
      // 调用 Tauri 命令获取真实硬件信息
      const data = await invoke<HardwareInfo>('get_hardware_info', {
        connectionId
      });
      console.log("🚀 ~ fetchHardwareInfo ~ data:", data)
      
      console.log('硬件Store - 获取到硬件信息:', data);
      hardwareInfo.value = data;
      lastUpdate.value = Date.now();
      
      // 清除之前的错误
      error.value = null;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : '获取硬件信息失败';
      error.value = errorMessage;
      console.error('硬件Store - 获取硬件信息失败:', err);
      
      // 检查是否是连接不存在的错误
      if (err instanceof Error && err.message.includes('连接不存在')) {
        console.log('硬件Store - 检测到连接不存在，清除硬件信息');
        // 连接不存在时，清除硬件信息，不显示任何数据
        hardwareInfo.value = null;
        lastUpdate.value = null;
        return;
      }
      
      // 对于其他错误，尝试错误恢复
      console.log('硬件Store - 尝试错误恢复机制');
      const recoverySuccess = await retryWithBackoff(connectionId);
      
      if (!recoverySuccess) {
        // 如果恢复失败，清除硬件信息
        console.log('硬件Store - 恢复失败，清除硬件信息');
        hardwareInfo.value = null;
        lastUpdate.value = null;
      }
    } finally {
      loading.value = false;
    }
  };

  // 刷新硬件信息
  const refreshHardwareInfo = async (connectionId?: string) => {
    await fetchHardwareInfo(connectionId);
  };

  // 开始自动刷新
  const startAutoRefresh = (connectionId?: string) => {
    if (refreshTimer.value) {
      clearInterval(refreshTimer.value);
    }
    
    if (autoRefresh.value && connectionId) {
      refreshTimer.value = setInterval(async () => {
        // 在自动刷新前检查连接状态
        try {
          const isConnected = await invoke<boolean>('check_connection_status', { connectionId });
          if (isConnected) {
            await fetchHardwareInfo(connectionId);
          } else {
            console.log('硬件Store - 自动刷新时检测到连接不存在，停止自动刷新');
            stopAutoRefresh();
          }
        } catch (error) {
          console.warn('硬件Store - 自动刷新时检查连接状态失败:', error);
          stopAutoRefresh();
        }
      }, refreshInterval.value);
    }
  };

  // 停止自动刷新
  const stopAutoRefresh = () => {
    if (refreshTimer.value) {
      clearInterval(refreshTimer.value);
      refreshTimer.value = null;
    }
  };

  // 设置自动刷新
  const setAutoRefresh = (enabled: boolean) => {
    autoRefresh.value = enabled;
    if (enabled) {
      // 需要重新启动自动刷新
      // 这里需要从外部传入 connectionId
    } else {
      stopAutoRefresh();
    }
  };

  // 设置刷新间隔
  const setRefreshInterval = (interval: number) => {
    refreshInterval.value = interval;
    if (autoRefresh.value && refreshTimer.value) {
      stopAutoRefresh();
      // 需要重新启动自动刷新
    }
  };

  // 错误恢复机制
  const retryWithBackoff = async (connectionId: string, maxRetries: number = 3) => {
    let retryCount = 0;
    let delay = 1000; // 1秒
    
    while (retryCount < maxRetries) {
      try {
        console.log(`硬件Store - 重试获取硬件信息 (${retryCount + 1}/${maxRetries}):`, connectionId);
        const data = await invoke<HardwareInfo>('get_hardware_info', {
          connectionId
        });
        
        console.log('硬件Store - 重试成功，获取到硬件信息:', data);
        hardwareInfo.value = data;
        lastUpdate.value = Date.now();
        error.value = null;
        return true;
      } catch (err) {
        retryCount++;
        console.warn(`硬件Store - 重试 ${retryCount} 失败:`, err);
        
        if (retryCount < maxRetries) {
          console.log(`硬件Store - 等待 ${delay}ms 后重试`);
          await new Promise(resolve => setTimeout(resolve, delay));
          delay *= 2; // 指数退避
        }
      }
    }
    
    console.error('硬件Store - 重试失败，清除硬件信息');
    hardwareInfo.value = null;
    lastUpdate.value = null;
    return false;
  };

  // 清除硬件信息
  const clearHardwareInfo = () => {
    hardwareInfo.value = null;
    lastUpdate.value = null;
    error.value = null;
    stopAutoRefresh();
  };

  // 清理函数
  const cleanup = () => {
    stopAutoRefresh();
  };

  return {
    // 状态
    hardwareInfo,
    loading,
    error,
    lastUpdate,
    autoRefresh,
    refreshInterval,
    
    // 计算属性
    isConnected,
    cpuUsage,
    memoryUsage,
    storageUsage,
    
    // 方法
    fetchHardwareInfo,
    refreshHardwareInfo,
    startAutoRefresh,
    stopAutoRefresh,
    setAutoRefresh,
    setRefreshInterval,
    clearHardwareInfo,
    retryWithBackoff,
    cleanup
  };
});
