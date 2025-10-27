<template>
  <!-- 连接项列表 -->
  <div class="flex flex-col h-full">
    <!-- 标题栏 -->
    <div
      class="flex items-center justify-between sticky top-0 p-2 bg-white z-10 transition-all duration-200"
      :class="{ scrolled: isScrolled }"
    >
      <div>
        <span
          class="text-medium font-medium text-gray-600 uppercase tracking-wider"
        >
          链接管理
        </span>
        <!-- 统计信息提示 -->
        <n-tooltip trigger="hover" placement="right">
          <template #trigger>
            <span class="text-xs text-gray-400 cursor-help">
              {{ connectionStore.connectedCount }}/{{
                connectionStore.connectionCount
              }}
            </span>
          </template>
          <div class="text-xs">
            <div>总链接数: {{ connectionStore.connectionCount }}</div>
            <div>已连接: {{ connectionStore.connectedCount }}</div>
          </div>
        </n-tooltip>
      </div>
      <div class="flex items-center gap-1">
        <!-- 添加按钮 -->
        <ConnectionForm>
          <template #trigger>
            <n-button quaternary circle size="tiny">
              <template #icon>
                <n-icon>
                  <PlusOutlined />
                </n-icon>
              </template>
            </n-button>
          </template>
        </ConnectionForm>
        <!-- 断开所有链接按钮 -->
        <n-button
          quaternary
          circle
          size="tiny"
          type="error"
          :disabled="!connectionStore.connectedCount"
          @click="disconnectAll"
        >
          <template #icon>
            <n-icon>
              <DisconnectOutlined />
            </n-icon>
          </template>
        </n-button>
      </div>
    </div>
    <div class="connection-content" ref="contentRef" @scroll="handleScroll">
      <n-card
        v-for="connection in connectionStore.connections"
        :key="connection.id"
        :class="[
          'cursor-pointer transition-all duration-200 group connection-item',
          connection.id === connectionStore.currentConnection?.id
            ? 'border-green-500! bg-green-50 border-1! border-solid! bg-green-50!'
            : '',
        ]"
        hoverable
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
            :loading="
              connectionStore.loading &&
              connectionStore.currentConnection?.id === connection.id
            "
            size="small"
            statusOnly
          />

          <!-- 连接信息 -->
          <div class="flex-1 min-w-0">
            <div
              :class="[
                'connection-name font-medium text-[1rem] whitespace-nowrap overflow-hidden text-ellipsis transition-all duration-200',
                connection.id === connectionStore.currentConnection?.id
                  ? 'text-green-600 font-bold'
                  : 'text-gray-800',
              ]"
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
            class="flex items-center gap-1 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
          >
            <!-- 断开连接按钮 -->
            <n-button
              v-if="connection.connected"
              quaternary
              size="tiny"
              type="error"
              @click.stop="disconnectConnection(connection)"
              :loading="
                connectionStore.loading &&
                connectionStore.currentConnection?.id === connection.id
              "
            >
              <template #icon>
                <n-icon>
                  <DisconnectOutlined />
                </n-icon>
              </template>
            </n-button>

            <!-- 编辑按钮 -->
            <ConnectionForm :connection="connection">
              <template #trigger>
                <n-button quaternary size="tiny">
                  <template #icon>
                    <n-icon>
                      <EditOutlined />
                    </n-icon>
                  </template>
                </n-button>
              </template>
            </ConnectionForm>

            <!-- 删除按钮 -->
            <div @click.stop>
              <n-popconfirm
                placement="right"
                @positive-click="deleteConnection(connection)"
              >
                <template #trigger>
                  <n-button quaternary size="tiny">
                    <template #icon>
                      <n-icon>
                        <DeleteOutlined />
                      </n-icon>
                    </template>
                  </n-button>
                </template>
                <div class="leading-6">
                  <p>确定要删除链接 "{{ connection.name }}" 吗？</p>
                  <p>删除后将无法恢复!</p>
                </div>
              </n-popconfirm>
            </div>
          </div>
        </div>
      </n-card>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from "vue";
import { ConnectionConfig, useConnectionStore } from "@/stores/connection";
import {
  EditOutlined,
  DeleteOutlined,
  DisconnectOutlined,
  PlusOutlined,
} from "@vicons/antd";
import { useMessage, NTooltip } from "naive-ui";

const connectionStore = useConnectionStore();
const message = useMessage();

// 滚动状态
const contentRef = ref<HTMLElement>();
const isScrolled = ref(false);

// 处理滚动事件
const handleScroll = () => {
  if (contentRef.value) {
    isScrolled.value = contentRef.value.scrollTop > 0;
  }
};

// 设置当前选中的链接
const selectConnection = async (connection: ConnectionConfig) => {
  // 检查该链接对应的标签页是否存在
  const existingTab = connectionStore.tabs.find(
    (tab) => tab.connection_id === connection.id
  );

  if (existingTab) {
    // 如果标签页已存在，先激活标签页，再设置当前链接
    await connectionStore.setActiveTab(existingTab.id);
    await connectionStore.setCurrentConnection(connection);
    console.log("激活已存在的标签页:", connection.name);
  } else {
    // 如果标签页不存在，先设置当前链接，再创建新标签页
    await connectionStore.setCurrentConnection(connection);
    try {
      await connectionStore.addTab(connection);
      console.log("创建新标签页:", connection.name);
    } catch (error) {
      console.error("创建标签页失败:", error);
    }
  }

  // 如果连接未建立，尝试建立连接
  if (!connection.connected) {
    console.log("连接未建立，尝试建立连接:", connection.name);
    try {
      const result = await connectionStore.connect(connection);
      if (result.success) {
        console.log("连接建立成功:", connection.name);
        message.success(`连接建立成功: ${connection.name}`);
      } else {
        console.warn("连接建立失败:", result.message);
        message.warning(`连接建立失败: ${result.message}`);
      }
    } catch (error) {
      console.error("连接建立出错:", error);
      message.error(`连接建立出错: ${error}`);
    }
  }

  console.log("设置当前选中的链接:", connection.name);
};

// 断开单个连接
const disconnectConnection = async (connection: ConnectionConfig) => {
  try {
    await connectionStore.disconnect(connection.id);
    message.success(`已断开连接: ${connection.name}`);
  } catch (error) {
    console.error("断开连接失败:", error);
    message.error(`断开连接失败: ${error}`);
  }
};

// 断开所有连接
const disconnectAll = async () => {
  try {
    await connectionStore.disconnectAll();
    message.success("已断开所有连接");
  } catch (error) {
    console.error("断开所有连接失败:", error);
    message.error(`断开所有连接失败: ${error}`);
  }
};

// 删除链接
const deleteConnection = async (connection: ConnectionConfig) => {
  try {
    // 先删除对应的标签页
    const existingTab = connectionStore.tabs.find(
      (tab) => tab.connection_id === connection.id
    );
    if (existingTab) {
      await connectionStore.removeTab(existingTab.id);
      console.log("已删除对应的标签页:", connection.name);
    }

    // 再删除链接
    await connectionStore.deleteConnection(connection.id);
    console.log("已删除链接:", connection.name);
  } catch (error) {
    console.error("删除链接失败:", error);
  }
};

// 组件挂载时加载连接
onMounted(() => {
  connectionStore.fetchConnections();
});
</script>

<style scoped>
.connection-content {
  max-height: 400px;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 4px;
  padding: 4px 8px;
}

.connection-item {
  margin-bottom: 0;
}

.connection-content::-webkit-scrollbar {
  width: 3px;
}

.connection-content::-webkit-scrollbar-track {
  background: transparent;
}

.connection-content::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 2px;
}

.connection-content::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}

/* 滚动时的阴影效果 */
.flex.items-center.justify-between.sticky.top-0.bg-white.z-10.scrolled {
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  background-color: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(8px);
}
</style>
