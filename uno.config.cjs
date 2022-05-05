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
      'error-highlight': 'var(--color-error-highlight)',
      'error-active': 'var(--color-error-active)',

      // warning
      warning: 'var(--color-warning)',
      'warning-highlight': 'var(--color-warning-highlight)',
      'warning-active': 'var(--color-warning-active)',
      
      // success
      success: 'var(--color-success)',
      'success-highlight': 'var(--color-success-highlight)',
      'success-active': 'var(--color-success-active)',
      
      // danger
      danger: 'var(--color-danger)',
      'danger-highlight': 'var(--color-danger-highlight)',
      'danger-active': 'var(--color-danger-active)',
      
      // link
      link: 'var(--color-link)',
      'link-highlight': 'var(--color-link-highlight)',
      'link-active': 'var(--color-link-active)',
    },
  },
};
