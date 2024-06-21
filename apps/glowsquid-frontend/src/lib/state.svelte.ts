import { emit, listen } from "@tauri-apps/api/event";
import { Map } from "svelte/reactivity";
import type { AuthState } from './bindings'

const internalStateStore = new Map<string, unknown>();

export const setupState = async () => {
    listen<{ key: string, value: unknown }>("state_change", (event) => {
        const { key, value } = event.payload;
        console.debug("state_change", { key, value })
        internalStateStore.set(key, value)
    });

    emit("sync_ready")
}

const tauriState = <T>(key: string, defaultValue: T) => {
    if (!internalStateStore.has(key)) {
        internalStateStore.set(key, defaultValue);
    }

    return {
        get value() {
            return internalStateStore.get(key) as T
        }
    }
}

export const authState = tauriState<AuthState>("auth", {
    profiles: [],
    currentProfileIndex: null,
})
