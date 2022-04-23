import adapter from '@sveltejs/adapter-static';
import path from 'path';
import {
	optimizeImports,
} from 'carbon-preprocess-svelte';
import preprocess from 'svelte-preprocess';
import WindiCSS from 'vite-plugin-windicss';
import autoprefixer from 'autoprefixer';

const production = process.env.NODE_ENV === 'production';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// so many preprocessors
	preprocess: [
	preprocess({
postcss: {
          plugins: [
            autoprefixer({
              overrideBrowserslist: ["last 1 version", "ie >= 11"],
            }),
          ],
        },
		}),
		optimizeImports(),
	],

	kit: {
		adapter: adapter({
			pages: '../../dist/apps/tnt',
			assets: '../../dist/apps/tnt',
			fallback: 'index.html',
		}),

		vite: {
			noExternal: ['@carbon/charts-svelte', '@carbon/charts'],
			optimizeDeps: {
				include: ['@carbon/charts', '@carbon/charts-svelte'],
			},
			ssr: {
				noExternal: ['@tauri-apps/api', production && '@carbon/charts'].filter(
					Boolean
				),
			},
			plugins: [WindiCSS()],
			esbuild: {
				target: ['esnext', 'chrome89', 'safari15.1', 'edge89', 'firefox89'],
			},
			build: {
				target: ['esnext', 'chrome89', 'safari15.1', 'edge89', 'firefox89'],
			},
			resolve: {
				alias: {
					"@glowsquid/glow-ui": path.resolve('../../libs/glow-ui/src/index.ts')
				}
			}
		},
	},
};

export default config;
