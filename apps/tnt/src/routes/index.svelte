<script lang="ts">import { animationTime } from '$lib/animations'
import { addInstanceModalActive } from '$lib/stores'
import { fade, fly } from 'svelte/transition'
import { Button } from '@glowsquid/glow-ui'
import { dev } from '$app/env'
import { invoke } from '$lib/invoke'

// get the last 4 used instances
// eslint-disable-next-line no-unused-vars
const quickLaunchList = [0, 4, 2, 3]

// eslint-disable-next-line no-unused-vars
const createNewInstance = () => {
  addInstanceModalActive.set(true)
}

function authenticateWithMicrosoft () {
  invoke('add_new_account', {
    dev
  })
}
</script>

<div class="flex flex-col content-center">
  <div class="w-4/5 self-center">
    <section>launch list</section>

    <hr
      class="mb-5 mt-5"
      in:fly={{ duration: animationTime, x: -1000 }}
      out:fly={{ duration: animationTime, x: 1000 }}
    />
    
    <Button on:click={authenticateWithMicrosoft}>
      Auth with MS
    </Button>

    <section>
      <Button class="mb-6">
        Add Instance
      </Button>
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
