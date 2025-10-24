<template>
  <div v-if="loading" class="loading-mask">
    <div class="terminal-loading">
      <div class="loading-content">
        <div class="loading-spinner">
          <LoadingOutlined />
        </div>
        <div class="loading-text">{{ text }}</div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { LoadingOutlined } from "@vicons/antd";

interface Props {
  loading: boolean;
  text?: string;
}
const { text = "正在连接终端..." } = defineProps<Props>();
</script>

<style scoped lang="scss">
.loading-mask {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(30, 30, 30, 0.8);
  z-index: 9999;
  display: flex;
  align-items: center;
  justify-content: center;
  font-family: "Courier New", "Monaco", "Consolas", monospace;

  .terminal-loading {
    display: flex;
    align-items: center;
    justify-content: center;

    .loading-content {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 16px;

      .loading-spinner {
        font-size: 32px;
        color: #00ff88;
        animation: spin 1s linear infinite;
      }

      .loading-text {
        color: #00ff88;
        font-size: 16px;
        font-weight: 500;
        text-align: center;
      }
    }
  }
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
