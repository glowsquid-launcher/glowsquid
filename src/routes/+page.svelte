<script lang="ts">
    import Instance from '$components/instance.svelte';
</script>

<h2>Instances</h2>

<div class="instances">
    {#each {length: 40} as _i}
        <!-- TODO: Arguments for instance -->
        <Instance />
    {/each}
</div>

<style>
    h2 {
        padding: 0 1rem;
    }

    .instances {
        padding: 0 1rem;
        /**
     * User input values.
     */
        --grid-layout-gap: 2rem;
        --grid-column-count: 6;
        --grid-item--min-width: 280px;

        /**
     * Calculated values.
     */
        --gap-count: calc(var(--grid-column-count) - 1);
        --total-gap-width: calc(
            var(--gap-count) * var(--grid-layout-gap)
        );
        --grid-item--max-width: calc(
            (100% - var(--total-gap-width)) / var(--grid-column-count)
        );

        display: grid;
        grid-template-columns: repeat(
            auto-fill,
            minmax(
                max(
                    var(--grid-item--min-width),
                    var(--grid-item--max-width)
                ),
                1fr
            )
        );
        grid-gap: var(--grid-layout-gap);
    }
</style>
