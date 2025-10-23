<template>
  <div class="modern-terminal" ref="terminalRef" @click="focusTerminal">
    <!-- 终端头部 -->
    <div class="terminal-header">
      <div class="terminal-title">{{ title || "Terminal" }}</div>
      <div class="terminal-status">
        <div v-if="isConnected" class="status-indicator connected">
          <div class="status-dot"></div>
          <span>已连接</span>
        </div>
        <div v-else class="status-indicator disconnected">
          <div class="status-dot"></div>
          <span>未连接</span>
        </div>
      </div>
    </div>

    <!-- 终端内容 -->
    <div class="terminal-content" ref="terminalContent">
      <div
        class="terminal-output"
        ref="terminalOutput"
        contenteditable="true"
        @keydown="handleKeyDown"
        @input="handleInput"
        @paste="handlePaste"
        @focus="handleFocus"
        @blur="handleBlur"
        spellcheck="false"
        autocomplete="off"
        autocorrect="off"
        autocapitalize="off"
      ></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";

// Props
interface Props {
  connectionId?: string;
  title?: string;
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

// 响应式数据
const terminalRef = ref<HTMLElement>();
const terminalContent = ref<HTMLElement>();
const terminalOutput = ref<HTMLElement>();
const isConnected = ref(false);
const currentPrompt = ref("$ ");
const commandHistory = ref<string[]>([]);
const historyIndex = ref(-1);
const isProcessingCommand = ref(false);

// 初始化终端
const initTerminal = async () => {
  if (!terminalOutput.value) return;

  // 设置初始内容
  terminalOutput.value.innerHTML = `
    <div class="welcome-line">欢迎使用现代终端</div>
    <div class="welcome-line">正在初始化...</div>
    <div class="input-line">
      <span class="prompt">${currentPrompt.value}</span>
      <span class="input-content" contenteditable="true"> </span>
    </div>
  `;

  // 聚焦到输入区域
  await nextTick();
  focusInput();

  // 如果自动连接，尝试连接SSH
  if (props.autoConnect && props.connectionId) {
    await connectSSH();
  }
};

// 连接SSH
const connectSSH = async () => {
  if (!props.connectionId) return;

  try {
    addOutputLine(`正在连接到 ${props.title || "SSH服务器"}...`);
    await invoke("connect_ssh", { connectionId: props.connectionId });

    isConnected.value = true;
    addOutputLine("SSH连接已建立");
    currentPrompt.value = "user@hostname:~$ ";

    emit("connected", `session_${Date.now()}`);
  } catch (error) {
    addOutputLine(`连接失败: ${error}`, "error");
    emit("error", String(error));
  }
};

// 添加输出行
const addOutputLine = (
  content: string,
  type: "output" | "error" = "output"
) => {
  if (!terminalOutput.value) return;

  const line = document.createElement("div");
  line.className = `output-line ${type}-line`;
  line.textContent = content;

  // 插入到最后一个输入行之前
  const inputLine = terminalOutput.value.querySelector(".input-line");
  if (inputLine) {
    terminalOutput.value.insertBefore(line, inputLine);
  } else {
    terminalOutput.value.appendChild(line);
  }

  scrollToBottom();
};

// 执行命令
const executeCommand = async (command: string) => {
  if (!command.trim() || isProcessingCommand.value) return;

  isProcessingCommand.value = true;

  try {
    // 添加到历史记录
    commandHistory.value.push(command);
    historyIndex.value = commandHistory.value.length;

    // 显示命令
    addCommandLine(command);

    // 清空当前输入
    clearCurrentInput();

    // 处理特殊命令
    if (command === "clear") {
      clearTerminal();
      return;
    }

    if (command === "exit") {
      isConnected.value = false;
      addOutputLine("连接已断开");
      emit("disconnected");
      return;
    }

    // 执行SSH命令
    if (isConnected.value && props.connectionId) {
      const output = await invoke<string>("execute_ssh_command", {
        connectionId: props.connectionId,
        command: command + "\n",
      });

      if (output) {
        const lines = output.split("\n");
        lines.forEach((line) => {
          if (line.trim()) {
            addOutputLine(line);
          }
        });
      }
    } else {
      // 模拟本地命令
      const result = await simulateLocalCommand(command);
      if (result) {
        addOutputLine(result);
      }
    }
  } catch (error) {
    addOutputLine(`命令执行失败: ${error}`, "error");
  } finally {
    isProcessingCommand.value = false;
    // 添加新的输入行
    addNewInputLine();
  }
};

// 添加命令行
const addCommandLine = (command: string) => {
  if (!terminalOutput.value) return;

  const line = document.createElement("div");
  line.className = "command-line";
  line.innerHTML = `<span class="prompt">${currentPrompt.value}</span><span class="command">${command}</span>`;

  // 插入到最后一个输入行之前
  const inputLine = terminalOutput.value.querySelector(".input-line");
  if (inputLine) {
    terminalOutput.value.insertBefore(line, inputLine);
  } else {
    terminalOutput.value.appendChild(line);
  }

  scrollToBottom();
};

// 添加新的输入行
const addNewInputLine = () => {
  if (!terminalOutput.value) return;

  const inputLine = document.createElement("div");
  inputLine.className = "input-line";
  inputLine.innerHTML = `
    <span class="prompt">${currentPrompt.value}</span>
    <span class="input-content" contenteditable="true"> </span>
  `;

  // 移除旧的输入行
  const oldInputLine = terminalOutput.value.querySelector(".input-line");
  if (oldInputLine) {
    oldInputLine.remove();
  }

  terminalOutput.value.appendChild(inputLine);
  focusInput();
};

// 清空当前输入
const clearCurrentInput = () => {
  const inputContent = terminalOutput.value?.querySelector(".input-content") as HTMLElement;
  if (inputContent) {
    inputContent.textContent = " ";
  }
};

// 清空终端
const clearTerminal = () => {
  if (!terminalOutput.value) return;

  terminalOutput.value.innerHTML = `
    <div class="input-line">
      <span class="prompt">${currentPrompt.value}</span>
      <span class="input-content" contenteditable="true"> </span>
    </div>
  `;

  focusInput();
};

// 模拟本地命令
const simulateLocalCommand = async (command: string): Promise<string> => {
  const cmd = command.toLowerCase();

  switch (cmd) {
    case "help":
      return `可用命令:
  help     - 显示帮助信息
  clear    - 清屏
  exit     - 退出终端
  pwd      - 显示当前目录
  ls       - 列出文件
  whoami   - 显示当前用户
  date     - 显示当前时间`;

    case "pwd":
      return "/home/user";

    case "ls":
      return `Desktop    Documents  Downloads  Pictures  Videos
Music     Public    Templates  bin       src`;

    case "whoami":
      return "user";

    case "date":
      return new Date().toString();

    default:
      return `命令未找到: ${command}`;
  }
};

// 处理键盘事件
const handleKeyDown = async (event: KeyboardEvent) => {
  const target = event.target as HTMLElement;

  // 只处理输入区域的键盘事件
  if (!target.classList.contains("input-content")) {
    return;
  }

  switch (event.key) {
    case "Enter":
      event.preventDefault();
      const command = target.textContent?.trim() || "";
      await executeCommand(command);
      break;

    case "ArrowUp":
      event.preventDefault();
      if (historyIndex.value > 0) {
        historyIndex.value--;
        target.textContent = commandHistory.value[historyIndex.value] || " ";
      }
      break;

    case "ArrowDown":
      event.preventDefault();
      if (historyIndex.value < commandHistory.value.length - 1) {
        historyIndex.value++;
        target.textContent = commandHistory.value[historyIndex.value] || " ";
      } else {
        historyIndex.value = commandHistory.value.length;
        target.textContent = " ";
      }
      break;

    case "Tab":
      event.preventDefault();
      // 简单的命令补全
      const partial = target.textContent?.trim() || "";
      const matches = commandHistory.value.filter((cmd) =>
        cmd.startsWith(partial)
      );
      if (matches.length === 1) {
        target.textContent = matches[0];
      }
      break;

    case "c":
      if (event.ctrlKey) {
        event.preventDefault();
        target.textContent = " ";
        addOutputLine("^C");
      }
      break;

    case "l":
      if (event.ctrlKey) {
        event.preventDefault();
        clearTerminal();
      }
      break;
  }
};

// 处理输入事件
const handleInput = (event: Event) => {
  const target = event.target as HTMLElement;
  if (target.classList.contains("input-content")) {
    // 确保输入区域始终有内容
    if (!target.textContent || target.textContent.trim() === "") {
      target.textContent = " ";
    }
  }
};

// 处理粘贴事件
const handlePaste = (event: ClipboardEvent) => {
  event.preventDefault();
  const text = event.clipboardData?.getData("text") || "";
  const target = event.target as HTMLElement;

  if (target.classList.contains("input-content")) {
    target.textContent = text;
  }
};

// 聚焦终端
const focusTerminal = () => {
  focusInput();
};

// 聚焦输入区域
const focusInput = () => {
  const inputContent = terminalOutput.value?.querySelector(".input-content") as HTMLElement;
  if (inputContent) {
    inputContent.focus();
    // 将光标移到末尾
    const range = document.createRange();
    const sel = window.getSelection();
    range.selectNodeContents(inputContent);
    range.collapse(false);
    sel?.removeAllRanges();
    sel?.addRange(range);
  }
};

// 焦点事件
const handleFocus = () => {
  // 终端获得焦点
};

const handleBlur = () => {
  // 终端失去焦点
};

// 滚动到底部
const scrollToBottom = () => {
  if (terminalContent.value) {
    terminalContent.value.scrollTop = terminalContent.value.scrollHeight;
  }
};

// 组件挂载
onMounted(() => {
  initTerminal();
});

// 组件卸载
onUnmounted(() => {
  // 清理工作
});

// 暴露方法
defineExpose({
  executeCommand,
  addOutputLine,
  clear: clearTerminal,
  focus: focusInput,
});
</script>

<style scoped lang="scss">
.modern-terminal {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #0d1117;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(240, 246, 252, 0.1);
  font-family: "Fira Code", "JetBrains Mono", "Cascadia Code", Consolas,
    monospace;
}

.terminal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 16px;
  background: #161b22;
  border-bottom: 1px solid rgba(240, 246, 252, 0.1);
  user-select: none;
}

.terminal-title {
  color: #f0f6fc;
  font-size: 14px;
  font-weight: 500;
}

.terminal-status {
  display: flex;
  align-items: center;
  gap: 8px;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;

  &.connected {
    color: #51cf66;
  }

  &.disconnected {
    color: #ff6b6b;
  }
}

.status-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: currentColor;
}

.terminal-content {
  flex: 1;
  overflow-y: auto;
  padding: 16px;

  &::-webkit-scrollbar {
    width: 8px;
  }

  &::-webkit-scrollbar-track {
    background: #161b22;
  }

  &::-webkit-scrollbar-thumb {
    background: #30363d;
    border-radius: 4px;
  }

  &::-webkit-scrollbar-thumb:hover {
    background: #484f58;
  }
}

.terminal-output {
  min-height: 100%;
  outline: none;

  .welcome-line {
    color: #fff;
    margin-bottom: 4px;
    line-height: 1.5;
  }

  .output-line {
    color: #fff;
    margin-bottom: 4px;
    line-height: 1.5;
    white-space: pre-wrap;

    &.error-line {
      color: #ff6b6b;
    }
  }

  .command-line {
    margin-bottom: 4px;
    line-height: 1.5;

    .prompt {
      color: #58a6ff;
      font-weight: 500;
    }

    .command {
      color: #f0f6fc;
    }
  }

  .input-line {
    display: flex;
    align-items: center;
    margin-bottom: 4px;
    line-height: 1.5;

    .prompt {
      color: #58a6ff;
      font-weight: 500;
      margin-right: 8px;
    }

    .input-content {
      color: #f0f6fc;
      flex: 1;
      outline: none;
      min-width: 1px;

      &:empty::before {
        content: " ";
        color: transparent;
      }

      &:focus {
        outline: none;
      }
    }
  }
}

// 光标样式
.terminal-output:focus .input-line .input-content {
  position: relative;

  &::after {
    content: "|";
    color: #58a6ff;
    font-weight: bold;
    animation: blink 1s infinite;
  }
}

@keyframes blink {
  0%,
  50% {
    opacity: 1;
  }
  51%,
  100% {
    opacity: 0;
  }
}
</style>
