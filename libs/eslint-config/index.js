module.exports = {
  env: {
    browser: true,
    es2021: true,
    node: true,
    // vitest has the same api as jest
    jest: true,
  },
  extends: [
    'standard',
    'plugin:@typescript-eslint/recommended',
    'plugin:@typescript-eslint/recommended-requiring-type-checking',
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 'latest',
    project: ['tsconfig.json', 'tsconfig.eslint.json'],
    sourceType: 'module',
  },
  ignorePatterns: [
    '**/node_modules/**',
    '**/coverage/**',
    '**/dist/**',
    '**/package/**',
    '**/.svelte-kit/**',
  ],
  plugins: ['svelte3', '@typescript-eslint', 'vitest', 'prettier'],
  rules: {
    'indent': ['error', 2],
    'quote-props': ['error', 'consistent-as-needed'],
    'comma-dangle': ['off'],
    'prettier/prettier': [
      'error',
      {
        semi: false,
        singleQuote: true,
        tabWidth: 2,
        quoteProps: 'consistent',
        endOfLine: 'lf',
        importOrder: [
          '^(child_process|crypto|events|fs|http|https|os|path)(\\/(.*))?$',
          '<THIRD_PARTY_MODULES>',
          '^~(\\/(.*))?$',
          '^@(\\/(.*))?$',
          '^@app(\\/(.*))?$',
          '^[./]',
        ],
        experimentalBabelParserPluginsList: ['jsx', 'typescript'],
      },
      {
        usePrettierrc: false,
      },
    ],
  },
  settings: {
    'svelte3/typescript': () => require('typescript'),
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
        'no-use-before-define': 0,
      },
    },
  ],
}
