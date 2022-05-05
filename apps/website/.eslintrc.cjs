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
  plugins: ['svelte3', '@typescript-eslint', 'vitest'],
  rules: {
    'no-use-before-define': 'off'
  },
  settings: {
    'svelte3/typescript': () => require('typescript')
  },
  overrides: [
    {
      files: ['*.svelte'],
      processor: 'svelte3/svelte3'
    }
  ]
}
