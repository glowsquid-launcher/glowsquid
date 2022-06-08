<script lang="ts">
  import { Meta, Story, Template } from '@storybook/addon-svelte-csf'
  import Dropdown from '$lib/components/dropdown/Dropdown.svelte'
  import { writable } from 'svelte/store'
  import Button from '../button/Button.svelte'

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
  <Dropdown {options} {selected} class="important-w-64">
    <span slot="selected" let:option class="text-center"
      >selected: {option}</span
    >
    <span slot="placeholder" class="text-center">Select an option</span>

    <span slot="optionTemplate" class="text-lg" let:selected let:option>
      {option}. selected: {selected}
    </span>

    <Button
      slot="append"
      class="w-full"
      on:click={() => {
        onClick()
        selected.set(null)
      }}
    >
      Clear
    </Button>
  </Dropdown>
</Template>

<Story
  name="Basic"
  template="text"
  args={{
    options: [1, 2, 3, 4, 5].map((i) => `option ${i}`),
  }}
/>
