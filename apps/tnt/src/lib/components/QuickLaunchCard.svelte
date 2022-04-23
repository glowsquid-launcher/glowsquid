<script lang="ts">
  import { animationTime } from '$lib/animations';
  import { fly } from 'svelte/transition';

  export let title: string;
  export let version: string;
  export let id: string;

  function launch() {
    console.log(`Launching instance ${id}`);
  }
</script>

TODO
