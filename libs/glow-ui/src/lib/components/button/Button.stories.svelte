<script>
  import 'uno.css';
  import { Meta, Template, Story } from '@storybook/addon-svelte-csf';

  import Button from './Button.svelte';
</script>

<!--👇 The title determines where your story goes in the story list -->
<Meta title="YourComponent" component={Button} argTypes={{}} />

<Template let:args>
  <Button {...args} />
</Template>

<Story name="FirstStory" args={{}} />
