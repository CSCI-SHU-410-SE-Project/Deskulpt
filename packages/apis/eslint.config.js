// @ts-check

// eslint: https://eslint.org/docs/latest/
// typescript-eslint: https://typescript-eslint.io/getting-started/

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
  {
    // It is hard to pass these rules when wrapping raw APIs
    files: ["src/index.ts"],
    rules: {
      "@typescript-eslint/ban-types": "off",
      "@typescript-eslint/no-explicit-any": "off",
      "@typescript-eslint/no-unsafe-argument": "off",
      "@typescript-eslint/no-unsafe-call": "off",
      "@typescript-eslint/no-unsafe-function-type": "off",
      "@typescript-eslint/no-unsafe-member-access": "off",
      "@typescript-eslint/no-unsafe-return": "off",
    },
  },
);
