<script lang="ts">
	import Header from './header.svelte';

    import "$lib/theme/breakpoints.css";
    import "$lib/theme/colors/dark.css";
    import "$lib/theme/sizing/default.css";
    import "$lib/theme/fonts/recursive.css";
    import "./app.css";
</script>


<Header />

<main>
    <slot />
</main>
