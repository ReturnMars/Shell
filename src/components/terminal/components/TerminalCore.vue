<template>
  <div class="h-full b-solid b-1px px-4 py-2">
    <div
      ref="terminalCoreContainer"
      class="terminal-core-container w-full h-full"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { useConnectionStore } from "@/stores/connection/index";
import { Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { WebLinksAddon } from "@xterm/addon-web-links";
import "@xterm/xterm/css/xterm.css";
import {
  config,
  getPrompt,
  splitShellDataToLines,
} from "../config/terminal_config";
import { invoke } from "@tauri-apps/api/core";
const connectionStore = useConnectionStore();
const currentConnection = computed(() => connectionStore.currentConnection);
const isConnected = computed(() => currentConnection.value?.connected);
const terminalCoreContainer = useTemplateRef("terminalCoreContainer");

const terminal = new Terminal(config);

// 命令缓冲区
let commandBuffer = "";

const initTerminal = async () => {
  console.log("=== initTerminal 开始 ===");
  console.log("currentConnection:", currentConnection.value);
  console.log("isConnected:", isConnected.value);

  await nextTick();
  if (!terminalCoreContainer.value) {
    console.log("terminalCoreContainer 不存在");
    return;
  }

  console.log("开始初始化终端...");
  const fitAddon = new FitAddon();
  const webLinksAddon = new WebLinksAddon();
  terminal.loadAddon(fitAddon);
  terminal.loadAddon(webLinksAddon);
  terminal.open(terminalCoreContainer.value);
  fitAddon.fit();

  // 确保终端获得焦点
  terminal.focus();
  console.log("终端初始化完成");

  terminal.writeln(`Hello ${currentConnection.value?.name}!`);
  terminal.writeln(`Welcome to the ShellMars terminal!`);
  if (isConnected.value) {
    terminal.writeln(
      `You are connected to ${currentConnection.value?.host}:${currentConnection.value?.port}!`
    );
    terminal.writeln("");

    await updatePrompt();
    writelnPrompt();
  } else {
    terminal.writeln(`You are not connected to any server!`);
    terminal.writeln("");
    writelnPrompt();
  }

  // onKey: 处理键盘事件和命令执行
  terminal.onKey(async (e: { key: string; domEvent: KeyboardEvent }) => {
    const ev = e.domEvent;
    const printable = !ev.altKey && !ev.ctrlKey && !ev.metaKey;

    // 检查连接状态
    if (!currentConnection.value?.connected) {
      console.log("连接未建立，跳过处理");
      terminal.write(`\r\n错误: SSH 连接未建立\r\n`);
      writelnPrompt();
      return;
    }

    // 处理特殊字符
    switch (ev.key) {
      case "Enter":
        await enterKeyHandler();
        break;
      case "Backspace":
        backspaceKeyHandler();
        break;
      default:
        if (ev.ctrlKey && ev.key === "c") {
          ctrlCKeyHandler();
        } else if (printable && e.key.length === 1) {
          printableKeyHandler(e);
        }
        break;
    }
  });
};

/**
 * 可打印字符处理
 */
const printableKeyHandler = (e: { key: string; domEvent: KeyboardEvent }) => {
  // 可打印字符：添加到缓冲区
  commandBuffer += e.key;
  terminal.write(e.key);
};

/**
 * 回车键处理
 */
const enterKeyHandler = async () => {
  if (commandBuffer.trim()) {
    try {
      const result = await invoke("execute_ssh_command", {
        connectionId: currentConnection.value?.id,
        command: commandBuffer,
      });
      // 将后端响应显示到终端
      if (result && typeof result === "string") {
        const { lines, prompt } = splitShellDataToLines(result);

        // 显示命令输出
        for (const line of lines) {
          terminal.writeln(line);
        }

        // 更新提示符并显示
        if (shouldUpdatePrompt(commandBuffer)) {
          currentPrompt.value = prompt;
        }
      }
    } catch (error) {
      console.error("执行命令失败:", error);
      terminal.write(`\r\n错误: ${error}\r\n`);
    }
  } else {
    terminal.writeln("");
  }

  // 清空缓冲区
  commandBuffer = "";

  writelnPrompt();
};

/**
 * 退格键处理
 */
const backspaceKeyHandler = () => {
  // 退格：删除缓冲区最后一个字符
  if (commandBuffer.length > 0) {
    commandBuffer = commandBuffer.slice(0, -1);
    terminal.write("\b \b");
  }
};

/**
 * Ctrl+C 键处理 中断命令
 */
const ctrlCKeyHandler = () => {
  terminal.write("^C");
  commandBuffer = "";
  terminal.writeln("");
  writelnPrompt();
};

const currentPrompt = ref<string>(getPrompt());

/**
 * 判断命令是否需要更新提示符
 * @param command 用户输入的命令
 * @returns 是否需要更新提示符
 */
const shouldUpdatePrompt = (command: string): boolean => {
  const trimmedCommand = command.trim().toLowerCase();

  // 需要更新提示符的命令
  const promptUpdateCommands = [
    "cd", // 改变目录
    "su", // 切换用户
    "sudo", // 以其他用户身份执行
    "exit", // 退出
    "logout", // 登出
    "source", // 加载环境变量
    ".", // 加载脚本
    "export", // 设置环境变量
    "unset", // 取消环境变量
  ];

  // 检查是否是需要更新提示符的命令
  for (const cmd of promptUpdateCommands) {
    if (trimmedCommand.startsWith(cmd)) {
      return true;
    }
  }

  return false;
};

// 定义本地的 prompt 函数
const writelnPrompt = () => {
  terminal.write(currentPrompt.value);
};
/**
 * 更新提示符
 * @param result 如果为空，则执行 whoami 命令，获取用户名
 * 如果结果不为空，则直接使用结果
 */
const updatePrompt = async (result?: string | number) => {
  let userResult = result;
  if (!result) {
    const remoteResult = await invoke("execute_ssh_command", {
      connectionId: currentConnection.value?.id,
      command: "whoami",
    });
    if (remoteResult && typeof remoteResult === "string") {
      userResult = remoteResult;
    }
  }

  currentPrompt.value = getPrompt(userResult);
};
onMounted(() => {
  initTerminal();
});
</script>
<style scoped lang="scss">
.terminal-core-container {
  // 确保 xterm.js 正确显示
  :deep(.xterm) {
    // 修复字体显示问题
    .xterm-rows {
      line-height: 1.2 !important;
    }

    // 隐藏测量元素
    .xterm-char-measure-element {
      position: absolute !important;
      top: -9999px !important;
      left: -9999px !important;
      visibility: hidden !important;
    }
  }
}
</style>
