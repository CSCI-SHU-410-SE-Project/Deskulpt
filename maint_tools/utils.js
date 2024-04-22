import chalk from "chalk";
import spawn from "cross-spawn";

const crossEmoji = "\u274C";
const checkEmoji = "\u2705";

/**
 * Run a command in the shell.
 *
 * @param cmd The command to run.
 * @param args The list of arguments that follows.
 * @param cwd The working directory to spawn.
 * @param title The title to display before running the command.
 * @returns False if an error occurred, true otherwise.
 */
export function executeCommand(cmd, args, cwd, title) {
  // Print the current job information
  console.log(chalk.blue.underline(title));
  console.log(chalk.blue(">>"), cmd, args.join(" "));
  console.log(chalk.blue(">>"), cwd);
  console.log();

  let anyError = false;
  try {
    const { status, error } = spawn.sync(cmd, args, { cwd: cwd, stdio: "inherit" });

    if (error) {
      // An error occurred in the command
      anyError = true;
      console.log(crossEmoji, chalk.red(error.toString()));
    } else if (status != 0) {
      // The command exited with a non-zero status, which indicates a known error which
      // should have already been printed to the console when running the command
      anyError = true;
      console.log(
        crossEmoji,
        chalk.red.underline(`Command failed with status code ${status}`),
      );
    } else {
      // The command passed successfully
      console.log(checkEmoji, chalk.green.underline(`No issues found!`));
    }
  } catch (err) {
    anyError = true;
    console.log(crossEmoji, chalk.red(err.toString()));
  }
  console.log();

  return !anyError;
}
