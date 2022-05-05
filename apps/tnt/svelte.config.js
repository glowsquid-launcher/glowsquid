import adapter from '@sveltejs/adapter-static'
import path from 'path'
import preprocess from 'svelte-preprocess'
import Unocss from 'unocss/vite'
import unoConfig from '../../uno.config.cjs'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: preprocess(),

  kit: {
    adapter: adapter({
      pages: './dist/',
      assets: './dist/',
      fallback: 'index.html'
    }),

    vite: {
      ssr: {
        noExternal: ['@tauri-apps/api']
      },
      plugins: [Unocss(unoConfig)],
      esbuild: {
        target: ['esnext', 'chrome89', 'safari15.1', 'edge89', 'firefox89']
      },
      build: {
        target: ['esnext', 'chrome89', 'safari15.1', 'edge89', 'firefox89']
      },
      resolve: {
        alias: {
          $locales: path.resolve('./src/lib/locales'),
          $lib: path.resolve('./src/lib')
        }
      }
    }
  }
}

export default config
