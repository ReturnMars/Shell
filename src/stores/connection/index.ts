import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { ConnectionConfig, ConnectionResult, TabInfo } from "./type";

export const useConnectionStore = defineStore("connection", () => {
  // 状态
  const connections = ref<ConnectionConfig[]>([]);
  // 当前选中的链接
  const currentConnection = ref<ConnectionConfig | null>(null);
  // 标签页列表（独立存储）
  const tabs = ref<TabInfo[]>([]);
  // 当前活动的标签页ID
  const activeTabId = ref<string | null>(null);
  // 加载状态
  const loading = ref(false);
  // 错误信息
  const error = ref<string | null>(null);

  // 计算属性
  const connectedConnections = computed(() =>
    connections.value.filter((conn) => conn.connected === true)
  );

  const connectionCount = computed(() => connections.value.length);
  const connectedCount = computed(() => connectedConnections.value.length);
  const hasConnections = computed(() => connections.value.length > 0);

  // 标签页相关计算属性
  const activeTab = computed(() => 
    tabs.value.find(tab => tab.id === activeTabId.value) || null
  );
  const tabCount = computed(() => tabs.value.length);
  const hasTabs = computed(() => tabs.value.length > 0);
  
  // 获取标签页对应的链接信息
  const getTabConnection = (tab: TabInfo) => {
    return connections.value.find(conn => conn.id === tab.connection_id);
  };

  // 获取所有保存的链接
  const fetchConnections = async () => {
    try {
      loading.value = true;
      error.value = null;
      const result = (await invoke(
        "get_saved_connections"
      )) as ConnectionConfig[];

      // 获取实际已连接的链接
      const connectedConnections = (await invoke(
        "get_connected_connections"
      )) as ConnectionConfig[];

      // 更新连接状态
      const connectedIds = new Set(connectedConnections.map((conn) => conn.id));
      connections.value = result.map((conn) => ({
        ...conn,
        connected: connectedIds.has(conn.id),
        active: conn.active || false,
      }));

      console.log("加载链接配置成功:", result.length, "个链接");
      console.log("实际已连接:", connectedConnections.length, "个链接");
    } catch (err) {
      error.value = `加载链接失败: ${err}`;
      console.error("加载链接失败:", err);
    } finally {
      loading.value = false;
    }
  };

  // 保存链接
  const saveConnection = async (config: ConnectionConfig) => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("save_connection", { config });

      // 重新加载链接列表
      await fetchConnections();
      
      // 自动选中新保存的链接
      const savedConnection = connections.value.find(conn => conn.id === config.id);
      if (savedConnection) {
        currentConnection.value = savedConnection;
        console.log("自动选中新保存的链接:", config.name);
        
        // 自动创建对应的标签页
        try {
          console.log("开始创建标签页，链接ID:", savedConnection.id, "链接名称:", savedConnection.name);
          console.log("当前标签页数量:", tabs.value.length);
          await addTab(savedConnection);
          console.log("自动创建标签页成功:", config.name);
          console.log("创建后标签页数量:", tabs.value.length);
        } catch (tabError) {
          console.error("创建标签页失败:", tabError);
          // 标签页创建失败不影响链接保存
        }
      }
      
      console.log("保存链接成功:", config.name);
    } catch (err) {
      error.value = `保存链接失败: ${err}`;
      console.error("保存链接失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 删除链接
  const deleteConnection = async (connectionId: string) => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("delete_connection", { connectionId });

      // 从本地状态中移除
      const index = connections.value.findIndex(
        (conn) => conn.id === connectionId
      );
      if (index > -1) {
        connections.value.splice(index, 1);
      }

      // 如果删除的是当前选中的链接，清空当前链接
      if (currentConnection.value?.id === connectionId) {
        currentConnection.value = null;
      }

      console.log("删除链接成功:", connectionId);
    } catch (err) {
      error.value = `删除链接失败: ${err}`;
      console.error("删除链接失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 更新链接
  const updateConnection = async (config: ConnectionConfig) => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("update_connection", { config });

      // 更新本地状态
      const index = connections.value.findIndex(
        (conn) => conn.id === config.id
      );
      if (index > -1) {
        connections.value[index] = config;
      }

      console.log("更新链接成功:", config.name);
    } catch (err) {
      error.value = `更新链接失败: ${err}`;
      console.error("更新链接失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 设置当前选中的链接
  const setCurrentConnection = async (connection: ConnectionConfig | null) => {
    currentConnection.value = connection;
    console.log("🚀 ~ setCurrentConnection ~ connection:", connection);

    // 如果选中了链接且未连接，则自动连接
    if (connection && connection.connected !== true) {
      try {
        loading.value = true;
        console.log(`链接 ${connection.name} 未连接，正在自动连接...`);
        const result = await connect(connection);
        if (result.success) {
          console.log(`自动连接成功: ${connection.name}`);
          // 更新连接状态
          const index = connections.value.findIndex(
            (conn) => conn.id === connection.id
          );
          if (index > -1) {
            connections.value[index].connected = true;
          }
        } else {
          console.warn(`自动连接失败: ${result.message}`);
          error.value = `自动连接失败: ${result.message}`;
        }
      } catch (err) {
        console.error(`自动连接出错: ${err}`);
        error.value = `自动连接出错: ${err}`;
      } finally {
        loading.value = false;
      }
    }
  };

  // 根据ID获取链接
  const getConnectionById = (id: string) => {
    return connections.value.find((conn) => conn.id === id);
  };

  // 清空所有链接
  const clearAllConnections = async () => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("delete_all_connections");

      connections.value = [];
      currentConnection.value = null;

      console.log("清空所有链接成功");
    } catch (err) {
      error.value = `清空链接失败: ${err}`;
      console.error("清空链接失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 导入链接
  const importConnections = async (jsonData: string) => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("import_connections", { jsonData });

      // 重新加载链接列表
      await fetchConnections();
      console.log("导入链接成功");
    } catch (err) {
      error.value = `导入链接失败: ${err}`;
      console.error("导入链接失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 导出链接
  const exportConnections = async () => {
    try {
      loading.value = true;
      error.value = null;
      const result = (await invoke("export_connections")) as string;
      console.log("导出链接成功");
      return result;
    } catch (err) {
      error.value = `导出链接失败: ${err}`;
      console.error("导出链接失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 测试连接
  const testConnection = async (
    config: ConnectionConfig
  ): Promise<ConnectionResult> => {
    try {
      loading.value = true;
      error.value = null;
      const result = (await invoke("test_connection", { config })) as string;
      console.log("连接测试成功:", result);
      return {
        success: true,
        message: "连接测试成功",
        connectionId: result,
      };
    } catch (err) {
      const errorMsg = `连接测试失败: ${err}`;
      console.error("连接测试失败:", err);
      return {
        success: false,
        message: errorMsg,
      };
    } finally {
      loading.value = false;
    }
  };

  // 建立连接
  const connect = async (
    config: ConnectionConfig
  ): Promise<ConnectionResult> => {
    try {
      loading.value = true;
      error.value = null;
      const connectionId = (await invoke("connect_ssh", { config })) as string;

      // 更新连接状态
      const index = connections.value.findIndex(
        (conn) => conn.id === config.id
      );
      if (index > -1) {
        connections.value[index].connected = true;
      }

      console.log("连接建立成功:", connectionId);
      return {
        success: true,
        message: "连接建立成功",
        connectionId,
      };
    } catch (err) {
      const errorMsg = `连接建立失败: ${err}`;
      console.error("连接建立失败:", err);
      return {
        success: false,
        message: errorMsg,
      };
    } finally {
      loading.value = false;
    }
  };

  // 断开连接
  const disconnect = async (connectionId: string) => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("disconnect_ssh", { connectionId });

      // 更新连接状态
      const index = connections.value.findIndex(
        (conn) => conn.id === connectionId
      );
      if (index > -1) {
        connections.value[index].connected = false;
      }

      console.log("断开连接成功:", connectionId);
    } catch (err) {
      error.value = `断开连接失败: ${err}`;
      console.error("断开连接失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 断开所有连接
  const disconnectAll = async () => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("disconnect_all_ssh");

      // 更新所有连接状态
      connections.value.forEach((conn) => {
        conn.connected = false;
      });

      console.log("断开所有连接成功");
    } catch (err) {
      error.value = `断开所有连接失败: ${err}`;
      console.error("断开所有连接失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 执行SSH命令
  const executeCommand = async (connectionId: string, command: string) => {
    try {
      loading.value = true;
      error.value = null;
      const result = (await invoke("execute_ssh_command", {
        connectionId,
        command,
      })) as string;
      console.log("命令执行成功:", result);
      return result;
    } catch (err) {
      error.value = `命令执行失败: ${err}`;
      console.error("命令执行失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 生成UUID
  const generateUuid = async (): Promise<string> => {
    try {
      const result = (await invoke("generate_uuid")) as string;
      return result;
    } catch (err) {
      console.error("生成UUID失败:", err);
      throw err;
    }
  };

  // 获取标签页列表
  const fetchTabs = async () => {
    try {
      loading.value = true;
      error.value = null;
      const result = (await invoke("get_tabs_list")) as TabInfo[];
      tabs.value = result;
      
      // 设置活动标签页ID
      const activeTab = result.find(tab => tab.active);
      activeTabId.value = activeTab?.id || null;
      
      console.log("加载标签页成功:", result.length, "个标签页");
    } catch (err) {
      error.value = `加载标签页失败: ${err}`;
      console.error("加载标签页失败:", err);
    } finally {
      loading.value = false;
    }
  };

  // 添加标签页
  const addTab = async (connection: ConnectionConfig) => {
    try {
      loading.value = true;
      error.value = null;
      console.log("调用 add_tab 命令，参数:", { connectionId: connection.id, title: connection.name });
      const tabId = (await invoke("add_tab", {
        connectionId: connection.id,
        title: connection.name,
      })) as string;
      
      console.log("add_tab 命令返回的 tabId:", tabId);
      
      // 重新加载标签页列表
      await fetchTabs();
      
      // 确保新创建的标签页被设置为活动状态
      activeTabId.value = tabId;
      console.log("设置活动标签页ID:", tabId);
      
      // 更新本地标签页状态
      tabs.value.forEach(tab => {
        tab.active = tab.id === tabId;
      });
      console.log("更新本地标签页活动状态");
      
      console.log("添加标签页成功:", connection.name);
    } catch (err) {
      error.value = `添加标签页失败: ${err}`;
      console.error("添加标签页失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 删除标签页
  const removeTab = async (tabId: string) => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("remove_tab", { tabId });
      
      // 重新加载标签页列表
      await fetchTabs();
      console.log("删除标签页成功:", tabId);
    } catch (err) {
      error.value = `删除标签页失败: ${err}`;
      console.error("删除标签页失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 设置活动标签页
  const setActiveTab = async (tabId: string) => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("set_active_tab", { tabId });
      
      // 更新本地状态
      activeTabId.value = tabId;
      tabs.value.forEach(tab => {
        tab.active = tab.id === tabId;
      });
      
      console.log("设置活动标签页:", tabId);
    } catch (err) {
      error.value = `设置活动标签页失败: ${err}`;
      console.error("设置活动标签页失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 关闭所有标签页
  const closeAllTabs = async () => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("close_all_tabs");
      
      tabs.value = [];
      activeTabId.value = null;
      console.log("关闭所有标签页成功");
    } catch (err) {
      error.value = `关闭所有标签页失败: ${err}`;
      console.error("关闭所有标签页失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 关闭其他标签页
  const closeOtherTabs = async (keepTabId: string) => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("close_other_tabs", { keepTabId });
      
      // 重新加载标签页列表
      await fetchTabs();
      console.log("关闭其他标签页成功，保留:", keepTabId);
    } catch (err) {
      error.value = `关闭其他标签页失败: ${err}`;
      console.error("关闭其他标签页失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 重置状态
  const reset = () => {
    connections.value = [];
    currentConnection.value = null;
    tabs.value = [];
    activeTabId.value = null;
    loading.value = false;
    error.value = null;
  };

  return {
    // 状态
    connections,
    currentConnection,
    tabs,
    activeTabId,
    loading,
    error,

    // 计算属性
    connectedConnections,
    connectionCount,
    connectedCount,
    hasConnections,
    activeTab,
    tabCount,
    hasTabs,

    // 方法
    fetchConnections,
    saveConnection,
    deleteConnection,
    updateConnection,
    setCurrentConnection,
    getConnectionById,
    clearAllConnections,
    importConnections,
    exportConnections,
    testConnection,
    connect,
    disconnect,
    disconnectAll,
    executeCommand,
    generateUuid,
    
    // 标签页方法
    fetchTabs,
    addTab,
    removeTab,
    setActiveTab,
    closeAllTabs,
    closeOtherTabs,
    getTabConnection,
    
    reset,
  };
});

// 导出类型
export * from "./type.d";
