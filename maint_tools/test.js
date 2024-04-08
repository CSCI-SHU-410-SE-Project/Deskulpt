import { Argument, Command } from "commander";
import { join } from "path";
import { fileURLToPath } from "url";
import spawn from "cross-spawn";
import chalk from "chalk";

import { logError, logContent } from "./utils.js";

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
      args: ["llvm-cov", "--open", "--"],
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
  .option("--jstests [name...]", "Run only matching jest tests")
  .option("--rstests [name...]", "Run only matching rust tests")
  .option("-c, --cov", "Check code coverage")
  .option("-s, --suppress", "Suppress stdout and stderr when no errors")
  .action((lang, options) => {
    const langs = lang === "all" ? ["rs"] : [lang];

    let anyErrors = false;
    langs.map((lang) => {
      const { dir, cmd, args } = commandMatrix[lang][options.cov ? "cov" : "run"];
      const cwd = join(basedir, dir);

      let newArgs = args.slice();
      if (lang == "rs") {
        newArgs = args.concat(options.rstests || []);
      }

      // Print the current job information
      console.log(chalk.blue.underline(`${lang}`));
      console.log(`  ${chalk.blue("cmd")} : ${cmd} ${newArgs.join(" ")}`);
      console.log(`  ${chalk.blue("cwd")} : ${cwd}`);
      console.log();

      try {
        const { stdout, stderr, status, error } = spawn.sync(cmd, newArgs, {
          cwd: cwd,
        });
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
