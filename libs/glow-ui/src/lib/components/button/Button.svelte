<script lang="ts">
  import { ButtonStyle } from '../../types';

  export let disabled = false;
  export let variant: ButtonStyle = ButtonStyle.Default;
</script>

<button
  on:click
  class="flex content-center pb-1 pl-4 pr-4 pt-1 outline-none {variant}"
  {disabled}
>
  <slot />
</button>

<style>
  .default {
    @apply bg-blue-500 hover-bg-blue-700 text-white;
  }
</style>
