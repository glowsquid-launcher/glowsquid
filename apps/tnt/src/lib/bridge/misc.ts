import { asyncReadable } from '@square/svelte-store'
import { getVersion } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api'
import { dev } from '$app/env'

export const versionStore = asyncReadable(
  'getting version...',
  async () => `v${await getVersion()}`
)

export const setInfo = () => {
  return invoke('set_info', {
    isDev: dev,
  })
}
