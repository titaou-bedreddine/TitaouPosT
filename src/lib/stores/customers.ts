import { writable, derived, get } from 'svelte/store';
import type { Customer } from '../types';
import { invoke } from '@tauri-apps/api/core';

export const customers = writable<Customer[]>([]);
export const customersLoaded = writable<boolean>(false);
export const selectedCustomerId = writable<number | null>(null);

export const DEFAULT_WALKIN_CUSTOMER_ID = 1;

export const selectedCustomer = derived(
  [customers, selectedCustomerId],
  ([$customers, $id]) => $customers.find((c) => c.id === $id) || null
);

export async function refreshCustomers() {
  try {
    const list = await invoke<Customer[]>('list_customers');
    customers.set(list);
    customersLoaded.set(true);
    // Default to the seeded walk-in customer (id 1) whenever none is
    // selected or the selection no longer exists.
    const currentId = get(selectedCustomerId);
    if (!currentId || !list.some((c) => c.id === currentId)) {
      const walkin = list.find((c) => c.id === DEFAULT_WALKIN_CUSTOMER_ID);
      selectedCustomerId.set(walkin ? walkin.id : list[0]?.id ?? null);
    }
  } catch (e) {
    console.error('Failed to load customers:', e);
    customersLoaded.set(true);
  }
}

export function selectCustomer(id: number | null) {
  // Never allow null — the walk-in default always applies.
  selectedCustomerId.set(id ?? DEFAULT_WALKIN_CUSTOMER_ID);
}
