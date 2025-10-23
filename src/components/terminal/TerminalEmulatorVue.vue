<template>
  <div class="terminal-emulator-vue">
    <!-- 连接状态指示器 -->
    <div class="terminal-status-indicator">
      <n-tag v-if="isConnected" type="success" size="small" round>
        <template #icon>
          <n-icon>
            <CheckCircleOutlined />
          </n-icon>
        </template>
        已连接 ({{ sessionId }})
      </n-tag>
      <n-button
        v-else
        size="small"
        type="primary"
        @click="reconnect"
        :loading="isLoading"
        ghost
      >
        <template #icon>
          <n-icon>
            <ApiOutlined />
          </n-icon>
        </template>
        重新连接
      </n-button>
    </div>

    <!-- vue-web-terminal 组件 -->
    <terminal
      v-if="isConnected"
      name="ssh-terminal"
      theme="dark"
      :drag-conf="dragConf"
      @exec-cmd="onExecCmd"
      ref="terminalRef"
    />

    <!-- 未连接时的提示 -->
    <div v-else class="terminal-disconnected">
      <n-empty description="终端未连接">
        <template #icon>
          <n-icon>
            <ApiOutlined />
          </n-icon>
        </template>
        <template #extra>
          <n-button @click="reconnect" :loading="isLoading">
            连接终端
          </n-button>
        </template>
      </n-empty>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, onUnmounted, watch } from "vue";
import { CheckCircleOutlined, ApiOutlined } from "@vicons/antd";
import { useConnectionStore } from "@/stores/connection";
import { invoke } from "@tauri-apps/api/core";

// Props
interface Props {       
  connectionId: string; 
  title?: string;
  autoConnect?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  title: "SSH连接",
  autoConnect: true,
});

// Emits
const emit = defineEmits<{
  connected: [sessionId: string];
  disconnected: [];
  error: [error: string];
  data: [data: string];
}>();

// 状态
const isConnected = ref(false);
const isLoading = ref(false);
const error = ref<string | null>(null);
const sessionId = ref<string | null>(null);
const terminalRef = ref<any>(null);

// 拖拽配置
const dragConf = reactive({
  width: "100%",
  height: "100%",
  zIndex: 100,
  init: { x: 0, y: 0 },
  pinned: true, // 固定位置，不可拖拽
});

// 获取连接store
const connectionStore = useConnectionStore();

// 连接SSH
const connectSSH = async () => {
  if (!props.connectionId) return;

  try {
    isLoading.value = true;
    error.value = null;

    console.log("开始连接SSH:", props.connectionId);

    // 检查连接状态
    const status = await invoke<
      "Connected" | "Disconnected" | "Connecting" | "Error"
    >("get_connection_status", { connectionId: props.connectionId });

    console.log("当前连接状态:", status);

    if (status !== "Connected") {
      console.log(`SSH连接状态为: ${status}，尝试重新建立连接`);

      // 如果连接不存在或断开，尝试重新建立连接
      if (status === "Disconnected" || status === "Error") {
        try {
          // 获取连接配置
          const connection = connectionStore.getConnectionById(
            props.connectionId
          );
          if (connection) {
            console.log("重新建立SSH连接:", connection.name);
            const result = await connectionStore.connect(connection);
            if (result.success) {
              console.log("重新建立连接成功");
            } else {
              console.warn("重新建立连接失败，但继续尝试:", result.message);
            }
          } else {
            console.warn("找不到连接配置，但继续尝试");
          }
        } catch (reconnectErr) {
          console.warn("重新建立连接失败，但继续尝试:", reconnectErr);
        }
      }

      console.log("继续尝试连接...");
    }

    // 连接成功
    sessionId.value = `session_${Date.now()}`;
    isConnected.value = true;

    console.log("终端连接状态已设置为:", isConnected.value);

    emit("connected", sessionId.value);
    console.log("SSH连接成功:", sessionId.value);
  } catch (err) {
    error.value = `SSH连接失败: ${err}`;
    emit("error", error.value);
    console.error("SSH连接失败:", err);
  } finally {
    isLoading.value = false;
  }
};

// 断开连接
const disconnect = () => {
  if (sessionId.value) {
    connectionStore.disconnect(props.connectionId);
    sessionId.value = null;
    isConnected.value = false;
    emit("disconnected");
    console.log("SSH连接已断开");
  }
};

// 重新连接
const reconnect = async () => {
  disconnect();
  await connectSSH();
};

// 处理命令执行
const onExecCmd = async (command: string, success: Function, failed: Function) => {
  console.log("执行命令:", command);

  if (!isConnected.value) {
    failed("终端未连接");
    return;
  }

  try {
    const output = await invoke<string>("execute_ssh_command", {
      connectionId: props.connectionId,
      command: command,
    });

    if (output) {
      // 解析输出并显示
      const lines = output.split('\n');
      lines.forEach(line => {
        if (line.trim()) {
          success({
            type: 'normal',
            class: 'success',
            content: line
          });
        }
      });
    } else {
      success({
        type: 'normal',
        class: 'success',
        content: ''
      });
    }
  } catch (err) {
    console.error("命令执行失败:", err);
    failed(`命令执行失败: ${err}`);
  }
};

// 监听连接ID变化
watch(
  () => props.connectionId,
  (newId, oldId) => {
    if (newId && newId !== oldId) {
      console.log("连接ID变化，重新连接:", newId);
      connectSSH();
    }
  }
);

// 组件挂载
onMounted(() => {
  if (props.autoConnect && props.connectionId) {
    console.log("组件挂载，自动连接:", props.connectionId);
    connectSSH();
  }
});

// 组件卸载
onUnmounted(() => {
  disconnect();
});

// 暴露方法
defineExpose({
  connect: connectSSH,
  disconnect,
  reconnect,
  isConnected: () => isConnected.value,
  sessionId: () => sessionId.value,
});
</script>

<style scoped>
.terminal-emulator-vue {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: #1e1e1e;
  border-radius: 8px;
  overflow: hidden;
  position: relative;
}

.terminal-status-indicator {
  padding: 8px 12px;
  background: #2d2d2d;
  border-bottom: 1px solid #404040;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.terminal-disconnected {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #1e1e1e;
}

/* vue-web-terminal 样式覆盖 */
:deep(.terminal-container) {
  height: 100% !important;
  background: #1e1e1e !important;
}

:deep(.terminal-content) {
  background: #1e1e1e !important;
  color: #ffffff !important;
}

:deep(.terminal-input) {
  background: #1e1e1e !important;
  color: #ffffff !important;
}
</style>
