import { Argument, Command } from "commander";
import { join } from "path";
import { fileURLToPath } from "url";
import { executeCommand } from "./utils.js";

const program = new Command();
const basedir = join(fileURLToPath(new URL(".", import.meta.url)), "..");

const commandMatrix = {
  rs: {
    run: {
      dir: "src-tauri",
      cmd: "cargo",
      args: ["test"],
    },
    cov: {
      dir: "src-tauri",
      cmd: "cargo",
      args: ["llvm-cov", "--html", "--"],
    },
  },
};

// Setup the commander
program
  .name("test")
  .description("Test the codebase")
  .addArgument(new Argument("<lang>", "The language to test").choices(["rs"]))
  .addArgument(new Argument("[testname]", "Select a subset of tests"))
  .option("--cov", "Generate coverage report")
  .action((lang, testname, options) => {
    const { dir, cmd, args } = commandMatrix[lang][options.cov ? "cov" : "run"];
    const cwd = join(basedir, dir);
    const fullArgs = [...args, ...(testname !== undefined ? [testname] : [])];
    const passed = executeCommand(cmd, fullArgs, cwd, `Testing for lang=${lang}`);

    // Exit with error code if errors were found; this is useful for CI/CD pipelines
    if (!passed) {
      process.exit(1);
    }
  });

program.parse();
