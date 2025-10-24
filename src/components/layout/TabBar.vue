<template>
  <div class="w-full tab-bar-container">
    <!-- 左侧：侧边栏切换按钮 -->
    <div class="prefix-container">
      <n-button
        quaternary
        circle
        size="small"
        @click="toggleSidebar"
        class="mr-1"
      >
        <template #icon>
          <n-icon><MenuOutlined /></n-icon>
        </template>
      </n-button>
    </div>

    <!-- 中间：标签页区域 -->
    <div class="tabs-container">
      <div class="tabs-list">
        <!-- 标签页列表 -->
        <div
          v-for="(tab, index) in connectionStore.tabs"
          :key="tab.id"
          :class="['tab-item', { 'tab-active': tab.active }]"
          @click="selectTab(tab)"
        >
          <!-- 连接状态指示器 -->
          <div class="flex items-center gap-1.5">
            <ConnectionStatus
              :connected="getTabConnection(tab)?.connected === true"
              size="small"
              :showText="false"
              :statusOnly="true"
            />
            <span class="text-[1rem] font-medium">{{ index + 1 }}</span>
          </div>
          <!-- 标签页标题 -->
          <span
            class="text-[1rem] font-medium text-gray-800 mr-1.5 whitespace-nowrap overflow-hidden text-ellipsis"
          >
            {{ tab.title }}
          </span>

          <!-- 关闭按钮 -->
          <n-button
            quaternary
            circle
            size="tiny"
            @click.stop="closeTab(tab)"
            class="tab-close-btn"
          >
            <template #icon>
              <n-icon><CloseOutlined /></n-icon>
            </template>
          </n-button>
        </div>
      </div>
    </div>

    <!-- 右侧：操作按钮 -->
    <div class="suffix-container">
      <!-- 添加按钮 -->
      <div class="tab-add-btn">
        <ConnectionForm>
          <template #trigger>
            <n-button quaternary circle>
              <template #icon>
                <n-icon>
                  <PlusOutlined />
                </n-icon>
              </template>
            </n-button>
          </template>
        </ConnectionForm>
      </div>
      <!-- 文件管理器按钮 -->
      <n-button quaternary circle @click="showFileManager" class="mr-1">
        <template #icon>
          <n-icon><FolderOutlined /></n-icon>
        </template>
      </n-button>

      <!-- 设置按钮 -->
      <n-dropdown
        :options="settingsOptions"
        @select="handleSettingsSelect"
        trigger="click"
      >
        <n-button quaternary circle>
          <template #icon>
            <n-icon><SettingOutlined /></n-icon>
          </template>
        </n-button>
      </n-dropdown>
      <!-- 标签页菜单 -->
      <n-dropdown :options="tabMenuOptions" @select="handleTabMenuSelect">
        <n-button quaternary circle>
          <template #icon>
            <n-icon><MoreOutlined /></n-icon>
          </template>
        </n-button>
      </n-dropdown>
    </div>
  </div>
</template>

<script setup lang="ts">
import { h, onMounted } from "vue";
import {
  CloseOutlined,
  PlusOutlined,
  FolderOutlined,
  MoreOutlined,
  SettingOutlined,
  MenuOutlined,
} from "@vicons/antd";
import ConnectionStatus from "../connection/ConnectionStatus.vue";
import { useConnectionStore } from "@/stores/connection/index";
import type { TabInfo } from "@/stores/connection/type.d";

// 使用连接存储
const connectionStore = useConnectionStore();

// 定义 emits
const emit = defineEmits<{
  toggleSidebar: [];
  settingsSelect: [key: string];
}>();

// 标签页菜单选项
const tabMenuOptions = [
  {
    label: "关闭所有标签页",
    key: "closeAll",
    icon: () => h("n-icon", null, { default: () => h(CloseOutlined) }),
  },
  {
    label: "关闭其他标签页",
    key: "closeOthers",
    icon: () => h("n-icon", null, { default: () => h(CloseOutlined) }),
  },
  {
    type: "divider",
  },
  {
    label: "标签页设置",
    key: "settings",
    icon: () => h("n-icon", null, { default: () => h(SettingOutlined) }),
  },
];

// 设置菜单选项
const settingsOptions = [
  {
    label: "主题设置",
    key: "theme",
    icon: () => h("n-icon", null, { default: () => h(SettingOutlined) }),
  },
  {
    label: "链接设置",
    key: "connection",
    icon: () => h("n-icon", null, { default: () => h(SettingOutlined) }),
  },
  {
    type: "divider",
  },
  {
    label: "清理所有链接",
    key: "clear_all",
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
  emit("toggleSidebar");
};

const selectTab = async (tab: TabInfo) => {
  // 激活标签页
  await connectionStore.setActiveTab(tab.id);

  // 获取标签页对应的链接
  const connection = connectionStore.getTabConnection(tab);
  if (connection) {
    // 设置对应的链接为当前选中状态
    await connectionStore.setCurrentConnection(connection);
    console.log("选中标签页，同步选中链接:", connection.name);

    // 如果连接未建立，尝试建立连接
    if (!connection.connected) {
      console.log("连接未建立，尝试建立连接:", connection.name);
      try {
        const result = await connectionStore.connect(connection);
        if (result.success) {
          console.log("连接建立成功:", connection.name);
        } else {
          console.warn("连接建立失败:", result.message);
        }
      } catch (error) {
        console.error("连接建立出错:", error);
      }
    }
  }
};

const closeTab = (tab: TabInfo) => {
  connectionStore.removeTab(tab.id);
};

const getTabConnection = (tab: TabInfo) => {
  return connectionStore.getTabConnection(tab);
};

const handleAddTab = () => {
  console.log("添加新标签页 - 请从左侧链接列表中选择链接");
  // 可以触发侧边栏显示，让用户选择链接
  // 或者显示一个链接选择对话框
};

const showFileManager = () => {
  console.log("打开文件管理器");
};

const handleTabMenuSelect = (key: string) => {
  switch (key) {
    case "closeAll":
      if (confirm("确定要关闭所有标签页吗？")) {
        connectionStore.closeAllTabs();
      }
      break;
    case "closeOthers":
      const currentActiveTab = connectionStore.activeTab;
      if (currentActiveTab) {
        connectionStore.closeOtherTabs(currentActiveTab.id);
      }
      break;
    case "settings":
      console.log("显示标签页设置");
      break;
  }
};

const handleSettingsSelect = (key: string) => {
  emit("settingsSelect", key);
};
</script>

<style scoped lang="scss">
.tab-bar-container {
  display: flex;
  align-items: center;
  width: 100%;
  height: 45px;
  background-color: #fff;
  border-bottom: 1px solid #e0e0e0;

  .prefix-container {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    padding: 0 8px;
    border-right: 1px solid #e0e0e0;
    margin-right: 8px;
  }

  .tabs-container {
    flex: 1;
    flex-shrink: 0;
    min-width: 0;
    height: 100%;
    overflow: auto;

    .tabs-list {
      display: flex;
      height: 100%;
      overflow-x: auto;
      position: relative;
      .tab-item {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 0 6px;
        border-right: 1px solid #e0e0e0;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
        min-width: 120px;
        flex-shrink: 0;
        position: relative;
        height: 100%;
        background-color: rgba(0, 0, 0, 0.01);
        &:hover {
          background-color: #f0f0f0;
        }

        &:hover .tab-close-btn {
          opacity: 1;
        }

        &.tab-active {
          background-color: #e3f2fd;
          // border-bottom: 1px solid #18a058;
          font-weight: bold;
          position: position;
          &::before {
            content: "";
            position: absolute;
            top: 0;
            left: 0;
            width: 100%;
            height: 2px;
            background-color: #18a058;
          }
        }

        &.tab-active:hover {
          background-color: #bbdefb;
        }

        .tab-close-btn {
          opacity: 0;
          transition: opacity 0.2s ease-in-out;
        }
      }

      .tab-add-btn {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 0 12px;
        cursor: pointer;
        transition: all 0.2s ease-in-out;
        color: #666;
        flex-shrink: 0;
        min-width: 40px;

        &:hover {
          color: #333;
          background-color: #f0f0f0;
        }
      }
    }
  }

  .suffix-container {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    padding: 0 8px;
    border-left: 1px solid #e0e0e0;
  }
}

/* 自定义滚动条 */
::-webkit-scrollbar {
  height: 4px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 2px;
}

::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}
</style>
