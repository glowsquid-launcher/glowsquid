<script lang="ts">
  import { slide } from 'svelte/transition'
  import type { Writable } from 'svelte/store'
  import {
    Listbox,
    ListboxButton,
    ListboxOptions,
    ListboxOption,
  } from '@rgossiaux/svelte-headlessui'
  import { createStyles } from '@svelteuidev/core'

  export let options: any[] = []
  export let keyName: string | null = null

  // This is a store as the parent needs access to the selected value
  export let selected: Writable<number | null>

  let clazz = ''
  export { clazz as class }

  const useFocusRingStyle = createStyles({
    root: {
      focusRing: 'auto',
    },
  })

  $: ({ getStyles: getFocusRing } = useFocusRingStyle())
</script>

<Listbox
  class="w-full rounded-lg text-white {clazz}"
  on:change={(e) => selected.set(e.detail)}
  let:open
  data-testid="dropdown"
>
  <ListboxButton
    class="h-10 w-full flex flex-row cursor-pointer items-center justify-between rounded-lg pl-2 pr-2 transition {getFocusRing()}"
  >
    {#if $selected !== null}
      <slot name="selected" option={options[$selected]} />
    {:else}
      <slot name="placeholder" />
    {/if}

    <div class="i-mdi-chevron-{open ? 'up' : 'down'}" />
  </ListboxButton>

  <ListboxOptions
    class="absolute mt-2 flex flex-col rounded-xl bg-secondary pa-2 {getFocusRing()}"
  >
    <div transition:slide>
      {#each options as option, i (keyName ? option[keyName] : i)}
        <ListboxOption
          let:selected
          let:active
          class="cursor-pointer rounded"
          value={i}
        >
          <slot name="optionTemplate" {selected} {option} {active} />
        </ListboxOption>
      {/each}

      <ListboxOption
        value={null}
        let:active
        let:selected
        class="cursor-pointer rounded"
      >
        <slot name="append" {active} {selected} />
      </ListboxOption>
    </div>
  </ListboxOptions>
</Listbox>
