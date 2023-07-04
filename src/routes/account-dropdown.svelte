<script lang="ts">
    import {createDropdownMenu} from '@melt-ui/svelte';
    import {slide} from 'svelte/transition';
    import Icon from '$components/icon.svelte';

    const {arrow, item, menu, open, separator, trigger} =
        createDropdownMenu();
</script>

<button {...$trigger} use:trigger.action>
    <img
        alt="TNTMan1671 Avatar"
        src="https://crafatar.com/renders/head/52ddf2f1a59f4a19822fa6157f705320?scale=2"
    />
    TNTMan1671

    <Icon name="chevron-down" />
</button>

{#if $open}
    <div
        {...$menu}
        use:menu.action
        class="menu"
        transition:slide={{axis: 'y'}}
    >
        <button {...item} use:item.action class="account">
            <img
                src="https://crafatar.com/renders/head/52ddf2f1a59f4a19822fa6157f705320?scale=2"
                width="24"
                height="24"
                alt="TNTMan1671 Avatar"
            />
            Some Other account
        </button>
        <button {...item} use:item.action class="account">
            <img
                src="https://crafatar.com/renders/head/52ddf2f1a59f4a19822fa6157f705320?scale=2"
                width="24"
                height="24"
                alt="TNTMan1671 Avatar"
            />
            Some Other account 2
        </button>
        <div {...separator} class="separator" />
        <button {...item} use:item.action class="account">
            <Icon name="user-plus" />
            Create account
        </button>
        <button {...item} use:item.action class="account">
            <Icon name="sliders-2" />
            Settings
        </button>
        <div {...$arrow} />
    </div>
{/if}

<style lang="scss">
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

        &:focus {
            outline: solid 2px var(--outline);
        }
    }

    button :global(iconify-icon) {
        transition: 300ms ease;
    }

    button[data-state='open'] :global(iconify-icon) {
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
