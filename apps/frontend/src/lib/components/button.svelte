<script lang="ts">
    import type {ActionArray} from '@smui/common/internal';

    import {
        forwardEventsBuilder,
        useActions
    } from '@smui/common/internal';
    // @ts-expect-error to use internal Svelte function
    import {get_current_component as getCurrentComponent} from 'svelte/internal';

    import Icon from './icon.svelte';

    export let use: ActionArray = [];
    export let color:
        | 'amber'
        | 'green'
        | 'primary'
        | 'red'
        | 'secondary'
        | 'yellow' = 'primary';
    export let icon: null | string = null;
    let className = '';
    export {className as class};

    const forwardEvents = forwardEventsBuilder(getCurrentComponent());
</script>

<button
    class={className}
    class:amber={color === 'amber'}
    class:green={color === 'green'}
    class:primary={color === 'primary'}
    class:red={color === 'red'}
    class:secondary={color === 'secondary'}
    class:yellow={color === 'yellow'}
    use:forwardEvents
    use:useActions={use}
>
    {#if icon}
        <Icon name={icon} />
    {/if}
    <slot />
</button>

<style>
    button {
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 0.5ch;

        color: var(--text);
        padding: 0.5rem;
        font-size: 1.3rem;
        border: none;
        border-radius: var(--rounding-small);
        cursor: pointer;

        & :hover {
            outline: solid 2px var(--outline);
        }
    }

    .primary {
        background-color: var(--primary-bg);
    }

    .secondary {
        background-color: var(--secondary-bg);
    }

    .red {
        background-color: var(--red);
    }

    .green {
        background-color: var(--green);
    }

    .yellow {
        background-color: var(--yellow);
    }

    .amber {
        background-color: var(--amber);
    }
</style>
