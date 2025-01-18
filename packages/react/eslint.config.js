// @ts-check

import eslint from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";

export default tseslint.config(
  {
    ignores: ["**/dist/**"],
  },

  // --- Language Options -------------------------------------------------------------

  {
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
      parserOptions: {
        project: true,
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },

  // --- Linter Rules -----------------------------------------------------------------

  eslint.configs.recommended,
  ...tseslint.configs.recommendedTypeChecked,
  ...tseslint.configs.stylisticTypeChecked,
  {
    rules: {
      "sort-imports": [
        "error",
        {
          ignoreDeclarationSort: true,
        },
      ],
    },
  },

  // --- Linter Overrides ------------------------------------------------------------

  {
    // Disables type checking for JavaScript files
    files: ["**/*.js"],
    ...tseslint.configs.disableTypeChecked,
  },
);
