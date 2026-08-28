<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Purchase, Supplier, Product } from '../../lib/types';
  import { currentUser } from '../../lib/stores/auth';
  import { printHtmlDirectly } from '../../lib/utils/printer';
  import UniversalSearchBar from '../../lib/components/UniversalSearchBar.svelte';
  import {
    FileSpreadsheet, Plus, Trash2, CheckCircle2, Search, Package,
    Printer, Eye, Edit3, X, Tag, DollarSign, ArrowRight
  } from 'lucide-svelte';

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

  // OmniSearch inside invoice
  let searchQuery = '';
  let searchType: 'all' | 'name' | 'barcode' | 'price' | 'qr' = 'all';

  interface ItemRow {
    product_id: number;
    name: string;
    barcode: string;
    quantity: number;
    unit_cost: number;
    sale_price: number;
    total: number;
  }

  let items: ItemRow[] = [];
  let isSaving = false;
  let errorMsg = '';
  let previewPurchase: Purchase | null = null;

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      purchases = await invoke<Purchase[]>('list_purchases');
      suppliers = await invoke<Supplier[]>('list_suppliers');
      if (suppliers.length > 0 && !selectedSupplierId) {
        selectedSupplierId = suppliers[0].id;
      }
      products = await invoke<Product[]>('search_products', { query: '', categoryId: null, searchType: 'all' });
    } catch (e) {
      console.error(e);
    }
  }

  async function handleOmniSearchProduct() {
    if (!searchQuery.trim()) return;
    try {
      const results = await invoke<Product[]>('search_products', {
        query: searchQuery.trim(),
        categoryId: null,
        searchType: searchType === 'qr' ? 'barcode' : searchType,
      });

      if (results.length > 0) {
        const p = results[0];
        await addItemAndFocus(p);
        searchQuery = '';
      }
    } catch (e) {
      console.error(e);
    }
  }

  async function addItemAndFocus(p: Product) {
    const existingIndex = items.findIndex(i => i.product_id === p.id);
    let targetIndex = 0;
    if (existingIndex > -1) {
      items[existingIndex].quantity += 1;
      items[existingIndex].total = items[existingIndex].quantity * items[existingIndex].unit_cost;
      items = [...items];
      targetIndex = existingIndex;
    } else {
      items = [...items, {
        product_id: p.id,
        name: p.name_fr || p.name_ar,
        barcode: (p.barcodes && p.barcodes[0]) || p.sku || '',
        quantity: 1,
        unit_cost: p.purchase_price,
        sale_price: p.sale_price,
        total: p.purchase_price,
      }];
      targetIndex = items.length - 1;
    }
    updateTotals();

    await tick();
    const qtyInput = document.getElementById(`qty-${targetIndex}`) as HTMLInputElement;
    if (qtyInput) {
      qtyInput.focus();
      qtyInput.select();
    }
  }

  function handleQtyKeyDown(e: KeyboardEvent, idx: number) {
    if (e.key === 'Enter') {
      e.preventDefault();
      const costInput = document.getElementById(`cost-${idx}`) as HTMLInputElement;
      if (costInput) {
        costInput.focus();
        costInput.select();
      }
    }
  }

  function handleCostKeyDown(e: KeyboardEvent, idx: number) {
    if (e.key === 'Enter') {
      e.preventDefault();
      const saleInput = document.getElementById(`sale-${idx}`) as HTMLInputElement;
      if (saleInput) {
        saleInput.focus();
        saleInput.select();
      }
    }
  }

  function handleSaleKeyDown(e: KeyboardEvent, idx: number) {
    if (e.key === 'Enter') {
      e.preventDefault();
      const searchInput = document.querySelector('.omni-purchase-search input') as HTMLInputElement;
      if (searchInput) {
        searchInput.focus();
        searchInput.select();
      }
    }
  }

  function removeItem(index: number) {
    items = items.filter((_, i) => i !== index);
    updateTotals();
  }

  function updateTotals() {
    items = items.map(i => ({ ...i, total: i.quantity * i.unit_cost }));
    paidAmount = subtotal;
  }

  $: subtotal = items.reduce((sum, i) => sum + i.total, 0);
  $: total = subtotal;

  function setPaidAll() {
    paidAmount = total;
  }

  function setPaidHalf() {
    paidAmount = Math.round(total / 2);
  }

  function setPaidZero() {
    paidAmount = 0;
  }

  async function handleCreatePurchase() {
    if (!selectedSupplierId || items.length === 0) {
      errorMsg = 'Please add products and select supplier / الرجاء إضافة منتجات واختيار المورد';
      return;
    }
    try {
      isSaving = true;
      errorMsg = '';
      const invNum = invoiceNumber || `PUR-${Date.now().toString().slice(-6)}`;
      await invoke('create_purchase', {
        input: {
          invoice_number: invNum,
          supplier_id: selectedSupplierId,
          purchase_date: invoiceDate,
          total_amount: total,
          paid_amount: paidAmount,
          payment_status: paidAmount >= total ? 'paid' : paidAmount > 0 ? 'partial' : 'unpaid',
          payment_method: paymentMethod,
          notes: notes || 'Facture Achat',
          items: items.map(i => ({
            product_id: i.product_id,
            quantity: i.quantity,
            unit_cost: i.unit_cost,
            total_cost: i.total,
            expiry_date: null,
            batch_number: null,
          })),
        }
      });

      isCreateOpen = false;
      items = [];
      invoiceNumber = '';
      paidAmount = 0;
      await loadData();
    } catch (e: any) {
      errorMsg = typeof e === 'string' ? e : e.message || 'Failed to save purchase';
    } finally {
      isSaving = false;
    }
  }

  function printLabelsForPurchase() {
    if (items.length === 0) return;
    const html = items.map(i => `
      <div style="width: 40mm; height: 20mm; padding: 2mm; border: 1px solid #000; text-align: center; page-break-after: always; display: flex; flex-direction: column; justify-content: space-between;">
        <p style="font-size: 8px; font-weight: bold; margin: 0; overflow: hidden; white-space: nowrap;">${i.name}</p>
        <p style="font-size: 11px; font-weight: 900; margin: 0;">${i.sale_price.toLocaleString()} DZD</p>
        <p style="font-size: 7px; font-family: monospace; margin: 0;">${i.barcode}</p>
      </div>
    `).join('');
    printHtmlDirectly(html, 'Price Tags');
  }
</script>

<div class="h-full flex flex-col bg-pos-bg p-4 overflow-hidden select-none">
  <!-- Header -->
  <div class="flex items-center justify-between pb-4 border-b border-pos-border shrink-0">
    <div class="flex items-center gap-3">
      <div class="w-10 h-10 rounded-2xl bg-sky-100 dark:bg-sky-950 text-sky-600 flex items-center justify-center font-bold">
        <FileSpreadsheet class="w-5 h-5" />
      </div>
      <div>
        <h1 class="text-xl font-black text-pos-text tracking-tight">Purchases & Invoices / المشتريات وفواتير الشراء</h1>
        <p class="text-xs text-pos-muted">Manage supplier purchases, stock replenishment & price tags</p>
      </div>
    </div>

    <div class="flex items-center gap-2">
      <button
        type="button"
        on:click={() => { isCreateOpen = true; items = []; }}
        class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white rounded-xl text-xs font-black transition shadow-xs flex items-center gap-2 cursor-pointer active:scale-95"
      >
        <Plus class="w-4 h-4" />
        <span>New Purchase (فاتورة شراء جديدة)</span>
      </button>
    </div>
  </div>

  <!-- Purchases History Table -->
  <div class="flex-1 overflow-y-auto mt-4 bg-pos-card border border-pos-border rounded-2xl shadow-xs">
    <table class="w-full text-start text-xs border-collapse">
      <thead class="bg-slate-50 dark:bg-slate-800/60 border-b border-pos-border text-pos-muted font-bold sticky top-0 z-10">
        <tr>
          <th class="p-3 text-start">Invoice #</th>
          <th class="p-3 text-start">Supplier / المورد</th>
          <th class="p-3 text-start">Date</th>
          <th class="p-3 text-end">Total Amount</th>
          <th class="p-3 text-end">Paid Amount</th>
          <th class="p-3 text-center">Status</th>
          <th class="p-3 text-end">Actions</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-pos-border/40">
        {#if purchases.length === 0}
          <tr>
            <td colspan="7" class="p-8 text-center text-pos-muted">No purchase invoices recorded yet.</td>
          </tr>
        {:else}
          {#each purchases as p}
            <tr class="hover:bg-slate-50 dark:hover:bg-slate-800/40 transition">
              <td class="p-3 font-mono font-bold text-sky-600">#{p.invoice_number}</td>
              <td class="p-3 font-bold text-pos-text">{p.supplier_name || 'Fournisseur Divers'}</td>
              <td class="p-3 font-mono text-pos-muted">{p.purchase_date}</td>
              <td class="p-3 text-end font-mono font-black text-pos-text">{p.total_amount.toLocaleString()} DZD</td>
              <td class="p-3 text-end font-mono font-bold text-emerald-600">{p.paid_amount.toLocaleString()} DZD</td>
              <td class="p-3 text-center">
                <span class="px-2 py-0.5 rounded-full text-[10px] font-black uppercase font-mono {p.payment_status === 'paid' ? 'bg-emerald-100 text-emerald-800 dark:bg-emerald-950 dark:text-emerald-300' : 'bg-amber-100 text-amber-800 dark:bg-amber-950 dark:text-amber-300'}">
                  {p.payment_status}
                </span>
              </td>
              <td class="p-3 text-end">
                <div class="flex items-center justify-end gap-1">
                  <button on:click={() => (previewPurchase = p)} class="p-1.5 hover:text-sky-600 rounded-lg cursor-pointer" title="Preview">
                    <Eye class="w-4 h-4" />
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
</div>

<!-- Modal: New Purchase Invoice -->
{#if isCreateOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl w-full max-w-4xl overflow-hidden animate-in zoom-in-95 duration-150 flex flex-col max-h-[92vh]">
      <!-- Header -->
      <div class="flex items-center justify-between px-6 py-4 border-b border-pos-border bg-slate-50 dark:bg-slate-800/60">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-2xl bg-sky-600/10 text-sky-600 flex items-center justify-center font-bold">
            <FileSpreadsheet class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-black text-base text-pos-text">New Purchase Invoice / فاتورة مشتريات جديدة</h3>
            <p class="text-xs text-pos-muted">Scan products, edit quantities, and auto-print shelf barcode tags</p>
          </div>
        </div>
        <button on:click={() => (isCreateOpen = false)} class="text-pos-muted hover:text-pos-text p-1.5 rounded-xl cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>

      {#if errorMsg}
        <div class="mx-6 mt-4 p-3 bg-rose-100 text-rose-800 text-xs font-bold rounded-xl">{errorMsg}</div>
      {/if}

      <div class="p-6 overflow-y-auto flex-1 space-y-4">
        <!-- Supplier & Metadata Row -->
        <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Supplier / المورد</label>
            <select bind:value={selectedSupplierId} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none">
              {#each suppliers as s}
                <option value={s.id}>{s.name} ({s.phone || 'No phone'})</option>
              {/each}
            </select>
          </div>
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Invoice / Bon Number</label>
            <input type="text" bind:value={invoiceNumber} placeholder="Ex: ACH-2026-001" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none" />
          </div>
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Date</label>
            <input type="date" bind:value={invoiceDate} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none" />
          </div>
        </div>

        <!-- OmniSearch Scanner Input -->
        <div class="omni-purchase-search p-3 bg-sky-50/50 dark:bg-sky-950/20 rounded-2xl border border-sky-200 dark:border-sky-800 flex items-center gap-2">
          <div class="flex-1">
            <UniversalSearchBar
              bind:query={searchQuery}
              bind:searchType
              onSearch={handleOmniSearchProduct}
            />
          </div>
          <button
            type="button"
            on:click={handleOmniSearchProduct}
            class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl cursor-pointer"
          >
            Add
          </button>
        </div>

        <!-- Items Table with auto-tabbing -->
        <div class="border border-pos-border rounded-2xl overflow-hidden">
          <table class="w-full text-xs text-start border-collapse">
            <thead class="bg-slate-100 dark:bg-slate-800 border-b border-pos-border font-bold text-pos-muted">
              <tr>
                <th class="p-2 text-start">Product</th>
                <th class="p-2 text-center w-24">Qty (Qté)</th>
                <th class="p-2 text-end w-32">Purchase Cost</th>
                <th class="p-2 text-end w-32">Sale Price</th>
                <th class="p-2 text-end w-28">Total Cost</th>
                <th class="p-2 text-center w-12"></th>
              </tr>
            </thead>
            <tbody class="divide-y divide-pos-border">
              {#if items.length === 0}
                <tr>
                  <td colspan="6" class="p-6 text-center text-pos-muted font-medium">
                    Scan a barcode or search above to add products to this invoice.
                  </td>
                </tr>
              {:else}
                {#each items as item, idx}
                  <tr class="hover:bg-slate-50 dark:hover:bg-slate-800/40">
                    <td class="p-2">
                      <p class="font-bold text-pos-text">{item.name}</p>
                      <p class="text-[10px] text-pos-muted font-mono">{item.barcode}</p>
                    </td>
                    <td class="p-2 text-center">
                      <input
                        id="qty-{idx}"
                        type="number"
                        min="1"
                        bind:value={item.quantity}
                        on:input={updateTotals}
                        on:keydown={(e) => handleQtyKeyDown(e, idx)}
                        class="w-18 text-center px-2 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg font-mono font-black text-xs outline-none focus:ring-2 focus:ring-sky-500"
                      />
                    </td>
                    <td class="p-2 text-end">
                      <input
                        id="cost-{idx}"
                        type="number"
                        min="0"
                        bind:value={item.unit_cost}
                        on:input={updateTotals}
                        on:keydown={(e) => handleCostKeyDown(e, idx)}
                        class="w-24 text-end px-2 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg font-mono font-bold text-xs outline-none focus:ring-2 focus:ring-sky-500"
                      />
                    </td>
                    <td class="p-2 text-end">
                      <input
                        id="sale-{idx}"
                        type="number"
                        min="0"
                        bind:value={item.sale_price}
                        on:keydown={(e) => handleSaleKeyDown(e, idx)}
                        class="w-24 text-end px-2 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg font-mono font-bold text-xs text-sky-600 outline-none focus:ring-2 focus:ring-sky-500"
                      />
                    </td>
                    <td class="p-2 text-end font-mono font-black text-pos-text">
                      {item.total.toLocaleString()} DZD
                    </td>
                    <td class="p-2 text-center">
                      <button on:click={() => removeItem(idx)} class="text-rose-500 hover:text-rose-700 p-1 cursor-pointer">
                        <Trash2 class="w-4 h-4" />
                      </button>
                    </td>
                  </tr>
                {/each}
              {/if}
            </tbody>
          </table>
        </div>

        <!-- Payment Settlement Row -->
        <div class="p-4 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border flex flex-col md:flex-row items-center justify-between gap-4">
          <div class="space-y-1">
            <span class="text-xs text-pos-muted font-bold">Paid to Supplier / المبلغ المدفوع للمورد</span>
            <div class="flex items-center gap-1.5">
              <input
                type="number"
                min="0"
                max={total}
                bind:value={paidAmount}
                class="px-3 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-mono font-black text-emerald-600 outline-none"
              />
              <button type="button" on:click={setPaidAll} class="px-2.5 py-1.5 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 text-pos-text rounded-lg text-xs font-bold cursor-pointer">All (الكل)</button>
              <button type="button" on:click={setPaidHalf} class="px-2.5 py-1.5 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 text-pos-text rounded-lg text-xs font-bold cursor-pointer">Half (النصف)</button>
              <button type="button" on:click={setPaidZero} class="px-2.5 py-1.5 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 text-pos-text rounded-lg text-xs font-bold cursor-pointer">0 (دين)</button>
            </div>
          </div>

          <div class="text-end">
            <span class="text-xs text-pos-muted font-bold block">Total Invoice:</span>
            <span class="text-2xl font-black font-mono text-sky-600 dark:text-sky-400">{total.toLocaleString()} DZD</span>
          </div>
        </div>
      </div>

      <!-- Footer Buttons -->
      <div class="px-6 py-4 border-t border-pos-border bg-slate-50 dark:bg-slate-800/60 flex items-center justify-between">
        <button
          type="button"
          on:click={printLabelsForPurchase}
          disabled={items.length === 0}
          class="px-4 py-2 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 disabled:opacity-40 text-pos-text font-bold text-xs rounded-xl flex items-center gap-1.5 cursor-pointer"
        >
          <Tag class="w-4 h-4 text-sky-500" />
          <span>Auto-Print Shelf Tags (طباعة ملصقات الأسعار)</span>
        </button>

        <div class="flex items-center gap-2">
          <button on:click={() => (isCreateOpen = false)} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-pos-text font-bold text-xs rounded-xl cursor-pointer">Cancel</button>
          <button on:click={handleCreatePurchase} disabled={items.length === 0 || isSaving} class="px-6 py-2 bg-sky-600 hover:bg-sky-700 disabled:opacity-40 text-white font-black text-xs rounded-xl shadow-md cursor-pointer flex items-center gap-1.5">
            <CheckCircle2 class="w-4 h-4" />
            <span>{isSaving ? 'Saving...' : 'Save Invoice (حفظ الفاتورة)'}</span>
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Modal: Preview Purchase -->
{#if previewPurchase}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl w-full max-w-lg overflow-hidden animate-in zoom-in-95 duration-150 flex flex-col">
      <div class="flex items-center justify-between px-6 py-4 border-b border-pos-border bg-slate-50 dark:bg-slate-800/60">
        <h3 class="font-black text-base text-pos-text">Purchase Invoice #{previewPurchase.invoice_number}</h3>
        <button on:click={() => (previewPurchase = null)} class="text-pos-muted hover:text-pos-text p-1.5 rounded-xl cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="p-6 space-y-3 text-xs">
        <div class="flex justify-between">
          <span class="text-pos-muted">Supplier:</span>
          <span class="font-bold text-pos-text">{previewPurchase.supplier_name}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-pos-muted">Date:</span>
          <span class="font-mono text-pos-text">{previewPurchase.purchase_date}</span>
        </div>
        <div class="flex justify-between">
          <span class="text-pos-muted">Total:</span>
          <span class="font-black font-mono text-sky-600">{previewPurchase.total_amount.toLocaleString()} DZD</span>
        </div>
        <div class="flex justify-between">
          <span class="text-pos-muted">Paid:</span>
          <span class="font-bold font-mono text-emerald-600">{previewPurchase.paid_amount.toLocaleString()} DZD</span>
        </div>
        <div class="flex justify-between">
          <span class="text-pos-muted">Status:</span>
          <span class="font-bold uppercase font-mono">{previewPurchase.payment_status}</span>
        </div>
      </div>

      <div class="px-6 py-4 border-t border-pos-border bg-slate-50 dark:bg-slate-800/60 flex items-center justify-end">
        <button on:click={() => (previewPurchase = null)} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-pos-text font-bold text-xs rounded-xl cursor-pointer">Close</button>
      </div>
    </div>
  </div>
{/if}