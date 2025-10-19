<template>
  <n-layout has-sider class="h-screen">
    <!-- 侧边栏 -->
    <n-layout-sider
      :collapsed="sidebarCollapsed"
      :collapsed-width="0"
      :width="280"
      :native-scrollbar="false"
      class="wrap-layout-sidebar bg-white border-r border-gray-200"
      @collapse="sidebarCollapsed = true"
      @expand="sidebarCollapsed = false"
    >
      <Sidebar />
    </n-layout-sider>

    <!-- 主内容区域 -->
    <div class="w-full flex flex-col h-screen">
      <!-- 顶部标题栏 -->
      <n-layout-header
        class="bg-white border-b border-gray-200 shadow-sm h-15 flex-shrink-0"
        bordered
      >
        <div class="flex items-center justify-between h-full px-4">
          <!-- 左侧：应用标题和移动端菜单 -->
          <div class="flex items-center gap-3">
            <!-- 移动端菜单按钮 -->
            <n-button
              quaternary
              circle
              @click="toggleSidebar"
              class="hidden md:block"
            >
              <template #icon>
                <n-icon>
                  <MenuOutlined />
                </n-icon>
              </template>
            </n-button>
          </div>

          <!-- 中间：连接状态 -->
          <div class="flex items-center gap-2">
            <ConnectionStatus :connected="isConnected" />
          </div>

          <!-- 右侧：操作按钮 -->
          <div class="flex items-center gap-2">
            <n-button type="primary" size="small" @click="showQuickConnect">
              <template #icon>
                <n-icon>
                  <PlusOutlined />
                </n-icon>
              </template>
              快速连接
            </n-button>

            <n-dropdown
              :options="settingsOptions"
              @select="handleSettingsSelect"
            >
              <n-button quaternary circle>
                <template #icon>
                  <n-icon>
                    <SettingOutlined />
                  </n-icon>
                </template>
              </n-button>
            </n-dropdown>
          </div>
        </div>
      </n-layout-header>

      <!-- 标签页区域 -->
      <TabBar />

      <!-- 内容区域 -->
      <n-layout-content class="flex-1 bg-gray-50 p-4 overflow-auto">
        <slot name="main-content"></slot>
      </n-layout-content>

      <!-- 底部状态栏 -->
      <n-layout-footer
        class="bg-white border-t border-gray-200 h-10 flex-shrink-0"
        bordered
      >
        <div
          class="flex items-center justify-between h-full px-4 text-xs text-gray-600"
        >
          <div class="flex items-center gap-4">
            <span>就绪</span>
            <span>|</span>
            <span>连接数: 0</span>
          </div>
          <div class="flex items-center">
            <span>TerminalMars v1.0.0</span>
          </div>
        </div>
      </n-layout-footer>
    </div>
  </n-layout>
</template>

<script setup lang="ts">
import { h, ref } from "vue";
import { MenuOutlined, PlusOutlined, SettingOutlined } from "@vicons/antd";
import Sidebar from "./Sidebar.vue";
import TabBar from "./TabBar.vue";
import ConnectionStatus from "./ConnectionStatus.vue";

// 响应式数据
const isConnected = ref(false);
const sidebarCollapsed = ref(false);

// 设置菜单选项
const settingsOptions = [
  {
    label: "主题设置",
    key: "theme",
    icon: () => h("n-icon", null, { default: () => h(SettingOutlined) }),
  },
  {
    label: "连接设置",
    key: "connection",
    icon: () => h("n-icon", null, { default: () => h(SettingOutlined) }),
  },
  {
    label: "关于",
    key: "about",
    icon: () => h("n-icon", null, { default: () => h(SettingOutlined) }),
  },
];

// 方法
const toggleSidebar = () => {
  sidebarCollapsed.value = !sidebarCollapsed.value;
};

const showQuickConnect = () => {
  console.log("显示快速连接对话框");
};

const handleSettingsSelect = (key: string) => {
  console.log("选择设置项:", key);
};
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
