<script lang="ts">
  import { versionStore } from '$bridge/misc'
  import state, { updateCurrentAccount } from '$state'
  import { $fetch as fetch } from 'ohmyfetch'
  import { goto } from '$app/navigation'
  import LL from '$locales/i18n-svelte'
  import {
    Button,
    ColorVariant,
    ButtonShape,
    Dropdown,
  } from '@glowsquid/glow-ui'
  import { derived, writable } from 'svelte/store'
  import { asyncDerived } from '@square/svelte-store'
  import type { PlayerDBMinecraftProfile } from '$lib/util'
  import PlayerCard from './cards/PlayerCard.svelte'

  const accountList = derived(state, (state) => state.accounts.list)
  const account = writable<number | null>(null)
  $: {
    if ($account) updateCurrentAccount($accountList[$account])
  }

  const profileList = asyncDerived(accountList, async (list) => {
    return await Promise.all(
      list.map((item) =>
        fetch<PlayerDBMinecraftProfile>(
          `https://playerdb.co/api/player/minecraft/${item}`
        )
      )
    ).then((res) => res.map((item) => item.data.player))
  })

  const currentProfile = asyncDerived(
    [profileList, account],
    async ([state, account]) => {
      if (account === null) return null
      return state[account]
    }
  )

  const buttonStyle = {
    variant: ColorVariant.Secondary,
    shape: ButtonShape.Square,
  }
</script>

<header
  class="flex flex-row items-center justify-between bg-primary pa-2 pl-4 pr-4"
>
  <div class="cursor-pointer" on:click={() => goto('/')}>
    <h1 class="text-white text-xl">
      {$LL.header.title()}
      <span class="text-sm text-white">{$versionStore}</span>
    </h1>
  </div>
  <div class="flex flex-row divide-background divide-x">
    <Button on:click={() => goto('/browse')} {...buttonStyle}>
      {$LL.header.tabs.browse()}
    </Button>
    <Button on:click={() => goto('/')} {...buttonStyle}>
      {$LL.header.tabs.home()}
    </Button>
    <Button on:click={() => goto('/instances')} {...buttonStyle}>
      {$LL.header.tabs.instances()}
    </Button>
  </div>
  <div>
    <Dropdown class="important-w-64" options={$profileList} selected={account}>
      <div slot="selected" class="text-center">
        {#if $currentProfile}
          <PlayerCard {...$currentProfile} />
        {/if}
      </div>
      <span slot="placeholder" class="text-center">
        {$LL.header.accounts.placeholderText()}
      </span>

      <div
        slot="optionTemplate"
        class="rounded-lg bg-{selected
          ? 'highlight'
          : 'primary'} pa-2 text-lg hover:bg-highlight hover:shadow-lg active:shadow-none transition"
        let:selected
        let:option
      >
        <PlayerCard {...option} />
      </div>

      <Button slot="append" class="mt-4 w-full">
        {$LL.header.accounts.addAccount()}
      </Button>
    </Dropdown>
  </div>
</header>
