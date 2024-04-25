import { Plugin, defineConfig } from "vite";
import { posix, resolve } from "path";
import react from "@vitejs/plugin-react";

const buildAssets = "assets";

export default defineConfig(({ command }) => ({
  plugins: [
    react(),
    importmapPlugin(command, {
      "@deskulpt-test/react": "src/.scripts/react.js",
      "@deskulpt-test/raw-apis": "src/.scripts/raw-apis.js",
    }),
  ],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      ignored: ["**/src-tauri/**", "**/website/**", "**/tooling/**"],
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
        entryFileNames: `${buildAssets}/[name].js`,
        chunkFileNames: `${buildAssets}/[name].js`,
        assetFileNames: `${buildAssets}/[name].[ext]`,
      },
      preserveEntrySignatures: "allow-extension",
    },
  },
}));

/**
 * Cutom plugin to deal with import maps.
 *
 * Note that the values of `imports` must be relative to the vite server root.
 *
 * @param command The vite command, either "serve" or "build".
 * @param imports The "imports" entry of the import map.
 * @returns The plugin object.
 */
function importmapPlugin(
  command: "serve" | "build",
  imports: Record<string, string>,
): Plugin {
  return {
    name: "vite-plugin-importmap",

    config: () => {
      if (command === "build") {
        // In production build, add import map targets as input entries to make sure
        // they are included in the final bundle
        return {
          build: {
            rollupOptions: {
              input: Object.fromEntries(
                Object.values(imports).map((v) => {
                  const { dir, name } = posix.parse(v);
                  return [posix.join(dir, name), resolve(__dirname, v)];
                }),
              ),
            },
          },
        };
      }
    },

    transformIndexHtml: {
      order: "pre",
      handler: (html) => {
        // In production build, import map targets are relative to the build assets; in
        // development build they start at the server root
        const base = command === "build" ? posix.join("/", buildAssets) : "/";
        const importPaths = Object.fromEntries(
          Object.entries(imports).map(([k, v]) => [k, posix.join(base, v)]),
        );

        return {
          html,
          tags: [
            // Inject the import map into the HTML head
            {
              tag: "script",
              attrs: { type: "importmap" },
              children: JSON.stringify({ imports: importPaths }, null, 2),
              injectTo: "head-prepend",
            },
          ],
        };
      },
    },
  };
}
