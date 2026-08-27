<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Sale } from '../../lib/types';
  import { ShoppingBag, Search, Printer, Calendar, User } from 'lucide-svelte';

  let sales: Sale[] = [];
  let startDate = '';
  let endDate = '';
  let selectedCashier: number | null = null;

  onMount(async () => {
    await loadSales();
  });

  async function loadSales() {
    try {
      sales = await invoke<Sale[]>('list_sales', {
        startDate: startDate || null,
        endDate: endDate || null,
        userId: selectedCashier,
        limit: 100,
      });
    } catch (e) {
      console.error(e);
    }
  }

  function handlePrintReceipt(s: Sale) {
    window.print();
  }
</script>

<div class="p-6 space-y-4 overflow-y-auto h-full select-none">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-black text-pos-text">Sales History (سجل المبيعات)</h1>
      <p class="text-xs text-pos-muted mt-1">Review sales transactions, filter by date or cashier, and reprint thermal receipts</p>
    </div>
  </div>

  <!-- Filter Bar -->
  <div class="bg-pos-card border border-pos-border rounded-2xl p-4 shadow-xs grid grid-cols-1 md:grid-cols-4 gap-3 items-end">
    <div>
      <label class="block text-xs font-bold text-pos-muted mb-1">From Date</label>
      <input type="date" bind:value={startDate} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text" />
    </div>
    <div>
      <label class="block text-xs font-bold text-pos-muted mb-1">To Date</label>
      <input type="date" bind:value={endDate} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text" />
    </div>
    <div>
      <label class="block text-xs font-bold text-pos-muted mb-1">Cashier</label>
      <select bind:value={selectedCashier} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text">
        <option value={null}>All Cashiers (الكل)</option>
      </select>
    </div>
    <button on:click={loadSales} class="px-5 py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-lg flex items-center justify-center gap-1.5 cursor-pointer shadow-xs">
      <Search class="w-4 h-4" />
      <span>Filter Sales</span>
    </button>
  </div>

  <!-- Sales Table -->
  <div class="bg-pos-card border border-pos-border rounded-2xl shadow-xs overflow-hidden">
    <table class="w-full text-start text-xs border-collapse">
      <thead>
        <tr class="border-b border-pos-border bg-slate-50 dark:bg-slate-800/40 text-pos-muted font-bold">
          <th class="p-3 text-start">Sale #</th>
          <th class="p-3 text-start">Date & Time</th>
          <th class="p-3 text-start">Cashier</th>
          <th class="p-3 text-start">Customer</th>
          <th class="p-3 text-end">Total Amount</th>
          <th class="p-3 text-end">Paid Amount</th>
          <th class="p-3 text-center">Payment Status</th>
          <th class="p-3 text-center">Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each sales as s}
          <tr class="border-b border-pos-border/60 hover:bg-slate-50 dark:hover:bg-slate-800/40">
            <td class="p-3 font-mono font-bold text-sky-600">{s.sale_number}</td>
            <td class="p-3 font-mono text-pos-muted">{s.created_at}</td>
            <td class="p-3 font-bold text-pos-text">{s.user_name || 'Admin'}</td>
            <td class="p-3 text-pos-muted">{s.customer_name || 'Comptoir'}</td>
            <td class="p-3 text-end font-mono font-black text-sm text-pos-text">{s.total_amount.toLocaleString()} DZD</td>
            <td class="p-3 text-end font-mono font-bold text-sm text-emerald-600">{s.paid_amount.toLocaleString()} DZD</td>
            <td class="p-3 text-center">
              <span class="px-2 py-0.5 rounded-full text-[11px] font-bold uppercase {s.payment_status === 'paid' ? 'bg-emerald-100 text-emerald-800' : 'bg-amber-100 text-amber-800'}">
                {s.payment_status}
              </span>
            </td>
            <td class="p-3 text-center">
              <button
                type="button"
                on:click={() => handlePrintReceipt(s)}
                class="p-1.5 text-sky-600 hover:bg-sky-50 dark:hover:bg-sky-950 rounded-lg cursor-pointer"
                title="Reprint Receipt"
              >
                <Printer class="w-3.5 h-3.5" />
              </button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>