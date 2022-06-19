const preprocess = require('svelte-preprocess');
const Unocss = require('unocss/vite').default;
const path = require('path');

module.exports = {
  stories: ['../src/**/*.stories.@(js|jsx|ts|tsx|svelte)'],
  addons: [
    '@storybook/addon-essentials',
    '@storybook/addon-links',
    '@storybook/addon-svelte-csf',
    '@storybook/addon-a11y',
    '@storybook/addon-storysource',
    'storybook-dark-mode'
  ],
  core: {
    builder: '@storybook/builder-vite',
  },
  framework: '@storybook/svelte',
  async viteFinal(config) {
    config.plugins = config.plugins ?? [];
    config.resolve = config.resolve ?? {};
    config.resolve.alias = config.resolve.alias ?? {};

    config.plugins.unshift(Unocss(require('../../../uno.config.cjs')));

    config.resolve.alias.$lib = path.resolve(__dirname, '../src/lib');
    config.resolve.alias.$app =
      path.resolve(__dirname, '../../../node_modules/@sveltejs/kit/assets/app');

    return config;
  },
  svelteOptions: {
    preprocess: preprocess(),
  },
};
