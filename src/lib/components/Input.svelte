<script lang="ts">
  export let type: string;
  export let placeholder: string;
  export let label: string | null = null;

  export let icon: string;
  let className: string = '';
  export { className as class };
</script>

<div class="input-wrapper">
  {#if label}
    <label>
      {label}
      <iconify-icon {icon} />
      <input {type} {placeholder} required class={className} />
    </label>
  {:else}
    <iconify-icon {icon} />
    <input class={className} {type} {placeholder} required />
  {/if}
</div>

<style>
  input {
    padding: 0.5rem;
    background: var(--secondary-bg);
    border-radius: var(--rounding-small);
    border: none;
    color: var(--text);
  }

  .input-wrapper {
    display: inline-block;
    position: relative;
  }

  iconify-icon {
    position: absolute;
    top: 8px;
    right: 0px;
  }
</style>
