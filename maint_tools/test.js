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
      args: ["test", "--quiet"],
    },
    cov: {
      dir: "src-tauri",
      cmd: "cargo",
      args: ["llvm-cov", "--quiet", "--html", "--"],
    },
  },
};

// Setup the commander
program
  .name("format")
  .description("Format the codebase")
  .addArgument(
    new Argument("[lang]", "The language to format")
      .choices(["rs", "all"])
      .default("all"),
  )
  .option("--rs [name...]", "Run only matching rs tests")
  .option("--cov", "Generate coverage report")
  .action((lang, options) => {
    const langs = lang === "all" ? ["rs"] : [lang];

    let anyError = false;
    langs.map((lang) => {
      const { dir, cmd, args } = commandMatrix[lang][options.cov ? "cov" : "run"];
      const cwd = join(basedir, dir);
      const fullArgs = [...args, ...(options[lang] || [])];
      const passed = executeCommand(cmd, fullArgs, cwd, `Testing for lang=${lang}`);
      if (!passed) {
        anyError = true;
      }
    });

    // Exit with error code if errors were found; this is useful for CI/CD pipelines
    if (anyError) {
      process.exit(1);
    }
  });

program.parse();
