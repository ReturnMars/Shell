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
              测试SSH连接
            </n-button>

            <n-button 
              v-if="connections.length > 0" 
              type="info" 
              size="small" 
              @click="testCommand"
            >
              测试命令
            </n-button>

            <n-button 
              v-if="connections.length > 0" 
              type="error" 
              size="small" 
              @click="disconnectAll"
            >
              断开所有连接
            </n-button>

            <n-button 
              v-if="connections.length > 0" 
              type="success" 
              size="small" 
              @click="saveCurrentConnection"
            >
              保存当前连接
            </n-button>

            <n-button 
              type="warning" 
              size="small" 
              @click="loadSavedConnections"
            >
              加载保存的连接
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
        class="bg-white border-t border-gray-200 h-9 flex-shrink-0"
        bordered
      >
        <div
          class="flex items-center justify-between h-full p-[10px] text-xs text-gray-600"
        >
          <div class="flex items-center gap-4">
            <span>就绪</span>
            <span>|</span>
            <span>连接数: {{ connections.length }}</span>
          </div>
          <div class="flex items-center">
            <span>ShellMars v1.0.0</span>
          </div>
        </div>
      </n-layout-footer>
    </div>
  </n-layout>
</template>

<script setup lang="ts">
import { h, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { MenuOutlined, PlusOutlined, SettingOutlined } from "@vicons/antd";
import Sidebar from "./Sidebar.vue";
import TabBar from "./TabBar.vue";
import ConnectionStatus from "./ConnectionStatus.vue";
import { NIcon } from "naive-ui";

// 响应式数据
const isConnected = ref(false);
const sidebarCollapsed = ref(false);
const connections = ref<any[]>([]);

// 设置菜单选项
const settingsOptions = [
  {
    label: "主题设置",
    key: "theme",
    icon: () => h(NIcon, { size: 16 }, { default: () => h(SettingOutlined) }),
  },
  {
    label: "连接设置",
    key: "connection",
    icon: () => h(NIcon, { size: 16 }, { default: () => h(SettingOutlined) }),
  },
  {
    label: "关于",
    key: "about",
    icon: () => h(NIcon, { size: 16 }, { default: () => h(SettingOutlined) }),
  },
];

// 方法
const toggleSidebar = () => {
  sidebarCollapsed.value = !sidebarCollapsed.value;
};

const showQuickConnect = async () => {
  console.log("显示快速连接对话框");
  
  // 测试SSH连接功能
  try {
    // 创建测试连接配置
    const testConfig = {
      id: "test-connection-" + Date.now(),
      name: "测试服务器",
      host: "47.109.195.0", // 使用GitHub SSH服务测试
      port: 22,
      username: "root",
      password: 'Aioreturn@123',
      private_key_path: null,
      auth_method: "Password",
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };

    console.log("尝试连接SSH:", testConfig);
    
    // 调用Tauri命令测试连接
    const result = await invoke("test_connection", { config: testConfig });
    console.log("连接结果:", result);
    
    // 更新连接状态
    isConnected.value = true;
    
    // 获取所有连接
    const allConnections = await invoke("get_connections");
    console.log("所有连接:", allConnections);
    connections.value = allConnections as any[];
    
  } catch (error) {
    console.error("SSH连接测试失败:", error);
    isConnected.value = false;
  }
};

const handleSettingsSelect = (key: string) => {
  console.log("选择设置项:", key);
};

// 测试SSH命令执行
const testCommand = async () => {
  if (connections.value.length > 0) {
    try {
      const connectionId = connections.value[0].id;
      const result = await invoke("execute_ssh_command", { 
        connectionId, 
        command: "echo 'Hello from SSH!'" 
      });
      console.log("命令执行结果:", result);
    } catch (error) {
      console.error("命令执行失败:", error);
    }
  }
};

// 断开所有连接
const disconnectAll = async () => {
  try {
    console.log("断开所有SSH连接...");
    await invoke("disconnect_all_ssh");
    
    // 更新连接状态
    isConnected.value = false;
    connections.value = [];
    
    console.log("所有SSH连接已断开");
  } catch (error) {
    console.error("断开连接失败:", error);
  }
};

// 保存当前连接
const saveCurrentConnection = async () => {
  if (connections.value.length > 0) {
    try {
      const currentConnection = connections.value[0];
      console.log("保存连接配置:", currentConnection);
      
      await invoke("save_connection", { config: currentConnection });
      console.log("连接配置已保存");
    } catch (error) {
      console.error("保存连接失败:", error);
    }
  }
};

// 加载保存的连接
const loadSavedConnections = async () => {
  try {
    console.log("加载保存的连接配置...");
    const savedConnections = await invoke("get_saved_connections");
    console.log("保存的连接:", savedConnections);
    
    // 显示保存的连接信息
    if (Array.isArray(savedConnections) && savedConnections.length > 0) {
      console.log(`找到 ${savedConnections.length} 个保存的连接:`);
      savedConnections.forEach((conn: any, index: number) => {
        console.log(`${index + 1}. ${conn.name} (${conn.host}:${conn.port})`);
      });
    } else {
      console.log("没有找到保存的连接");
    }
  } catch (error) {
    console.error("加载保存的连接失败:", error);
  }
};

// 页面加载时测试
const initTest = async () => {
  console.log("初始化SSH功能测试...");
  await showQuickConnect();
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
