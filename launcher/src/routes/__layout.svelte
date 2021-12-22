<script lang="ts">
	import { Theme } from 'carbon-components-svelte';
	import { themeStore } from '$lib/store';
	import 'virtual:windi.css';

	// if you want to enable windi devtools
	import { browser } from '$app/env';
	// @ts-ignore
	if (browser) import('virtual:windi-devtools');
</script>

<Theme bind:theme={$themeStore}>
	<slot />
</Theme>
