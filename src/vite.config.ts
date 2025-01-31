import { defineConfig } from "vite";
import { resolve } from "path";
import react from "@vitejs/plugin-react";

export default defineConfig({
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
        "generated/jsx-runtime": resolve(__dirname, "generated/jsx-runtime.js"),
        "generated/raw-apis": resolve(__dirname, "generated/raw-apis.js"),
        "generated/react": resolve(__dirname, "generated/react.js"),
        "generated/ui": resolve(__dirname, "generated/ui.js"),
      },
      output: {
        // Make sure scripts are at the root of the build output so that their import
        // paths are consistent with in the dev server
        entryFileNames: ({ name }) =>
          name.startsWith("generated/") ? "[name].js" : "assets/[name].js",
      },
      // Make sure exports of the scripts are preserved so that they can be imported
      // deterministically
      preserveEntrySignatures: "allow-extension",
    },
  },
});
