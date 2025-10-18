<template>
  <n-config-provider inline-theme-disabled>
    <MainLayout>
      <template #main-content>
        <!-- 欢迎界面 -->
        <div style="height: 100%; display: flex; align-items: center; justify-content: center; background: #f5f5f5;">
          <div style="text-align: center;">
            <!-- Tailwind 测试 -->
          
            <div style="font-size: 64px; margin-bottom: 16px;">🚀</div>
            <h2 style="font-size: 24px; font-weight: 600; margin-bottom: 8px; color: #333;">欢迎使用 TerminalMars</h2>
            <p style="color: #666; margin-bottom: 24px;">现代化的SSH终端工具</p>

            <!-- 快速开始卡片 -->
            <n-card style="max-width: 400px; margin: 0 auto;" hoverable>
              <div>
                <h3 style="font-size: 18px; font-weight: 600; text-align: center; margin-bottom: 8px;">快速开始</h3>
                <p style="font-size: 14px; color: #666; text-align: center; margin-bottom: 16px;">
                  基于 Tauri + Vue + Naive UI 构建
                </p>

                <!-- 测试表单 -->
                <n-form @submit.prevent="greet">
                  <n-form-item>
                    <n-input-group>
                      <n-input v-model:value="name" placeholder="输入名称..." style="flex: 1;" />
                      <n-button type="primary" @click="greet">问候</n-button>
                    </n-input-group>
                  </n-form-item>
                </n-form>

                <n-alert v-if="greetMsg" type="success" style="margin-top: 16px;">
                  {{ greetMsg }}
                </n-alert>

                <!-- 快速操作按钮 -->
                <div style="display: flex; justify-content: center; gap: 8px; margin-top: 16px;">
                  <n-button type="primary" size="small">新建连接</n-button>
                  <n-button size="small">导入配置</n-button>
                </div>
              </div>
            </n-card>
          </div>
        </div>
      </template>
    </MainLayout>
  </n-config-provider>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import MainLayout from "./components/core/MainLayout.vue";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<style scoped>
/* 可以在这里添加App特定的样式 */
</style>