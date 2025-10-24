<template>
  <div class="terminal-wrapper h-full">
    <TerminalLoading :loading="connectionLoading" />
    <TerminalView v-if="isConnected" />
  </div>
</template>
<script setup lang="ts">
import { computed } from "vue";
import TerminalView from "./components/TerminalView.vue";
import TerminalLoading from "./components/TerminalLoading.vue";
import { useConnectionStore } from "@/stores/connection/index";

const connectionStore = useConnectionStore();
const connectionLoading = computed(() => connectionStore.loading);
const isConnected = computed(() => connectionStore.currentConnection?.connected);
</script>
<style scoped lang="scss">
.terminal-wrapper {
  position: relative;
  background: #1a1a1a;
}
</style>
