import { asyncReadable } from '@square/svelte-store'
import { getVersion } from '@tauri-apps/api/app'
import { invoke } from '@tauri-apps/api'
import { dev } from '$app/env'

export const versionStore = asyncReadable(
  'getting version...',
  async () => `v${await getVersion()}`
)

/**
 * Sets the info in the tauri client
 * Currently, it sets if the app is running in dev mode for handling various things
 */
export const setInfo = () => {
  return invoke('set_info', {
    isDev: dev,
  })
}
