<script lang="ts">
    import type {LayoutData} from './$types';

    export let data: LayoutData;
</script>

<article id="modpack" data-flip-id="modpack-{data.id}">
    <header>
        <img
            id="modpack-icon"
            src="https://placehold.co/128"
            alt="Modpack Title icon"
            data-flip-id="modpack-icon-{data.id}"
        />

        <hgroup>
            <h1
                id="modpack-title"
                data-flip-id="modpack-title-{data.id}"
            >
                Modpack Title
            </h1>
            <h2
                id="modpack-version"
                data-flip-id="modpack-version-{data.id}"
            >
                1.2.3
            </h2>

            <p>Last Played Yesterday</p>
            <p>Last Updated 2 days ago</p>
        </hgroup>
    </header>

    <footer
        id="modpack-buttons"
        data-flip-id="modpack-buttons-{data.id}"
    >
        <button>
            <iconify-icon icon="pixelarticons:play" inline />
            Play
        </button>
        <button>
            <iconify-icon icon="pixelarticons:reload" inline />
            Check for updates
        </button>
        <button>
            <iconify-icon icon="pixelarticons:trash" inline />
            Delete
        </button>
        <button on:click={() => window.history.back()}>
            <iconify-icon icon="pixelarticons:arrow-left" inline />
            Back
        </button>
    </footer>
</article>

<slot />

<style lang="scss">
    article {
        margin: 1rem;
        border: solid 2px var(--outline-bg);
        background-color: var(--secondary-bg);
        border-radius: var(--rounding-large);

        display: flex;
        align-items: stretch;
        justify-content: space-between;
    }

    header {
        padding: 2rem;
        display: flex;
        gap: 1rem;

        img {
            border-radius: var(--rounding-medium);
        }

        h1 {
            font-size: 2.5rem;
            margin: 0;
        }

        h2 {
            margin-top: 0.1rem;
            margin-bottom: 0.1rem;

            font-size: 1.2rem;
        }

        p {
            font-size: 1rem;
            margin-bottom: 0;
            margin-top: 0;
        }

        h2,
        p {
            color: color-mix(in srgb, var(--text) 70%, transparent);
        }
    }

    footer {
        display: flex;
        flex-direction: column;

        button {
            text-align: left;
            font-size: 1.5rem;
            padding: 0.3rem 0.5rem;
            border: none;
            background-color: var(--primary-bg);
            color: var(--text);
            border-left: solid 2px var(--outline-bg);
            border-bottom: solid 2px var(--outline-bg);

            &:first-child {
                position: relative;
                border-radius: 0 var(--rounding-large) 0 0;
            }

            &:last-child {
                border-radius: 0 0 0 var(--rounding-large);
            }
        }
    }
</style>
