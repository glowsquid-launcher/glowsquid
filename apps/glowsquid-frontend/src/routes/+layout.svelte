<script lang="ts">
	import Header from './header.svelte';

    import "@repo/ui/theme/breakpoints.css";
    import "@repo/ui/theme/colors/dark.css";
    import "@repo/ui/theme/sizing/default.css";
    import "@repo/ui/theme/fonts/recursive.css";
    import "./app.css";
	import type { Snippet } from 'svelte';
	import { setupState } from '$lib/state.svelte';

    const { children }: { children: Snippet } = $props();

    setupState()
</script>


<Header />

<main>
    {@render children()}
</main>
