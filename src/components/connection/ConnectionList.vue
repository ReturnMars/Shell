<template>
  <!-- 连接项列表 -->
  <div class="flex flex-col gap-1">
    <n-card
      v-for="connection in connections"
      :key="connection.id"
      :class="{ 'connection-active': connection.active }"
      hoverable
      class="cursor-pointer transition-all duration-200 group"
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
import { ref } from "vue";
import { ConnectionListItem } from "./type";
import { invoke } from "@tauri-apps/api/core";
import { EditOutlined, DeleteOutlined } from "@vicons/antd";

const connections = ref<ConnectionListItem[]>([]);
const selectConnection = (connection: ConnectionListItem) => {
  // 取消其他连接的激活状态

  console.log("选择连接:", connection.name);
};

const editConnection = (connection: ConnectionListItem) => {
  console.log("编辑连接:", connection.name);
};

const deleteConnection = (connection: ConnectionListItem) => {
  if (confirm(`确定要删除连接 "${connection.name}" 吗？`)) {
    const index = connections.value.findIndex(
      (conn) => conn.id === connection.id
    );
    if (index > -1) {
      connections.value.splice(index, 1);
    }
  }
};
const getConnections = async () => {
  connections.value = (await invoke(
    "get_saved_connections"
  )) as ConnectionListItem[];
};
getConnections();
</script>

<style scoped></style>
