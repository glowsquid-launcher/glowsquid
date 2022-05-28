/* eslint-disable no-unused-vars */

declare namespace App {
  type Locales = import('$locales/i18n-types').Locales
  // interface Locals { }

  // interface Platform { }

  interface Session {
    locale?: Locales
  }

  // interface Stuff { }
}

declare module '@glowsquid/glow-ui' {
  const types = await import('../../../libs/glow-ui/package')
  export = types
}
