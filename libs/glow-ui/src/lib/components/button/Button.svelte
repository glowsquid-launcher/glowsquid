<script lang="ts">import { ButtonVariant } from '../../types'

export let variant: ButtonVariant = ButtonVariant.Primary
$: disabled = variant === ButtonVariant.Disabled
</script>

<button on:click data-testid="button" class="btn {variant}" {disabled}>
  {#if variant === ButtonVariant.Link}
        <div class="i-mdi-open-in-new" />
  {/if}
  <slot />
</button>

<style>
  .primary {
    @apply bg-primary text-white hover-bg-highlight active-bg-active;
  }

  .secondary {
    @apply bg-secondary text-white hover-bg-highlight active-bg-active;
  }

  .success {
    @apply bg-success;
  }

  .danger {
    @apply bg-error;
  }

  .warning {
    @apply bg-warning important-text-black;
  }
  
  .link {
    @apply bg-link;
  }

  .btn {
    @apply font-bold text-white text-size-5 rounded-xl pb-2 pt-2 pl-4 pr-4 transition duration-300 ease-in-out ;
  }
  .btn:not(:disabled) {
    @apply hover-shadow-md;
  }

  .disabled {
    @apply important-bg-disabled important-text-gray-500;
  }
</style>
