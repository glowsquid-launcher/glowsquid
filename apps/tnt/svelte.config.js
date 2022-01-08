import adapter from '@sveltejs/adapter-static';
import {
  elements,
  icons,
  optimizeCss,
  optimizeImports,
  pictograms
} from 'carbon-preprocess-svelte';
import preprocess from 'svelte-preprocess';
import WindiCSS from 'vite-plugin-windicss';

const production = process.env.NODE_ENV === 'production';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // so many preprocessors
  preprocess: [
    preprocess({}),
    optimizeImports(),
    icons(),
    pictograms(),
    elements({
      theme: 'all',
    }),
  ],

  kit: {
    adapter: adapter({
      pages: '../../dist/tnt',
      assets: '../../dist/tnt',
      // for an SPA / dynamic app
      fallback: 'index.html',
    }),

    // hydrate the <div id="svelte"> element in src/app.html
    target: '#svelte',

    ssr: false,

    vite: {
      optimizeDeps: {
        include: ['@carbon/charts'],
      },
      ssr: {
        noExternal: ['@tauri-apps/api', production && '@carbon/charts'].filter(
          Boolean
        ),
      },
      plugins: [WindiCSS(), production && optimizeCss()],
      esbuild: {
        target: ['esnext', 'chrome89', 'safari15.1', 'edge89', 'firefox89'],
      },
      build: {
        target: ['esnext', 'chrome89', 'safari15.1', 'edge89', 'firefox89'],
      },
    },
  },
};

export default config;
