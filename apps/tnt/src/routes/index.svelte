<script lang="ts">
  import { animationTime } from '$lib/animations'
  import { fade, fly } from 'svelte/transition'
</script>

<div class="flex flex-col content-center">
  <div class="w-4/5 self-center">
    <section>launch list</section>

    <hr
      class="mb-5 mt-5"
      in:fly={{ duration: animationTime, x: -1000 }}
      out:fly={{ duration: animationTime, x: 1000 }}
    />

    [INSERT NEWS HERE]

    <hr
      class="mb-5 mt-5"
      in:fly={{ duration: animationTime, x: -1000 }}
      out:fly={{ duration: animationTime, x: 1000 }}
    />

    <section transition:fade={{ duration: animationTime }}>
      <h2 class="mb-6">Playtime stats</h2>
      TODO
    </section>
  </div>
</div>
