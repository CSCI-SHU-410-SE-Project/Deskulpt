import { quote } from "shell-quote";

const isWin = process.platform === "win32";

export default {
  "**/*.{js,jsx,mjs,ts,tsx,mts}": (filenames) => {
    const fileArgs = escape(filenames);
    return [
      `prettier --write ${fileArgs}`,
      `eslint --max-warnings=0 --fix ${fileArgs}`,
    ];
  },
  "**/*.{json,json5,md,html,css,scss,yml,yaml}": (filenames) => {
    const fileArgs = escape(filenames);
    return [`prettier --write ${fileArgs}`];
  },
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
