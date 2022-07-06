/* eslint-disable */
import adapter from '@sveltejs/adapter-static'
import path from 'node:path'
import preprocess from 'svelte-preprocess'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: preprocess(),

  kit: {
    adapter: adapter({
      pages: './dist/',
      assets: './dist/',
      fallback: 'index.html',
    }),

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
