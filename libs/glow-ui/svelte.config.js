import adapter from '@sveltejs/adapter-auto'
import preprocess from 'svelte-preprocess'
import 'vitest/config'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: preprocess(),

  kit: {
    adapter: adapter(),
    package: {
      exports: (file) => file === 'index.js',
    },
  },
}

export default config
