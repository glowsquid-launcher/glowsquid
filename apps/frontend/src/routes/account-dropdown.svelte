<script lang="ts">
    import Icon from '$components/icon.svelte';
    import {createDropdownMenu, melt} from '@melt-ui/svelte';
    import {fly} from 'svelte/transition';

    const {
        elements: {arrow, item, menu, separator, trigger},
        states: {open}
    } = createDropdownMenu({
        forceVisible: true
    });
</script>

<button use:melt={$trigger}>
    <img
        alt="TNTMan1671 Avatar"
        src="https://crafatar.com/renders/head/52ddf2f1a59f4a19822fa6157f705320?scale=2"
    />
    TNTMan1671

    <Icon name="chevron-down" />
</button>

{#if $open}
    <div
        class="menu"
        transition:fly={{duration: 150, y: -10}}
        use:melt={$menu}
    >
        <button class="account" use:melt={$item}>
            <img
                alt="TNTMan1671 Avatar"
                height="24"
                src="https://crafatar.com/renders/head/52ddf2f1a59f4a19822fa6157f705320?scale=2"
                width="24"
            />
            Some Other account
        </button>
        <button class="account" use:melt={$item}>
            <img
                alt="TNTMan1671 Avatar"
                height="24"
                src="https://crafatar.com/renders/head/52ddf2f1a59f4a19822fa6157f705320?scale=2"
                width="24"
            />
            Some Other account 2
        </button>
        <div class="separator" use:melt={$separator} />
        <button class="account" use:melt={$item}>
            <Icon name="user-plus" />
            {"Create Account"}
        </button>
        <button class="account" use:melt={$item}>
            <Icon name="sliders-2" />
            {"Settings"}
        </button>
        <div use:melt={$arrow} />
    </div>
{/if}

<style>
    button {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        font-size: 1.3rem;

        background: var(--secondary-bg);
        padding: 0.5rem 1rem;
        border: none;
        border-radius: var(--rounding-small);
        color: var(--text);

        & :focus {
            outline: solid 2px var(--outline);
        }
    }

    button :global(iconify-icon) {
        transition: 300ms ease;
    }

    button:is([data-state='open']) :global(iconify-icon) {
        rotate: 180deg;
    }

    .separator {
        margin: 0.1rem;
        height: 0.1rem;
        background: var(--primary-bg);
    }

    .menu {
        z-index: 10;

        display: flex !important;
        flex-direction: column;
        gap: 0.5rem;
        font-size: 1.2rem;

        max-height: 400px;
        min-width: 200px;
        padding: 1rem;
        background: var(--secondary-bg);

        border-radius: var(--rounding-medium);
        outline: solid 1px var(--outline);
        box-shadow: 8.0px 16.0px 16.0px oklch(0% 0 0 / 25%);
    }

    .account {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        background: var(--primary-bg);
        border-radius: var(--rounding-small);
        padding: 0.5rem;
    }
</style>
