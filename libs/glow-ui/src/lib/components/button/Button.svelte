<script lang="ts">import { ButtonVariant } from '../../types'

/**
   * Should the button be disabled?
   */
export let disabled = false
export let variant: ButtonVariant = ButtonVariant.Primary
</script>

<button on:click data-testid="button" class="btn {variant}" {disabled}>
  {#if variant === ButtonVariant.Link}
        <div class="i-mdi-open-in-new" />
  {/if}
  <slot />
</button>

<style>
  .primary {
    @apply bg-primary text-white;
  }

  .secondary {
    @apply bg-secondary text-white;
  }

  .primary:not(:disabled),
  .secondary:not(:disabled) {
    @apply hover-bg-highlight active-bg-active;
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
  .btn:disabled {
    @apply important-bg-disabled important-text-gray-500;
  }
</style>
