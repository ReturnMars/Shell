<template>
  <div class="simple-terminal">
    <!-- 连接状态 -->
    <div class="terminal-header">
      <n-tag v-if="isConnected" type="success" size="small">
        <template #icon>
          <n-icon><CheckCircleOutlined /></n-icon>
        </template>
        已连接
      </n-tag>
      <n-button
        v-else
        size="small"
        type="primary"
        @click="connect"
        :loading="isLoading"
      >
        连接
      </n-button>
    </div>

    <!-- 终端容器 -->
    <div ref="terminalContainer" class="terminal-container"></div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from "vue";
import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { CheckCircleOutlined } from "@vicons/antd";
import { useConnectionStore } from "@/stores/connection";
import { invoke } from "@tauri-apps/api/core";

// Props
interface Props {
  connectionId: string;
  title?: string;
}

const props = withDefaults(defineProps<Props>(), {
  title: "SSH连接",
});

// 状态
const terminalContainer = ref<HTMLElement>();
const terminal = ref<Terminal>();
const fitAddon = ref<FitAddon>();
const isConnected = ref(false);
const isLoading = ref(false);

// Store
const connectionStore = useConnectionStore();

// 初始化终端
const initTerminal = async () => {
  if (!terminalContainer.value) return;

  // 创建终端实例
  terminal.value = new Terminal({
    theme: {
      background: "#1e1e1e",
      foreground: "#ffffff",
      cursor: "#ffffff",
    },
    fontFamily: "'Fira Code', 'JetBrains Mono', Consolas, monospace",
    fontSize: 14,
    lineHeight: 1.2,
    cursorBlink: true,
    cursorStyle: "block",
    scrollback: 1000,
  });

  // 添加适配插件
  fitAddon.value = new FitAddon();
  terminal.value.loadAddon(fitAddon.value);

  // 挂载终端
  terminal.value.open(terminalContainer.value);

  // 适配大小
  await nextTick();
  fitAddon.value.fit();

  // 监听窗口大小变化
  const resizeObserver = new ResizeObserver(() => {
    fitAddon.value?.fit();
  });
  resizeObserver.observe(terminalContainer.value);

  // 监听用户输入
  terminal.value.onData(async (data) => {
    if (!isConnected.value) return;

    // 直接发送到SSH后端
    try {
      const output = await invoke<string>("execute_ssh_command", {
        connectionId: props.connectionId,
        command: data,
      });
      console.log("🚀 ~ initTerminal ~ output:", output);

      if (output && terminal.value) {
        terminal.value.write(output);
      }
    } catch (err) {
      console.error("命令执行失败:", err);
    }
  });

  // 显示欢迎信息
  terminal.value.writeln("欢迎使用 ShellMars 终端");
  terminal.value.writeln(`准备连接到: ${props.title}`);
};

// 连接SSH
const connect = async () => {
  if (!props.connectionId) return;

  try {
    isLoading.value = true;

    // 检查连接状态
    const status = await invoke<
      "Connected" | "Disconnected" | "Connecting" | "Error"
    >("get_connection_status", { connectionId: props.connectionId });

    // 如果未连接，尝试建立连接
    if (status !== "Connected") {
      const connection = connectionStore.getConnectionById(props.connectionId);
      if (connection) {
        await connectionStore.connect(connection);
      }
    }

    // 连接成功
    isConnected.value = true;

    if (terminal.value) {
      terminal.value.clear();
      terminal.value.writeln("SSH连接已建立");
      terminal.value.writeln(`连接到: ${props.title}`);
      terminal.value.write("$ ");
    }
  } catch (err) {
    console.error("SSH连接失败:", err);
    if (terminal.value) {
      terminal.value.writeln(`连接失败: ${err}`);
    }
  } finally {
    isLoading.value = false;
  }
};

// 组件挂载
onMounted(() => {
  initTerminal();
});

// 组件卸载
onUnmounted(() => {
  terminal.value?.dispose();
});
</script>

<style scoped>
.simple-terminal {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: #1e1e1e;
  border-radius: 8px;
  overflow: hidden;
}

.terminal-header {
  padding: 8px 12px;
  background: #2d2d2d;
  border-bottom: 1px solid #404040;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.terminal-container {
  flex: 1;
  background: #1e1e1e;
}

/* xterm.js 样式 */
:deep(.xterm) {
  height: 100%;
  background: #1e1e1e;
}

:deep(.xterm-viewport) {
  background: #1e1e1e;
}

:deep(.xterm-screen) {
  background: #1e1e1e;
}
</style>
