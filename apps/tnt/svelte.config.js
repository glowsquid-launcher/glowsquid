import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';
import Unocss from 'unocss/vite';
import {
  presetIcons,
  presetTypography,
  presetUno,
  presetAttributify,
} from 'unocss';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // so many preprocessors
  preprocess: [preprocess()],

  kit: {
    adapter: adapter({
      pages: './dist/',
      assets: './dist/',
      fallback: 'index.html',
    }),

    vite: {
      ssr: {
        noExternal: ['@tauri-apps/api'],
      },
      plugins: [
        Unocss({
          presets: [
            presetAttributify(),
            presetUno(),
            presetTypography(),
            presetIcons(),
          ],
        }),
      ],
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
