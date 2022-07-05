import { writable, derived } from 'svelte/store'

const defaultState = {
  modals: {
    createInstance: false,
    addAccount: false,
  },
  accounts: {
    current: null as string | null,
    list: [] as string[],
  },
}
const state = writable(defaultState)

/**
 * Toggles the visibility of the instance modal
 */
export const toggleInstanceModal = () => {
  state.update((s) => {
    s.modals.createInstance = !s.modals.createInstance
    return s
  })
}

/**
 * Toggles the visibility of the new account modal
 */
export const toggleNewAccountModal = () => {
  state.update((s) => {
    s.modals.addAccount = !s.modals.addAccount
    return s
  })
}

/**
 * sets the current active account to the given account ID
 */
export const updateCurrentAccount = (account: string): void => {
  state.update((s) => {
    if (s.accounts.list.includes(account)) {
      s.accounts.current = account
      return s
    } else {
      throw new Error('Account not found')
    }
  })
}

export default derived(state, ($state) => $state)
