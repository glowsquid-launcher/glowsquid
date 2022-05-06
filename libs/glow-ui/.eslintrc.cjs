module.exports = {
  env: {
    browser: true,
    es2021: true,
    node: true,
    // vitest has the same api as jest
    jest: true
  },
  extends: ['standard'],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module'
  },
  ignorePatterns: ['**/node_modules/**', '**/coverage/**', '**/dist/**', '**/package/**', '**/.svelte-kit/**'],
  plugins: ['svelte3', '@typescript-eslint', 'vitest'],
  rules: {},
  settings: {
    'svelte3/typescript': () => require('typescript')
  },
  overrides: [
    {
      files: ['*.svelte'],
      processor: 'svelte3/svelte3',
      rules: {
        'import/first': 0,
        'import/no-duplicates': 0,
        'imports/no-mutable-exports': 0,
        'imports/prefer-default-export': 0,
        'no-use-before-define': 0
      }
    }
  ]
}
