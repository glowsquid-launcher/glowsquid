import { writable, readable } from 'svelte/store'

const defaultState = {
  modals: {
    createInstance: false,
  },
}
const state = writable(defaultState)

export const toggleInstanceModal = () => {
  state.update((s) => {
    s.modals.createInstance = !s.modals.createInstance
    return s
  })
}

const exposedState = readable<typeof defaultState>(defaultState, (set) =>
  state.subscribe((v) => set(v))
)

export default exposedState
