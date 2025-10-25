<template>
  <n-card
    size="small"
    class="storage-monitor"
    :content-style="{ padding: '8px' }"
    :header-style="{ padding: '8px 8px 4px 8px' }"
  >
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <n-icon size="16" color="#722ed1">
            <HddOutlined />
          </n-icon>
          <span class="text-sm font-medium">存储</span>
        </div>
        <n-tag size="small" type="info">
          {{ storageList.length }} 个设备
        </n-tag>
      </div>
    </template>

    <div class="storage-content">
      <div class="storage-grid">
        <div
          class="storage-drive"
          v-for="(storage, index) in storageList"
          :key="index"
        >
          <!-- 左侧：图标和盘符 -->
          <div class="drive-left">
            <div class="drive-icon">
              <n-icon size="16" :color="getStorageIconColor(storage.type)">
                <component :is="getStorageIcon(storage.type)" />
              </n-icon>
            </div>
            <div class="drive-label">{{ storage.mountPoint }}</div>
          </div>

          <!-- 中间：容量和使用率 -->
          <div class="drive-center">
            <div class="drive-usage">
              <div class="usage-percentage">{{ storage.usage }}%</div>
              <div class="usage-bar">
                <div
                  class="usage-fill"
                  :style="{
                    width: storage.usage + '%',
                    backgroundColor: getUsageColor(storage.usage),
                  }"
                ></div>
              </div>
            </div>
            <div class="drive-capacity">
              <span class="capacity-used">{{
                formatBytes(storage.used * 1024)
              }}</span>
              <span class="capacity-separator">/</span>
              <span class="capacity-total">{{
                formatBytes(storage.total * 1024)
              }}</span>
            </div>
          </div>

          <!-- 右侧：类型和性能 -->
          <div class="drive-right">
            <div class="drive-type">
              <!-- <n-tag size="tiny" :type="getStorageTypeTag(storage.type)">
                <span class="text-xs">
                  {{ storage.type.toUpperCase() }}
                </span>
              </n-tag> -->
              <span :class="getStorageTypeTag(storage.type)">{{
                storage.type.toUpperCase()
              }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { HddOutlined, CloudServerOutlined } from "@vicons/antd";
import type { StorageInfo } from "../types";

interface Props {
  storageList: StorageInfo[];
}

defineProps<Props>();

const getStorageIcon = (type: "ssd" | "hdd") => {
  return type === "ssd" ? CloudServerOutlined : HddOutlined;
};

const getStorageIconColor = (type: "ssd" | "hdd") => {
  return type === "ssd" ? "#1890ff" : "#722ed1";
};

const getStorageTypeTag = (type: "ssd" | "hdd") => {
  const baseClass = "text-10px font-xs";
  return type === "ssd"
    ? `${baseClass} text-green-500 `
    : `${baseClass} text-blue-500`;
};

const formatBytes = (bytes: number) => {
  if (bytes === 0) return "0 B";

  const gb = bytes / (1024 * 1024);
  if (gb >= 1) {
    return `${gb.toFixed(1)} GB`;
  }

  const mb = bytes / 1024;
  return `${mb.toFixed(0)} MB`;
};

const getUsageColor = (usage: number) => {
  if (usage >= 90) return "#ff4d4f"; // 红色 - 危险
  if (usage >= 70) return "#faad14"; // 橙色 - 警告
  if (usage >= 50) return "#52c41a"; // 绿色 - 正常
  return "#1890ff"; // 蓝色 - 良好
};
</script>

<style scoped>
.storage-monitor {
  margin-bottom: 2px;
}

.storage-content {
  overflow-y: auto;
}

.storage-grid {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 2px;
}

.storage-drive {
  display: flex;
  align-items: center;
  padding: 4px 6px;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  background: linear-gradient(135deg, #f8f9fa 0%, #ffffff 100%);
  transition: all 0.2s ease;
  cursor: pointer;
  position: relative;
  min-height: 36px;
}

.storage-drive:hover {
  border-color: #1890ff;
  box-shadow: 0 2px 8px rgba(24, 144, 255, 0.15);
  transform: translateY(-1px);
}

.drive-left {
  display: flex;
  align-items: center;
  gap: 6px;
  min-width: 50px;
}

.drive-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border-radius: 4px;
  background: rgba(24, 144, 255, 0.1);
}

.drive-label {
  font-size: 11px;
  font-weight: 700;
  color: #1890ff;
}

.drive-center {
  flex: 1;
  margin: 0 6px;
}

.drive-right {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 1px;
  min-width: 55px;
}

.drive-type {
  display: flex;
  justify-content: flex-end;
}

.drive-usage {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 2px;
}

.usage-percentage {
  font-size: 10px;
  font-weight: 700;
  color: #333;
  min-width: 28px;
}

.usage-bar {
  flex: 1;
  height: 2px;
  background-color: #f0f0f0;
  border-radius: 2px;
  overflow: hidden;
  position: relative;
}

.usage-fill {
  height: 100%;
  border-radius: 2px;
  transition: width 0.3s ease;
  position: relative;
}

.usage-fill::after {
  content: "";
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(
    90deg,
    transparent,
    rgba(255, 255, 255, 0.3),
    transparent
  );
  animation: shimmer 2s infinite;
}

@keyframes shimmer {
  0% {
    transform: translateX(-100%);
  }
  100% {
    transform: translateX(100%);
  }
}

.drive-capacity {
  display: flex;
  align-items: center;
  gap: 1px;
  font-size: 7px;
}

.capacity-used {
  color: #f97316;
  font-weight: 600;
}

.capacity-separator {
  color: #999;
}

.capacity-total {
  color: #666;
  font-weight: 500;
}

.drive-performance {
  display: flex;
  gap: 3px;
  justify-content: flex-end;
}

.perf-item {
  display: flex;
  align-items: center;
  gap: 1px;
  font-size: 6px;
}

.perf-icon {
  font-size: 8px;
}

.perf-value {
  color: #333;
  font-weight: 600;
}

/* 响应式设计 */
@media (max-width: 480px) {
  .storage-drive {
    min-height: 35px;
    padding: 4px 6px;
  }

  .drive-left {
    min-width: 40px;
    gap: 4px;
  }

  .drive-icon {
    width: 16px;
    height: 16px;
  }

  .drive-label {
    font-size: 11px;
  }

  .drive-center {
    margin: 0 6px;
  }

  .usage-percentage {
    font-size: 10px;
    min-width: 25px;
  }

  .drive-capacity {
    font-size: 7px;
  }

  .drive-right {
    min-width: 50px;
  }

  .perf-item {
    font-size: 6px;
  }
}

/* 自定义滚动条 */
.storage-content::-webkit-scrollbar {
  width: 3px;
}

.storage-content::-webkit-scrollbar-track {
  background: transparent;
}

.storage-content::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 2px;
}

.storage-content::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}
</style>
