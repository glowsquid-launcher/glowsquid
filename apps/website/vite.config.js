import { sveltekit } from '@sveltejs/kit/vite'
import Unocss from 'unocss/vite'
import unoConfig from '../../uno.config.mjs'
import EnvironmentPlugin from 'vite-plugin-environment'

/** @type import('vite').UserConfig */
const config = {
  plugins: [
    sveltekit(),
    Unocss(unoConfig),
    EnvironmentPlugin(['MICROSOFT_CLIENT_ID', 'MICROSOFT_CLIENT_SECRET'], {
      defineOn: 'import.meta.env',
    }),
  ],
}

export default config
