import { listen } from "@tauri-apps/api/event";
import { Map } from "svelte/reactivity";
import type { AuthState } from './bindings'

const internalStateStore = new Map<string, unknown>();

export const setupStateListener = () => {
    listen<{ key: string, value: unknown }>("state_update", (event) => internalStateStore.set(event.payload.key, event.payload.value));
}

const tauriState = <T>(key: string, defaultValue: T) => {
    internalStateStore.set(key, defaultValue);

    return {
        get value() {
            return internalStateStore.get(key) as T
        }
    }
}

export const authState = tauriState<AuthState>("auth", {
    profiles: [],
    currentProfile: null,
})
