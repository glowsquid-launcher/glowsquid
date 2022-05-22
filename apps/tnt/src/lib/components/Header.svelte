<script lang="ts">
  import { versionStore } from '$lib/invoke/misc'
  import { goto } from '$app/navigation'
  import { Button } from '@glowsquid/glow-ui'
</script>

<header class="flex flex-row justify-between bg-primary pa-2">
  <div class="cursor-pointer" on:click={() => goto('/')}>
    <h1 class="text-white text-xl">
      Glowsquid
      <span class="text-sm text-white">{$versionStore}</span>
    </h1>
  </div>
  <div class="flex flex-row">
    <Button on:click={() => goto('/instances')}>Instances</Button>
  </div>
  <!-- TODO: add account switcher and settings button -->
  <div>end</div>
</header>
