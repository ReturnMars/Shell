<template>
  <div class="custom-terminal">
    <!-- 终端头部 -->
    <div class="terminal-header">
      <div class="terminal-title">
        {{ title || "Terminal" }}
      </div>
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

    <!-- 终端内容区域 -->
    <div class="terminal-content" ref="terminalContent" @click="focusInput">
      <div class="terminal-output" ref="terminalOutput">
        <div
          v-for="(line, index) in outputLines"
          :key="index"
          class="output-line"
          :class="{
            'command-line': line.type === 'command',
            'output-line': line.type === 'output',
            'error-line': line.type === 'error',
          }"
        >
          <span v-if="line.type === 'command'" class="prompt">{{
            line.prompt
          }}</span>
          <span class="content" v-html="formatContent(line.content)"></span>
        </div>

        <!-- 当前输入行 - 使用textarea -->
        <div class="current-line" v-if="showPrompt">
          <span class="prompt" v-html="formatContent(currentPrompt)"></span>
          <textarea
            ref="inputTextarea"
            v-model="currentInput"
            class="input-textarea"
            @keydown="handleKeyDown"
            @input="handleInput"
            @paste="handlePaste"
            @focus="handleFocus"
            @blur="handleBlur"
            spellcheck="false"
            autocomplete="off"
            autocorrect="off"
            autocapitalize="off"
        
          ></textarea>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Convert from "ansi-to-html";

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
const terminalContent = ref<HTMLElement>();
const terminalOutput = ref<HTMLElement>();
const inputTextarea = ref<HTMLTextAreaElement>();
const currentInput = ref("");
const outputLines = ref<
  Array<{
    type: "command" | "output" | "error";
    content: string;
    prompt?: string;
  }>
>([]);
const commandHistory = ref<string[]>([]);
const historyIndex = ref(-1);
const isConnected = ref(false);
const showPrompt = ref(true);
const currentPrompt = ref("$ ");

// ANSI转换器
const convert = new Convert({
  fg: "#b1bac4", // 默认前景色
  bg: "#0d1117", // 默认背景色
  newline: true, // 转换换行符
  escapeXML: true, // 转义XML
  colors: {
    0: "#484f58", // 黑色
    1: "#ff6b6b", // 红色
    2: "#51cf66", // 绿色
    3: "#ffd43b", // 黄色
    4: "#74c0fc", // 蓝色
    5: "#da77f2", // 紫色
    6: "#3bc9db", // 青色
    7: "#b1bac4", // 白色
  },
});

// 初始化终端
const initTerminal = async () => {
  // 显示欢迎信息
  addOutputLine("output", "欢迎使用自定义终端");
  addOutputLine("output", "正在初始化...");

  // 如果自动连接，尝试连接SSH
  if (props.autoConnect && props.connectionId) {
    await connectSSH();
  } else {
    // 如果没有自动连接，显示提示
    addOutputLine("output", "请选择一个SSH连接");
  }

  // 聚焦到输入区域
  await nextTick();
  focusInput();
};

// 连接SSH
const connectSSH = async () => {
  if (!props.connectionId) return;

  try {
    // 先检查连接状态
    const status = await invoke("get_connection_status", {
      connectionId: props.connectionId,
    });

    if (status === "Connected") {
      // 已经连接，直接更新状态
      isConnected.value = true;
      // SSH连接已存在 这句不存在才输出这句话
      const contentLine = `SSH连接: ${props.title}`;
      if (
        !outputLines.value.some(
          (line) => line.type === "output" && line.content === contentLine
        )
      ) {
        addOutputLine("output", contentLine);
      }
      // const result = await executeCommand("whoami", false);
      // const lines = result?.split("\n");
      // if (!lines?.length) return;
      // currentPrompt.value = lines[lines.length - 1];
      // console.log("🚀 ~ connectSSH ~ lines:", lines);
      // console.log("🚀 ~ connectSSH ~ result:", result);
      getWhoiamAndPwd();
      return;
    }

    // 先获取连接配置
    const connections = await invoke<any[]>("get_connections");
    const connection = connections.find(
      (c: any) => c.id === props.connectionId
    );

    if (!connection) {
      throw new Error("连接配置不存在");
    }

    // 调用SSH连接逻辑，传递完整的连接配置
    await invoke("connect_ssh", { config: connection });

    isConnected.value = true;
    addOutputLine("output", "SSH连接已建立");
    // 执行命令或者当前用户
    const result = await executeCommand("whoami", false);
    console.log("🚀 ~ connectSSH ~ result:", result);
    // currentPrompt.value = `${whoami}@${hostname}:~$ `;
    emit("connected", `session_${Date.now()}`);
  } catch (error) {
    addOutputLine("error", `连接失败: ${error}`, "user@hostname:~$ ");
    emit("error", String(error));
  }
};

// 添加输出行
const addOutputLine = (
  type: "command" | "output" | "error",
  content: string,
  prompt?: string
) => {
  outputLines.value.push({ type, content, prompt });
  scrollToBottom();
};

// 执行命令
const executeCommand = async (command: string, showCommand: boolean = true) => {
  if (!command) return;

  // 添加到历史记录
  commandHistory.value.push(command);
  historyIndex.value = commandHistory.value.length;

  // 清空当前输入
  currentInput.value = "";
  // output 输出
  let output: string | undefined;
  try {
    // 处理特殊命令
    if (command === "clear") {
      // 显示命令
      if (showCommand) {
        addOutputLine("command", command, currentPrompt.value);
      }
      outputLines.value = [];
      return;
    }

    if (command === "exit") {
      // 显示命令
      if (showCommand) {
        addOutputLine("command", command, currentPrompt.value);
      }
      
      // 真正断开SSH连接
      if (isConnected.value && props.connectionId) {
        try {
          await invoke("disconnect_ssh", {
            connectionId: props.connectionId,
          });
          console.log("SSH连接已断开");
        } catch (error) {
          console.error("断开SSH连接失败:", error);
        }
      }
      
      isConnected.value = false;
      addOutputLine("output", "连接已断开");
      emit("disconnected");
      return;
    }

    // 显示命令（对于非特殊命令）
    if (showCommand) {
      addOutputLine("command", command, currentPrompt.value);
    }
    // 执行SSH命令
    if (isConnected.value && props.connectionId) {
      output = await invoke<string>("execute_ssh_command", {
        connectionId: props.connectionId,
        command: command + "\n",
      });
      if (output) {
        // 只移除终端控制序列，保留ANSI颜色代码用于渲染
        const cleanedOutput = output
          .replace(/\x1b\]0;.*?\x07/g, "") // 移除终端标题设置
          .replace(/\x1b\[[0-9]*[A-Z]/g, "") // 移除其他ANSI控制序列
          .replace(/\r/g, "") // 移除回车符
          .trim();

        // 分割输出行并添加
        const lines = cleanedOutput.split("\n");
        lines.pop();
        lines.shift();
        lines.forEach((line) => {
          if (line.trim() && showCommand) {
            addOutputLine("output", line);
          }
        });
        // 对于特定命令 更新用户信息和当前目录
        if (shouldUpdatePrompt(command)) {
          getWhoiamAndPwd(output);
        }
      }
    } else {
      // 模拟本地命令
      output = await simulateLocalCommand(command);
      if (output) {
        addOutputLine("output", output);
      }
    }
  } catch (error) {
    output = `命令执行失败: ${error}`;
    addOutputLine("error", output);
  } finally {
    return output;
  }
};
//
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
  switch (event.key) {
    case "Enter":
      event.preventDefault();
      await executeCommand(currentInput.value);
      break;

    case "ArrowUp":
      event.preventDefault();
      if (historyIndex.value > 0) {
        historyIndex.value--;
        currentInput.value = commandHistory.value[historyIndex.value];
      }
      break;

    case "ArrowDown":
      event.preventDefault();
      if (historyIndex.value < commandHistory.value.length - 1) {
        historyIndex.value++;
        currentInput.value = commandHistory.value[historyIndex.value];
      } else {
        historyIndex.value = commandHistory.value.length;
        currentInput.value = "";
      }
      break;

    case "Tab":
      event.preventDefault();
      // 简单的命令补全
      const partial = currentInput.value;
      const matches = commandHistory.value.filter((cmd) =>
        cmd.startsWith(partial)
      );
      if (matches.length === 1) {
        currentInput.value = matches[0];
      }
      break;

    case "c":
      if (event.ctrlKey) {
        event.preventDefault();
        currentInput.value = "";
        addOutputLine("output", "^C");
      }
      break;

    case "l":
      if (event.ctrlKey) {
        event.preventDefault();
        outputLines.value = [];
      }
      break;
  }
};
// 判断命令是否需要更新提示符
const shouldUpdatePrompt = (command: string): boolean => {
  const cmd = command.toLowerCase().trim();
  
  // 需要更新提示符的命令列表
  const promptUpdateCommands = [
    'cd',           // 改变目录
    'su',           // 切换用户
    'sudo',         // 以管理员权限执行
    'login',        // 登录
    'logout',       // 登出
    'exit',         // 退出（可能影响提示符）
    'bash',         // 启动新的bash会话
    'zsh',          // 启动zsh会话
    'sh',           // 启动sh会话
    'source',       // 执行脚本文件
    'exec',         // 执行命令
    'env',          // 环境变量相关
    'export',       // 导出环境变量
    'unset',        // 取消环境变量
    'alias',        // 设置别名
    'unalias',      // 取消别名
    'history',      // 历史命令（某些情况下可能影响提示符）
  ];
  
  // 检查是否匹配需要更新的命令
  for (const updateCmd of promptUpdateCommands) {
    if (cmd === updateCmd || cmd.startsWith(updateCmd + ' ')) {
      return true;
    }
  }
  
  // 特殊处理：cd命令的各种变体
  if (cmd.startsWith('cd ') || cmd === 'cd') {
    return true;
  }
  
  // 特殊处理：su命令的各种变体
  if (cmd.startsWith('su ') || cmd === 'su') {
    return true;
  }
  
  // 特殊处理：sudo命令的各种变体
  if (cmd.startsWith('sudo ')) {
    return true;
  }
  
  return false;
};

// 获取用户信息和当前目录
const getWhoiamAndPwd = async (result?: string) => {
  let userResult = result;
  try {
    if (!result) {
      // 获取用户名
      userResult = await executeCommand("whoami", false);
    }
    console.log("🚀 ~ getWhoiamAndPwd ~ userResult:", userResult);

    if (userResult === null || userResult === undefined) return;

    // 正则表达式匹配提示符格式 [user@hostname directory]
    const promptRegex =
      /\[([a-zA-Z0-9_-]+@[a-zA-Z0-9_.-]+\s+[a-zA-Z0-9_./~-]+)\]/g;
    const promptMatches = userResult?.match(promptRegex) || [];

    if (promptMatches.length > 0) {
      // 过滤掉包含ANSI转义序列的匹配项
      const validPrompts = promptMatches.filter(
        (match) =>
          !match.includes("\x1b") &&
          !match.includes("\u001b") &&
          match.includes("@") &&
          match.length < 100 // 限制长度，避免匹配到过长的内容
      );

      if (validPrompts.length > 0) {
        const lastPrompt = validPrompts[validPrompts.length - 1];
        currentPrompt.value = `${lastPrompt}# `;
        console.log("🚀 ~ 提取的提示符:", lastPrompt);
      } else {
        // 如果没有找到有效提示符，使用默认值
        currentPrompt.value = "user@localhost:~# ";
        console.log("🚀 ~ 未找到有效提示符，使用默认值");
      }
    } else {
      // 如果没有找到标准格式，使用默认提示符
      currentPrompt.value = "user@localhost:~# ";
      console.log("🚀 ~ 未找到提示符格式，使用默认值");
    }
  } catch (error) {
    console.error("获取用户信息失败:", error);
    // 使用默认提示符
    currentPrompt.value = "user@localhost:~$ ";
  }
};

// 处理输入事件
const handleInput = () => {
  // textarea 自动处理输入，这里可以添加额外逻辑
};

// 处理粘贴事件
const handlePaste = () => {
  // 让浏览器处理粘贴，这里可以添加额外逻辑
};

const handleFocus = () => {
  // 获得焦点
};

const handleBlur = () => {
  // 失去焦点
};

// 聚焦输入区域
const focusInput = () => {
  if (inputTextarea.value) {
    inputTextarea.value.focus();
  }
};

// 滚动到底部
const scrollToBottom = async () => {
  await nextTick();
  if (terminalOutput.value) {
    terminalOutput.value.scrollTop = terminalOutput.value.scrollHeight;
  }
};

// 格式化内容
const formatContent = (content: string): string => {
  // 使用 ansi-to-html 库处理ANSI转义序列
  return convert.toHtml(content);
};

// 监听连接ID变化
watch(
  () => props.connectionId,
  (newId, oldId) => {
    if (newId && newId !== oldId && props.autoConnect) {
      console.log("连接ID变化，重新连接:", oldId, "->", newId);
      connectSSH();
    }
  }
);

// 监听连接状态变化
watch(
  () => props.autoConnect,
  (newAutoConnect, oldAutoConnect) => {
    if (
      newAutoConnect &&
      newAutoConnect !== oldAutoConnect &&
      props.connectionId
    ) {
      console.log(
        "自动连接状态变化，重新连接:",
        oldAutoConnect,
        "->",
        newAutoConnect
      );
      connectSSH();
    }
  }
);

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
  clear: () => {
    outputLines.value = [];
  },
  focus: focusInput,
});
</script>

<style scoped lang="scss">
.custom-terminal {
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
  flex: 1;
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
  display: flex;
  flex-direction: column;
  overflow: hidden;
  position: relative;
}

.terminal-output {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
  scroll-behavior: smooth;

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

.output-line {
  margin-bottom: 4px;
  line-height: 1.5;
  word-wrap: break-word;

  &.command-line {
    .prompt {
      color: #58a6ff;
      font-weight: 500;
    }

    .content {
      color: #f0f6fc;
    }
  }

  &.output-line {
    .content {
      color: #b1bac4;
    }
  }

  &.error-line {
    .content {
      color: #ff6b6b;
    }
  }
}

.current-line {
  display: flex;
  align-items: flex-start;
  margin-bottom: 4px;
  line-height: 1.5;

  .prompt {
    color: #58a6ff;
    font-weight: 500;
    margin-right: 8px;
    white-space: nowrap;
    margin-top: 2px;
  }

  .input-textarea {
    color: #f0f6fc;
    background: transparent;
    border: none;
    outline: none;
    resize: none;
    flex: 1;
    font-family: "Fira Code", "JetBrains Mono", "Cascadia Code", Consolas,
      monospace;
    font-size: inherit;
    line-height: inherit;
    padding: 0;
    margin: 0;
    min-height: 1.5em;
    overflow: hidden;

    // 自定义光标样式
    caret-color: #58a6ff; // 光标颜色

    // 通过增加字体粗细来让光标看起来更宽
    font-weight: 500;

    // 使用 letter-spacing 来增加字符间距，间接影响光标宽度
    letter-spacing: 1px;

    &:focus {
      outline: none;
    }
  }
}

// 响应式设计
@media (max-width: 768px) {
  .terminal-header {
    padding: 8px 12px;
  }

  .terminal-output {
    padding: 12px;
  }

  .current-line {
    padding: 0 12px 12px;
  }
}
</style>
