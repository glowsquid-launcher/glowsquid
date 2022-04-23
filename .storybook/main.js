const Unocss = require('unocss/vite').default;

const {
  presetUno,
  presetTypography,
  presetAttributify,
  presetIcons,
} = require('unocss');

module.exports = {
  stories: [],
  addons: ['@storybook/addon-essentials'],
  core: {
    builder: '@storybook/builder-vite',
  },
  async viteFinal(config, { configType }) {
    config.plugins = config.plugins ?? [];
    config.plugins.push(
      Unocss({
        presets: [
          presetAttributify(),
          presetUno(),
          presetTypography(),
          presetIcons(),
        ],
      })
    );
    return config;
  },
};
