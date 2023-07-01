<script lang="ts">
    // @ts-ignore Need to use internal Svelte function
    import {get_current_component as getCurrentComponent} from 'svelte/internal';
    import type {ActionArray} from '@smui/common/internal';
    import {
        forwardEventsBuilder,
        useActions
    } from '@smui/common/internal';

    export let use: ActionArray = [];

    const forwardEvents = forwardEventsBuilder(getCurrentComponent());

    let className = '';
    export {className as class};
</script>

<button use:useActions={use} use:forwardEvents class={className}>
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
