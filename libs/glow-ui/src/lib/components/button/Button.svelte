<script lang="ts">import { ButtonVariant } from '../../types'

/**
   * Should the button be disabled?
   */
export let disabled = false
export let variant: ButtonVariant = ButtonVariant.Primary
</script>

<button on:click data-testid="button" class="btn {variant}" {disabled}>
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
    @apply bg-success text-white;
  }

  .danger {
    @apply bg-error text-white;
  }

  .warning {
    @apply bg-warning text-gray-600;
  }

  .btn {
    @apply rounded-xl pb-2 pt-2 pl-4 pr-4 transition duration-300 ease-in-out font-bold text-white text-size-5;
  }
  .btn:not(:disabled) {
    @apply hover-shadow-md;
  }
  .btn:disabled {
    @apply important-bg-disabled;
  }
</style>
