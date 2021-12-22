import preprocess from 'svelte-preprocess';
import adapter from '@sveltejs/adapter-static';
import WindiCSS from 'vite-plugin-windicss'
import { optimizeImports, optimizeCss, elements, icons, pictograms } from "carbon-preprocess-svelte";

const production = process.env.NODE_ENV === 'production';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// so many preprocessors
	preprocess: [
		optimizeImports(),
		icons(),
		pictograms(),
		elements({
			theme: "all"
		}),
		preprocess()
	],

	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			// for an SPA / dynamic app
			fallback: 'app.html'
		}),

		// hydrate the <div id="svelte"> element in src/app.html
		target: '#svelte',

		ssr: false,

		vite: {
			optimizeDeps: {
				include: ['@carbon/charts'],
			},
			ssr: {
				noExternal: ["@tauri-apps/api", production && '@carbon/charts'].filter(Boolean),
			},
			plugins: [WindiCSS(), production && optimizeCss()],
		}
	},
};

export default config;
