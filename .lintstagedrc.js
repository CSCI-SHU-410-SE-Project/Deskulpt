import { quote } from "shell-quote";

const isWin = process.platform === "win32";

const escape = (filenames) => {
  return filenames
    .map((filename) => (isWin ? `"${filename}"` : quote([filename])))
    .join(" ");
};

export default {
  "**/*.{js,jsx,mjs,ts,tsx,mts,json,json5,md,mdx,html,css,scss,yml,yaml}": (
    filenames,
  ) => {
    return [`prettier --write ${escape(filenames)}`];
  },
  "**/*.rs": (filenames) => {
    return [`rustfmt +nightly -- ${escape(filenames)}`];
  },
};
