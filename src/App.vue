<template>
  <n-config-provider class="app-container">
    <n-message-provider>
      <router-view></router-view>
    </n-message-provider>
  </n-config-provider>
</template>
<script setup lang="ts"></script>

<style scoped lang="scss">
.app-container {
  width: 100%;
  height: 100%;
}
</style>
