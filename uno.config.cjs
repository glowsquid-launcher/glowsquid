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
  presets: [
    presetUno(),
    presetTypography(),
    presetIcons({
      extraProperties: {
        display: 'inline-block',
        'vertical-align': 'middle',
        // ...
      },
    }),
  ],
  theme: {
    // even though I use english(traditional) spelling I'll use english(simplified) as thats what css uses
    colors: {
      primary: 'var(--color-primary)',
      secondary: 'var(--color-secondary)',
      highlight: 'var(--color-highlight)',
      active: 'var(--color-active)',
      disabled: 'var(--color-disabled)',
      error: 'var(--color-error)',
      warning: 'var(--color-warning)',
      success: 'var(--color-success)',
      danger: 'var(--color-danger)',
      link: 'var(--color-link)'
    },
  },
};
