const Unocss = require('unocss/vite').default;

const {
  presetUno,
  presetTypography,
  presetAttributify,
  presetIcons,
  extractorSvelte,
} = require('unocss');

module.exports = {
  stories: [],
  addons: ['@storybook/addon-essentials'],
  core: {
    builder: '@storybook/builder-vite',
  },
  async viteFinal(config) {
    config.plugins = config.plugins ?? [];
    config.plugins.unshift(
      Unocss({
        extractors: [extractorSvelte],
        presets: [
          presetAttributify({
            nonValuedAttribute: true,
          }),
          presetUno(),
          presetTypography(),
          presetIcons(),
        ],
      })
    );
    return config;
  },
};
