import { Argument, Command } from "commander";
import { join } from "path";
import { fileURLToPath } from "url";
import spawn from "cross-spawn";
import chalk from "chalk";

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
  .option("--rs [name...]", "Run only matching cargo tests")
  .action((lang, options) => {
    const langs = lang === "all" ? ["rs"] : [lang];

    let anyError = false;
    langs.map((lang) => {
      const { dir, cmd, args } = commandMatrix[lang][options.cov ? "cov" : "run"];
      const cwd = join(basedir, dir);

      // Concatenate test pattern specifications to the command
      const newArgs = [...args, ...(options[lang] || [])];

      // Print the current job information
      console.log(chalk.blue.underline(`Testing for lang=${lang}`));
      console.log(chalk.blue(">>>"), cmd, newArgs.join(" "));
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
            chalk.red.underline(`[lang=${lang}] Testing failed with status=${status}`),
          );
        } else {
          console.log(
            "\u2705",
            chalk.green.underline(`[lang=${lang}] All tests passed!`),
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
