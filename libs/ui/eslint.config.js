import baseConfig from '@repo/eslint-config';

/** @type {import('eslint').Linter.FlatConfig[]} */
export default [
  {
    ignores: ['build/', '.svelte-kit/', 'dist/']
  },
  ...baseConfig
];
