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

  onMount(() => {
    refreshLocales()
  })
</script>

<SvelteUIProvider
  config={{
    light: { bg: theme.color.background.value, color: 'var(--color-text)' },
    dark: { bg: theme.color.background.value, color: 'var(--color-text)' },
  }}
  themeObserver={null}
  class={theme}
  withNormalizeCSS
  withGlobalStyles
>
  <AppShell>
    <AddInstanceModal />

    <Header slot="header" />

    <slot>
      <div class="pt-16">
        <PageTransition refresh={key}>
          <slot />
        </PageTransition>
      </div>
    </slot>
  </AppShell>
</SvelteUIProvider>
