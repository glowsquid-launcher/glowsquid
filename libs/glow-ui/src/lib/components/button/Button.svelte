<script lang="ts">
  import { ButtonStyle } from '../../types';

  export let disabled = false;
  export let variant: ButtonStyle = ButtonStyle.Default;
</script>

<button
  on:click
  class="rounded-xl pb-2 pt-2 pl-4 pr-4 hover-shadow-md transition duration-300 ease-in-out {variant}"
  {disabled}
>
  <slot />
</button>

<style>
  .default {
    @apply bg-primary hover-bg-highlight active-bg-active text-white;
  }
</style>
