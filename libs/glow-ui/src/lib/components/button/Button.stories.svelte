<script lang="ts">
  import 'uno.css'
  import '$lib/themes.css'
  import '@unocss/reset/tailwind.css'

  import { Meta, Story, Template } from '@storybook/addon-svelte-csf'
  import { ButtonShape, ButtonVariant } from '$lib/types'
  import { setTheme } from '$lib/helpers'

  import Button from './Button.svelte'
  import { onMount } from 'svelte'

  onMount(() => setTheme('dark'))
</script>

<Meta
  argTypes={{
    variant: {
      options: Object.values(ButtonVariant),
      control: 'select',
    },

    shape: {
      options: Object.values(ButtonShape),
      control: 'select',
    },

    cssTheme: {
      type: 'string',
      options: ['light', 'dark'],
      defaultValue: 'dark',
      control: 'inline-radio',
    },

    onClick: {
      action: 'clicked',
    },
  }}
  component={Button}
  title="Buttons/Basic"
/>

<Template let:args={{ variant, shape, onClick, cssTheme, text }} id="text">
  <Button
    {variant}
    {shape}
    on:click={() => {
      setTheme(cssTheme)
      onClick()
    }}
  >
    {text}</Button
  >
</Template>

<Template let:args={{ variant, onClick, shape, cssTheme }} id="complex">
  <Button
    {variant}
    {shape}
    on:click={() => {
      setTheme(cssTheme)
      onClick()
    }}
  >
    <div>
      <div class="i-mdi-folder-zip-outline" />
      Import from ZIP
    </div>
  </Button>
</Template>

<Story
  name="Basic"
  template="text"
  args={{
    text: 'set colour',
  }}
/>

<Story name="Complex" template="complex" />
