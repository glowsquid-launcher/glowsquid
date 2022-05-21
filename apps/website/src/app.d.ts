/* eslint-disable no-unused-vars */
import '@sveltejs/kit'

declare module '@glowsquid/glow-ui' {
  const { setTheme, Button } = await import('../../../libs/glow-ui/package')
  export { setTheme, Button }
}
