import { quote } from "shell-quote";

export default {
  "**/*.{js,jsx,mjs,ts,tsx,mts}": (filenames) => {
    const fileArgs = quote(filenames);
    return [
      `prettier --write ${fileArgs}`,
      `eslint --max-warnings=0 --no-warn-ignored --fix ${fileArgs}`,
    ];
  },
  "**/*.{json,json5,md,html,css,scss,yml,yaml}": (filenames) => {
    const fileArgs = quote(filenames);
    return [`prettier --write ${fileArgs}`];
  },
  "**/*.rs": (filenames) => {
    const fileArgs = quote(filenames);
    return [`rustfmt -- ${fileArgs}`];
  },
};
