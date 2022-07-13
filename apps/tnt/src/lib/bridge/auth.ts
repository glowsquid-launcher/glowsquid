import { invoke } from '@tauri-apps/api'
import { emit, listen, type EventCallback } from '@tauri-apps/api/event'

export enum AddAccountProcessPayload {
  WaitingForBrowser = 'WaitingForBrowser',
  RequestRecieved = 'RequestRecieved',
  Complete = 'Complete',
  Failed = 'Failed',
}

interface InternalAccount {
  id: string
  username: string
  accessToken: string
  refreshToken: string
  expiresAt: string
  lastRefreshed: string
}

export interface Account {
  id: string
  username: string
  accessToken: string
  refreshToken: string
  expiresAt: Date
  lastRefreshed: Date
}

export async function addAccount(cb: EventCallback<AddAccountProcessPayload>) {
  const unlisten = await listen('add_account_process', cb)
  invoke<void>('add_new_account').catch(async (err: string) => {
    console.log(err)
    await emit('add_account_process', AddAccountProcessPayload.Failed)
  })
  return unlisten
}

export async function getAccounts() {
  return invoke<InternalAccount[]>('get_accounts').then((internAccs) =>
    internAccs.map<Account>((acc) => ({
      id: acc.id,
      username: acc.username,
      accessToken: acc.accessToken,
      refreshToken: acc.refreshToken,
      expiresAt: new Date(acc.expiresAt),
      lastRefreshed: new Date(acc.lastRefreshed),
    }))
  )
}
