import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";
import AutoImport from "unplugin-auto-import/vite";
import { NaiveUiResolver } from "unplugin-vue-components/resolvers";
import Components from "unplugin-vue-components/vite";
import { visualizer } from "rollup-plugin-visualizer";
// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(),
    tailwindcss(),
    AutoImport({
      resolvers: [NaiveUiResolver()],
    }),
    Components({
      resolvers: [NaiveUiResolver()],
    }),
    // 打包分析插件 - 始终启用
    visualizer({
      filename: "stats.html",
      open: true,
      gzipSize: true,
      brotliSize: true,
      template: "treemap", // 可选: treemap, sunburst, network
    }),
  ].filter(Boolean),

  // 构建配置
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          // Vue 核心库
          vue: ['vue'],
          
          // Naive UI 组件库
          'naive-ui': ['naive-ui'],
          
          // 图标库
          'vicons-antd': ['@vicons/antd'],
          
          // Tauri API
          'tauri-api': ['@tauri-apps/api']
        }
      }
    },
    // 启用代码分割
    chunkSizeWarningLimit: 1000,
    // 生成 source map（可选）
    sourcemap: false
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));
