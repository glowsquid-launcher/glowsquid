/* eslint-disable */
import adapter from '@sveltejs/adapter-static'
import path from 'node:path'
import preprocess from 'svelte-preprocess'
import Unocss from 'unocss/vite'
import { searchForWorkspaceRoot } from 'vite'
import unoConfig from '../../uno.config.cjs'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: preprocess(),

  kit: {
    adapter: adapter({
      pages: './dist/',
      assets: './dist/',
      fallback: 'index.html',
    }),

    vite: {
      ssr: {
        noExternal: ['@tauri-apps/api', 'typesafe-i18n', '@glowsquid/glow-ui'],
      },
      optimizeDeps: {
        exclude: ['@glowsquid/glow-ui'],
      },
      plugins: [Unocss(unoConfig)],
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
    },

    alias: {
      $locales: path.resolve('./src/lib/locales'),
      $lib: path.resolve('./src/lib'),
      $components: path.resolve('./src/lib/components'),
      $bridge: path.resolve('./src/lib/bridge'),
      $state: path.resolve('./src/lib/state.ts'),
    },
  },
}

export default config
