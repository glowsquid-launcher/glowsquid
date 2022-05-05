<script lang="ts">import 'uno.css'
import '$lib/themes.css'
import '@unocss/reset/tailwind.css'

import { Meta, Story, Template } from '@storybook/addon-svelte-csf'
import { ButtonVariant } from '$lib/types'
import { setTheme } from '$lib/helpers'

import Button from './Button.svelte'
import { onMount } from 'svelte'

onMount(() => setTheme('dark'))
</script>

<Meta
  argTypes={{
    variant: {
      options: Object.values(ButtonVariant),
      control: 'select'
    },

    cssTheme: {
      type: 'string',
      options: ['light', 'dark'],
      control: 'inline-radio'
    },

    onClick: {
      action: 'clicked'
    }
  }}
  component={Button}
  title="Buttons/Basic"
/>

<Template let:args={{ variant, onClick, cssTheme, disabled }} id="text">
  <Button
    {variant}
    on:click={() => {
      setTheme(cssTheme)
      onClick()
    }}
  >
    Set colour</Button
  >
</Template>

<Template let:args={{ variant, onClick, cssTheme, disabled }} id="complex">
  <Button
    {variant}
    on:click={() => {
      setTheme(cssTheme)
      onClick()
    }}
  >
    <div>
      <div class="i-mdi-folder-zip-outline" />
      Import ZIP
    </div>
  </Button>
</Template>

<Story
  name="Basic"
  template="text"
/>

<Story
  name="Complex"
  template="complex"
/>
