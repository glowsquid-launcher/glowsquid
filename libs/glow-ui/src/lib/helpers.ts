import { browser } from '$app/env'

/**
 * sets the theme of the app
 *
 * @param {string} theme The theam. As of now can be `light` or `dark`
 *
 * @remarks All this does is edit the class of the main document. Its more of a utility function than some complex thing
 **/
export function setTheme (theme: 'dark' | 'light') {
  if (browser) document.documentElement.className = theme
}
