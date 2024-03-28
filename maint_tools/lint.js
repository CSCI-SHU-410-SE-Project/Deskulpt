import { Argument, Command } from "commander";
import { join } from "path";
import { fileURLToPath } from "url";
import spawn from "cross-spawn";
import chalk from "chalk";
import indentString from "indent-string";

const program = new Command();
const basedir = join(fileURLToPath(new URL(".", import.meta.url)), "..");

const commandMatrix = {
  js: {
    format: {
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
    lint: {
      check: {
        dir: ".",
        cmd: "npx",
        args: ["eslint", "--max-warnings=0", "."],
      },
      fix: {
        dir: ".",
        cmd: "npx",
        args: ["eslint", "--fix", "--max-warnings=0", "."],
      },
    },
  },
  rs: {
    format: {
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
    lint: {
      check: {
        dir: "src-tauri",
        cmd: "cargo",
        args: ["clippy", "--", "-D", "warnings"],
      },
      fix: {
        dir: "src-tauri",
        cmd: "cargo",
        args: [
          "clippy",
          "--fix",
          "--allow-dirty",
          "--allow-staged",
          "--",
          "-D",
          "warnings",
        ],
      },
    },
  },
};

/**
 * Utility function to log error information.
 * @param {string} info A brief description of the error.
 * @param {Error | null} err The error object to log.
 */
const logError = (info, err) => {
  console.log(`  ${chalk.red.underline("!!! LINTING ERRORS !!!")}`);
  console.log(`    ${chalk.red(info)}`);
  console.log();
  if (err) {
    const errString = err.toString().trim();
    if (errString) {
      console.error(indentString(errString, 4));
      console.log();
    }
  }
};

/**
 * Utility function to log stdout and stderr content.
 * @param {Buffer} stdout The stdout buffer to log.
 * @param {Buffer} stderr The stderr buffer to log.
 */
const logContent = (stdout, stderr) => {
  const outString = stdout.toString().trim();
  const errString = stderr.toString().trim();
  if (outString) {
    console.log(`  ${chalk.underline("Captured stdout")}`);
    console.log(indentString(outString, 4));
    console.log();
  }
  if (errString) {
    console.log(`  ${chalk.underline("Captured stderr")}`);
    console.error(indentString(errString, 4));
    console.log();
  }
};

// Setup the commander
program
  .name("lint")
  .description("Lint the codebase")
  .addArgument(
    new Argument("[lang]", "The language to lint")
      .choices(["js", "rs", "all"])
      .default("all"),
  )
  .addArgument(
    new Argument("[type]", "The type of linting")
      .choices(["format", "lint", "all"])
      .default("all"),
  )
  .option("-f, --fix", "Automatically fix linting errors where possible")
  .option("-s, --suppress", "Suppress stdout and stderr when no linting errors")
  .action((lintLang, lintType, options) => {
    const lintLangs = lintLang === "all" ? ["js", "rs"] : [lintLang];
    const lintTypes = lintType === "all" ? ["format", "lint"] : [lintType];

    let anyErrors = false;
    lintLangs.flatMap((lang) =>
      lintTypes.map((type) => {
        const { dir, cmd, args } =
          commandMatrix[lang][type][options.fix ? "fix" : "check"];
        const cwd = join(basedir, dir);

        // Print the current job information
        console.log(chalk.blue.underline(`${lang}::${type}`));
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
      }),
    );

    // Exit with error code if errors were found; this is useful for CI/CD pipelines
    if (anyErrors) {
      process.exit(1);
    }
  });

program.parse();
