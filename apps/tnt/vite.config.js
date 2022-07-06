import { sveltekit } from '@sveltejs/kit/vite'
import Unocss from 'unocss/vite'
import { searchForWorkspaceRoot } from 'vite'
import unoConfig from '../../uno.config.mjs'

/** @type {import('vite').UserConfig} */
const config = {
  plugins: [sveltekit(), Unocss(unoConfig)],

  ssr: {
    noExternal: ['@tauri-apps/api', 'typesafe-i18n', '@glowsquid/glow-ui'],
  },
  optimizeDeps: {
    exclude: ['@glowsquid/glow-ui'],
  },
  esbuild: {
    target: ['esnext', 'chrome89', 'safari15.1', 'edge89', 'firefox89'],
  },
  build: {
    target: ['esnext', 'chrome89', 'safari15.1', 'edge89', 'firefox89'],
  },
  server: {
    fs: {
      allow: [searchForWorkspaceRoot(process.cwd())],
    },
  },
};

export default config;
