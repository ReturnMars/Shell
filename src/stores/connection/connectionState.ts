/**
 * 连接状态管理模块
 * 统一管理前后端连接状态，解决状态不同步问题
 */

import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { ConnectionConfig } from "./type";

// 连接状态枚举
export enum ConnectionStatus {
  DISCONNECTED = "disconnected",
  CONNECTING = "connecting",
  CONNECTED = "connected",
  ERROR = "error",
  RECONNECTING = "reconnecting",
}

// 连接状态信息
export interface ConnectionState {
  id: string;
  config: ConnectionConfig;
  status: ConnectionStatus;
  lastCheck: number;
  error?: string;
  retryCount: number;
  maxRetries: number;
}

// 连接状态管理器
class ConnectionStateManager {
  private states = ref<Map<string, ConnectionState>>(new Map());
  private healthCheckInterval: ReturnType<typeof setInterval> | null = null;
  private readonly HEALTH_CHECK_INTERVAL = 30000; // 30秒检查一次
  private readonly MAX_RETRIES = 3;

  // 计算属性
  get allStates() {
    return computed(() => Array.from(this.states.value.values()));
  }

  get connectedStates() {
    return computed(() =>
      this.allStates.value.filter(
        (state) => state.status === ConnectionStatus.CONNECTED
      )
    );
  }

  get errorStates() {
    return computed(() =>
      this.allStates.value.filter(
        (state) => state.status === ConnectionStatus.ERROR
      )
    );
  }

  // 获取连接状态
  getConnectionState(connectionId: string): ConnectionState | undefined {
    return this.states.value.get(connectionId);
  }

  // 设置连接状态
  setConnectionState(connectionId: string, state: Partial<ConnectionState>) {
    const currentState = this.states.value.get(connectionId);
    if (currentState) {
      this.states.value.set(connectionId, { ...currentState, ...state });
    } else {
      // 创建新状态
      this.states.value.set(connectionId, {
        id: connectionId,
        config: state.config!,
        status: state.status || ConnectionStatus.DISCONNECTED,
        lastCheck: Date.now(),
        retryCount: 0,
        maxRetries: this.MAX_RETRIES,
        ...state,
      });
    }
  }

  // 更新连接状态
  updateConnectionStatus(
    connectionId: string,
    status: ConnectionStatus,
    error?: string
  ) {
    const currentState = this.states.value.get(connectionId);
    if (currentState) {
      this.states.value.set(connectionId, {
        ...currentState,
        status,
        lastCheck: Date.now(),
        error,
        retryCount:
          status === ConnectionStatus.ERROR ? currentState.retryCount + 1 : 0,
      });
    }
  }

  // 移除连接状态
  removeConnectionState(connectionId: string) {
    this.states.value.delete(connectionId);
  }

  // 检查连接健康状态
  async checkConnectionHealth(connectionId: string): Promise<boolean> {
    try {
      // 调用后端检查连接状态
      const isConnected = await invoke<boolean>("check_connection_status", {
        connectionId,
      });

      if (isConnected) {
        this.updateConnectionStatus(connectionId, ConnectionStatus.CONNECTED);
        return true;
      } else {
        this.updateConnectionStatus(
          connectionId,
          ConnectionStatus.DISCONNECTED
        );
        return false;
      }
    } catch (error) {
      console.error(`检查连接 ${connectionId} 健康状态失败:`, error);
      this.updateConnectionStatus(
        connectionId,
        ConnectionStatus.ERROR,
        String(error)
      );
      return false;
    }
  }

  // 启动健康检查
  startHealthCheck() {
    if (this.healthCheckInterval) {
      return; // 已经启动
    }

    this.healthCheckInterval = setInterval(async () => {
      console.log("连接状态管理器 - 开始健康检查");

      for (const [connectionId, state] of this.states.value) {
        if (state.status === ConnectionStatus.CONNECTED) {
          const isHealthy = await this.checkConnectionHealth(connectionId);
          if (!isHealthy) {
            console.warn(`连接 ${connectionId} 健康检查失败`);

            // 尝试自动重连
            if (state.retryCount < state.maxRetries) {
              console.log(`连接 ${connectionId} 尝试自动重连`);
              await this.autoReconnect(connectionId);
            } else {
              console.error(
                `连接 ${connectionId} 已达到最大重试次数，停止重连`
              );
            }
          }
        }
      }
    }, this.HEALTH_CHECK_INTERVAL);

    console.log("连接状态管理器 - 健康检查已启动");
  }

  // 停止健康检查
  stopHealthCheck() {
    if (this.healthCheckInterval) {
      clearInterval(this.healthCheckInterval);
      this.healthCheckInterval = null;
      console.log("连接状态管理器 - 健康检查已停止");
    }
  }

  // 自动重连
  async autoReconnect(connectionId: string): Promise<boolean> {
    const state = this.states.value.get(connectionId);
    if (!state || state.retryCount >= state.maxRetries) {
      return false;
    }

    try {
      this.updateConnectionStatus(connectionId, ConnectionStatus.RECONNECTING);

      // 调用后端重连
      const result = await invoke<string>("reconnect_ssh", {
        config: state.config,
      });

      if (result) {
        // 重连成功，重置重试计数
        this.setConnectionState(connectionId, {
          status: ConnectionStatus.CONNECTED,
          retryCount: 0,
          lastCheck: Date.now(),
        });
        console.log(`连接 ${connectionId} 自动重连成功`);
        return true;
      } else {
        this.updateConnectionStatus(
          connectionId,
          ConnectionStatus.ERROR,
          "重连失败"
        );
        return false;
      }
    } catch (error) {
      this.updateConnectionStatus(
        connectionId,
        ConnectionStatus.ERROR,
        String(error)
      );
      console.error(`连接 ${connectionId} 自动重连失败:`, error);
      return false;
    }
  }

  // 同步后端连接状态
  async syncBackendConnections() {
    try {
      console.log("连接状态管理器 - 开始同步后端连接状态");
      console.log("连接状态管理器 - states类型:", typeof this.states.value);
      console.log(
        "连接状态管理器 - states是否为Map:",
        this.states.value instanceof Map
      );
      console.log("连接状态管理器 - states值:", this.states.value);

      // 安全检查：确保states.value是Map对象
      if (!this.states.value || !(this.states.value instanceof Map)) {
        this.states.value = new Map();
        return;
      }

      const backendConnections = await invoke<ConnectionConfig[]>(
        "get_connected_connections"
      );
      const backendIds = new Set(backendConnections.map((conn) => conn.id));

      // 更新所有连接状态
      for (const [connectionId, _state] of this.states.value) {
        if (backendIds.has(connectionId)) {
          this.updateConnectionStatus(connectionId, ConnectionStatus.CONNECTED);
        } else {
          this.updateConnectionStatus(
            connectionId,
            ConnectionStatus.DISCONNECTED
          );
        }
      }

      console.log("连接状态管理器 - 已同步后端连接状态");
    } catch (error) {
      console.error("同步后端连接状态失败:", error);
    }
  }

  // 清理资源
  cleanup() {
    this.stopHealthCheck();
    // 注意：不要清空states，因为连接状态需要保持
    // this.states.value.clear();
  }
}

// 导出单例实例
export const connectionStateManager = new ConnectionStateManager();
