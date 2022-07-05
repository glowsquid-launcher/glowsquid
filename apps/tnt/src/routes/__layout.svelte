<script context="module" lang="ts">
  import type { Load } from '@sveltejs/kit'

  export const load: Load = async ({ url }) => {
    return {
      props: {
        key: url.pathname,
      },
    }
  }
</script>

<script lang="ts">
  import Header from '$components/Header.svelte'
  import AddInstanceModal from '$components/modals/AddInstanceModal.svelte'
  import PageTransition from '$components/PageTransition.svelte'
  import { AppShell, SvelteUIProvider } from '@svelteuidev/core'
  import { theme } from '$lib/themes'

  import 'uno.css'
  import '@unocss/reset/tailwind.css'
  import '$lib/themes/default.css'

  import { refreshLocales } from '$lib/util'
  import { onMount } from 'svelte'

  export let key: string
  const config = {
    light: { bg: 'primary', color: 'var(--color-text)' },
    dark: { bg: 'background', color: 'var(--color-text)' },
  }

  onMount(() => {
    refreshLocales()
  })
</script>

<SvelteUIProvider {config} class={theme} withGlobalStyles>
  <AppShell class="bg-background text-white">
    <Header slot="header" />

    <slot>
      <PageTransition refresh={key}>
        <slot />
      </PageTransition>
    </slot>
  </AppShell>
</SvelteUIProvider>

<style>
  :global(html) {
    background-color: var(--color-background);
  }
</style>
