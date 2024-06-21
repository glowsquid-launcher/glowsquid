<script lang="ts">
	import { Input } from "@repo/ui";
	import AccountDropdown from "./accountDropdown.svelte";

    import Search from "~icons/material-symbols/search";
</script>

<header>
    <nav>
        <h1>
            <a href="/">Glowsquid</a>
        </h1>

        <Input type="text" placeholder="Search anything...">
            {#snippet startIcon()}
                <Search />
            {/snippet}
        </Input>

        <AccountDropdown />
    </nav>
</header>

<style>
    nav {
        display: flex;
        align-items: center;
        justify-content: space-between;
        gap: var(--size-md);
        padding: var(--size-sm);
        background-color: var(--bg-surface);
        height: calc(var(--size-lg) * 2.5);

        & h1 {
            font-size: var(--font-size-xl);

            & >a {
                color: var(--fg-base);
                text-decoration: none;
            }

            & :hover {
                text-decoration: underline;
            }
        }

        & :global(div:has(input)) {
            flex: 1;

            & :global(input) {
                width: 100%;
            }
        }
    }

</style>
