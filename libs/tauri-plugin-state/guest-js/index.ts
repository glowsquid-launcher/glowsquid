import { emit, listen } from '@tauri-apps/api/event';

let internalStateStore: Map<string, unknown> = new Map();

/**
 * Set's up the js side for state management. This should be called once on page start/reload.
 *
 * @param mapImpl The map implementation to use for the state store. Defaults to a new Map instance. If you're using Svelte 5, you can use the map from `svelte/reactivity` to make the state reactive.
 */
export const setupState = (mapImpl: Map<string, unknown> = new Map()) => {
    internalStateStore.forEach((value, key) => {
        mapImpl.set(key, value)
    })

    internalStateStore = mapImpl

    listen<{ key: string, value: unknown }>("plugin:state-state_change", (event) => {
        const { key, value } = event.payload;
        internalStateStore?.set(key, value)
    });

    emit("plugin:state-sync_ready")
}

/**
 * Create a state getter object that can be used to get the current value of the state.
 * @param key The key to use for the state.
 * @param defaultValue The default value to use for the state when it's not set.
 * @returns A getter object with a `value` property that returns the current value of the state.
 */
export const tauriState = <T>(key: string, defaultValue: T) => {
    if (!internalStateStore.has(key)) {
        internalStateStore.set(key, defaultValue);
    }

    return {
        get value() {
            return internalStateStore.get(key) as T
        }
    }
}
