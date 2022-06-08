import { browser } from '$app/env'
import { invoke as tauriInvoke } from '@tauri-apps/api/tauri'
import type { Object } from 'ts-toolbelt'

type FunctionType = (args: any) => any // skipcq: JS-0323

interface InvokeMap {
  'get_app_path'(): string
  'add_new_account'(args: { dev: boolean }): void
  'get_version'(): string
}

/**
 * invokes a command except its fully typed
 *
 */
export default function invoke<
  T extends InvokeMap,
  C extends Object.SelectKeys<T, FunctionType>
>(c: C, args: Parameters<T[C]>[0]): Promise<ReturnType<T[C]>> {
  if (browser) return tauriInvoke(c, args)
  else throw new Error('Invoke not supported in browser')
}
