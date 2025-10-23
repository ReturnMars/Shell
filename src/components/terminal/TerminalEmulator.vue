<template>
  <div class="terminal-emulator">
    <!-- 终端容器 -->
    <div
      ref="terminalContainer"
      class="terminal-container"
      :class="{ 'terminal-loading': isLoading, 'terminal-error': error }"
    ></div>

    <!-- 加载状态 -->
    <div v-if="isLoading" class="terminal-loading-overlay">
      <n-spin size="large">
        <template #description>
          <div class="loading-text">正在初始化终端...</div>
        </template>
      </n-spin>
    </div>

    <!-- 错误状态 -->
    <div v-if="error" class="terminal-error-overlay">
      <n-alert type="error" :title="error" closable @close="error = null">
        <n-button size="small" @click="reconnect">重试</n-button>
      </n-alert>
    </div>

    <!-- 连接状态指示器 -->
    <div v-if="!isLoading && !error" class="terminal-status-indicator">
      <n-tag v-if="isConnected" type="success" size="small" round>
        <template #icon>
          <n-icon>
            <CheckCircleOutlined />
          </n-icon>
        </template>
        已连接
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
  </div>
</template>

<script setup lang="ts">
import {
  ref,
  onMounted,
  onUnmounted,
  watch,
  nextTick,
  useTemplateRef,
  shallowRef,
} from "vue";
import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { WebLinksAddon } from "@xterm/addon-web-links";
import { SearchAddon } from "@xterm/addon-search";
import { Unicode11Addon } from "@xterm/addon-unicode11";
import { CheckCircleOutlined, ApiOutlined } from "@vicons/antd";
import { invoke } from "@tauri-apps/api/core";
import { useConnectionStore } from "@/stores/connection";

import "@xterm/xterm/css/xterm.css";

// Props
interface Props {
  connectionId: string;
  title: string;
  autoConnect?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  autoConnect: true,
});

// Emits
const emit = defineEmits<{
  connected: [sessionId: string];
  disconnected: [];
  error: [error: string];
  data: [data: string];
}>();

// 终端容器引用
const terminalContainer = useTemplateRef("terminalContainer");

// 终端相关状态
const terminal = shallowRef<Terminal | null>(null);
const fitAddon = ref<FitAddon | null>(null);
const searchAddon = ref<SearchAddon | null>(null);
const isConnected = ref(false);
const isLoading = ref(false);
const error = ref<string | null>(null);
const sessionId = ref<string | null>(null);

// 不再需要输入缓冲区，直接发送到后端

// 获取连接store
const connectionStore = useConnectionStore();

// 初始化终端
const initTerminal = async () => {
  if (!terminalContainer.value) return;

  try {
    isLoading.value = true;
    error.value = null;

    // 创建终端实例
    terminal.value = new Terminal({
      allowProposedApi: true,
      disableStdin: false, // 确保输入不被禁用
      theme: {
        background: "#0d1117",
        foreground: "#f0f6fc",
        cursor: "#58a6ff",
        cursorAccent: "#0d1117",
        black: "#484f58",
        red: "#f85149",
        green: "#3fb950",
        yellow: "#d29922",
        blue: "#58a6ff",
        magenta: "#bc8cff",
        cyan: "#39d353",
        white: "#b1bac4",
        brightBlack: "#6e7681",
        brightRed: "#ff7b72",
        brightGreen: "#56d364",
        brightYellow: "#e3b341",
        brightBlue: "#79c0ff",
        brightMagenta: "#d2a8ff",
        brightCyan: "#56d364",
        brightWhite: "#f0f6fc",
      },
      fontFamily:
        "'Fira Code', 'JetBrains Mono', 'Cascadia Code', Consolas, monospace",
      fontSize: 14,
      lineHeight: 1.3,
      cursorBlink: true,
      cursorStyle: "block",
      scrollback: 5000,
      tabStopWidth: 4,
      convertEol: true,
      macOptionIsMeta: true,
      // 确保终端可以接收输入
      cols: 80,
      rows: 24,
    });

    // 添加插件
    fitAddon.value = new FitAddon();
    searchAddon.value = new SearchAddon();
    const webLinksAddon = new WebLinksAddon();
    const unicode11Addon = new Unicode11Addon();

    // 按顺序加载插件
    terminal.value.loadAddon(fitAddon.value);
    terminal.value.loadAddon(searchAddon.value);
    terminal.value.loadAddon(webLinksAddon);
    terminal.value.loadAddon(unicode11Addon);

    // 挂载终端
    terminal.value.open(terminalContainer.value);

    // 自适应大小
    await nextTick();
    fitAddon.value.fit();

    // 监听窗口大小变化
    const resizeObserver = new ResizeObserver(() => {
      if (fitAddon.value) {
        fitAddon.value.fit();
      }
    });
    resizeObserver.observe(terminalContainer.value);
    // 监听用户输入 - 最简单的转发方式
    let currentCommand = "";
    terminal.value.onData(async (data) => {
      if (!isConnected.value || !terminal.value) return;
      console.log("🚀 ~ initTerminal ~ data:", data);

      try {
        // 直接转发输入到SSH
        // 当用户输入回车时，执行命令
        currentCommand += data;
        if (data === "\r") {
          console.log("🚀 ~ initTerminal ~ currentCommand:", currentCommand);
          const output = await invoke<string>("execute_ssh_command", {
            connectionId: props.connectionId,
            command: currentCommand,
          });
          console.log("🚀 ~ initTerminal ~ output:", output);
          // 如果有输出就显示
          terminal.value.write(output);
          if (output) {
            terminal.value.write("");
          }
          currentCommand = "";
        } else {
          terminal.value.write(data);
        }
      } catch (err) {
        console.error("SSH命令执行失败:", err);
        terminal.value?.writeln(`错误: ${err}`);
      } finally {
      }
    });

    console.log("终端初始化成功");
  } catch (err) {
    error.value = `终端初始化失败: ${err}`;
    console.error("终端初始化失败:", err);
  } finally {
    isLoading.value = false;
  }
};

// 连接SSH
const connectSSH = async () => {
  if (!terminal.value) return;

  // 如果已经连接且是同一个连接ID，跳过
  if (isConnected.value && sessionId.value) {
    console.log("已经连接，跳过重复连接:", props.connectionId);
    return;
  }

  // 如果已经连接，先断开
  if (isConnected.value) {
    console.log("已连接，先断开当前连接");
    await disconnect();
    await new Promise((resolve) => setTimeout(resolve, 100));
  }

  try {
    isLoading.value = true;
    error.value = null;
    isConnected.value = false;

    terminal.value.writeln("");
    terminal.value.writeln("正在连接到SSH服务器...");

    // 检查连接状态
    const status = await invoke<
      "Connected" | "Disconnected" | "Connecting" | "Error"
    >("get_connection_status", { connectionId: props.connectionId });

    console.log("SSH连接状态检查:", {
      connectionId: props.connectionId,
      status,
    });

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

    // 等待一下让SSH连接稳定
    await new Promise((resolve) => setTimeout(resolve, 200));

    // 清屏并显示连接成功信息
    terminal.value.clear();
    terminal.value.writeln("SSH连接已建立");
    terminal.value.writeln(`连接到: ${props.title}`);
    terminal.value.writeln("欢迎使用远程终端");

    // 显示默认提示符
    terminal.value.write("$ ");

    emit("connected", sessionId.value);
    console.log("SSH连接成功:", sessionId.value);
  } catch (err) {
    error.value = `SSH连接失败: ${err}`;
    terminal.value?.writeln(`连接失败: ${err}`);
    emit("error", error.value);
    console.error("SSH连接失败:", err);
  } finally {
    isLoading.value = false;
  }
};

// 断开连接
const disconnect = async () => {
  if (terminal.value) {
    terminal.value.writeln("");
    terminal.value.writeln(`与 ${props.title} 的连接已断开`);
    terminal.value.write("$ ");
  }

  isConnected.value = false;
  sessionId.value = null;

  // 同步更新store中的连接状态
  try {
    await connectionStore.disconnect(props.connectionId);
    console.log("已同步更新store中的连接状态");
  } catch (err) {
    console.warn("更新store连接状态失败:", err);
  }

  emit("disconnected");
  console.log("SSH连接已断开");
};

// 重连
const reconnect = async () => {
  try {
    isLoading.value = true;
    error.value = null;

    console.log("开始重新连接:", props.title);

    // 获取连接配置
    const connection = connectionStore.getConnectionById(props.connectionId);
    if (!connection) {
      throw new Error("找不到连接配置");
    }

    // 先断开当前连接（如果存在）
    if (isConnected.value) {
      await disconnect();
    }

    // 重新建立SSH连接
    const result = await connectionStore.connect(connection);
    if (result.success) {
      console.log("重新建立连接成功");
      // 连接建立后，重新初始化终端连接
      await connectSSH();
    } else {
      throw new Error(`重新建立连接失败: ${result.message}`);
    }

    console.log("重新连接完成:", props.title);
  } catch (err) {
    error.value = `重新连接失败: ${err}`;
    console.error("重新连接失败:", err);
  } finally {
    isLoading.value = false;
  }
};

// 监听连接ID变化
watch(
  () => props.connectionId,
  (newId, oldId) => {
    // 只有在连接ID真正变化且不为空时才重新连接
    if (newId && newId !== oldId && props.autoConnect) {
      console.log("连接ID变化，重新连接:", oldId, "->", newId);
      connectSSH();
    }
  },
  { immediate: false }
);

// 组件挂载
onMounted(async () => {
  await initTerminal();
  // 只有在明确需要连接时才连接
  if (props.autoConnect && props.connectionId) {
    console.log("组件挂载，开始连接:", props.connectionId);
    await connectSSH();
  } else {
    console.log("组件挂载，跳过自动连接:", {
      autoConnect: props.autoConnect,
      connectionId: props.connectionId,
    });
  }
});

// 组件卸载
onUnmounted(() => {
  if (terminal.value) {
    try {
      terminal.value.dispose();
    } catch (err) {
      console.warn("终端释放时出现警告:", err);
    }
  }
});

// 暴露方法给父组件
defineExpose({
  terminal,
  isConnected,
  isLoading,
  error,
  sessionId,
  connectSSH,
  disconnect,
  reconnect,
});
</script>

<style scoped lang="scss">
.terminal-emulator {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #0d1117;
  overflow: hidden;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(240, 246, 252, 0.1);
  padding: 12px;
  position: relative;
  width: 100%;
  height: 100%;
}

.terminal-container {
  flex: 1;
  position: relative;
  background: #0d1117;
  overflow: hidden;
}

.terminal-loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  background: rgba(13, 17, 23, 0.9);
  backdrop-filter: blur(4px);
  z-index: 10;
}

.terminal-error-overlay {
  position: absolute;
  top: 16px;
  left: 16px;
  right: 16px;
  z-index: 10;
}

.terminal-status-indicator {
  position: absolute;
  top: 16px;
  right: 16px;
  z-index: 5;
}

.loading-text {
  color: #f0f6fc;
  font-size: 14px;
  font-weight: 500;
}

/* 终端样式 - 使用xterm.js官方样式 */
:deep(.xterm) {
  height: 100%;
  font-family: "Fira Code", "JetBrains Mono", "Cascadia Code", Consolas,
    monospace !important;
}

:deep(.xterm .xterm-viewport) {
  font-family: "Fira Code", "JetBrains Mono", "Cascadia Code", Consolas,
    monospace !important;
}

:deep(.xterm .xterm-screen) {
  font-family: "Fira Code", "JetBrains Mono", "Cascadia Code", Consolas,
    monospace !important;
}
</style>
