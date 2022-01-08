import { invoke as tauriInvoke } from '@tauri-apps/api'
import type { Object } from 'ts-toolbelt'
import type { Instance } from './instances'

type FunctionType = (arg: Record<string, unknown>) => unknown

interface InvokeMap {
	'get_instance'(args: { name: string }): Instance
	'get_app_path'(): string
}

export function invoke<T extends InvokeMap, C extends Object.SelectKeys<T, FunctionType>>(
	c: C,
	args: Parameters<T[C]>[0]
): Promise<ReturnType<T[C]>> {
	return tauriInvoke(c, args)
}
