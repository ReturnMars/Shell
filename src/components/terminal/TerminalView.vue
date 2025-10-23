<template>
  <div class="terminal-view">
    <!-- 调试信息 -->
    <!-- <div class="debug-info" v-if="showDebug">
      <n-alert type="info" title="调试信息">
        <p>连接数: {{ connectionStore.connections.length }}</p>
        <p>标签页数: {{ connectionStore.tabs.length }}</p>
        <p>活动标签页: {{ connectionStore.activeTabId }}</p>
        <p>活动连接: {{ activeConnection?.name || "无" }}</p>
        <p>连接状态: {{ activeConnection?.connected ? "已连接" : "未连接" }}</p>
        <p>终端状态: {{ isLoading ? "加载中" : "就绪" }}</p>
        <p>错误: {{ error || "无" }}</p>
      </n-alert>
    </div> -->

    <!-- 如果没有连接，显示提示 -->
    <div v-if="!activeConnection" class="no-connections">
      <n-empty description="请选择一个SSH连接">
        <template #extra>
          <n-button type="primary" @click="showConnectionForm = true">
            创建连接
          </n-button>
        </template>
      </n-empty>
    </div>

    <!-- 自定义终端 -->
    <keep-alive>
      <CustomTerminal
        v-if="activeConnection"
        :key="`terminal-${activeConnection.id}-${activeConnection.connected}`"
        :connection-id="activeConnection.id"
        :title="activeConnection.name"
        @connected="handleTerminalConnected"
        @disconnected="handleTerminalDisconnected"
        @error="handleTerminalError"
        @data="handleTerminalData"
      />
    </keep-alive>
    <!-- 连接表单 -->
    <ConnectionForm v-model:show="showConnectionForm" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import CustomTerminal from "./CustomTerminal.vue";
import ConnectionForm from "@/components/connection/ConnectionForm.vue";
import { useConnectionStore } from "@/stores/connection";

// 状态管理
const connectionStore = useConnectionStore();
const showConnectionForm = ref(false);

// 计算属性
const activeConnection = computed(() => {
  const activeTab = connectionStore.activeTab;
  if (activeTab) {
    return connectionStore.connections.find(
      (c) => c.id === activeTab.connection_id
    );
  }
  return null;
});

// 事件处理函数
const handleTerminalConnected = (sessionId: string) => {
  console.log("终端已连接:", sessionId);
};

const handleTerminalDisconnected = () => {
  console.log("终端已断开连接");
};

const handleTerminalError = (error: string) => {
  console.error("终端错误:", error);
};

const handleTerminalData = (data: string) => {
  console.log("终端数据:", data);
};
</script>

<style scoped>
.terminal-view {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #0d1117;
  gap: 16px;
}

.debug-info {
  margin-bottom: 16px;
}

.no-connections,
.not-connected {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #161b22;
  border-radius: 8px;
  border: 1px solid rgba(240, 246, 252, 0.1);
}

.terminal-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
}
</style>
