<script context="module" lang="ts">
  import type { Load } from '@sveltejs/kit';

  export const load: Load = async ({ url }) => {
    return {
      props: {
        key: url.pathname,
      },
    };
  };
</script>

<script lang="ts">
  import AddInstanceModal from '$lib/components/modals/AddInstanceModal.svelte';
  import PageTransition from '$lib/components/PageTransition.svelte';
  import LL from '$locales/i18n-svelte';
  // import { getVersion } from '@tauri-apps/api/app';
  import 'uno.css';
  import '@unocss/reset/tailwind.css';
  import { refreshLocales } from '$lib/util';

  export let key: string;

  refreshLocales();
</script>

<AddInstanceModal />

<div class="h-screen w-screen">
  Header here: i18n test: {$LL.HI({
    name: 'TABS',
  })}
  <div class="pt-16">
    <PageTransition refresh={key}>
      <slot />
    </PageTransition>
  </div>
</div>
