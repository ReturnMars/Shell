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
import { IEvent, Terminal } from "@xterm/xterm";
import { FitAddon } from "@xterm/addon-fit";
import { WebLinksAddon } from "@xterm/addon-web-links";
import "@xterm/xterm/css/xterm.css";
import { config } from "../config/terminal_config";
const connectionStore = useConnectionStore();
const currentConnection = computed(() => connectionStore.currentConnection);
const isConnected = computed(() => currentConnection.value?.connected);
const terminalCoreContainer = useTemplateRef("terminalCoreContainer");

const terminal = new Terminal(config);

const initTerminal = async () => {
  await nextTick();
  if (!terminalCoreContainer.value) return;
  const fitAddon = new FitAddon();
  const webLinksAddon = new WebLinksAddon();
  terminal.loadAddon(fitAddon);
  terminal.loadAddon(webLinksAddon);
  terminal.open(terminalCoreContainer.value);
  fitAddon.fit();

  terminal.writeln(`Hello ${currentConnection.value?.name}!`);
  terminal.writeln(`Welcome to the ShellMars terminal!`);
  if (isConnected.value) {
    terminal.writeln(
      `You are connected to ${currentConnection.value?.host}:${currentConnection.value?.port}!`
    );
    terminal.writeln("");
    writelnPrompt();
  } else {
    terminal.writeln(`You are not connected to any server!`);
    terminal.writeln("");
    writelnPrompt();
  }

  // 使用官方示例的键盘事件处理方式
  terminal.onKey((e: { key: string; domEvent: KeyboardEvent }) => {
    const ev = e.domEvent;
    const printable = !ev.altKey && !ev.ctrlKey && !ev.metaKey;

    if (ev.key === "Enter") {
      writelnPrompt();
    } else if (ev.key === "Backspace") {
      // Do not delete the prompt
      if ((terminal as any)._core.buffer.x > 2) {
        terminal.write("\b \b");
      }
    } else if (printable) {
      terminal.write(e.key);
    }
  });
};
// 定义本地的 prompt 函数
const writelnPrompt = () => {
  terminal.write("\r\n$ ");
};
onMounted(() => {
  initTerminal();
});
</script>
<style scoped lang="scss"></style>
