import { asyncReadable } from '@square/svelte-store'
import { getVersion } from '@tauri-apps/api/app'

export const versionStore = asyncReadable(
  'getting version...',
  async () => `v${await getVersion()}`
)
