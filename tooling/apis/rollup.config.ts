import { defineConfig, RollupLog } from "rollup";
import typescript from "@rollup/plugin-typescript";
import terser from "@rollup/plugin-terser";
import { fileURLToPath } from "url";
import { join } from "path";
import { Dir, opendirSync, rmSync } from "fs";

// Cleanup `dist/` directory
const __dirname = fileURLToPath(new URL(".", import.meta.url));
cleanDir(join(__dirname, "./dist"));

export default defineConfig([
  // ESM build to be used internally
  {
    input: "src/index.ts",
    output: {
      format: "esm",
      file: "../../public/apis.txt",
    },
    // resolved at runtime by an importmap
    external: ["@deskulpt-test/raw-apis"],
    plugins: [typescript(), terser()],
    onwarn,
  },
  // CJS build for publishing
  {
    input: "src/index.ts",
    output: {
      format: "es",
      dir: "./dist",
    },
    external: ["@deskulpt-test/raw-apis"],
    plugins: [
      typescript({
        declaration: true,
        declarationDir: "./dist",
        emitDeclarationOnly: true,
        rootDir: "./src",
      }),
      terser(),
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
