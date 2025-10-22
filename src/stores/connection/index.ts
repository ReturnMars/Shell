import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { ConnectionConfig, ConnectionResult } from "./type";

export const useConnectionStore = defineStore("connection", () => {
  // 状态
  const connections = ref<ConnectionConfig[]>([]);
  // 当前连接
  const currentConnection = ref<ConnectionConfig | null>(null);
  // 加载状态
  const loading = ref(false);
  // 错误信息
  const error = ref<string | null>(null);

  // 计算属性
  const connectedConnections = computed(() =>
    connections.value.filter((conn) => conn.connected)
  );

  const connectionCount = computed(() => connections.value.length);

  const hasConnections = computed(() => connections.value.length > 0);

  // 获取所有保存的连接
  const fetchConnections = async () => {
    try {
      loading.value = true;
      error.value = null;
      const result = (await invoke(
        "get_saved_connections"
      )) as ConnectionConfig[];
      connections.value = result;
      console.log("加载连接配置成功:", result.length, "个连接");
    } catch (err) {
      error.value = `加载连接失败: ${err}`;
      console.error("加载连接失败:", err);
    } finally {
      loading.value = false;
    }
  };

  // 保存连接
  const saveConnection = async (config: ConnectionConfig) => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("save_connection", { config });

      // 重新加载连接列表
      await fetchConnections();
      console.log("保存连接成功:", config.name);
    } catch (err) {
      error.value = `保存连接失败: ${err}`;
      console.error("保存连接失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 删除连接
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

        // 如果删除的是当前连接，清空当前连接
        if (currentConnection.value?.id === connectionId) {
          currentConnection.value = null;
        }

      console.log("删除连接成功:", connectionId);
    } catch (err) {
      error.value = `删除连接失败: ${err}`;
      console.error("删除连接失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 更新连接
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

      console.log("更新连接成功:", config.name);
    } catch (err) {
      error.value = `更新连接失败: ${err}`;
      console.error("更新连接失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 设置当前连接
  const setCurrentConnection = (connection: ConnectionConfig | null) => {
    currentConnection.value = connection;
    console.log("设置当前连接:", connection?.name || "无");
  };

  // 根据ID获取连接
  const getConnectionById = (id: string) => {
    return connections.value.find((conn) => conn.id === id);
  };

  // 清空所有连接
  const clearAllConnections = async () => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("delete_all_connections");

      connections.value = [];
      currentConnection.value = null;

      console.log("清空所有连接成功");
    } catch (err) {
      error.value = `清空连接失败: ${err}`;
      console.error("清空连接失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 导入连接
  const importConnections = async (jsonData: string) => {
    try {
      loading.value = true;
      error.value = null;
      await invoke("import_connections", { jsonData });

      // 重新加载连接列表
      await fetchConnections();
      console.log("导入连接成功");
    } catch (err) {
      error.value = `导入连接失败: ${err}`;
      console.error("导入连接失败:", err);
      throw err;
    } finally {
      loading.value = false;
    }
  };

  // 导出连接
  const exportConnections = async () => {
    try {
      loading.value = true;
      error.value = null;
      const result = (await invoke("export_connections")) as string;
      console.log("导出连接成功");
      return result;
    } catch (err) {
      error.value = `导出连接失败: ${err}`;
      console.error("导出连接失败:", err);
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

  // 重置状态
  const reset = () => {
    connections.value = [];
    currentConnection.value = null;
    loading.value = false;
    error.value = null;
  };

  return {
    // 状态
    connections,
    currentConnection,
    loading,
    error,

    // 计算属性
    connectedConnections,
    connectionCount,
    hasConnections,

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
    reset,
  };
});

// 导出类型
export * from "./type.d";
