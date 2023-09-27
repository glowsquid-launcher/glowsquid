<script lang="ts">
    import type { Writable } from 'svelte/store';

    import Instance from '$components/instance.svelte';
    import { getContext } from 'svelte';

    import type {PageData} from './$types';

    export let data: PageData;

    $: isMinified = getContext<Writable<boolean>>('instances-minified');
</script>

<div class="instances" class:collapsed={$isMinified}>
    {#each data.instances as instanceId (instanceId)}
        <!-- TODO: Arguments for instance -->
        <Instance collapsed={$isMinified} id={instanceId} />
    {/each}
</div>

<style>
    .instances {
        padding: 0 1rem;

        display: flex;
        flex-wrap: wrap;
        gap: 1rem;
    }

    .instances :global(article) {
        flex: 1 1 280px;
    }

    .instances :global(article.collapsed) {
        flex: 0 0 64px;
    }
</style>
