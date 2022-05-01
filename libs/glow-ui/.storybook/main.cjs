const preprocess = require('svelte-preprocess');
const Unocss = require('unocss/vite').default;

module.exports = {
  stories: ['../src/**/*.stories.@(js|jsx|ts|tsx|svelte)'],
  addons: [
    '@storybook/addon-essentials',
    '@storybook/addon-links',
    '@storybook/addon-a11y',
  ],
  core: {
    builder: '@storybook/builder-vite',
  },
  framework: '@storybook/svelte',
  async viteFinal(config) {
    config.plugins = config.plugins ?? [];
    config.resolve = config.resolve ?? {};
    config.resolve.alias = config.resolve.alias ?? {};
    config.json = config.json ?? {};

    config.plugins.unshift(Unocss(require('../../../uno.config.cjs')));

    config.resolve.alias.$lib = '../src/lib';
    config.resolve.alias.$app =
      '../../../../node_modules/@sveltejs/kit/assets/app';

    return config;
  },
  svelteOptions: {
    preprocess: preprocess(),
  },
};
