<template>
  <n-layout has-sider class="layout-container">
    <n-layout-sider
      bordered
      show-trigger="bar"
      :collapsed-width="siderConfig.collapsedWidth"
      :width="siderConfig.width"
      :native-scrollbar="false"
      :show-collapsed-content="false"
      position="static"
    >
      <div class="sider-content">
        <!-- 链接管理 -->
        <div class="sider-top">
          <LinkList />
        </div>
        <!-- 硬件信息 -->
        <div class="sider-bottom">
          <HardwareInformation />
        </div>
      </div>
    </n-layout-sider>
    <n-layout-content>
      <div class="w-full h-full flex flex-col">
        <div class="flex-1 flex-shrink-0">
          <router-view></router-view>
        </div>
        <n-layout-footer class="flex-shrink-0">
          <div class="px-4 py-1 text-right">Footer Footer Footer</div>
        </n-layout-footer>
      </div>
    </n-layout-content>
  </n-layout>
</template>

<script setup lang="ts">
import HardwareInformation from "../../components/HardwareInformation/index.vue";
import LinkList from "../../components/LinkList/index.vue";

const siderConfig = {
  width: "clamp(180px, 18vw, 240px)",
  collapsedWidth: 0,
};
</script>
<style scoped lang="scss">
.layout-container {
  width: 100%;
  height: 100%;
  :deep(.n-scrollbar-container) {
    .n-scrollbar-content {
      height: 100%;
    }
    height: 100%;
  }
}
.sider-content {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;

  .sider-top {
    flex: 1 1 0;
    min-height: 0;
    overflow: auto;
  }

  .sider-bottom {
    flex: 0 0 auto;
    overflow: visible;
  }
}

</style>
