<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Purchase, Supplier, Product } from '../../lib/types';
  import { currentUser } from '../../lib/stores/auth';
  import { FileSpreadsheet, Plus, Trash2, CheckCircle2, Search, Package } from 'lucide-svelte';

  let purchases: Purchase[] = [];
  let suppliers: Supplier[] = [];
  let products: Product[] = [];

  let isCreateOpen = false;
  let selectedSupplierId: number | null = null;
  let invoiceNumber = '';
  let invoiceDate = new Date().toISOString().split('T')[0];
  let paidAmount = 0;
  let paymentMethod = 'cash';
  let notes = '';

  interface ItemRow {
    product_id: number;
    name: string;
    quantity: number;
    unit_cost: number;
    total: number;
  }

  let items: ItemRow[] = [];

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      purchases = await invoke<Purchase[]>('list_purchases');
      suppliers = await invoke<Supplier[]>('list_suppliers');
      products = await invoke<Product[]>('search_products', { query: '', categoryId: null, searchType: 'all' });
    } catch (e) {
      console.error(e);
    }
  }

  function addItem(p: Product) {
    const existing = items.find(i => i.product_id === p.id);
    if (existing) {
      existing.quantity += 1;
      existing.total = existing.quantity * existing.unit_cost;
      items = [...items];
    } else {
      items = [...items, {
        product_id: p.id,
        name: p.name_ar || p.name_fr,
        quantity: 1,
        unit_cost: p.purchase_price,
        total: p.purchase_price,
      }];
    }
    updateTotals();
  }

  function removeItem(index: number) {
    items = items.filter((_, i) => i !== index);
    updateTotals();
  }

  function updateTotals() {
    items = items.map(i => ({ ...i, total: i.quantity * i.unit_cost }));
  }

  $: subtotal = items.reduce((sum, i) => sum + i.total, 0);
  $: total = subtotal;

  async function handleCreatePurchase() {
    if (!selectedSupplierId || !$currentUser || items.length === 0) return;
    try {
      const invNum = invoiceNumber || `PUR-${Date.now().toString().slice(-6)}`;
      await invoke('create_purchase', {
        input: {
          invoice_number: invNum,
          supplier_id: selectedSupplierId,
          user_id: $currentUser.id,
          date: invoiceDate,
          subtotal,
          discount: 0,
          tax: 0,
          total,
          paid_amount: paidAmount,
          payment_method: paymentMethod,
          items: items.map(i => ({
            product_id: i.product_id,
            quantity: i.quantity,
            unit_cost: i.unit_cost,
            discount: 0,
            tax: 0,
            total: i.total,
          })),
          notes: notes || null,
        }
      });
      isCreateOpen = false;
      items = [];
      invoiceNumber = '';
      paidAmount = 0;
      await loadData();
    } catch (e) {
      console.error(e);
    }
  }
</script>

<div class="p-6 space-y-4 overflow-y-auto h-full select-none">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-black text-pos-text">Purchase Invoices / Factures d'Achat (فواتير الشراء)</h1>
      <p class="text-xs text-pos-muted mt-1">Record supplier purchase bills, auto-update stock levels, and compare with delivery notes</p>
    </div>
    <button
      on:click={() => { items = []; isCreateOpen = true; }}
      class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-xl transition flex items-center gap-1.5 shadow-xs cursor-pointer"
    >
      <Plus class="w-4 h-4" />
      <span>New Purchase Invoice</span>
    </button>
  </div>

  {#if isCreateOpen}
    <div class="bg-pos-card border border-pos-border rounded-2xl p-5 shadow-sm space-y-4 animate-in fade-in duration-150">
      <h3 class="font-extrabold text-sm text-pos-text">Record Purchase Bill (فاتورة شراء جديدة)</h3>

      <div class="grid grid-cols-1 md:grid-cols-4 gap-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Select Supplier</label>
          <select bind:value={selectedSupplierId} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text">
            <option value={null}>-- Select Supplier --</option>
            {#each suppliers as s}
              <option value={s.id}>{s.name}</option>
            {/each}
          </select>
        </div>

        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Supplier Invoice Number / N° Facture</label>
          <input type="text" bind:value={invoiceNumber} placeholder="e.g. FACT-2026-991" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>

        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Invoice Date</label>
          <input type="date" bind:value={invoiceDate} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text" />
        </div>

        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Payment Method</label>
          <select bind:value={paymentMethod} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text">
            <option value="cash">Cash (نقداً)</option>
            <option value="bank_transfer">Bank Transfer (تحويل)</option>
            <option value="credit">Credit / Unpaid (دَيْن على الحساب)</option>
          </select>
        </div>
      </div>

      <!-- Add Product Quick Picker -->
      <div class="p-3 bg-slate-50 dark:bg-slate-800/40 rounded-xl border border-pos-border space-y-2">
        <span class="text-xs font-bold text-pos-muted">Add Products to Purchase Invoice:</span>
        <div class="flex gap-2 overflow-x-auto pb-1">
          {#each products as p}
            <button
              type="button"
              on:click={() => addItem(p)}
              class="px-3 py-1.5 bg-pos-card border border-pos-border hover:border-sky-500 rounded-lg text-xs font-bold text-pos-text shrink-0 cursor-pointer"
            >
              + {p.name_ar || p.name_fr} ({p.purchase_price} DZD)
            </button>
          {/each}
        </div>
      </div>

      <!-- Items Table -->
      <table class="w-full text-start text-xs border-collapse">
        <thead>
          <tr class="border-b border-pos-border bg-slate-50 dark:bg-slate-800/40 text-pos-muted font-bold">
            <th class="p-2.5 text-start">Product</th>
            <th class="p-2.5 text-center">Quantity</th>
            <th class="p-2.5 text-end">Purchase Cost (DZD)</th>
            <th class="p-2.5 text-end">Line Total (DZD)</th>
            <th class="p-2.5 text-center"></th>
          </tr>
        </thead>
        <tbody>
          {#each items as item, idx}
            <tr class="border-b border-pos-border/60">
              <td class="p-2.5 font-bold text-pos-text">{item.name}</td>
              <td class="p-2.5 text-center">
                <input type="number" bind:value={item.quantity} on:input={updateTotals} min="1" class="w-16 px-2 py-1 text-center bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs font-mono font-bold text-pos-text" />
              </td>
              <td class="p-2.5 text-end">
                <input type="number" bind:value={item.unit_cost} on:input={updateTotals} class="w-24 px-2 py-1 text-end bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs font-mono font-bold text-pos-text" />
              </td>
              <td class="p-2.5 text-end font-mono font-bold">{item.total.toLocaleString()} DZD</td>
              <td class="p-2.5 text-center">
                <button type="button" on:click={() => removeItem(idx)} class="text-rose-500 hover:text-rose-700 cursor-pointer">
                  <Trash2 class="w-3.5 h-3.5" />
                </button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>

      <!-- Totals & Payment Breakdown -->
      <div class="flex items-center justify-between pt-3 border-t border-pos-border/60">
        <div class="flex items-center gap-3">
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Amount Paid to Supplier (DZD)</label>
            <input type="number" bind:value={paidAmount} placeholder="0" class="w-48 px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-lg font-mono font-bold text-pos-text" />
          </div>
          {#if total - paidAmount > 0}
            <div class="text-xs font-bold text-rose-600 pt-4 font-mono">
              Remaining Supplier Debt: {(total - paidAmount).toLocaleString()} DZD
            </div>
          {/if}
        </div>

        <div class="text-end space-y-1">
          <span class="text-xs font-black text-pos-muted uppercase">Invoice Grand Total</span>
          <div class="text-3xl font-black font-mono text-sky-600">{total.toLocaleString()} DZD</div>
        </div>
      </div>

      <div class="flex justify-end gap-2 pt-2 border-t border-pos-border/60">
        <button on:click={() => isCreateOpen = false} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-lg">Cancel</button>
        <button on:click={handleCreatePurchase} disabled={items.length === 0 || !selectedSupplierId} class="px-6 py-2 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-40 text-white text-xs font-black rounded-lg shadow-sm">
          Save & Increment Stock
        </button>
      </div>
    </div>
  {/if}

  <!-- Purchases History List -->
  <div class="bg-pos-card border border-pos-border rounded-2xl shadow-xs overflow-hidden">
    <table class="w-full text-start text-xs border-collapse">
      <thead>
        <tr class="border-b border-pos-border bg-slate-50 dark:bg-slate-800/40 text-pos-muted font-bold">
          <th class="p-3 text-start">Invoice #</th>
          <th class="p-3 text-start">Date</th>
          <th class="p-3 text-start">Supplier</th>
          <th class="p-3 text-start">Recorded By</th>
          <th class="p-3 text-end">Total Amount</th>
          <th class="p-3 text-end">Paid Amount</th>
          <th class="p-3 text-center">Status</th>
        </tr>
      </thead>
      <tbody>
        {#each purchases as p}
          <tr class="border-b border-pos-border/60 hover:bg-slate-50 dark:hover:bg-slate-800/40">
            <td class="p-3 font-mono font-bold text-sky-600">{p.invoice_number}</td>
            <td class="p-3 font-mono text-pos-muted">{p.date}</td>
            <td class="p-3 font-bold text-pos-text">{p.supplier_name || 'N/A'}</td>
            <td class="p-3 text-pos-muted">{p.user_name || 'Admin'}</td>
            <td class="p-3 text-end font-mono font-bold text-sm">{p.total.toLocaleString()} DZD</td>
            <td class="p-3 text-end font-mono font-bold text-sm text-emerald-600">{p.paid_amount.toLocaleString()} DZD</td>
            <td class="p-3 text-center">
              <span class="px-2 py-0.5 rounded-full text-[11px] font-bold bg-emerald-100 text-emerald-800 uppercase">
                {p.status}
              </span>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>