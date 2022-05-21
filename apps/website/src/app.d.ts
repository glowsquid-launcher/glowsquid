/* eslint-disable no-unused-vars */
import '@sveltejs/kit'

declare module '@glowsquid/glow-ui' {
  export const { setTheme, Button, ButtonVariant } = await import(
    '../../../libs/glow-ui/src/lib'
  )
}
