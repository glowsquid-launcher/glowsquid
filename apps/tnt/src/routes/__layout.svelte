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
  import '@carbon/charts/styles-g100.css';
  import "carbon-components-svelte/css/all.css"
  import { getVersion } from '@tauri-apps/api/app';
  import { Button, Header, HeaderUtilities, Theme } from 'carbon-components-svelte';
  import { Cube16, Home16, Settings32 } from 'carbon-icons-svelte';
  import 'virtual:windi.css';

  // @ts-ignore
  if (browser) import('virtual:windi-devtools');
  export let key: string;
</script>

<Theme theme="g100" persist persistKey="__carbon-theme"  tokens={{
    "interactive-01": "#d02670",
    "hover-primary": "#ee5396",
    "active-primary": "#9f1853",
  }} />

<AddInstanceModal />

<div class="w-screen h-screen">
  <Header>
    <span slot="platform" class="flex">
      Glowsquid
      {#await getVersion()}
        Trying to get version...
      {:then version}
        v{version}
      {/await}
    </span>

    <HeaderUtilities>
      <div class="pr-12 grid grid-cols-3">
        <Button iconDescription="home" kind="ghost" icon={Home16} href="/"
          >Home</Button
        >
        <Button
          iconDescription="Instances"
          kind="ghost"
          icon={Cube16}
          href="/instances">Instances</Button
        >
        <Button
          iconDescription="Instances"
          kind="ghost"
          icon={Settings32}
          href="/instances">Settings</Button
        >
      </div>
    </HeaderUtilities>
  </Header>

  <div class="pt-16">
    <PageTransition refresh={key}>
      <slot />
    </PageTransition>
  </div>
</div>
