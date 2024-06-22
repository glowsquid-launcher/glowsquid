import type { AuthState } from './bindings'
import { tauriState } from 'tauri-plugin-state'

export const authState = tauriState<AuthState>('auth', {
    profiles: [],
    currentProfileIndex: null,
})
