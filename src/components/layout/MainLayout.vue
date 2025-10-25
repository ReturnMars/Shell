<template>
  <n-layout has-sider class="h-screen">
    <!-- 侧边栏 -->
    <n-layout-sider
      :collapsed="sidebarCollapsed"
      :collapsed-width="0"
      :width="280"
      :native-scrollbar="false"
      class="wrap-layout-sidebar bg-white b-r-1px b-r-solid b-gray-200"
      @collapse="sidebarCollapsed = true"
      @expand="sidebarCollapsed = false"
    >
      <Sidebar />
    </n-layout-sider>

    <!-- 主内容区域 -->
    <div class="w-full flex flex-col h-screen overflow-auto">
      <!-- 标签页区域 -->
      <TabBar
        @toggle-sidebar="toggleSidebar"
        @settings-select="handleSettingsSelect"
      />

      <!-- 内容区域 -->
      <n-layout-content class="flex-1 overflow-auto">
        <!-- 终端视图 -->
        <TerminalView />
      </n-layout-content>

      <!-- 底部状态栏 -->
      <AppFooter />
    </div>
  </n-layout>

  <!-- 链接表单模态框 -->
  <ConnectionForm v-model:show="showConnectionForm" />
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Sidebar from "./Sidebar.vue";
import TabBar from "./TabBar.vue";
import AppFooter from "./AppFooter.vue";
import ConnectionForm from "../connection/ConnectionForm.vue";
import TerminalView from "../terminal/index.vue";
import { useMessage } from "naive-ui";
import { useConnectionStore } from "@/stores/connection/index";

// 响应式数据
const isConnected = ref(false);
const sidebarCollapsed = ref(false);
const connections = ref<any[]>([]);
const showConnectionForm = ref(false);

const message = useMessage();
const connectionStore = useConnectionStore();

// 设置菜单选项已移动到 TabBar 组件

// 方法
const toggleSidebar = () => {
  sidebarCollapsed.value = !sidebarCollapsed.value;
};

// showQuickConnect 方法已移除，功能在 TabBar 中处理

const handleSettingsSelect = async (key: string) => {
  console.log("选择设置项:", key);

  if (key === "clear_all") {
    await clearAllConnections();
  }
};

// 清理所有保存的链接
const clearAllConnections = async () => {
  try {
    // 先断开所有活跃连接
    await invoke("disconnect_all_ssh");

    // 清空所有标签页
    await connectionStore.closeAllTabs();
    console.log("已清空所有标签页");

    // 使用 store 清理所有链接
    await connectionStore.clearAllConnections();

    // 更新链接状态
    isConnected.value = false;
    connections.value = [];

    message.success("所有链接已清理完成");
    console.log("所有链接已清理完成");
  } catch (error) {
    console.error("清理链接失败:", error);
    message.error(`清理链接失败: ${error}`);
  }
};

// 页面加载时初始化
const initApp = async () => {
  console.log("初始化应用...");
  // 加载链接配置
  await connectionStore.fetchConnections();
  // 加载标签页
  await connectionStore.fetchTabs();
};
initApp();
</script>

<style scoped lang="scss">
/* 侧边栏包装器样式 */
.wrap-layout-sidebar {
  :deep(.n-scrollbar-content) {
    height: 100%;
  }
}
.wrap-layout-main-content {
  .n-layout {
    :deep(.n-layout-scroll-container) {
      display: flex;
      flex-direction: column;
    }
  }
}
/* 移动端适配 */
@media (max-width: 768px) {
  .mobile-menu-btn {
    display: block !important;
  }

  .header-center {
    display: none;
  }

  .app-sidebar {
    position: absolute;
    z-index: 1000;
    height: 100vh;
  }
}

/* 平板端适配 */
@media (max-width: 1024px) and (min-width: 769px) {
  .app-sidebar {
    width: 240px;
  }
}
</style>
