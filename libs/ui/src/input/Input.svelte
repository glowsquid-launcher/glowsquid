<script lang="ts">
	import type { HTMLInputAttributes } from "svelte/elements";
    import type { BaseColor } from "../theme";
	import type { Snippet } from "svelte";

    interface Props extends HTMLInputAttributes {
        color?: BaseColor;
        startIcon?: Snippet
    }

    let { value = $bindable(), startIcon, color = "overlay", ...props }: Props = $props();
</script>

<div>
    {#if startIcon}
        <span>
            {@render startIcon()}
        </span>
    {/if}

    <input
        {...props}
        bind:value
        class:base={color === "base"}
        class:surface={color === "surface"}
        class:overlay={color === "overlay"}
        class:primary={color === "primary"}
    >
</div>


<style>
    div {
        position: relative;

        &:has(span) input {
            padding-left: calc(var(--size-md) * 2);
        }
    }

    span {
        position: absolute;
        left: var(--size-sm);

        top: 0;
        bottom: 0;
        margin: auto 0;
        height: min-content;
    }

    input {
        border: none;
        border-radius: var(--rounded-md);
        padding: var(--size-thin) var(--size-sm);
        height: var(--size-xl);

        font-size: var(--font-size-base);
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
</style>
