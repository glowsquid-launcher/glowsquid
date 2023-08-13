import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';
import { preprocessMeltUI } from '@melt-ui/pp';
import sequence from 'svelte-sequential-preprocessor';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    kit: {
        adapter: adapter({
            fallback: 'index.html'
        }),

        alias: {
            $components: 'src/lib/components',
            $i18n: 'src/i18n/i18n-svelte'
        }
    },

    preprocess: sequence([vitePreprocess(), preprocessMeltUI()])
};

export default config;
