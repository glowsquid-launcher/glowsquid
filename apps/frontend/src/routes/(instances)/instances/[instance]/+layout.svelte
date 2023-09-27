<script lang="ts">
    import type { Writable } from 'svelte/store';

    import {goto} from '$app/navigation';
    import Button from '$components/button.svelte';
    import Icon from '$components/icon.svelte';
    import Instance from '$components/instance.svelte';
    import {createTabs, melt} from '@melt-ui/svelte';
    import { getContext } from 'svelte';
    import { backOut } from 'svelte/easing';
    import { crossfade } from 'svelte/transition';

    import type {LayoutData} from './$types';

    export let data: LayoutData;
    const needsUpdate = true;

    $: instances = data?.instances.filter((id) => id !== data.id);
    const {
        elements: {list, root, trigger},
        states: { value }
    } = createTabs({
        defaultValue: 'home',
    });

    $: switch ($value) {
        case 'home': {
            goto(`/instances/${data.id}`);
            break;
        }

        case 'stats': {
            goto(`/instances/${data.id}/stats`);
            break;
        }

        case 'settings': {
            goto(`/instances/${data.id}/settings`);
            break;
        }

        default: {
            throw new Error(
                'Should not be able to select a non-existent tab'
            );
        }
    }

    const [send, receive] = crossfade({
      duration: 250,
      easing: backOut,
    });

    $: isMinified = getContext<Writable<boolean>>('instances-minified');
</script>

<div class="instances-container">
    <aside class="instances-sidebar">
        {#each instances as id (id)}
            <!-- TODO: Arguments for instance -->
            <Instance collapsed={$isMinified} {id} />
        {/each}
    </aside>

    <article class="instance">
        <header data-flip-id="modpack-{data.id}" id="modpack">
            <img
                alt="Modpack Title icon"
                class="header-image"
                height="148"
                src="https://placehold.co/128"
                width="148"
            />

            <hgroup>
                <h1>
                    {data.id}
                </h1>
                <h2>
                    1.2.3 | Last Played Yesterday | Last Updated 2
                    days ago
                </h2>
            </hgroup>

            <div class="buttons">
                <Button icon="play">Play</Button>
                {#if needsUpdate}
                    <Button color="amber" icon="reload">
                        Update
                    </Button>
                {/if}
            </div>
        </header>
        <div class="root" use:melt={$root}>
            <div
                aria-label="Manage your account"
                class="list"
                use:melt={$list}
            >
                <button
                    class="trigger"
                    use:melt={$trigger('home')}
                >
                    {#if $value === 'home'}
                        <div
                          class="slider"
                          in:receive={{ key: 'slider' }}
                          out:send={{ key: 'slider' }}
                        />
                    {/if}
                    <Icon name="bulletlist" />
                    Instance Settings
                </button>
                <button
                    class="trigger"
                    use:melt={$trigger('stats')}
                >
                    {#if $value === 'stats'}
                        <div
                          class="slider"
                          in:receive={{ key: 'slider' }}
                          out:send={{ key: 'slider' }}
                        />
                    {/if}
                    <Icon name="trending" />
                    Stats
                </button>
                <button
                    class="trigger"
                    use:melt={$trigger('settings')}
                >
                    {#if $value === 'settings'}
                        <div
                          class="slider"
                          in:receive={{ key: 'slider' }}
                          out:send={{ key: 'slider' }}
                        />
                    {/if}
                    <Icon name="minecraft-alt" set="arcticons" />
                    Minecraft Options
                </button>
            </div>

            <div class="content">
                <slot />
            </div>
        </div>
    </article>
</div>

<style>
    .list {
        margin: 1rem 0;
        border-radius: var(--rounding-large);
        display: grid;
        grid-template-rows: repeat(1, 1fr);
        grid-template-columns: repeat(3, 1fr);
        gap: 1rem;
        background-color: var(--secondary-bg);
        overflow: hidden;

        & .trigger {
            display: flex;
            align-items: center;
            justify-content: center;
            gap: 0.5ch;
            background-color: transparent;
            z-index: 1;
            position: relative;

            color: var(--text);
            padding: 0.5rem 0.3rem;
            font-size: 1.3rem;
            border: none;
            border-radius: var(--rounding-large);
            cursor: pointer;

            & .slider {
                position: absolute;
                left: 0;
                top: 0;
                background-color: var(--primary-bg);
                border-radius: var(--rounding-large);
                height: 100%;
                width: 100%;
                z-index: -1;
            }

            & :hover {
                outline: solid 2px var(--outline);
            }
        }
    }

    .instances-container {
        display: grid;
        grid-template: 'sidebar content';
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

        & img {
            grid-area: img;
            border-radius: var(--rounding-small);
            margin-right: 1rem;
        }

        & hgroup {
            grid-area: title;
        }

        & .buttons {
            grid-area: buttons;
            display: flex;
            gap: 0.5rem;
        }

        & h1 {
            font-size: 2.5rem;
            margin: 0;
        }

        & h2 {
            margin: 0;
            font-size: 1.2rem;
            color: color-mix(in srgb, var(--text) 70%, transparent);
        }
    }
</style>
