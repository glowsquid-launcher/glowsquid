<script lang="ts">
	import Header from './header.svelte';

    import "@repo/ui/theme/breakpoints.css";
    import "@repo/ui/theme/colors/dark.css";
    import "@repo/ui/theme/sizing/default.css";
    import "@repo/ui/theme/fonts/recursive.css";
    import "./app.css";
	import { type Snippet } from 'svelte';
    import { setupState  } from 'tauri-plugin-state';
    import { Map } from "svelte/reactivity";

    setupState(new Map())

    const { children }: { children: Snippet } = $props();
</script>


<Header />

<main>
    {@render children()}
</main>
