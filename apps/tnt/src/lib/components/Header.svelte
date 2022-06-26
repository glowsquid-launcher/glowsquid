<script lang="ts">
  import { versionStore } from '$bridge/misc'
  import state, { updateCurrentAccount } from '$state'
  import { $fetch as fetch } from 'ohmyfetch'
  import { goto } from '$app/navigation'
  import LL from '$locales/i18n-svelte'
  import { Dropdown } from '@glowsquid/glow-ui'
  import {
    Button,
    Center,
    createStyles,
    Header,
    UnstyledButton,
    type ButtonProps,
  } from '@svelteuidev/core'
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

  function getButtonProps(
    buttonLocation: 'left' | 'middle' | 'right'
  ): Partial<ButtonProps> & { class: string; ripple: boolean } {
    let roundedType = ''

    if (buttonLocation == 'left') {
      roundedType = 'important-rounded-l-lg'
    } else if (buttonLocation == 'right') {
      roundedType = 'important-rounded-r-lg'
    }

    return {
      radius: 0,
      ripple: true,
      class: `transition important-bg-secondary ${roundedType} hover:important-bg-highlight hover:shadow-lg`,
    }
  }

  const useOptionStyles = createStyles(
    (
      _,
      { active, lastElement }: { active: boolean; lastElement: boolean }
    ) => ({
      root: {
        'borderColor': active
          ? 'var(--color-background)'
          : 'var(--color-active)',
        'mb': !lastElement ? '$mdPX' : 'none',
        'bs': active ? '$xs' : 'none',
        '&:active': {
          bs: 'none',
        },
      },
    })
  )

  function createOptionStyles(active: boolean, lastElement: boolean): string {
    const { cx, getStyles } = useOptionStyles({ active, lastElement })
    return cx(
      getStyles(),
      'transition',
      'px-2',
      'py-1',
      'rounded-lg',
      'text-lg',
      'border-4'
    )
  }
</script>

<Header
  class="flex flex-row items-center justify-between bg-primary! px-4 py-2"
>
  <UnstyledButton on:click={() => goto('/')}>
    <h1 class="text-white text-xl">
      {$LL.header.title()}
      <span class="text-sm text-white">{$versionStore}</span>
    </h1>
  </UnstyledButton>

  <div class="flex gap-1 rounded-lg bg-background">
    <Button on:click={() => goto('/browse')} {...getButtonProps('left')}>
      {$LL.header.tabs.browse()}
    </Button>
    <Button on:click={() => goto('/')} {...getButtonProps('middle')}>
      {$LL.header.tabs.home()}
    </Button>
    <Button on:click={() => goto('/instances')} {...getButtonProps('right')}>
      {$LL.header.tabs.instances()}
    </Button>
  </div>

  <div>
    <Dropdown
      class="important-w-64 bg-secondary hover:bg-highlight active:bg-active"
      options={$profileList}
      selected={account}
    >
      <div slot="selected" class="text-center text-white">
        {#if $currentProfile}
          <PlayerCard {...$currentProfile} />
        {/if}
      </div>

      <span slot="placeholder" class="text-center text-white">
        {$LL.header.accounts.placeholderText()}
      </span>

      <Center
        slot="optionTemplate"
        let:active
        let:option
        class={createOptionStyles(active, false)}
      >
        <PlayerCard {...option} />
      </Center>

      <p slot="append" let:active class={createOptionStyles(active, true)}>
        {$LL.header.accounts.addAccount()}
      </p>
    </Dropdown>
  </div>
</Header>
