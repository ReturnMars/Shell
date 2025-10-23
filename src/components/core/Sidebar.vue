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

      <!-- 分组区域 -->
      <div class="mb-4 flex-1 overflow-y-auto">
        <div class="flex items-center justify-between mb-2">
          <span
            class="text-xs font-medium text-gray-600 uppercase tracking-wider"
            >分组</span
          >
          <n-button quaternary circle size="small">
            <template #icon>
              <n-icon>
                <PlusOutlined />
              </n-icon>
            </template>
          </n-button>
        </div>

        <div class="flex flex-col gap-1">
          <n-card hoverable size="small" class="cursor-pointer">
            <div class="flex items-center gap-3">
              <n-icon size="16" color="#666">
                <CloudServerOutlined />
              </n-icon>
              <span class="text-sm text-gray-800 flex-1">服务器</span>
              <n-tag size="small" type="info" round>3</n-tag>
            </div>
          </n-card>

          <n-card hoverable size="small" class="cursor-pointer">
            <div class="flex items-center gap-3">
              <n-icon size="16" color="#666">
                <FileTextOutlined />
              </n-icon>
              <span class="text-sm text-gray-800 flex-1">开发环境</span>
              <n-tag size="small" type="info" round>2</n-tag>
            </div>
          </n-card>
        </div>
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
  CloudServerOutlined,
  FileTextOutlined,
  SettingOutlined,
  QuestionCircleOutlined,
  CodeOutlined,
  PlusOutlined,
} from "@vicons/antd";
import ConnectionList from "../connection/ConnectionList.vue";

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
