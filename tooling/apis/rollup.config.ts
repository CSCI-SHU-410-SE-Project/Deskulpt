import { defineConfig, RollupLog } from "rollup";
import typescript from "@rollup/plugin-typescript";
import terser from "@rollup/plugin-terser";
import replace from "@rollup/plugin-replace";
import { fileURLToPath } from "url";
import { join } from "path";
import { Dir, opendirSync, rmSync } from "fs";

// Cleanup `dist/` directory
const __dirname = fileURLToPath(new URL(".", import.meta.url));
cleanDir(join(__dirname, "./dist"));

export default defineConfig([
  // ESM build of raw APIs to be used internally
  {
    input: "src/raw.ts",
    output: {
      format: "esm",
      file: "../../src/.scripts/raw-apis.js",
    },
    external: ["@tauri-apps/api/core"],
    plugins: [typescript(), terser()],
    onwarn,
  },
  // ESM build of wrapped APIs, but with raw APIs externalized; essentially we replace
  // the relative import of raw APIs with `@deskulpt-test/raw-apis` and externalize it;
  // `@deskulpt-test/raw-apis` does not really exist but should be mapped to the actual
  // raw APIs bundled above
  {
    input: "src/index.ts",
    output: {
      format: "esm",
      file: "../../public/.wrap-apis.js.txt",
    },
    external: ["@tauri-apps/api/core", "@deskulpt-test/raw-apis"],
    plugins: [
      replace({
        "./raw": "@deskulpt-test/raw-apis",
        delimiters: ["", ""],
        preventAssignment: true,
      }),
      typescript(),
      terser(),
    ],
    onwarn,
  },
  // ESM build of wrapped APIs for publishing
  {
    input: "src/index.ts",
    output: {
      format: "esm",
      dir: "./dist",
    },
    external: ["@tauri-apps/api/core"],
    plugins: [
      typescript({
        declaration: true,
        declarationDir: "./dist",
        rootDir: "./src",
      }),
    ],
    onwarn,
  },
]);

function onwarn(warning: RollupLog) {
  throw Object.assign(new Error(), warning);
}

function cleanDir(path: string) {
  let dir: Dir;
  try {
    dir = opendirSync(path);
  } catch (err: unknown) {
    if (err instanceof Error && "code" in err) {
      switch (err.code) {
        case "ENOENT":
          return;
        case "ENOTDIR":
          throw new Error(`'${path}' is not a directory`);
        default:
          throw err;
      }
    } else {
      throw err;
    }
  }

  let file = dir.readSync();
  while (file) {
    const filePath = join(path, file.name);
    rmSync(filePath, { recursive: true });
    file = dir.readSync();
  }
  dir.closeSync();
}
