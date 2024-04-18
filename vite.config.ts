import { defineConfig } from "vite";
import { resolve } from "path";
import react from "@vitejs/plugin-react";
import path from "path";

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
      ignored: ["**/src-tauri/**", "**/website/**"],
    },
  },
  build: {
    rollupOptions: {
      input: {
        main: resolve(__dirname, "views/index.html"),
        canvas: resolve(__dirname, "views/canvas.html"),
      },
      // output: {
      //   preserveModules: true,
      //   // remove hash from chunk names so that we can explicitly import them
      //   // in the importmap in views/canvas.html
      //   entryFileNames: `assets/[name].js`,
      //   chunkFileNames: `assets/[name].js`,
      //   assetFileNames: `assets/[name].[ext]`,
      // },
      // preserveEntrySignatures: "allow-extension",
      output: {
        minifyInternalExports: false,
        // only keep the strucgture of the @deskulpt/xxx modules
        manualChunks(id) {
          // Normalize the incoming module ID and the base path for @deskulpt
          const normalizedId = normalizePath(id);
          const deskulptPath = normalizePath(path.resolve(__dirname, "src/@deskulpt"));
          if (normalizedId.includes(deskulptPath)) {
            console.log(normalizedId);
            const pathParts = normalizedId.split("/");
            const deskulptIndex = pathParts.indexOf("@deskulpt");
            const specificPath = pathParts
              .slice(deskulptIndex)
              .join("/")
              .replace(/\.tsx?$/, "");
            return `${specificPath}`;
          }
        },
        chunkFileNames(chunkInfo) {
          if (chunkInfo.name.startsWith("@deskulpt")) {
            return `[name].js`;
          }
          return `assets/[name]-[hash].js`;
        },
      },
    },
  },
});
