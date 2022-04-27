const Unocss = require('unocss/vite').default;

const {
  presetUno,
  presetTypography,
  presetIcons,
  extractorSvelte,
  transformerDirectives,
  transformerVariantGroup,
} = require('unocss');

module.exports = {
  extractors: [extractorSvelte],
  transformers: [transformerDirectives(), transformerVariantGroup()],
  presets: [presetUno(), presetTypography(), presetIcons()],
  theme: {
    // even though I use bri-ish english spelling I'll use americanish as thats what css uses
    colors: {
      primary: 'var(--color-primary)',
      secondary: 'var(--color-secondary)',
      highlight: 'var(--color-highlight)',
      active: 'var(--color-active)',
    },
  },
}
