import { beforeAll, expect, test, vi } from 'vitest';
import { randomFillSync } from 'crypto';

import { mockIPC } from '@tauri-apps/api/mocks';
import { invoke } from '@tauri-apps/api/tauri';

// jsdom doesn't come with a WebCrypto implementation
beforeAll(() => {
	Object.defineProperty(window, 'crypto', {
		value: {
			getRandomValues: (buffer) => {
				return randomFillSync(buffer);
			}
		}
	});
});

test('IPC mocks work', async () => {
	mockIPC((cmd, args) => {
		// simulated rust command called "add" that just adds two numbers
		if (cmd === 'add') {
			return (args.a as number) + (args.b as number);
		}
	});

	// we can use the spying tools provided by vitest to track the mocked function
	const spy = vi.spyOn(window, '__TAURI_IPC__');

	expect(invoke('add', { a: 12, b: 15 })).resolves.toBe(27);
	expect(spy).toHaveBeenCalled();
});
