<script lang="ts">
    // @ts-ignore Need to use internal Svelte function
    import { get_current_component } from 'svelte/internal';
    import type { ActionArray } from '@smui/common/internal';
    import { useActions } from '@smui/common/internal';
    import { forwardEventsBuilder } from '@smui/common/internal';

    export let use: ActionArray = [];

    const forwardEvents = forwardEventsBuilder(
        get_current_component()
    );

    let className = '';
    export { className as class };
</script>

<button use:useActions={use} on:click class={className}>
    <slot />
</button>

<style lang="scss">
    button {
        background-color: var(--primary-bg);
        color: var(--text);
        padding: 0.5rem;
        font-size: 1.3rem;
        border: none;
        border-radius: var(--rounding-small);
        cursor: pointer;

        &:hover {
            outline: solid 2px var(--outline-bg);
        }
    }
</style>
