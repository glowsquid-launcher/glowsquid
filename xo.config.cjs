module.exports = {
  extends: [
    'plugin:svelte/prettier',
    'plugin:perfectionist/recommended-natural'
  ],
  extensions: ['svelte'],
  overrides: [
    {
      files: ['**/*.svelte'],
      parser: 'svelte-eslint-parser',
      parserOptions: {
        parser: '@typescript-eslint/parser'
      },
      rules: {
        // This breaks svelte props
        'import/no-mutable-exports': 'off'
      }
    }
  ],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    extraFileExtensions: ['.svelte']
  },
  plugins: ['perfectionist'],
  prettier: true,
  rules: {
    // This breaks sveltekit throw error()
    '@typescript-eslint/no-throw-literal': 'off',
    // The rule no-unsafe seems to be broken
    '@typescript-eslint/no-unsafe-argument': 'off',
    '@typescript-eslint/no-unsafe-assignment': 'off',
    '@typescript-eslint/no-unsafe-call': 'off',
    '@typescript-eslint/no-unsafe-return': 'off',
    // Messes with node-style esm
    'import/extensions': 'off',
    'n/file-extension-in-import': 'off',
    // Messes with builtin
    'perfectionist/sort-imports': 'off',
    'perfectionist/sort-jsx-props': 'off'
  },
  space: 4
};
