import chalk from "chalk";
import indentString from "indent-string";

/**
 * Utility function to log error information.
 * @param {string} info A brief description of the error.
 * @param {Error | null} err The error object to log.
 */
export function logError(info, err) {
  console.log(`  ${chalk.red.underline("!!! ERRORS OCCURRED !!!")}`);
  console.log(`    ${chalk.red(info)}`);
  console.log();
  if (err) {
    const errString = err.toString().trim();
    if (errString) {
      console.error(indentString(errString, 4));
      console.log();
    }
  }
}

/**
 * Utility function to log stdout and stderr content.
 * @param {Buffer} stdout The stdout buffer to log.
 * @param {Buffer} stderr The stderr buffer to log.
 */
export function logContent(stdout, stderr) {
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
}
