<script lang="ts">
  import { animationTime } from '$lib/animations';
  import { addInstanceModalActive } from '$lib/stores';
  import { fade, fly } from 'svelte/transition';

  // get the last 4 used instances
  let quickLaunchList = [0, 4, 2, 3];

  const createNewInstance = () => {
    addInstanceModalActive.set(true);
  };
</script>

<div class="flex flex-col content-center">
  <div class="w-4/5 self-center">
    <section>launch list</section>

    <hr
      class="mb-5 mt-5"
      in:fly={{ duration: animationTime, x: -1000 }}
      out:fly={{ duration: animationTime, x: 1000 }}
    />

    <section>
      <h2 transition:fade={{ duration: animationTime }} class="mb-6">
        Add New Instance
      </h2>
    </section>

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
