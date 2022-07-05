<script lang="ts">
  import state, { toggleNewAccountModal } from '$state'
  import { Modal } from '@glowsquid/glow-ui'
  import { Button, Center } from '@svelteuidev/core'

  function startAuth() {
    console.log('starting auth...')
  }
</script>

<Modal isOpened={$state.modals.addAccount} on:close={toggleNewAccountModal}>
  <h2 slot="title">Add new account</h2>

  <Center>
    <Button
      color="var(--color-secondary)"
      class="transition hover:shadow-lg active:shadow-none"
      on:click={startAuth}
    >
      <span class="i-mdi-microsoft-xbox mr-2 text-xl" />
      Login with Xbox/Microsoft
    </Button>
  </Center>

  <div slot="buttons">
    <Button color="var(--color-danger)" on:click={toggleNewAccountModal}>
      Cancel
    </Button>
  </div>
</Modal>
