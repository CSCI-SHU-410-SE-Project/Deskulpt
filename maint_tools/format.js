import { Argument, Command } from "commander";
import { join } from "path";
import { fileURLToPath } from "url";
import { executeCommand } from "./utils.js";

const program = new Command();
const basedir = join(fileURLToPath(new URL(".", import.meta.url)), "..");

const commandMatrix = {
  js: {
    check: {
      dir: ".",
      cmd: "pnpm",
      args: ["prettier", "--check", "."],
    },
    fix: {
      dir: ".",
      cmd: "pnpm",
      args: ["prettier", "--write", "."],
    },
  },
  rs: {
    check: {
      dir: "src-tauri",
      cmd: "cargo",
      args: ["fmt", "--", "--check"],
    },
    fix: {
      dir: "src-tauri",
      cmd: "cargo",
      args: ["fmt"],
    },
  },
};

// Setup the commander
program
  .name("format")
  .description("Format the codebase")
  .addArgument(
    new Argument("[lang]", "The language to format")
      .choices(["js", "rs", "all"])
      .default("all"),
  )
  .option("-f, --fix", "Automatically fix errors where possible")
  .action((lang, options) => {
    const langs = lang === "all" ? ["js", "rs"] : [lang];

    let anyError = false;
    langs.map((lang) => {
      const { dir, cmd, args } = commandMatrix[lang][options.fix ? "fix" : "check"];
      const cwd = join(basedir, dir);
      const passed = executeCommand(cmd, args, cwd, `Formatting for lang=${lang}`);
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
