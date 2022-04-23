import { browser } from "$app/env"
import type { Writable } from 'svelte/store'
import { get, writable } from 'svelte/store'

/**
 * creates a store that is persisted in localStorage
 * @param {string} key - the key to store the value under
 * @param {T} initValue the initial value of the store 
**/
const storage = <T>(key: string, initValue: T): Writable<T> => {
	const store = writable(initValue)
	if (!browser) return store

	const storedValueStr = localStorage.getItem(key)
	if (storedValueStr !== null) store.set(JSON.parse(storedValueStr))

	store.subscribe((val) => {
		if ([null, undefined].includes(val)) {
			localStorage.removeItem(key)
		} else {
			localStorage.setItem(key, JSON.stringify(val))
		}
	})

	window.addEventListener('storage', () => {
		const storedValueStr = localStorage.getItem(key)
		if (storedValueStr === null) return

		const localValue: T = JSON.parse(storedValueStr)
		if (localValue !== get(store)) store.set(localValue)
	})

	return store
}

export const instancesPath = storage<string>('instancesPath', '')
export const addInstanceModalActive = writable(false)
export const transitioning = writable(false)

if (browser) {
	import('./invoke')
		.then(({ invoke }) => invoke('get_app_path', undefined))
		.then((path) => instancesPath.set(`${path}instances`))
		// TODO: add error handling via a notification
		.catch(err => err)
}
