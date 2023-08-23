import {sveltekit} from '@sveltejs/kit/vite';
import {defineConfig} from 'vitest/config';
import browserslist from 'browserslist';
import {browserslistToTargets} from 'lightningcss';
import { fileURLToPath } from 'url';

// https://tauri.app/v1/guides/faq#recommended-browserlist
const targets = browserslistToTargets(
    browserslist(['last 3 Chrome versions', 'safari 13'])
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
    resolve: {
      alias: {
        "@glowsquid/i18n": fileURLToPath(
          new URL("/libs/i18n/src/index.ts", import.meta.url)
        )
      }
    },
    plugins: [sveltekit()],
    test: {
        include: ['src/**/*.{test,spec}.{js,ts}']
    }
});
