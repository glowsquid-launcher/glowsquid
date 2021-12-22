<script lang="ts">
	import { Theme } from 'carbon-components-svelte';
	import { themeStore } from '$lib/store';
</script>

<Theme bind:theme={$themeStore}>
	<slot />
</Theme>
