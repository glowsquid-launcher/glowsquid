import { writable, derived } from 'svelte/store'

const defaultState = {
  modals: {
    createInstance: false,
  },
  accounts: {
    current: null as string | null,
    list: ['52ddf2f1-a59f-4a19-822f-a6157f705320'],
  },
}
const state = writable(defaultState)

export const toggleInstanceModal = () => {
  state.update((s) => {
    s.modals.createInstance = !s.modals.createInstance
    return s
  })
}

export const updateCurrentAccount = (account: string) => {
  state.update((s) => {
    if (s.accounts.list.includes(account)) {
      s.accounts.current = account
      return s
    } else {
      throw new Error('Account not found')
    }
  })
}

const exposedState = derived(state, (v) => v, defaultState)

export default exposedState
