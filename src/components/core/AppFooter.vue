<template>
  <n-layout-footer
    class="bg-white border-t border-gray-200 h-9 shrink-0"
    bordered
  >
    <div
      class="flex items-center justify-between h-full p-[10px] text-xs text-gray-600"
    >
      <div class="flex items-center gap-4">
        <span>就绪</span>
        <span>|</span>
        <span>已连接数: {{ connectionStore.connectedCount }}</span>
      </div>
      <div class="flex items-center">
        <span>ShellMars v1.0.0</span>
      </div>
    </div>
  </n-layout-footer>
</template>

<script setup lang="ts">
import { useConnectionStore } from "@/stores/connection";
import { NLayoutFooter } from "naive-ui";

const connectionStore = useConnectionStore();
</script>

<style scoped></style>
