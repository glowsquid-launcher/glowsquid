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
      // MAIN COLOURS
      primary: 'var(--color-primary)',
      secondary: 'var(--color-secondary)',
      highlight: 'var(--color-highlight)',
      active: 'var(--color-active)',
      disabled: 'var(--color-disabled)',

      // ACCENT/UI COMPONENT VARIATION COLOURS

      // error
      error: 'var(--color-error)',
      errorHighlight: 'var(--color-error-highlight)',
      errorActive: 'var(--color-error-active)',

      // danger
      danger: 'var(--color-danger)',
      dangerHighlight: 'var(--color-danger-highlight)',
      dangerActive: 'var(--color-danger-active)',

      // warning
      warning: 'var(--color-warning)',
      warningHighlight: 'var(--color-warning-highlight)',
      warningActive: 'var(--color-warning-active)',

      // success
      success: 'var(--color-success)',
      successHighlight: 'var(--color-success-highlight)',
      successActive: 'var(--color-success-active)',

      // danger
      danger: 'var(--color-danger)',
      dangerHighlight: 'var(--color-danger-highlight)',
      dangerActive: 'var(--color-danger-active)',

      // link
      link: 'var(--color-link)',
      linkHighlight: 'var(--color-link-highlight)',
      linkActive: 'var(--color-link-active)',
    },
  },
};
