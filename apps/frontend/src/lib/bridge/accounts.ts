import { derived, writable } from 'svelte/store';
import { Stronghold } from 'tauri-plugin-stronghold-api';

interface Account {}

interface AccountStore {
  accounts: Account[];
  selectedAccount?: number;
}

const internalAccountStore = writable<AccountStore>({
  accounts: []
});

export const loadAccounts = async (password: string) => {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const stronghold = await Stronghold.load('accounts', password);
};

export const availableAccounts = derived(
  internalAccountStore,
  ($internalAccountStore) => $internalAccountStore.accounts
);
export const selectedAccount = derived(
  internalAccountStore,
  ($internalAccountStore) =>
    $internalAccountStore.accounts[
      $internalAccountStore.selectedAccount
    ]
);
