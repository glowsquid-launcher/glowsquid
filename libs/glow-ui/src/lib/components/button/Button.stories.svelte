<script>
  import 'uno.css';
  import '@unocss/reset/tailwind.css';

  import { Meta, Story, Template } from '@storybook/addon-svelte-csf';
  import { ButtonStyle } from '../../types';

  import Button from './Button.svelte';
</script>

<Meta argTypes={{}} component={Button} title="Button" />

<Template let:args>
  {#if (args.type = 'basic')}
    <Button {...args} on:click={() => console.log('clicked')}>Hi</Button>
  {/if}
</Template>

<Story
  args={{
    type: 'basic',
    variant: ButtonStyle.Default,
  }}
  name="Text"
/>
