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

/**
 * Starts the process of adding an account via microsoft authentication
 * It takes a callback for when the current state of adding the account changes
 */
export async function addAccount(cb: EventCallback<AddAccountProcessPayload>) {
  const unlisten = await listen('add_account_process', cb)
  invoke<void>('add_new_account').catch(async (err: string) => {
    await emit('add_account_process', AddAccountProcessPayload.Failed)
    throw err
  })
  return unlisten
}

/**
 * Fetches all accounts from the internal database
 * @returns A list of accounts
 */
export async function getAccounts() {
  const internAccs = await invoke<InternalAccount[]>('get_accounts')

  return internAccs.map<Account>((acc) => ({
    id: acc.id,
    username: acc.username,
    accessToken: acc.accessToken,
    refreshToken: acc.refreshToken,
    expiresAt: new Date(acc.expiresAt),
    lastRefreshed: new Date(acc.lastRefreshed),
  }))
}
