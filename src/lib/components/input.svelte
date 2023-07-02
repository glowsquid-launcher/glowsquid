<script lang="ts">
    export let type: string;
    export let placeholder: string;
    export let label: null | string = null;
    /**
     * The icon to use
     */
    export let icon: string;

    import Icon from './icon.svelte';

    let className: string = '';
    export {className as class};
</script>

<div class="input-wrapper">
    {#if label}
        <label>
            {label}
            <Icon name={icon} />
            <input {type} {placeholder} required class={className} />
        </label>
    {:else}
        <Icon name={icon} />
        <input class={className} {type} {placeholder} required />
    {/if}
</div>

<style>
    input {
        padding: 0.5rem;
        background: var(--secondary-bg);
        border-radius: var(--rounding-small);
        border: none;
        color: var(--text);
    }

    .input-wrapper {
        display: inline-block;
        position: relative;

    }

    .input-wrapper :global(iconify-icon) {
        position: absolute;
        top: 6px;
        right: 0;
    }
</style>
