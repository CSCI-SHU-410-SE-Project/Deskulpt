import { defineConfig } from "vite";
import { resolve } from "path";
import react from "@vitejs/plugin-react";
import path from "path";
import { PreRenderedChunk } from "rollup";
// import copy from "rollup-plugin-copy";
// import { createHtmlPlugin } from "vite-plugin-html";

function normalizePath(p: string) {
  return p.replace(/\\/g, "/");
}

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      "@deskulpt/react": path.resolve(__dirname, "src/@deskulpt/react/index.ts"),
      "@deskulpt/apis": path.resolve(__dirname, "src/@deskulpt/apis/index.ts"),
    },
  },
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "views/index.html"),
        canvas: resolve(__dirname, "views/canvas.html"),
      },
      output: {
        manualChunks(id) {
          // Normalize the incoming module ID and the base path for @deskulpt
          const normalizedId = normalizePath(id);
          const deskulptPath = normalizePath(path.resolve(__dirname, "src/@deskulpt"));

          if (normalizedId.includes(deskulptPath)) {
            console.log(normalizedId);
            // Map `__diranme/src/@deskulpt/**/*` to `__dirname/dist/@deskulpt/**/*`
            const pathParts = normalizedId.split("/");
            const deskulptIndex = pathParts.indexOf("@deskulpt");
            const specificPath = pathParts
              .slice(deskulptIndex)
              .join("/")
              .replace(/\.tsx?$/, "");
            return `${specificPath}`;
          }
        },
        chunkFileNames(chunkInfo: PreRenderedChunk) {
          if (chunkInfo.name.startsWith("@deskulpt")) {
            return `[name].js`;
          }
          return `assets/[name]-[hash].js`;
        },
      },
    },
  },
});
