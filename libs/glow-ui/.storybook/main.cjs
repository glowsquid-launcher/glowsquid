const Unocss = require('../src/lib/unocssPlugin.cjs');
const preprocess = require('svelte-preprocess');

module.exports = {
  stories: ['../src/**/*.stories.@(js|jsx|ts|tsx|svelte)'],
  addons: ['@storybook/addon-essentials'],
  core: {
    builder: '@storybook/builder-vite',
  },
  framework: '@storybook/svelte',
  async viteFinal(config) {
    config.plugins = config.plugins ?? [];
    config.resolve = config.resolve ?? {};
    config.resolve.alias = config.resolve.alias ?? {};

    config.plugins.unshift(Unocss);

    config.resolve.alias.$lib = '../src/lib';
    config.resolve.alias.$app =
      '../../../../node_modules/@sveltejs/kit/assets/app';

    return config;
  },
  svelteOptions: {
    preprocess: preprocess(),
  },
};
