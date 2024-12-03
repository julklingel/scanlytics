import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import path from "path";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [sveltekit()],
  resolve: process.env.VITEST
  ? {
      conditions: ['browser']
    }
  : undefined
  ,

  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: ['src/tests/setupTests.ts'],
    include: ['src/**/*.{test,spec}.{js,ts}'],
    deps: {
      inline: [/^svelte/, /@sveltejs/]
    }
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});