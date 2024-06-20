import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { svelteTesting } from '@testing-library/svelte/vite';
import Icons from 'unplugin-icons/vite'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
	plugins: [
        sveltekit(),
        svelteTesting(),
        Icons({
            compiler: 'svelte',
            autoInstall: true
        })
    ],

	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent vite from obscuring rust errors
	clearScreen: false,
	// 2. tauri expects a fixed port, fail if that port is not available
	server: {
		port: 1420,
		strictPort: true,
        fs: {
            allow: ["../../"]
        }
	},
	test: {
		coverage: {
			reporter: ['json']
		},
		environment: 'jsdom',
		setupFiles: ['./setup-vitest.js']
	},
}));
