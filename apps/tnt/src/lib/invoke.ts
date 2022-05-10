import { invoke as tauriInvoke } from '@tauri-apps/api/tauri'
import type { Object } from 'ts-toolbelt'

type FunctionType = (arg: Record<string | number | symbol, any>) => any

interface InvokeMap {
  'get_app_path'(): string
  'add_new_account'(arg: {dev: boolean}): void
}

/**
 * invokes a command except its fully typed
 *
*/
export function invoke<T extends InvokeMap, C extends Object.SelectKeys<T, FunctionType>> (
  c: C,
  args: Parameters<T[C]>[0]
): Promise<ReturnType<T[C]>> {
  return tauriInvoke(c, args)
}
