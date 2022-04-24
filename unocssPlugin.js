const Unocss = require('unocss/vite').default;

const {
  presetUno,
  presetTypography,
  presetIcons,
  extractorSvelte,
  transformerDirectives,
  transformerVariantGroup,
} = require('unocss');

module.exports = Unocss({
  extractors: [extractorSvelte],
  transformers: [transformerDirectives(), transformerVariantGroup()],
  presets: [presetUno(), presetTypography(), presetIcons()],
  theme: {
    primary: '',
  },
});
