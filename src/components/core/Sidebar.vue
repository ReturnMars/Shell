<template>
  <div class="h-full flex flex-col">
    <!-- 侧边栏头部 -->
    <div class="p-4 border-b border-gray-200">
      <div class="flex items-center gap-2 text-lg font-semibold text-gray-800">
        <n-icon size="24" color="#18a058">
          <CodeOutlined />
        </n-icon>
        <span>TerminalMars</span>
      </div>
    </div>

    <!-- 连接列表 -->
    <div class="flex flex-col flex-1 overflow-y-auto p-2">
      <!-- 快速连接区域 -->
      <div class="mb-4 max-h-[70%] overflow-y-auto relative">
        <div
          class="flex items-center justify-between mb-2 sticky top-0 bg-white z-10"
        >
          <span
            class="text-xs font-medium text-gray-600 uppercase tracking-wider"
            >快速连接</span
          >
          <n-button quaternary circle size="small" @click="showAddConnection">
            <template #icon>
              <n-icon>
                <PlusOutlined />
              </n-icon>
            </template>
          </n-button>
        </div>

        <!-- 连接项列表 -->
        <div class="flex flex-col gap-1">
          <n-card
            v-for="connection in connections"
            :key="connection.id"
            :class="{ 'connection-active': connection.active }"
            hoverable
            class="cursor-pointer transition-all duration-200"
            :style="{
              '--n-padding-left': '14px',
              '--n-padding-right': '14px',
              '--n-padding-top': '8px',
              '--n-padding-bottom': '8px',
            }"
            @click="selectConnection(connection)"
          >
            <div class="flex items-center gap-3">
              <!-- 连接状态指示器 -->
              <ConnectionStatus
                :connected="connection.connected"
                size="small"
                statusOnly
              />

              <!-- 连接信息 -->
              <div class="flex-1 min-w-0">
                <div
                  class="font-medium text-sm text-gray-800 whitespace-nowrap overflow-hidden text-ellipsis"
                >
                  {{ connection.name }}
                </div>
                <div
                  class="text-xs text-gray-600 whitespace-nowrap overflow-hidden text-ellipsis mt-0.5"
                >
                  {{ connection.host }}:{{ connection.port }}
                </div>
              </div>

              <!-- 操作按钮 -->
              <div
                class="flex items-center gap-1 opacity-0 transition-opacity duration-200 connection-actions"
              >
                <n-button
                  quaternary
                  circle
                  size="tiny"
                  @click.stop="editConnection(connection)"
                >
                  <template #icon>
                    <n-icon>
                      <EditOutlined />
                    </n-icon>
                  </template>
                </n-button>
                <n-button
                  quaternary
                  circle
                  size="tiny"
                  @click.stop="deleteConnection(connection)"
                >
                  <template #icon>
                    <n-icon>
                      <DeleteOutlined />
                    </n-icon>
                  </template>
                </n-button>
              </div>
            </div>
          </n-card>
        </div>
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
    <div class="mt-auto border-t border-gray-200 bg-gray-50">
      <div class="p-3">
        <div class="flex flex-col gap-1">
          <n-button quaternary block size="small" @click="showSettings">
            <template #icon>
              <n-icon>
                <SettingOutlined />
              </n-icon>
            </template>
            设置
          </n-button>

          <n-button quaternary block size="small" @click="showHelp">
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
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import {
  PlusOutlined,
  EditOutlined,
  DeleteOutlined,
  CloudServerOutlined,
  FileTextOutlined,
  SettingOutlined,
  QuestionCircleOutlined,
  CodeOutlined,
} from "@vicons/antd";
import ConnectionStatus from "./ConnectionStatus.vue";

// 连接数据接口
interface Connection {
  id: string;
  name: string;
  host: string;
  port: number;
  connected: boolean;
  active: boolean;
}

// 响应式数据
const connections = ref<Connection[]>([
  {
    id: "1",
    name: "生产服务器",
    host: "192.168.1.100",
    port: 22,
    connected: false,
    active: false,
  },
  {
    id: "2",
    name: "开发服务器",
    host: "192.168.1.101",
    port: 22,
    connected: true,
    active: true,
  },
  {
    id: "3",
    name: "测试服务器",
    host: "192.168.1.102",
    port: 22,
    connected: false,
    active: false,
  },
]);

// 方法
const selectConnection = (connection: Connection) => {
  // 取消其他连接的激活状态
  connections.value.forEach((conn) => {
    conn.active = conn.id === connection.id;
  });
  console.log("选择连接:", connection.name);
};

const editConnection = (connection: Connection) => {
  console.log("编辑连接:", connection.name);
};

const deleteConnection = (connection: Connection) => {
  if (confirm(`确定要删除连接 "${connection.name}" 吗？`)) {
    const index = connections.value.findIndex(
      (conn) => conn.id === connection.id
    );
    if (index > -1) {
      connections.value.splice(index, 1);
    }
  }
};

const showAddConnection = () => {
  console.log("显示添加连接对话框");
};

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
