const rootMain = require('../../../.storybook/main.cjs');
const preprocess = require('svelte-preprocess');

module.exports = {
  ...rootMain,
  stories: [
    '../src/**/*.stories.mdx',
    '../src/**/*.stories.@(js|jsx|ts|tsx|svelte)',
  ],
  addons: [...rootMain.addons],
  framework: '@storybook/svelte',
  svelteOptions: {
    preprocess: preprocess(),
  },
};
