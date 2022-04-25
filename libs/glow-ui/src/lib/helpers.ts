import { browser } from '$app/env';

export function setTheme(theme: 'dark' | 'light') {
  if (browser) document.documentElement.className = theme;
}
