<script lang="ts">
    import { createDropdownMenu, melt } from '@melt-ui/svelte'
    import { fly } from 'svelte/transition'

    import DownArrow from '~icons/material-symbols/keyboard-arrow-down'
    import Settings from '~icons/material-symbols/settings-outline'
    import Plus from '~icons/material-symbols/add'
	import { commands } from '$lib/bindings'

    const addAccount = (e: Event) => {
        console.log('Add account')

        commands.addAccount()
            .then((response) => {
                if (response.status === 'ok') {
                    console.log('Account added', response.data)
                }
                else {
                    console.error('Failed to add account', response.error.error, response.error.report)
                }
            })
    }

    const {
        elements: { trigger, menu, item, separator },
        states: { open },
    } = createDropdownMenu({
        forceVisible: true,
        loop: true,
    })
</script>

<button
    class="dropdown-button"
    use:melt={$trigger}
    aria-label="Open account dropdown"
>
    <img
        alt="TNTMan1671's avatar"
        src="https://minepic.org/avatar/TNTMan1671/32"
        width="32"
        height="32"
    >

    <p>TNTMan1671</p>

    <DownArrow class="account-arrow" />
</button>

{#if $open}
    <ul
        use:melt={$menu}
        transition:fly={{ duration: 150, y: -10 }}
    >
        <li use:melt={$item}>
            <button>
                <img
                    alt="TNTMan1671's avatar"
                    src="https://minepic.org/avatar/TNTMan1671/32"
                    width="32"
                    height="32"
                >

                <p>TNTMan1671</p>
            </button>
        </li>

        <li use:melt={$item}>
            <button>
                <img
                    alt="KrazyMiner001's avatar"
                    src="https://minepic.org/avatar/KrazyMiner001/32"
                    width="32"
                    height="32"
                >

                <p>KrazyMiner001</p>
            </button>
        </li>

        <hr use:melt={$separator}>

        <li use:melt={$item}>
            <button onclick={addAccount}>
                <Plus />

                <p>Add account</p>
            </button>
        </li>

        <li use:melt={$item}>
            <button>
                <Settings />

                <p>Manage accounts</p>
            </button>
        </li>
    </ul>
{/if}

<style>
    button {
        display: flex;
        align-items: center;

        font-size: var(--font-size-md);
        border-radius: var(--rounded-md);
        background-color: var(--bg-overlay);
        color: var(--fg-overlay);

        border: none;
        padding: var(--size-sm);
    }

    img {
        margin-right: 0.5ch;
    }

    ul {
        list-style: none;
        display: flex;
        flex-direction: column;
        gap: var(--size-sm);
        align-items: start;

        background-color: var(--bg-overlay);
        border-radius: var(--rounded-md);

        padding: var(--size-sm);
    }

    li {
        display: flex;
        align-items: center;
        gap: var(--size-sm);
        padding: var(--size-sm);
        border-radius: var(--rounded-md);
        background-color: var(--bg-overlay);
        font-size: var(--font-size-md);
        color: var(--fg-overlay);

        & button {
            display: contents;
        }
    }

    hr {
        width: 90%;
        align-self: center;
        border-color: var(--muted);
    }

    :global(.account-arrow) {
        transition: transform 0.25s;
    }

    :global(.dropdown-button[data-state="open"] .account-arrow) {
        transform: rotate(180deg);
    }
</style>
