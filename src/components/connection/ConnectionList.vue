<template>
  <!-- 连接项列表 -->
  <div class="flex flex-col gap-1">
    <n-card
      v-for="connection in connectionStore.connections"
      :key="connection.id"
      :class="[
        'cursor-pointer transition-all duration-200 group',
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
          size="small"
          statusOnly
        />

        <!-- 连接信息 -->
        <div class="flex-1 min-w-0">
          <div
            :class="[
              'connection-name font-medium text-[0.875rem] whitespace-nowrap overflow-hidden text-ellipsis transition-all duration-200',
              connection.id === connectionStore.currentConnection?.id
                ? 'text-green-600 font-bold text-[1rem]!'
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
          <n-button
            quaternary
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
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { ConnectionConfig, useConnectionStore } from "@/stores/connection";
import { EditOutlined, DeleteOutlined } from "@vicons/antd";

const connectionStore = useConnectionStore();

// 设置当前连接
const selectConnection = (connection: ConnectionConfig) => {
  connectionStore.setCurrentConnection(connection);
  console.log("设置当前连接:", connection.name);
};

// 编辑连接
const editConnection = (connection: ConnectionConfig) => {
  console.log("编辑连接:", connection.name);
  // TODO: 打开编辑表单
};

// 删除连接
const deleteConnection = async (connection: ConnectionConfig) => {
  if (confirm(`确定要删除连接 "${connection.name}" 吗？`)) {
    try {
      await connectionStore.deleteConnection(connection.id);
    } catch (error) {
      console.error("删除连接失败:", error);
    }
  }
};

// 组件挂载时加载连接
onMounted(() => {
  connectionStore.fetchConnections();
});
</script>

<style scoped></style>
