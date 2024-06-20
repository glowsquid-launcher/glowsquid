<script lang="ts">
	import AccountDropdown from "./accountDropdown.svelte";
</script>

<header>
    <nav>
        <h1>
            <a href="/">Glowsquid</a>
        </h1>

        <input type="text" placeholder="Search anything..." />

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
    }

    input {
        border: none;
        border-radius: var(--rounded-md);
        background-color: var(--bg-overlay);
        padding: var(--size-thin) var(--size-md);
        height: calc(var(--size-md) * 2.2);
        flex: 1;
    }
</style>
