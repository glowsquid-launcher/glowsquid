<script lang="ts">
  import state, { toggleNewAccountModal } from '$state'
  import { Modal } from '@glowsquid/glow-ui'
  import { Button } from '@svelteuidev/core'
</script>

<Modal isOpened={$state.modals.addAccount} on:close={toggleNewAccountModal}>
  <h2 slot="title">Add new account</h2>

  TODO: add form

  <div slot="buttons">
    <Button color="var(--color-danger)" on:click={toggleNewAccountModal}> Cancel </Button>
  </div>
</Modal>
