import {sveltekit} from '@sveltejs/kit/vite';
import {defineConfig} from 'vitest/config';
import browserslist from 'browserslist';
import {browserslistToTargets} from 'lightningcss';
import { nxViteTsPaths } from '@nx/vite/plugins/nx-tsconfig-paths.plugin';

// https://tauri.app/v1/guides/faq#recommended-browserlist
const targets = browserslistToTargets(
    browserslist(['last 3 Chrome versions', 'safari 13'])
);

export default defineConfig({
    cacheDir: '../../node_modules/.vite/frontend',
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
    plugins: [
      nxViteTsPaths(),
      sveltekit()
    ],
    test: {
        include: ['src/**/*.{test,spec}.{js,ts}']
    }
});
