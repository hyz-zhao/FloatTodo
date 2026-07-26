import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// Tauri 推荐的 Vite 配置：固定端口、关闭自动打开浏览器、不清理 src-tauri 目录
const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [vue()],
  // 防止 Vite 遮蔽 Rust 编译错误
  clearScreen: false,
  // Tauri 期望固定端口
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
      // 忽略 Rust 端变更，避免重启 Vite
      ignored: ["**/src-tauri/**"],
    },
  },
}));
