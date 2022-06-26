<script lang="ts">
  import { Meta, Story, Template } from '@storybook/addon-svelte-csf'
  import Dropdown from '$lib/components/dropdown/Dropdown.svelte'
  import { writable } from 'svelte/store'
  import { SvelteUIProvider } from '@svelteuidev/core'

  let selected = writable<number | null>(null)
  $: console.log($selected)
</script>

<Meta
  argTypes={{
    onSelect: {
      action: 'selected',
    },
    onClick: {
      action: 'clicked',
    },
  }}
  component={Dropdown}
  title="Selection/Dropdown"
/>

<Template let:args={{ options, onClick }} id="text">
  <SvelteUIProvider>
    <Dropdown {options} {selected} class="important-w-64">
      <span slot="selected" let:option class="text-center text-white"
        >selected: {option}</span
      >
      <span slot="placeholder" class="text-center text-white"
        >Select an option</span
      >

      <div
        slot="optionTemplate"
        class="transition rounded pa-2 mb-2 bg-primary text-lg {active
          ? 'bg-highlight shadow-md'
          : ''}"
        let:selected
        let:option
        let:active
      >
        {option}
      </div>

      <p
        slot="append"
        let:active
        class="transition w-full cursor-pointer rounded bg-primary pa-2 {active
          ? 'bg-highlight shadow-md'
          : ''}"
        on:click={() => {
          onClick()
          selected.set(null)
        }}
      >
        Clear
      </p>
    </Dropdown>
  </SvelteUIProvider>
</Template>

<Story
  name="Basic"
  template="text"
  args={{
    options: [1, 2, 3, 4, 5].map((i) => `option ${i}`),
  }}
/>
