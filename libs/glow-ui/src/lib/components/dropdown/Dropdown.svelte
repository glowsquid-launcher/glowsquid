<script lang="ts">
  import { slide } from 'svelte/transition'
  import type { Writable } from 'svelte/store'

  export let options: any[] = []
  export let keyName: string | null = null
  export let selected: Writable<number | null>
  let clazz = ''
  export { clazz as class }

  let isHidden = true
</script>

<div class="w-full rounded-lg bg-primary text-white {clazz}">
  <div
    class="h-10 flex flex-row cursor-pointer items-center justify-between rounded-lg pl-2 pr-2 transition hover:bg-secondary"
    on:click={() => {
      isHidden = !isHidden
    }}
  >
    {#if $selected !== null}
      <slot name="selected" option={options[$selected]} />
    {:else}
      <slot name="placeholder" />
    {/if}

    <div class="i-mdi-chevron-{isHidden ? 'down' : 'up'}" />
  </div>

  {#if !isHidden}
    <ul
      class="absolute mt-2 flex flex-col rounded-xl bg-secondary pa-2"
      transition:slide
    >
      {#each options as option, i (keyName ? option[keyName] : i)}
        <li
          class="cursor-pointer rounded"
          on:click={() => {
            $selected = i
            isHidden = true
          }}
        >
          <slot name="optionTemplate" selected={i === $selected} {option} />
        </li>
      {/each}

      <li>
        <slot name="append" />
      </li>
    </ul>
  {/if}
</div>
