<script lang="ts">
    // @ts-ignore Need to use internal Svelte function
    import {get_current_component as getCurrentComponent} from 'svelte/internal';
    import type {ActionArray} from '@smui/common/internal';
    import {
        forwardEventsBuilder,
        useActions
    } from '@smui/common/internal';
    import Icon from './icon.svelte'

    export let use: ActionArray = [];
    export let color: 'primary' | 'secondary' | 'red' | 'green' | 'yellow' | 'amber'  = 'primary';
    export let icon: string | undefined = undefined;
    let className = ''
    export {className as class}

    const forwardEvents = forwardEventsBuilder(getCurrentComponent());
</script>

<button
    use:useActions={use}
    use:forwardEvents
    class={className}
    class:primary={color === 'primary'}
    class:secondary={color === 'secondary'}
    class:red={color === 'red'}
    class:green={color === 'green'}
    class:yellow={color === 'yellow'}
    class:amber={color === 'amber'}
>
    {#if icon}
        <Icon name={icon} />
    {/if}
    <slot/>
</button>

<style lang="scss">
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

        &:hover {
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
