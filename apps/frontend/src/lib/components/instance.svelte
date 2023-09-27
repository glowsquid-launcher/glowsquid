<script lang="ts">
    import {goto} from '$app/navigation';

    import Icon from './icon.svelte';

    export let id: string;
    export let collapsed: boolean = false;
</script>

<article class:collapsed data-flip-id="modpack-{id}" id="modpack">
    <header>
        <div class="image-button">
          <img alt="Modpack Title icon" src="https://placehold.co/64" />

          <div class="overlay">
            <button>
              <Icon name="play" />
            </button>
            <button on:click={() => goto(`/instances/${id}`)}>
              <Icon name="sliders-2" />
            </button>
          </div>
        </div>

        <hgroup>
            <h1>
                {id}
            </h1>
            {#if !collapsed}
              <h2>1.2.3</h2>
            {/if}
        </hgroup>
    </header>

    {#if !collapsed}
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
    {/if}
</article>

<style>
    article {
        display: grid;
        grid-auto-columns: auto auto auto;
        background: var(--secondary-bg);
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

            & .image-button {
              position: relative;
              border-radius: var(--rounding-large);

              & .overlay {
                position: absolute;
                top: 0;
                left: 0;
                right: 0;
                height: 100%;
                width: 100%;

                opacity: 0;
                transition: .3s ease;

                background-color: color-mix(in srgb, var(--secondary-bg) 70%, transparent);
                border-radius: var(--rounding-medium);

                display: flex;
                flex-direction: row;
                align-items: center;
                justify-content: center;


                & button {
                  padding: 0;
                  margin: 0;
                  width: 100%;
                  height: 100%;

                  color: var(--text);
                  background-color: color-mix(in srgb, var(--secondary-bg) 70%, transparent);
                  border: none;

                  @nest &:hover {
                    background-color: transparent;
                  }

                  &:first-child {
                      position: relative;
                      border-radius: var(--rounding-medium) 0 0 var(--rounding-medium);

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
                      border-radius: 0 var(--rounding-medium) var(--rounding-medium) 0;
                  }
                }
              }

              @nest &:hover .overlay {
                opacity: 1;
              }
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
                background: var(--primary-bg);
                color: var(--text);
                border: 0px solid var(--secondary-bg);

                &:first-child {
                    position: relative;
                    border-radius: 0 0 0 var(--rounding-large);
                    border-left-width: 2px;
                    border-bottom-width: 2px;

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
                    border-right-width: 2px;
                    border-bottom-width: 2px;
                }
            }
        }
    }

    article.collapsed {
        & header {
            max-width: 6rem;
            gap: 0;
            flex-direction: column;

            padding-left: 0rem;
            padding-right: 0rem;
            padding-top: 0rem;

            & img {
              aspect-ratio: 1/1;
              width: 6rem;
            }

            & h1 {
              font-size: 1.2rem;
              overflow-wrap: break-word;
              text-align: center;
              padding: 0 0.5rem;
              padding-bottom: 0.5rem;
            }
        }
    }
</style>
