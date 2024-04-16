import { defineConfig } from "vite";
import { resolve } from "path";
import react from "@vitejs/plugin-react";
import path from "path";
// import copy from "rollup-plugin-copy";
// import { createHtmlPlugin } from "vite-plugin-html";

// function normalizePath(p: string) {
//   return p.replace(/\\/g, "/");
// }

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
    // TODO: Set up vite config to avoid "React" being renamed to "R" in production
    minify: "terser",
    terserOptions: {
      // compress: false,
      mangle: false,
      // format: {
      //   beautify: true, 
      // },
    },
    rollupOptions: {
      // treeshake: false,
      input: {
        main: resolve(__dirname, "views/index.html"),
        canvas: resolve(__dirname, "views/canvas.html"),
      },
      output: {
        preserveModules: true,
        // remove hash from chunk names
        entryFileNames: `assets/[name].js`,
        chunkFileNames: `assets/[name].js`,
        assetFileNames: `assets/[name].[ext]`,

        // Map `__diranme/src/@deskulpt/**/*.tsx?` to `__dirname/dist/@deskulpt/**/*`
        // manualChunks(id) {
        //   // Normalize the incoming module ID and the base path for @deskulpt
        //   const normalizedId = normalizePath(id);
        //   const deskulptPath = normalizePath(path.resolve(__dirname, "src/@deskulpt"));

        //   if (normalizedId.includes(deskulptPath)) {
        //     console.log(normalizedId);
        //     const pathParts = normalizedId.split("/");
        //     const deskulptIndex = pathParts.indexOf("@deskulpt");
        //     const specificPath = pathParts
        //       .slice(deskulptIndex)
        //       .join("/")
        //       .replace(/\.tsx?$/, "");
        //     return `${specificPath}`;
        //   }
        // },
        // chunkFileNames(chunkInfo) {
        //   if (chunkInfo.name.startsWith("@deskulpt")) {
        //     return `[name].js`;
        //   }
        //   return `assets/[name]-[hash].js`;
        // },
      },
      preserveEntrySignatures: "allow-extension",
    },
  },
});
