import { defineConfig } from "vite";
import { resolve } from "path";
import react from "@vitejs/plugin-react";
// import copy from "rollup-plugin-copy";
// import { createHtmlPlugin } from "vite-plugin-html";

export default defineConfig({
  plugins: [
    react(),
    // copy({
    //   targets: [
    //     // we compile default deps first, and then copy them to the dist folder
    //     { src: "default_deps_dist", dest: "dist/src", rename: "@deskulpt" },
    //   ],
    //   hook: "writeBundle",
    // }),
    // createHtmlPlugin({
    //   template: 'views/canvas.html',
    //   inject: {
    //     tags: [
    //       {
    //         injectTo: 'head',
    //         tag: 'script',
    //         attrs: {
    //           type: 'module',
    //           src: '/src/@deskulpt/apis/index.js'
    //         }
    //       }
    //     ]
    //   }
    // }),
  ],
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
        preserveModules: true,
        entryFileNames: "assets/[name].js",
        chunkFileNames: "assets/[name].js",
        assetFileNames: "assets/[name].[ext]",
      },
      preserveEntrySignatures: "allow-extension",
      // external: [
      //   "/src/@deskulpt/react/index.js",
      //   "/src/@deskulpt/apis/index.js",
      // ]
    },
  },
  // optimizeDeps: {
  //   exclude: [
  //     "/default_deps_dist/**/*.js",
  //   ]
  // },
});
