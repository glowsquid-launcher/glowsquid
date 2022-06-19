<script lang="ts">
  import { Meta, Story, Template } from '@storybook/addon-svelte-csf'
  import Modal from '$lib/components/modal/Modal.svelte'

  import Button from '$lib/components/button/Button.svelte'
  import { ColorVariant } from '$lib/types'

  let isOpened = false
</script>

<Meta
  argTypes={{
    variant: {
      options: Object.values(ColorVariant),
      control: 'select',
    },

    onClosed: {
      action: 'Closed',
    },
  }}
  component={Modal}
  title="Selection/Dropdown"
/>

<Template let:args={{ headerText, onClosed, variant, description }} id="text">
  <Button on:click={() => (isOpened = true)}>open modal</Button>

  <Modal {isOpened} on:close={() => (isOpened = false)} {variant}>
    <h2 slot="title">{headerText}</h2>

    {description}

    <div slot="buttons">
      <Button
        on:click={() => {
          isOpened = false
          onClosed()
        }}
        variant={ColorVariant.Primary}
      >
        Close
      </Button>
    </div>
  </Modal>
</Template>

<Story
  name="Basic"
  template="text"
  args={{
    headerText: 'me when Lorem',
    description: `
Lorem ipsum dolor sit amet consectetur adipisicing elit. Itaque pariatur
maiores possimus vero, eum dicta in aliquid iste. Tempora ipsa similique
deleniti accusamus nisi hic vel, necessitatibus suscipit quo dolore!
	  `,
    variant: ColorVariant.Primary,
  }}
/>
