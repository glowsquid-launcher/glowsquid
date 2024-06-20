<script lang="ts">

</script>

<header>
    <nav>
        <h1>
            <a href="/">Glowsquid</a>
        </h1>

    </nav>
</header>

<style>
    nav {
        display: flex;
        justify-content: space-between;
        padding: var(--size-sm);
        background-color: var(--bg-surface);

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
</style>
