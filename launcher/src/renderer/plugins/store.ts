/* eslint-disable @typescript-eslint/no-unused-vars */
/* eslint-disable no-unused-vars */
import Store from 'electron-store'
import { Plugin } from '@nuxt/types'

export const store = new Store<Record<'users' | 'selectedUser' | 'instances', any>>()
declare module 'vue/types/vue' {
  interface Vue {
    $electronStore: typeof store
  }
}

declare module '@nuxt/types' {
  interface NuxtAppOptions {
    $electronStore: typeof store
  }
  interface Context {
    $electronStore: typeof store
  }
}

declare module 'vuex/types/index' {
  interface Store<S> {
    $electronStore: typeof store
  }
}

const plugin: Plugin = (_, inject) => {
  console.log(store.path)
  store.set('active', true)
  inject('$electronStore', store)
}

export default plugin
