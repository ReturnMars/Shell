<template>
  <div class="h-full flex flex-col">
    <!-- 侧边栏头部 -->
    <div
      class="border-b border-gray-200 h-[45px] flex items-center justify-start pl-3"
    >
      <div class="flex items-center gap-2 text-lg font-semibold text-gray-800">
        <n-icon size="24" color="#18a058">
          <CodeOutlined />
        </n-icon>
        <span class="flex h-[1em]">ShellMars</span>
      </div>
    </div>

    <!-- 连接列表 -->
    <div class="flex flex-col flex-1 overflow-y-auto p-2">
      <!-- 链接管理区域 -->
      <div class="mb-4 max-h-[70%] overflow-y-auto relative">
        <ConnectionList />
      </div>

      <!-- 硬件监控区域 -->
      <div class="flex-1 overflow-y-auto">
        <HardwareMonitor />
      </div>
    </div>

    <!-- 侧边栏底部固定区域 -->
    <div class="mt-auto border-t h-9 border-gray-200 bg-gray-50">
      <div class="flex gap-1 p-[6px]">
        <n-button
          class="flex-1"
          quaternary
          block
          size="tiny"
          @click="showSettings"
        >
          <template #icon>
            <n-icon>
              <SettingOutlined />
            </n-icon>
          </template>
          设置
        </n-button>
        <n-button class="flex-1" quaternary block size="tiny" @click="showHelp">
          <template #icon>
            <n-icon>
              <QuestionCircleOutlined />
            </n-icon>
          </template>
          帮助
        </n-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  SettingOutlined,
  QuestionCircleOutlined,
  CodeOutlined,
} from "@vicons/antd";
import ConnectionList from "../connection/ConnectionList.vue";
import HardwareMonitor from "../hardware/index.vue";

// 方法

const showSettings = () => {
  console.log("显示设置对话框");
};

const showHelp = () => {
  console.log("显示帮助对话框");
};
</script>

<style scoped>
/* 连接项悬停效果 */
.n-card:hover .connection-actions {
  opacity: 1;
}

/* 激活的连接项 */
.connection-active {
  border-color: #18a058 !important;
  background-color: #f6ffed !important;
}

/* 自定义滚动条 */
::-webkit-scrollbar {
  width: 4px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 2px;
}

::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}
</style>
