<script lang="ts">import { ButtonVariant } from '../../types'

/**
   * Should the button be disabled?
   */
export let disabled = false
export let variant: ButtonVariant = ButtonVariant.Default
</script>

<button on:click data-testid="button" class="btn {variant}" {disabled}>
  <slot />
</button>

<style>
  .default {
    @apply bg-primary text-white;
  }
  .default:not(:disabled) {
    @apply hover-bg-highlight active-bg-active;
  }

  .error {
    @apply bg-error text-white;
  }

  .warning {
    @apply bg-warning text-gray-700;
  }

  .btn {
    @apply rounded-xl pb-2 pt-2 pl-4 pr-4 transition duration-300 ease-in-out;
  }
  .btn:not(:disabled) {
    @apply hover-shadow-md;
  }
  .btn:disabled {
    @apply important-bg-disabled;
  }
</style>
