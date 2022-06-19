<script lang="ts">
  import { ButtonShape, ColorVariant } from '../../types'

  export let variant: ColorVariant = ColorVariant.Primary
  export let shape: ButtonShape = ButtonShape.Rounded
  $: cssShape = (() => {
    switch (shape) {
      case ButtonShape.Rounded:
        return 'rounded-xl'
      case ButtonShape.Square:
        return 'rounded-none'
      case ButtonShape.Circle:
        return 'rounded-full'
    }
  })()
  $: disabled = variant === ColorVariant.Disabled
  let clazz = ''
  export { clazz as class }
</script>

<button
  on:click
  data-testid="button"
  class="btn {variant} {cssShape} {clazz}"
  {disabled}
>
  {#if variant === ColorVariant.Link}
    <div class="i-mdi-open-in-new" />
  {/if}
  <slot />
</button>

<style>
  .primary {
    @apply bg-primary hover-bg-highlight active-bg-active;
  }

  .secondary {
    @apply bg-secondary hover-bg-highlight active-bg-active;
  }

  .success {
    @apply bg-success hover-bg-success-highlight active-bg-success-active;
  }

  .danger {
    @apply bg-danger hover-bg-danger-highlight active-bg-danger-active;
  }

  .link {
    @apply bg-link hover-bg-link-highlight active-bg-link-active;
  }

  .btn {
    @apply shadow-md font-bold text-white text-size-5 pa-2 transition duration-300 ease-in-out;
  }
  .btn:not(:disabled) {
    @apply hover-shadow-xl active-shadow-none;
  }

  .disabled {
    @apply bg-disabled text-gray-500 shadow-none;
  }
</style>
