import { VueConstructor } from 'vue'
import { accessorType } from '@/store'
import { store } from '~/plugins/store'

declare const __resources : string

declare module 'vue/types/vue' {
  interface Vue {
    $accessor: typeof accessorType
    $electronStore: typeof store
  }
}

declare module '@nuxt/types' {
  interface NuxtAppOptions {
    $accessor: typeof accessorType
    $electronStore: typeof store
  }
}
declare module '*.vue' {
  const _default: VueConstructor
  export default _default
}
