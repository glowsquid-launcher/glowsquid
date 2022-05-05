<script>import { animationTime, transitionDelay } from '$lib/animations'
import { fly } from 'svelte/transition'
export let refresh = ''
</script>

{#key refresh}
  <div
    in:fly={{ x: -5, duration: animationTime, delay: transitionDelay }}
    out:fly={{ x: 5, duration: animationTime }}
  >
    <slot />
  </div>
{/key}
