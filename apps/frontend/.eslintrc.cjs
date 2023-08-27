module.exports = {
  parser: '@typescript-eslint/parser',
  extends: [
    '../../.eslintrc.json',
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'prettier',
    "plugin:svelte/recommended",
  ],
  plugins: ['@typescript-eslint'],
  ignorePatterns: ['!**/*', '*.cjs', 'vite.config.ts', 'node_modules'],
  overrides: [
    {
      files: ['*.svelte'],
      parser: 'svelte-eslint-parser',
      parserOptions: {
        parser: '@typescript-eslint/parser'
      }
    },
    {
      files: [
        "*.json"
      ],
      parser: "jsonc-eslint-parser",
      rules: {
        "@nx/dependency-checks": "error"
      }
    }
  ],
  parserOptions: {
    sourceType: 'module',
    ecmaVersion: 2020,
    extraFileExtensions: ['.svelte']
  },
  env: {
    browser: true,
    es2017: true,
    node: true,
  },
};
