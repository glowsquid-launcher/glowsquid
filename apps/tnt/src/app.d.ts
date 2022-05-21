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
  export const { Button, ButtonVariant, setTheme } = await import(
    '../../../libs/glow-ui/src/lib'
  )
}
