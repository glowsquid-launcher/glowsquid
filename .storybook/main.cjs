const Unocss = require('../unocssPlugin');

module.exports = {
  stories: [],
  addons: ['@storybook/addon-essentials'],
  core: {
    builder: '@storybook/builder-vite',
  },
  async viteFinal(config) {
    config.plugins = config.plugins ?? [];
    config.plugins.unshift(Unocss);
    return config;
  },
};
