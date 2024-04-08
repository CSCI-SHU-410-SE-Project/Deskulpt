import { Argument, Command } from "commander";
import { join } from "path";
import { fileURLToPath } from "url";
import spawn from "cross-spawn";
import chalk from "chalk";

const program = new Command();
const basedir = join(fileURLToPath(new URL(".", import.meta.url)), "..");

const commandMatrix = {
  js: {
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
  rs: {
    check: {
      dir: "src-tauri",
      cmd: "cargo",
      args: ["clippy", "--", "-D", "warnings"],
    },
    fix: {
      dir: "src-tauri",
      cmd: "cargo",
      args: ["clippy", "--fix", "--allow-dirty", "--allow-staged", "--", "-Dwarnings"],
    },
  },
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
  .option("-f, --fix", "Automatically fix errors where possible")
  .action((lang, options) => {
    const langs = lang === "all" ? ["js", "rs"] : [lang];

    let anyError = false;
    langs.map((lang) => {
      const { dir, cmd, args } = commandMatrix[lang][options.fix ? "fix" : "check"];
      const cwd = join(basedir, dir);

      // Print the current job information
      console.log();
      console.log(chalk.blue.underline(`Linting for lang=${lang}`));
      console.log(chalk.blue(">>>"), cmd, args.join(" "));
      console.log(chalk.blue(">>>"), cwd);
      console.log();

      try {
        const { status, error } = spawn.sync(cmd, args, { cwd: cwd, stdio: "inherit" });

        if (error) {
          anyError = true;
          console.log("\u274C", chalk.red(error.toString()));
        } else if (status != 0) {
          anyError = true;
          console.log(
            "\u274C",
            chalk.red.underline(`[lang=${lang}] Linting failed with status=${status}`),
          );
        } else {
          console.log(
            "\u2705",
            chalk.green.underline(`[lang=${lang}] No linting issues found!`),
          );
        }
      } catch (err) {
        anyError = true;
        console.log("\u274C", chalk.red(err.toString()));
      }
      console.log();
    });

    // Exit with error code if errors were found; this is useful for CI/CD pipelines
    if (anyError) {
      process.exit(1);
    }
  });

program.parse();
