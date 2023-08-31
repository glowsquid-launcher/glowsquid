<script lang="ts">
    import 'iconify-icon';
    import '@fontsource-variable/manrope';
    import '@fontsource/ibm-plex-sans/300.css';
    import '@fontsource/ibm-plex-sans/400.css';
    import '@fontsource/ibm-plex-sans/500.css';
    import '../app.css';
    import '../app.dark.css';
    import AccountDropdown from './account-dropdown.svelte';
    import type {LayoutData} from './$types';
    import Input from '$components/input.svelte';
    import {LL, setLocale} from '@glowsquid/i18n';
    import {testConnection} from '$lib/bridge'

    export let data: LayoutData;

    setLocale(data.locale);

    testConnection().catch(console.error)
</script>

<header>
    <nav>
        <h1><a href="/">Glowsquid</a></h1>
        <div class="quick-search">
            <Input
                type="text"
                placeholder={$LL.header.quickSearch()}
                icon="search"
            />
        </div>
        <div>
            <AccountDropdown />
        </div>
    </nav>
</header>

<main>
    <slot />
</main>

<style>
    header {
        background: var(--primary-bg);
        padding: 1rem;
        font-size: 1.3rem;
    }

    nav {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    h1 {
        margin: 0;
    }

    a {
        color: inherit;
        text-decoration: none;
    }

    .quick-search {
        flex-grow: 1;
        margin: 0 5rem;

        & :global(.input-wrapper) {
            width: 100%;
        }

        & :global(input) {
            width: 100%;
        }
    }
</style>
