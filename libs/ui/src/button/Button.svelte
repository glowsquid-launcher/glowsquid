<script lang="ts">
	import type { HTMLButtonAttributes } from "svelte/elements";
    import type { BaseColor, MeaningfulColor } from "../theme";

    interface Props extends HTMLButtonAttributes {
        color?: MeaningfulColor | BaseColor;
    }

    let { color = "overlay", children, ...props }: Props = $props();
</script>

<button
    {...props}
    class:base={color === "base"}
    class:surface={color === "surface"}
    class:overlay={color === "overlay"}
    class:primary={color === "primary"}
    class:secondary={color === "secondary"}
    class:success={color === "success"}
    class:warning={color === "warning"}
    class:danger={color === "danger"}
>
    {@render children?.()}
</button>

<style>
    button {
        cursor: pointer;
        border: none;
        border-radius: var(--rounded-md);
        padding: var(--size-thin) var(--size-sm);
        font-size: var(--font-size-base);

        &:hover {
            outline: var(--size-thin) solid var(--muted);
        }
    }

    .base {
        color: var(--fg-base);
        background-color: var(--bg-base);
    }

    .surface {
        color: var(--fg-surface);
        background-color: var(--bg-surface);
    }

    .overlay {
        color: var(--fg-overlay);
        background-color: var(--bg-overlay);
    }

    .primary {
        color: var(--fg-primary);
        background-color: var(--bg-primary);
    }

    .secondary {
        color: var(--fg-secondary);
        background-color: var(--bg-overlay);
    }

    .success {
        color: var(--fg-success);
        background-color: var(--bg-overlay);
    }

    .warning {
        color: var(--fg-warning);
        background-color: var(--bg-overlay);
    }

    .danger {
        color: var(--fg-danger);
        background-color: var(--bg-overlay);
    }
</style>
