<script lang="ts">
	import Header from './header.svelte';

    import "@repo/ui/theme/breakpoints.css";
    import "@repo/ui/theme/colors/dark.css";
    import "@repo/ui/theme/sizing/default.css";
    import "@repo/ui/theme/fonts/recursive.css";
    import "./app.css";
</script>


<Header />

<main>
    <slot />
</main>
