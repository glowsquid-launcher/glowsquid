import { invoke as tauriInvoke } from '@tauri-apps/api/tauri'
import type { Object } from 'ts-toolbelt'
import type { Instance } from './instances'

type FunctionType = (arg: Record<string, unknown>) => unknown

interface InvokeMap {
  'get_instance'(args: { name: string }): Instance
  /**
   * gets `amount` or all instances
   *
   *
   * @param amount The amount of instances. Sorted by recently used
   * */
  'get_instances'(args: { amount?: number }): Instance[]
  'get_app_path'(): string
}

/**
 * invokes a command except its fully typed
 *
*/
export function invoke<T extends InvokeMap, C extends Object.SelectKeys<T, FunctionType>>(
  c: C,
  args: Parameters<T[C]>[0]
): Promise<ReturnType<T[C]>> {
  return tauriInvoke(c, args)
}
