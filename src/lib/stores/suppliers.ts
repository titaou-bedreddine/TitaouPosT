import { writable, derived, get } from 'svelte/store';
import type { Supplier } from '../types';
import { invoke } from '@tauri-apps/api/core';

export const suppliers = writable<Supplier[]>([]);
export const suppliersLoaded = writable<boolean>(false);
export const selectedSupplierId = writable<number | null>(null);

export const DEFAULT_WALKIN_SUPPLIER_ID = 1;

export const selectedSupplier = derived(
  [suppliers, selectedSupplierId],
  ([$suppliers, $id]) => $suppliers.find((s) => s.id === $id) || null
);

export async function refreshSuppliers() {
  try {
    const list = await invoke<Supplier[]>('list_suppliers');
    suppliers.set(list);
    suppliersLoaded.set(true);
    const currentId = get(selectedSupplierId);
    if (!currentId || !list.some((s) => s.id === currentId)) {
      const walkin = list.find((s) => s.id === DEFAULT_WALKIN_SUPPLIER_ID);
      selectedSupplierId.set(walkin ? walkin.id : list[0]?.id ?? null);
    }
  } catch (e) {
    console.error('Failed to load suppliers:', e);
    suppliersLoaded.set(true);
  }
}
