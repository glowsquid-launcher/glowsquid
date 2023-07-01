<script lang="ts">
    import {gsap} from 'gsap/dist/gsap';
    import {Flip} from 'gsap/dist/Flip';
    import {afterNavigate, beforeNavigate} from '$app/navigation';
    import Button from "$components/button.svelte";

    gsap.registerPlugin(Flip);
    let state: Flip.FlipState | undefined;
    const targets =
        '#modpack, #modpack-title, #modpack-icon, #modpack-version, #modpack-buttons, #nav';

    beforeNavigate(() => {
        state = Flip.getState(targets);
    });

    afterNavigate(() => {
        if (!state) return;

        Flip.from(state, {
            duration: 0.6,
            ease: 'elastic.out(1,1)',
            scale: true,
            targets
        });
    });
</script>

<div class="instance-bar">
    <h2>Instances</h2>
    <div class="instances-actions">
        <Button>
            <iconify-icon icon="pixelarticons:plus" inline />
            Add Instance
        </Button>
        <Button>
            <iconify-icon icon="pixelarticons:search" inline />
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
