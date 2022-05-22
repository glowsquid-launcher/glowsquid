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
  import Header from '$lib/components/Header.svelte'
  import AddInstanceModal from '$lib/components/modals/AddInstanceModal.svelte'
  import PageTransition from '$lib/components/PageTransition.svelte'
  import { setTheme } from '@glowsquid/glow-ui'
  // import { getVersion } from '@tauri-apps/api/app';
  import 'uno.css'
  import '@unocss/reset/tailwind.css'
  import '$lib/themes/default.css'
  import { refreshLocales } from '$lib/util'
  import { onMount } from 'svelte'

  export let key: string

  onMount(() => setTheme('dark'))
  refreshLocales()
</script>

<AddInstanceModal />

<div class="h-screen w-screen">
  <Header />
  <div class="pt-16">
    <PageTransition refresh={key}>
      <slot />
    </PageTransition>
  </div>
</div>
