import { quote } from "shell-quote";

const isWin = process.platform === "win32";

export default {
  "**/*.{js,jsx,mjs,ts,tsx,mts}": (filenames) => {
    const fileArgs = escape(filenames);
    return [
      `prettier --write ${fileArgs}`,
      `eslint --max-warnings=0 --no-warn-ignored --fix ${fileArgs}`,
    ];
  },
  "**/*.{json,json5,md,html,css,scss,yml,yaml}": (filenames) => {
    const fileArgs = escape(filenames);
    return [`prettier --write ${fileArgs}`];
  },
  // Note that cargo-clippy is not run because it will need to compile the project and
  // cannot be limited only to staged files, making it not a good idea for a pre-commit
  // hook; we shall rely on CI to discover cargo-clippy issues and recommend users to
  // manually fix or use `npm run lint rs lint -- --fix`
  "**/*.rs": (filenames) => {
    const fileArgs = escape(filenames);
    return [`rustfmt -- ${fileArgs}`];
  },
};

function escape(filenames) {
  return filenames
    .map((filename) => (isWin ? `"${filename}"` : quote([filename])))
    .join(" ");
}
