<script lang="ts">
    import type {LayoutData} from './$types';
    import Button from '$components/button.svelte'
    import Instance from "$components/instance.svelte";

    export let data: LayoutData;
    const needsUpdate = true;

    $: instances = data
        .instances
        .filter((id) => id !== data.id)
    console.log(instances)
</script>

<div class="instances-container">
    <aside class="instances-sidebar">
        {#each instances as id (id)}
            <!-- TODO: Arguments for instance -->
            <Instance {id}/>
        {/each}
    </aside>

    <article class="instance">
        <header data-flip-id="modpack-{data.id}" id="modpack">
            <img
                alt="Modpack Title icon"
                class="header-image"
                data-flip-id="modpack-icon-{data.id}"
                height="148"
                id="modpack-icon"
                src="https://placehold.co/128"
                width="148"
            />

            <hgroup>
                <h1
                    data-flip-id="modpack-title-{data.id}"
                    id="modpack-title"
                >
                    {data.id}
                </h1>
                <h2
                    data-flip-id="modpack-version-{data.id}"
                    id="modpack-version"
                >
                    1.2.3 | Last Played Yesterday | Last Updated 2 days ago
                </h2>
            </hgroup>

            <div class="buttons"
                 data-flip-id="modpack-buttons-{data.id}"
                 id="modpack-buttons"
            >
                <Button>
                    <iconify-icon icon="pixelarticons:play" inline/>
                    Play
                </Button>
                {#if needsUpdate}
                    <Button color="amber">
                        <iconify-icon icon="pixelarticons:reload" inline/>
                        Update
                    </Button>
                {/if}
            </div>
        </header>

        <slot/>
    </article>
</div>

<style lang="scss">
    .instances-container {
        display: grid;
        grid-template:
            'sidebar content';
        grid-template-columns: auto minmax(0, 1fr);
        gap: 1rem;
    }

    .instances-sidebar {
        grid-area: sidebar;
        display: flex;
        flex-direction: column;
        margin-left: 1rem;
        gap: 1rem;
        max-width: 24rem;
    }

    .instance {
        grid-area: content;
        margin-right: 1rem;
    }

    .back {
        margin-top: 1rem;
        margin-left: 1rem;
    }

    header {
        border: solid 2px var(--outline);
        background-color: var(--secondary-bg);
        border-radius: var(--rounding-large);
        padding: 1rem;

        display: grid;
        grid-template:
            'img title'
            'img buttons';

        grid-template-columns: auto minmax(0, 1fr);

        img {
            grid-area: img;
            border-radius: var(--rounding-small);
            margin-right: 1rem;
        }

        hgroup {
            grid-area: title;
        }

        .buttons {
            grid-area: buttons;
            display: flex;
            gap: 0.5rem;
        }

        h1 {
            font-size: 2.5rem;
            margin: 0;
        }

        h2 {
            margin: 0;
            font-size: 1.2rem;
            color: color-mix(in srgb, var(--text) 70%, transparent);
        }
    }

    footer {
        display: flex;
        flex-direction: column;
    }
</style>
