import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    kit: {
        adapter: adapter({
            fallback: 'index.html'
        }),

        alias: {
            $components: 'src/lib/components'
        }
    },

    preprocess: vitePreprocess()
};

export default config;
