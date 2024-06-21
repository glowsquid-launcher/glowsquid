<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements";

    interface Props extends HTMLAttributes<HTMLParagraphElement> {
        sponsor: string;
    }

    const { sponsor, ...rest }: Props = $props();
</script>

<p {...rest}>This app is sponsored by {sponsor}!</p>
