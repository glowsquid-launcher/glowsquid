import { beforeAll, expect, test, vi } from 'vitest'
import { randomFillSync } from 'crypto'

import { mockIPC } from '@tauri-apps/api/mocks'
import { invoke } from '@tauri-apps/api/core'

// jsdom doesn't come with a WebCrypto implementation
beforeAll(() => {
    Object.defineProperty(window, 'crypto', {
        value: {
            // @ts-expect-error buffer is fine
            getRandomValues: (buffer) => {
                return randomFillSync(buffer)
            },
        },
    })
})

test('IPC mocks work', async () => {
    // @ts-expect-error we know it's a number output
    mockIPC(async (cmd, payload) => {
        switch (cmd) {
            case 'add':
                // @ts-expect-error we know it's a number
                return (payload?.a as number) + (payload?.b as number)
            default:
                break
        }
    })

    // we can use the spying tools provided by vitest to track the mocked function
    // @ts-expect-error tauri internals does indeed exist due to mockIPC
    const spy = vi.spyOn(window.__TAURI_INTERNALS__, 'invoke')

    expect(invoke('add', { a: 12, b: 15 })).resolves.toBe(27)
    expect(spy).toHaveBeenCalled()
})
