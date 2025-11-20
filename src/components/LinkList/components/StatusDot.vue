<template>
  <div class="status-dot" :class="`status-${status}`">
    <div class="dot-inner"></div>
    <div v-if="status === 'connecting'" class="dot-pulse"></div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  status: "info" | "success" | "connecting";
}

defineProps<Props>();
</script>

<style scoped lang="scss">
.status-dot {
  position: relative;
  width: 8px;
  height: 8px;
  flex-shrink: 0;
}

.dot-inner {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  transition: background-color 0.2s ease;
}

.status-info .dot-inner {
  background-color: rgb(156, 163, 175);
}

.status-success .dot-inner {
  background-color: rgb(34, 197, 94);
}

.status-connecting .dot-inner {
  background-color: rgb(59, 130, 246);
  animation: blink 1.1s ease-in-out infinite;
}

.dot-pulse {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: rgb(59, 130, 246);
  animation: pulse 1.5s ease-in-out infinite;
}

@keyframes blink {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.4;
  }
}

@keyframes pulse {
  0% {
    transform: translate(-50%, -50%) scale(1);
    opacity: 0.6;
  }
  50% {
    transform: translate(-50%, -50%) scale(2);
    opacity: 0.2;
  }
  100% {
    transform: translate(-50%, -50%) scale(2.5);
    opacity: 0;
  }
}
</style>

