<script lang="ts">
  import { AddAccountProcessPayload, addAccount } from '$bridge';
  import LL from '$locales/i18n-svelte'

  import state, { toggleNewAccountModal, updateAccounts } from '$state'
  import { Modal } from '@glowsquid/glow-ui'
  import { Button, Center } from '@svelteuidev/core'

  let current_state: AddAccountProcessPayload | null = null; 

  function startAuth() {
    addAccount(v => {
      current_state = v.payload === AddAccountProcessPayload.Complete ? null : v.payload
      if(current_state === null) {
        toggleNewAccountModal()
        updateAccounts().catch(console.log)
      }
    })
  }
</script>

<Modal isOpened={$state.modals.addAccount} on:close={toggleNewAccountModal}>
  <h2 slot="title">Add new account</h2>

  <Center>
    {#if current_state}
      <span class="i-mdi-microsoft-xbox mr-2 text-xl" />
        { $LL.accounts.modal.states[current_state]() }
    {:else}
    <Button
      color="var(--color-secondary)"
      class="transition hover:shadow-lg active:shadow-none"
      on:click={startAuth}
    >
      <span class="i-mdi-microsoft-xbox mr-2 text-xl" />
        { $LL.accounts.modal.login() }
    </Button>
    {/if}
  </Center>

  <div slot="buttons">
    <Button color="var(--color-danger)" on:click={toggleNewAccountModal}>
      Cancel
    </Button>
  </div>
</Modal>
