import preprocess from 'svelte-preprocess'
import adapter from '@sveltejs/adapter-vercel'
import Unocss from 'unocss/vite'
import unoConfig from '../../uno.config.cjs'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: preprocess(),

  kit: {
    adapter: adapter({
      edge: true
    }),
    vite: {
      plugins: [Unocss(unoConfig)]
    }
  }
}
export default config
