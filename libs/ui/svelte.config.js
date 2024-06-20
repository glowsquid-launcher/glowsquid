import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { preprocessMeltUI } from '@melt-ui/pp'

export default {
	preprocess: [vitePreprocess(), preprocessMeltUI()]
};
