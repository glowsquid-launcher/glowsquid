<script context="module" lang="ts">
  import type { Load } from '@sveltejs/kit';
  export const load: Load = async ({ url }) => ({
    props: {
      key: url.pathname,
    },
  });
</script>

<script lang="ts">
  import { browser } from '$app/env';
  import AddInstanceModal from '$lib/components/modals/AddInstanceModal.svelte';
  import PageTransition from '$lib/components/PageTransition.svelte';
  // import { getVersion } from '@tauri-apps/api/app';
  import 'virtual:windi.css';

  // @ts-expect-error windi devtools exists during development and is an empty module at build
  if (browser) import('virtual:windi-devtools');
  export let key: string;
</script>

<AddInstanceModal />

<div class="w-screen h-screen">
	Header here
 <div class="pt-16">
    <PageTransition refresh={key}>
      <slot />
    </PageTransition>
  </div>
</div>
