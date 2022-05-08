import preprocess from 'svelte-preprocess'
import adapter from '@sveltejs/adapter-auto'
import Unocss from 'unocss/vite'
import unoConfig from '../../uno.config.cjs'
import EnvironmentPlugin from 'vite-plugin-environment'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: preprocess(),

  kit: {
    adapter: adapter(),
    vite: {
      plugins: [Unocss(unoConfig), EnvironmentPlugin(['MICROSOFT_CLIENT_ID', 'MICROSOFT_CLIENT_SECRET'], { defineOn: 'import.meta.env' })]
    }
  }
}
export default config
