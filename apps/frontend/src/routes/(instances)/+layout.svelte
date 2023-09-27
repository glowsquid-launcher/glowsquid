<script lang="ts">
    import {gsap} from 'gsap/dist/gsap';
    import {Flip} from 'gsap/dist/Flip';
    import {afterNavigate, beforeNavigate} from '$app/navigation';
    import Button from '$components/button.svelte';
    import Icon from '$components/icon.svelte';
    import { setContext } from 'svelte';
    import { writable } from 'svelte/store';

    gsap.registerPlugin(Flip);
    let state: Flip.FlipState | null = null;
    const targets = '#modpack';

    beforeNavigate(() => {
        state = Flip.getState(targets);
    });

    afterNavigate(async () => {
        if (!state) return;

        Flip.from(state, {
            duration: 0.6,
            ease: 'elastic.out(1,1)',
            scale: true,
            targets
        });

        state = null;
    });

    let instancesMinified = writable(false);
    $: setContext('instances-minified', instancesMinified);
</script>

<div class="instance-bar">
    <div class="instances-actions">
      <h2>
        Instances
      </h2>
    </div>
    <div class="instances-actions">
        <Button on:click={() => $instancesMinified = !$instancesMinified}>
            <Icon inline name={$instancesMinified ? "plus" : 'minus'} />
            {$instancesMinified ? 'Expand' : 'Collapse'} Instances
        </Button>
        <Button>
            <Icon inline name="plus" />
            Add Instance
        </Button>
        <Button>
            <Icon inline name="search" />
            Browse Modpacks
        </Button>
    </div>
</div>

<slot />

<style>
    .instance-bar {
        padding: 0 1rem;
        display: flex;

        align-items: center;
        justify-content: space-between;
    }

    .instances-actions {
        display: flex;
        gap: 1rem;
    }
</style>
