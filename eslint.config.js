// @ts-check

// eslint: https://eslint.org/docs/latest/
// typescript-eslint: https://typescript-eslint.io/getting-started/

import eslint from "@eslint/js";
import globals from "globals";
import tseslint from "typescript-eslint";

export default tseslint.config(
  {
    ignores: [
      "**/dist/**",
      "**/fixtures/**",
      "**/target/**",
      "**/website/**",
      "**/tooling/**",
      "src/.scripts/**",
    ],
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
      // Disables checking an asynchronous function passed as a JSX attribute expected
      // to be a function that returns void. This is useful for event handlers of React
      // components, e.g., `onClick` of a button
      "@typescript-eslint/no-misused-promises": [
        "error",
        {
          checksVoidReturn: {
            attributes: false,
          },
        },
      ],
      // Sort within multiple imports from the same module; sorting across modules is
      // disabled because it cannot be autofixed and sorts by declarations instead of
      // import specifiers; TODO: complement with eslint-plugin-import when it supports
      // eslint flat configuration
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
