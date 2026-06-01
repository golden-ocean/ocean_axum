//  @ts-check
import { tanstackConfig } from "@tanstack/eslint-config";

export default [
  {
    ignores: ["dist/**", "node_modules/**", ".tanstack/**"],
  },
  {
    include: ["src/**/*", "eslint.config.js"],
  },
  ...tanstackConfig,
  {
    rules: {
      "import/no-cycle": "off",
      "import/order": "off",
      "sort-imports": "off",
      "@typescript-eslint/array-type": "off",
      "@typescript-eslint/require-await": "off",
      "pnpm/json-enforce-catalog": "off",
    },
  },
];
