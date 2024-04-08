import { Argument, Command } from "commander";
import { join } from "path";
import { fileURLToPath } from "url";
import spawn from "cross-spawn";
import chalk from "chalk";

import { logError, logContent } from "./utils.js";

const program = new Command();
const basedir = join(fileURLToPath(new URL(".", import.meta.url)), "..");

const commandMatrix = {
  js: {
    check: {
      dir: ".",
      cmd: "npx",
      args: ["prettier", "--check", "."],
    },
    fix: {
      dir: ".",
      cmd: "npx",
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
  .option("-s, --suppress", "Suppress stdout and stderr when no errors")
  .action((lang, options) => {
    const langs = lang === "all" ? ["js", "rs"] : [lang];

    let anyErrors = false;
    langs.map((lang) => {
      const { dir, cmd, args } = commandMatrix[lang][options.fix ? "fix" : "check"];
      const cwd = join(basedir, dir);

      // Print the current job information
      console.log(chalk.blue.underline(`${lang}`));
      console.log(`  ${chalk.blue("cmd")} : ${cmd} ${args.join(" ")}`);
      console.log(`  ${chalk.blue("cwd")} : ${cwd}`);
      console.log();

      try {
        const { stdout, stderr, status, error } = spawn.sync(cmd, args, { cwd: cwd });
        if (error) {
          // Unexpected error
          logError("FATAL: process failed", error);
          anyErrors = true;
        } else if (status != 0) {
          // Most likely a linting error
          logError(`Failed with status code ${status}`, null);
          logContent(stdout, stderr);
          anyErrors = true;
        } else if (!options.suppress) {
          // No errors and log not suppressed
          logContent(stdout, stderr);
        }
      } catch (err) {
        // Unexpected error
        logError(err);
        anyErrors = true;
      }
    });

    // Exit with error code if errors were found; this is useful for CI/CD pipelines
    if (anyErrors) {
      process.exit(1);
    }
  });

program.parse();
