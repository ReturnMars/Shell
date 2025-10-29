// 硬件信息状态管理
import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { HardwareInfo } from "@/components/hardware/types";

export const useHardwareStore = defineStore("hardware", () => {
  // 硬件信息池 - 每个连接ID对应一个硬件信息
  const hardwareInfoPool = ref<Map<string, HardwareInfo>>(new Map());
  // 当前连接ID
  const currentConnectionId = ref<string | null>(null);
  // 加载状态
  const loading = ref(false);
  // 错误信息
  const error = ref<string | null>(null);
  // 最后更新时间池
  const lastUpdatePool = ref<Map<string, number>>(new Map());
  // 自动刷新
  const autoRefresh = ref(true);
  const refreshInterval = ref(2_000); // 2秒刷新一次
  // 每个连接的定时器池
  const refreshTimers = ref<Map<string, NodeJS.Timeout>>(new Map());

  // 当前连接的硬件信息
  const hardwareInfo = computed(() => {
    if (!currentConnectionId.value) return null;
    return hardwareInfoPool.value.get(currentConnectionId.value) || null;
  });

  // 当前连接的最后更新时间
  const lastUpdate = computed(() => {
    if (!currentConnectionId.value) return null;
    return lastUpdatePool.value.get(currentConnectionId.value) || null;
  });

  // 计算属性
  const isConnected = computed(() => hardwareInfo.value !== null);
  const cpuUsage = computed(() => hardwareInfo.value?.cpu.usage || 0);
  const memoryUsage = computed(() => hardwareInfo.value?.memory.usage || 0);
  const storageUsage = computed(() => {
    if (!hardwareInfo.value?.storage) return [];
    return hardwareInfo.value.storage.map((storage) => ({
      device: storage.device,
      mount_point: storage.mount_point,
      filesystem: storage.filesystem,
      usage: storage.usage,
      total: storage.total,
      used: storage.used,
      free: storage.free,
      type: storage.type,
    }));
  });

  // 设置当前连接ID
  const setCurrentConnectionId = (connectionId: string | null) => {
    currentConnectionId.value = connectionId;
  };

  // 从池中移除连接的数据
  const removeConnectionData = (connectionId: string) => {
    hardwareInfoPool.value.delete(connectionId);
    lastUpdatePool.value.delete(connectionId);
  };

  // 获取硬件信息
  const fetchHardwareInfo = async (targetConnectionId?: string) => {
    const connectionId = targetConnectionId || currentConnectionId.value;

    if (!connectionId) {
      error.value = "未指定连接ID";
      return;
    }

    // 检查连接状态
    try {
      const isConnected = await invoke<boolean>("check_connection_status", {
        connectionId,
      });
      if (!isConnected) {
        removeConnectionData(connectionId);
        if (connectionId === currentConnectionId.value) {
          error.value = "连接不存在";
        }
        return;
      }
    } catch (checkError) {
      console.warn("硬件Store - 检查连接状态失败:", checkError);
    }

    // 设置loading状态（只影响当前连接）
    const isCurrentConnection = connectionId === currentConnectionId.value;
    if (isCurrentConnection) {
      loading.value = true;
      error.value = null;
    }

    try {
      const data = await invoke<HardwareInfo>("get_hardware_info", {
        connectionId,
      });
      console.log("🚀 ~ fetchHardwareInfo ~ data:", data);

      // 存储到硬件信息池中
      hardwareInfoPool.value.set(connectionId, data);
      lastUpdatePool.value.set(connectionId, Date.now());

      if (isCurrentConnection) {
        error.value = null;
      }
    } catch (err) {
      const errorMessage =
        err instanceof Error ? err.message : "获取硬件信息失败";
      console.error("硬件Store - 获取硬件信息失败:", connectionId, err);

      // 处理连接不存在错误
      if (err instanceof Error && err.message.includes("连接不存在")) {
        removeConnectionData(connectionId);
        if (isCurrentConnection) {
          error.value = "连接不存在";
        }
        return;
      }

      if (isCurrentConnection) {
        error.value = errorMessage;
      }

      // 尝试错误恢复
      const recoverySuccess = await retryWithBackoff(connectionId);
      if (!recoverySuccess) {
        removeConnectionData(connectionId);
      }
    } finally {
      if (isCurrentConnection) {
        loading.value = false;
      }
    }
  };

  // 刷新硬件信息
  const refreshHardwareInfo = async (connectionId?: string) => {
    await fetchHardwareInfo(connectionId);
  };

  // 开始自动刷新
  const startAutoRefresh = (targetConnectionId?: string) => {
    const connectionId = targetConnectionId || currentConnectionId.value;

    if (!connectionId) {
      console.warn("硬件Store - 无法启动自动刷新：未指定连接ID");
      return;
    }

    // 如果该连接已有定时器，先清除
    const existingTimer = refreshTimers.value.get(connectionId);
    if (existingTimer) {
      clearInterval(existingTimer);
    }

    if (autoRefresh.value) {
      const timer = setInterval(async () => {
        // 在自动刷新前检查连接状态
        try {
          const isConnected = await invoke<boolean>("check_connection_status", {
            connectionId,
          });
          if (isConnected) {
            await fetchHardwareInfo(connectionId);
          } else {
            console.log(
              "硬件Store - 自动刷新时检测到连接不存在，停止自动刷新:",
              connectionId
            );
            stopAutoRefresh(connectionId);
          }
        } catch (error) {
          console.warn(
            "硬件Store - 自动刷新时检查连接状态失败:",
            connectionId,
            error
          );
          stopAutoRefresh(connectionId);
        }
      }, refreshInterval.value);

      refreshTimers.value.set(connectionId, timer);
      console.log("硬件Store - 启动自动刷新:", connectionId);
    }
  };

  // 停止自动刷新
  const stopAutoRefresh = (targetConnectionId?: string) => {
    const connectionId = targetConnectionId || currentConnectionId.value;

    if (connectionId) {
      // 停止特定连接的定时器
      const timer = refreshTimers.value.get(connectionId);
      if (timer) {
        clearInterval(timer);
        refreshTimers.value.delete(connectionId);
        console.log("硬件Store - 停止自动刷新:", connectionId);
      }
    } else {
      // 停止所有定时器
      refreshTimers.value.forEach((timer, connId) => {
        clearInterval(timer);
        console.log("硬件Store - 停止自动刷新:", connId);
      });
      refreshTimers.value.clear();
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
    // 如果有正在运行的定时器，需要重新启动所有定时器
    if (autoRefresh.value && refreshTimers.value.size > 0) {
      // 保存所有连接ID
      const connectionIds = Array.from(refreshTimers.value.keys());
      // 停止所有定时器
      stopAutoRefresh();
      // 为所有连接重新启动定时器
      connectionIds.forEach((connId) => {
        startAutoRefresh(connId);
      });
    }
  };

  // 错误恢复机制
  const retryWithBackoff = async (
    connectionId: string,
    maxRetries: number = 3
  ) => {
    let retryCount = 0;
    let delay = 1000; // 1秒

    while (retryCount < maxRetries) {
      try {
        console.log(
          `硬件Store - 重试获取硬件信息 (${retryCount + 1}/${maxRetries}):`,
          connectionId
        );
        const data = await invoke<HardwareInfo>("get_hardware_info", {
          connectionId,
        });

        console.log(
          "硬件Store - 重试成功，获取到硬件信息:",
          connectionId,
          data
        );

        // 存储到硬件信息池中
        hardwareInfoPool.value.set(connectionId, data);
        lastUpdatePool.value.set(connectionId, Date.now());

        if (connectionId === currentConnectionId.value) {
          error.value = null;
        }
        return true;
      } catch (err) {
        retryCount++;
        console.warn(`硬件Store - 重试 ${retryCount} 失败:`, err);

        if (retryCount < maxRetries) {
          console.log(`硬件Store - 等待 ${delay}ms 后重试`);
          await new Promise((resolve) => setTimeout(resolve, delay));
          delay *= 2; // 指数退避
        }
      }
    }

    console.error("硬件Store - 重试失败，清除硬件信息:", connectionId);
    hardwareInfoPool.value.delete(connectionId);
    lastUpdatePool.value.delete(connectionId);
    return false;
  };

  // 清除硬件信息
  const clearHardwareInfo = (targetConnectionId?: string) => {
    const connectionId = targetConnectionId || currentConnectionId.value;

    if (connectionId) {
      // 清除特定连接的硬件信息
      removeConnectionData(connectionId);
      stopAutoRefresh(connectionId);

      if (connectionId === currentConnectionId.value) {
        error.value = null;
      }
    } else {
      // 清除所有硬件信息
      hardwareInfoPool.value.clear();
      lastUpdatePool.value.clear();
      currentConnectionId.value = null;
      error.value = null;
      stopAutoRefresh();
    }
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
    setCurrentConnectionId,
    fetchHardwareInfo,
    refreshHardwareInfo,
    startAutoRefresh,
    stopAutoRefresh,
    setAutoRefresh,
    setRefreshInterval,
    clearHardwareInfo,
    retryWithBackoff,
    cleanup,
  };
});
