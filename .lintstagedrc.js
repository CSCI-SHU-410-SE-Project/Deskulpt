import { quote } from "shell-quote";

export default {
  "**/*.{js,jsx,mjs,ts,tsx,mts}": (filenames) => {
    const fileArgs = quote(filenames);
    return [
      `prettier --write ${fileArgs}`,
      `eslint --max-warnings=0 --fix ${fileArgs}`,
    ];
  },
  "**/*.{json,md,html,css,scss,yml,yaml}": (filenames) => {
    const fileArgs = quote(filenames);
    return [`prettier --write ${fileArgs}`];
  },
  // Note that cargo-clippy is not run because it will need to compile the project,
  // which is not a good idea for a pre-commit hook; we shall rely on CI to discover
  // potential cargo-clippy issues
  "**/*.rs": (filenames) => {
    const fileArgs = quote(filenames);
    return [`rustfmt -- ${fileArgs}`];
  },
};
