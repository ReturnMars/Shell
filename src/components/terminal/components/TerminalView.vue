<template>
  <div class="h-full">
    <div class="terminal-container">
      <div class="terminal-title">
        <div class="terminal-title-text">
          <span>{{ currentConnection?.name }}</span>
        </div>
        <div class="terminal-title-status">
          <!-- <n-tag text size="small" round :type="isConnected ? 'success' : 'error'">
            <template #icon>
              <ConnectionStatus
                class="ml-0.5"
                :connected="isConnected"
                status-only
                :loading="connectionStore.loading"
              />
            </template>

            <span class="mr-0.5">{{ isConnected ? "已连接" : "未连接" }}</span>
          </n-tag> -->
          <div class="flex items-center gap-2">
            <ConnectionStatus
              class="ml-0.5"
              :connected="isConnected"
              status-only
              :loading="connectionStore.loading"
            />
            <span
              class="text-sm"
              :class="isConnected ? 'text-green-600' : 'text-red-600'"
              >{{ isConnected ? "已连接" : "未连接" }}</span
            >
          </div>
        </div>
      </div>
      <div class="terminal-content">
        <keep-alive>
          <TerminalCore v-if="currentConnection" :key="currentConnection.id" />
        </keep-alive>
      </div>
      <div class="terminal-footer">
        <div>
          <template v-if="isConnected">
            <span>{{ currentConnection?.username }}</span>
            <span
              >@{{
                `${currentConnection?.host}:${currentConnection?.port}`
              }}</span
            >
          </template>
        </div>
        <div class="flex items-center gap-1">
          <div class="mr-2 flex items-center gap-1 line-height-1">
            <n-time class="line-height-1" :time="timestamp" />
          </div>
          <div class="text-gray-500 text-xs">
            <span v-if="isConnected">ID: {{ currentConnection?.id }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useConnectionStore } from "@/stores/connection/index";
import { useTimestamp } from "@vueuse/core";
import TerminalCore from "./TerminalCore.vue";
const connectionStore = useConnectionStore();
const currentConnection = computed(() => connectionStore.currentConnection);
const isConnected = computed(() => currentConnection.value?.connected);
const timestamp = useTimestamp();
</script>
<style scoped lang="scss">
.terminal-container {
  --border-color: rgba(255, 255, 255, 0.3);
  --border: 1px solid var(--border-color);
  --padding: 0.5rem 1rem;
  height: 100%;
  display: flex;
  flex-flow: column;
  .terminal-content {
    flex: 1 0;
    width: 100%;
    height: 100%;
  }
  .terminal-footer {
    padding: var(--padding);
    border-top: var(--border);
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 0.875rem;
    font-weight: 500;
    color: #fff;
    background-color: transparent;
  }
}
.terminal-title {
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: var(--padding);
  border-bottom: var(--border);
  .terminal-title-status-tag {
    width: fit-content;
    display: flex;
    align-items: center;
    gap: 4px;
    font-size: 0.875rem;
    font-weight: 500;
    color: #fff;
    background-color: transparent;
    padding: 4px 8px;
    border-radius: 0.25rem;
    border: 1px solid #fff;
    &.terminal-title-status-tag-connected {
      border-color: #00ff88;
      color: #00ff88;
    }
    &.terminal-title-status-tag-disconnected {
      border-color: #ff0000;
      color: #ff0000;
    }
  }
}
</style>
