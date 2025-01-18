import { defineConfig } from "vite";
import { resolve } from "path";
import react from "@vitejs/plugin-react";

export default defineConfig({
  resolve: {
    alias: {
      "@": __dirname,
    },
  },
  plugins: [
    react({
      jsxImportSource: "@emotion/react",
      babel: {
        plugins: ["@emotion/babel-plugin"],
      },
    }),
  ],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
  build: {
    rollupOptions: {
      input: {
        manager: resolve(__dirname, "views/manager.html"),
        canvas: resolve(__dirname, "views/canvas.html"),
        // Make the scripts entrypoints so that they are preserved even if not imported
        ".scripts/jsx-runtime": resolve(__dirname, ".scripts/jsx-runtime.js"),
        ".scripts/raw-apis": resolve(__dirname, ".scripts/raw-apis.js"),
        ".scripts/react": resolve(__dirname, ".scripts/react.js"),
        ".scripts/ui": resolve(__dirname, ".scripts/ui.js"),
      },
      output: {
        // Make sure scripts are at the root of the build output so that their import
        // paths are consistent with in the dev server
        entryFileNames: ({ name }) =>
          name.startsWith(".scripts/") ? "[name].js" : "assets/[name].js",
      },
      // Make sure exports of the scripts are preserved so that they can be imported
      // deterministically
      preserveEntrySignatures: "allow-extension",
    },
  },
});
