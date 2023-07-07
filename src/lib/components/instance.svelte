<script lang="ts">
    import Icon from './icon.svelte';
    import {goto} from '$app/navigation';

    export let id: string;
</script>

<article id="modpack" data-flip-id="modpack-{id}">
    <header>
        <img src="https://placehold.co/64" alt="Modpack Title icon" />

        <hgroup>
            <h1>
                {id}
            </h1>
            <h2>1.2.3</h2>
        </hgroup>
    </header>

    <p>
        Last Played today <br />
        Last updated 5 days ago <br />
    </p>

    <footer>
        <button>
            <Icon name="play" />
            Play
        </button>
        <button on:click={() => goto(`/instances/${id}`)}>
            <Icon name="sliders-2" />
            Options
        </button>
    </footer>
</article>

<style>
    article {
        display: grid;
        grid-auto-columns: auto auto auto;
        background: var(--primary-bg);
        border-radius: var(--rounding-large);

        & header {
            padding-left: 1rem;
            padding-right: 1rem;
            padding-top: 1rem;
            display: flex;
            gap: 1rem;

            & img {
                border-radius: var(--rounding-medium);
                aspect-ratio: 1/1;
                height: 100%;
            }

            & h1 {
                font-size: 1.6rem;
                margin: 0;
            }

            & h2 {
                margin-top: 0.25rem;
                font-size: 1.2rem;
                margin-bottom: 0;
                color: color-mix(
                    in srgb,
                    var(--text) 70%,
                    transparent
                );
            }
        }

        & p {
            padding: 1rem;
            margin: 0;
            font-size: 1.2rem;
        }

        & footer {
            display: grid;
            grid-template-columns: 1fr 1fr;

            & button {
                display: flex;
                align-items: center;
                justify-content: center;
                gap: 0.5ch;

                padding: 0.8rem 0;
                font-size: 1.2rem;
                background: var(--secondary-bg);
                color: var(--text);
                border: 0px solid var(--primary-bg);

                &:first-child {
                    position: relative;
                    border-radius: 0 0 0 var(--rounding-large);

                    &::after {
                        content: '';
                        background: var(--text);
                        position: absolute;
                        bottom: 25%;
                        right: 0;
                        height: 50%;
                        width: 1px;
                    }
                }

                &:last-child {
                    border-radius: 0 0 var(--rounding-large) 0;
                }
            }
        }
    }
</style>
