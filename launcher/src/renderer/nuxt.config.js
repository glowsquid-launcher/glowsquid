const plugin = require('tailwindcss/plugin')

/**
 * By default, Nuxt.js is configured to cover most use cases.
 * This default configuration can be overwritten in this file
 * @link {https://nuxtjs.org/guide/configuration/}
 */

module.exports = {
  ssr: false,
  css: ['@/assets/extra.scss', '@/assets/toast.scss'],
  head: {
    title: 'glowsquid-next',
    meta: [{ charset: 'utf-8' }]
  },
  plugins: [
    { ssr: true, src: '@/plugins/icons.js' },
    { ssr: false, src: '@/plugins/store.ts' }

  ],
  modules: [
    '@nuxtjs/axios',
    '@nuxtjs/toast'
  ],
  buildModules: [
    '@nuxtjs/vuetify',
    ['@nuxt/typescript-build', {
      // we disable this as the dev env already know about this stuff
      ignoreNotFoundWarnings: true,
      typeCheck: false
    }],
    '@nuxtjs/tailwindcss',
    [
      'nuxt-i18n',
      {
        strategy: 'prefix',
        locales: [
          {
            code: 'en',
            file: 'en.js'
          },
          {
            code: 'fr',
            file: 'fr.js'
          },
          {
            code: 'cs',
            file: 'cs.js'
          },
          {
            code: 'ru',
            file: 'ru.js'
          },
          {
            code: 'nl',
            file: 'nl.js'
          }
        ],
        lazy: true,
        langDir: 'locales/',
        defaultLocale: 'en'
      }
    ]
  ],
  vuetify: {
    theme: {
      themes: {
        dark: {
          primary: '#002B36',
          accent: '#D33682',
          secondary: '#053744',
          success: '#859900',
          info: '#2AA198',
          warning: '#CB4B16',
          error: '#DC322F'
        },
        light: {
          primary: '#FDF6E3',
          accent: '#D33682',
          secondary: '#EEE8D5',
          success: '#859900',
          info: '#2AA198',
          warning: '#CB4B16',
          error: '#DC322F'
        }
      },
      dark: true,
      options: { customProperties: true }
    },
    icons: {
      iconfont: 'mdi'
    },
    treeShake: true
  },
  tailwindcss: {
    config: {
      purge: [
        './components/**/*.{vue,js}',
        './layouts/**/*.vue',
        './pages/**/*.vue',
        './plugins/**/*.{js,ts}',
        './nuxt.config.{js,ts}'
      ],
      theme: {
        extend: {}
      },
      variants: {
        extend: {
          width: ['hover', 'focus'],
          transitionProperty: ['important']
        }
      },
      plugins: [
        plugin(function ({ addVariant }) {
          addVariant('important', ({ container }) => {
            container.walkRules(rule => {
              rule.selector = `.\\!${rule.selector.slice(1)}`
              rule.walkDecls(decl => {
                decl.important = true
              })
            })
          })
        })
      ]
    }
  },
  toast: {
    position: 'top-right',
    iconPack: 'mdi',
    duration: 1500,
    containerClass: 'toasted-container'
  }
}
