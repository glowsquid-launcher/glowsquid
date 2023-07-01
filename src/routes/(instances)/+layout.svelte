<script lang="ts">
    import {gsap} from 'gsap/dist/gsap';
    import {Flip} from 'gsap/dist/Flip';
    import {afterNavigate, beforeNavigate} from '$app/navigation';

    gsap.registerPlugin(Flip);
    let state: Flip.FlipState | undefined;
    const targets =
        '#modpack, #modpack-title, #modpack-icon, #modpack-version, #modpack-buttons';

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

<slot />
