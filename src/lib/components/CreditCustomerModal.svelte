<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { t, currentLocale } from '../i18n';
  import type { Customer } from '../types';
  import { X, Check, Users, Search, DollarSign, Layers } from 'lucide-svelte';

  export let isOpen = false;
  export let totalAmount = 0;
  export let onClose: () => void;
  export let onConfirmCredit: (customerId: number, customerName: string) => void;

  let customers: Customer[] = [];
  let searchQuery = '';
  let selectedCustomerId: number | null = null;
  let selectedCustomerName = '';

  onMount(async () => {
    await loadCustomers();
  });

  async function loadCustomers() {
    try {
      customers = await invoke<Customer[]>('list_customers');
      if (customers.length > 0) {
        selectedCustomerId = customers[0].id;
        selectedCustomerName = customers[0].name;
      }
    } catch (e) {
      console.error(e);
    }
  }

  $: filteredCustomers = customers.filter(c =>
    c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    (c.phone && c.phone.includes(searchQuery))
  );

  function selectCustomer(c: Customer) {
    selectedCustomerId = c.id;
    selectedCustomerName = c.name;
  }

  function handleConfirm() {
    if (selectedCustomerId) {
      onConfirmCredit(selectedCustomerId, selectedCustomerName);
      onClose();
    }
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl w-full max-w-lg overflow-hidden animate-in zoom-in-95 duration-150 flex flex-col max-h-[85vh]">
      <div class="flex items-center justify-between px-6 py-4 border-b border-pos-border bg-slate-50 dark:bg-slate-800/60">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-2xl bg-amber-600/10 text-amber-600 flex items-center justify-center font-bold">
            <Layers class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-black text-base text-pos-text">Credit Sale / بيع بالدين (Dette Client)</h3>
            <p class="text-xs text-amber-600 font-bold">Total Due: {totalAmount.toLocaleString()} DZD</p>
          </div>
        </div>
        <button on:click={onClose} class="text-pos-muted hover:text-pos-text p-1.5 rounded-xl cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="p-4 border-b border-pos-border">
        <div class="relative">
          <Search class="w-4 h-4 text-pos-muted absolute start-3 top-2.5" />
          <input
            type="text"
            bind:value={searchQuery}
            placeholder="Search customer name or phone..."
            class="w-full ps-9 pe-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none"
          />
        </div>
      </div>

      <div class="p-4 overflow-y-auto flex-1 space-y-2 max-h-60">
        {#each filteredCustomers as c}
          <button
            type="button"
            on:click={() => selectCustomer(c)}
            class="w-full p-3 rounded-2xl border text-start transition flex items-center justify-between cursor-pointer {selectedCustomerId === c.id ? 'bg-amber-500/10 border-amber-500 ring-2 ring-amber-400' : 'bg-pos-card border-pos-border hover:bg-slate-50 dark:hover:bg-slate-800/50'}"
          >
            <div>
              <p class="text-xs font-black text-pos-text">{c.name}</p>
              <p class="text-[10px] text-pos-muted">{c.phone || 'No phone'} • {c.address || 'Alger'}</p>
            </div>
            <div class="text-end">
              <span class="text-[10px] text-pos-muted font-bold block">Current Debt:</span>
              <span class="text-xs font-mono font-black text-rose-600">{c.balance.toLocaleString()} DZD</span>
            </div>
          </button>
        {/each}
      </div>

      <div class="px-6 py-4 border-t border-pos-border bg-slate-50 dark:bg-slate-800/60 flex items-center justify-between">
        <span class="text-xs font-bold text-pos-muted">Prints 2 copies with CREDIT watermark</span>
        <div class="flex items-center gap-2">
          <button on:click={onClose} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-pos-text font-bold text-xs rounded-xl cursor-pointer">Cancel</button>
          <button on:click={handleConfirm} disabled={!selectedCustomerId} class="px-6 py-2 bg-amber-600 hover:bg-amber-700 disabled:opacity-40 text-white font-black text-xs rounded-xl shadow-md cursor-pointer flex items-center gap-1.5">
            <Check class="w-4 h-4" />
            <span>Confirm Credit Sale (تأكيد البيع بالدين)</span>
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}