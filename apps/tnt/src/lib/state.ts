import { writable, derived } from 'svelte/store'
import { type Account, getAccounts } from '$bridge'

const defaultState = {
  modals: {
    createInstance: false,
    addAccount: false,
  },
  accounts: {
    current: null as Account | null,
    list: [] as Account[],
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
export const updateCurrentAccount = (accountId: string): void => {
  state.update((s) => {
    const acc = s.accounts.list.find((acc) => acc.id === accountId)
    if (acc) {
      s.accounts.current = acc
      return s
    } else {
      throw new Error('Account not found')
    }
  })
}

/**
 * Refetches the accounts from the internal database and adds them to the store
 */
export const updateAccounts = async (): Promise<void> => {
  const newAccounts = await getAccounts()
  state.update((s) => {
    s.accounts.list = newAccounts
    return s
  })
}

export default derived(state, ($state) => $state)
