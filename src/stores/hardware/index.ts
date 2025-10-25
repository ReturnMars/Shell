// 硬件信息状态管理
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { 
  HardwareInfo, 
  StorageInfo,
  NetworkInfo 
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
      name: storage.name,
      usage: storage.usage,
      total: storage.total,
      used: storage.used,
      free: storage.free
    }));
  });

  // 获取硬件信息
  const fetchHardwareInfo = async (connectionId?: string) => {
    if (!connectionId) {
      error.value = '未指定连接ID';
      return;
    }

    loading.value = true;
    error.value = null;

    try {
      // 这里将来会调用 Tauri 命令获取真实硬件信息
      // 现在使用模拟数据
      const mockData = generateMockHardwareInfo();
      hardwareInfo.value = mockData;
      lastUpdate.value = Date.now();
    } catch (err) {
      error.value = err instanceof Error ? err.message : '获取硬件信息失败';
      console.error('获取硬件信息失败:', err);
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
      refreshTimer.value = setInterval(() => {
        fetchHardwareInfo(connectionId);
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

  // 清除硬件信息
  const clearHardwareInfo = () => {
    hardwareInfo.value = null;
    lastUpdate.value = null;
    error.value = null;
    stopAutoRefresh();
  };

  // 生成模拟硬件信息
  const generateMockHardwareInfo = (): HardwareInfo => {
    const now = Date.now();
    
    // 生成随机但合理的CPU使用率
    const cpuUsage = Math.random() * 80 + 10; // 10-90%
    
    // 生成随机但合理的内存使用率
    const memoryUsage = Math.random() * 70 + 20; // 20-90%
    const totalMemory = 8192; // 8GB
    const usedMemory = Math.floor((totalMemory * memoryUsage) / 100);
    
    // 生成硬盘信息
    const storage: StorageInfo[] = [
      {
        name: 'System SSD',
        total: 500,
        used: Math.floor(Math.random() * 300 + 100),
        free: 0,
        usage: 0,
        type: 'ssd',
        mountPoint: 'C:',
        readSpeed: Math.random() * 500 + 200,
        writeSpeed: Math.random() * 300 + 100
      },
      {
        name: 'Data HDD',
        total: 1000,
        used: Math.floor(Math.random() * 600 + 200),
        free: 0,
        usage: 0,
        type: 'hdd',
        mountPoint: 'D:',
        readSpeed: Math.random() * 150 + 50,
        writeSpeed: Math.random() * 100 + 30
      },
      {
        name: 'Backup HDD',
        total: 2000,
        used: Math.floor(Math.random() * 800 + 400),
        free: 0,
        usage: 0,
        type: 'hdd',
        mountPoint: 'E:',
        readSpeed: Math.random() * 120 + 40,
        writeSpeed: Math.random() * 80 + 25
      }
    ];

    // 计算硬盘使用率
    storage.forEach(disk => {
      disk.free = disk.total - disk.used;
      disk.usage = Math.floor((disk.used / disk.total) * 100);
    });

    // 生成网络信息
    const network: NetworkInfo = {
      interfaces: [
        {
          name: 'eth0',
          rx: Math.random() * 1000 + 100,
          tx: Math.random() * 800 + 50,
          rxSpeed: Math.random() * 100 + 10,
          txSpeed: Math.random() * 80 + 5,
          status: 'up'
        },
        {
          name: 'wlan0',
          rx: Math.random() * 500 + 50,
          tx: Math.random() * 400 + 25,
          rxSpeed: Math.random() * 50 + 5,
          txSpeed: Math.random() * 40 + 3,
          status: 'up'
        }
      ],
      totalRx: Math.random() * 10000 + 1000,
      totalTx: Math.random() * 8000 + 500,
      rxSpeed: Math.random() * 100 + 10,
      txSpeed: Math.random() * 80 + 5
    };

    return {
      cpu: {
        usage: Math.floor(cpuUsage),
        cores: 8,
        model: 'Intel Core i7-12700K',
        temperature: Math.floor(Math.random() * 20 + 40), // 40-60°C
        frequency: Math.floor(Math.random() * 1000 + 3000) // 3-4GHz
      },
      memory: {
        total: totalMemory,
        used: usedMemory,
        free: totalMemory - usedMemory,
        usage: Math.floor(memoryUsage),
        swap: {
          total: 2048, // 2GB
          used: Math.floor(Math.random() * 500 + 100),
          free: 0,
          usage: 0
        }
      },
      storage,
      network,
      timestamp: now
    };
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
    cleanup
  };
});
