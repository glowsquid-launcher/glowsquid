import {sveltekit} from '@sveltejs/kit/vite';
import {defineConfig} from 'vitest/config';
import browserslist from 'browserslist';
import {browserslistToTargets} from 'lightningcss';

// https://tauri.app/v1/guides/faq#recommended-browserlist
const targets = browserslistToTargets(
    browserslist(['es2021', 'last 3 Chrome versions', 'safari 13'])
);

export default defineConfig({
    build: {
        cssMinify: 'lightningcss',
        target: ['es2021', 'safari13']
    },
    css: {
        devSourcemap: true,
        lightningcss: {
            drafts: {
                nesting: true
            },
            targets
        },
        transformer: 'lightningcss'
    },
    plugins: [sveltekit()],
    test: {
        include: ['src/**/*.{test,spec}.{js,ts}']
    }
});
