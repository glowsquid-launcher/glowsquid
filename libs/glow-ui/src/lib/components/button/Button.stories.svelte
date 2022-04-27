<script>
  import 'uno.css';
  import '$lib/themes.css';
  import '@unocss/reset/tailwind.css';

  import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
  import { ButtonStyle } from '$lib/types';
  import { setTheme } from '$lib/helpers';

  import Button from './Button.svelte';
  import { onMount } from 'svelte';

  onMount(() => setTheme('dark'));
</script>

<Meta
  argTypes={{
    onClick: {
      action: 'clicked',
    },
  }}
  component={Button}
  title="Button"
/>

<Template let:args>
  {#if (args.type = 'basic')}
    <Button variant={args.variant} on:click={args.onClick}>
      Set colour theme</Button
    >
  {/if}
</Template>

<Story
  args={{
    type: 'basic',
    cssTheme: 'dark',
    variant: ButtonStyle.Default,
  }}
  name="Text"
/>
