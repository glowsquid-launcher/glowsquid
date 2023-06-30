<script lang="ts">
  import { createDropdownMenu } from '@melt-ui/svelte';
  const { menu, item, trigger, arrow, separator } =
    createDropdownMenu();
</script>

<button {...$trigger} use:trigger.action>
  <img
    src="https://crafatar.com/renders/head/52ddf2f1a59f4a19822fa6157f705320?scale=2"
    alt="TNTMan1671 Avatar"
  />
  TNTMan1671

  <iconify-icon icon="pixelarticons:chevron-down" />
</button>
<div {...$menu} use:menu.action class="menu">
  <div {...item} use:item.action class="account">
    <img
      src="https://crafatar.com/renders/head/52ddf2f1a59f4a19822fa6157f705320?scale=1"
      alt="TNTMan1671 Avatar"
    />
    Some Other account
  </div>
  <div {...separator} class="separator" />
  <div {...item} use:item.action class="account">
    <iconify-icon icon="pixelarticons:user-plus" />
    Create account
  </div>
  <div {...$arrow} />
</div>

<style>
  button {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 1.3rem;

    background: var(--secondary-bg);
    padding: 0.5rem 1rem;
    border: none;
    border-radius: var(--rounding-small);
    color: var(--text);
  }

  .separator {
    margin: 0.1rem;
    height: 0.1rem;
    background: var(--primary-bg);
  }

  .menu {
    z-index: 10;

    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    font-size: 1.2rem;

    max-height: 400px;
    min-width: 200px;
    padding: 1rem;
    background: var(--secondary-bg);

    border-radius: var(--rounding-medium);
    outline: solid 1px var(--outline-bg);
  }

  .account {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
</style>
