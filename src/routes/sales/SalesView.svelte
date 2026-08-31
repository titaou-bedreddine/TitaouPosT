<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Sale, User } from '../../lib/types';
  import { currentUser } from '../../lib/stores/auth';
  import { printHtmlDirectly } from '../../lib/utils/printer';
  import DateQuickFilters from '../../lib/components/DateQuickFilters.svelte';
  import { entityQrPayload, entityQrUrl } from '../../lib/utils/printer';
  import { cartItems, clearCart } from '../../lib/stores/cart';
  import { selectedCustomerId } from '../../lib/stores/customers';
  import {
    ShoppingBag, Search, Printer, Calendar, User as UserIcon,
    DollarSign, Eye, Trash2, X, Check, AlertTriangle, Layers,
    CreditCard, Banknote, ShieldAlert, TrendingUp, Pencil
  } from 'lucide-svelte';

  let sales: Sale[] = [];
  let users: User[] = [];

  // History defaults to today; the user can widen the range.
  let startDate = new Date().toISOString().split('T')[0];
  let endDate = new Date().toISOString().split('T')[0];
  let selectedCashier: number | null = null;
  let selectedStatus: string = 'all';
  let searchQuery = '';

  // Sale Details Modal
  let isDetailModalOpen = false;
  let selectedSale: Sale | null = null;
  let saleItems: any[] = [];
  let isLoadingItems = false;

  // Protected Delete Modal
  let isDeleteModalOpen = false;
  let adminPassword = '';
  let deleteError = '';
  let isDeleting = false;

  onMount(async () => {
    await loadUsers();
    await loadSales();
  });

  async function loadUsers() {
    try {
      users = await invoke<User[]>('get_active_users');
    } catch (e) {
      console.error(e);
    }
  }

  async function loadSales() {
    try {
      sales = await invoke<Sale[]>('list_sales', {
        startDate: startDate || null,
        endDate: endDate || null,
        userId: selectedCashier ? Number(selectedCashier) : null,
        limit: 200,
      });
    } catch (e) {
      console.error(e);
    }
  }

  $: filteredSales = sales.filter(s => {
    // Omni-search: sale number, customer name, exact amount, or the QR
    // payload (scan a receipt QR → 'SALE:POS-...' or plain code).
    const q = searchQuery.trim().toLowerCase();
    const qr = entityQrPayload('SALE', s.sale_number).toLowerCase();
    const stripped = q.startsWith('sale:') ? q.slice(5) : q;
    const matchesSearch =
      !q ||
      s.sale_number.toLowerCase().includes(stripped) ||
      (s.customer_name || '').toLowerCase().includes(stripped) ||
      String(s.total_amount) === stripped ||
      qr === q ||
      qr.includes(stripped);

    if (!matchesSearch) return false;

    if (selectedStatus === 'all') return true;
    if (selectedStatus === 'paid') return s.payment_status === 'paid';
    if (selectedStatus === 'partial') return s.payment_status === 'partial';
    if (selectedStatus === 'unpaid') return s.payment_status === 'unpaid';
    if (selectedStatus === 'cash') return s.payment_method === 'cash';
    if (selectedStatus === 'tpe') return s.payment_method === 'tpe';
    if (selectedStatus === 'credit') return s.payment_method === 'credit';
    if (selectedStatus === 'refunded') return s.status === 'refunded' || s.status === 'partially_refunded';
    return true;
  });

  // Top Real Stats
  $: totalSalesCount = filteredSales.length;
  $: totalGrossRevenue = filteredSales.reduce((sum, s) => sum + s.total_amount, 0);
  $: totalNetPaid = filteredSales.reduce((sum, s) => sum + s.paid_amount, 0);
  $: totalDueCredit = totalGrossRevenue - totalNetPaid;

  async function openSaleDetails(s: Sale) {
    selectedSale = s;
    isDetailModalOpen = true;
    try {
      isLoadingItems = true;
      saleItems = await invoke<any[]>('get_sale_items', { saleId: s.id });
    } catch (e) {
      console.error('Failed to load sale items:', e);
      saleItems = [];
    } finally {
      isLoadingItems = false;
    }
  }

  async function printReceipt(s: Sale) {
    try {
      const settings = await invoke<Record<string, string>>('get_all_settings');
      const items = await invoke<any[]>('get_sale_items', { saleId: s.id });
      const shopName = settings['shop_name_fr'] || 'TitaouPOS';
      const shopPhone = settings['shop_phone'] || '0553444057';
      const shopAddress = settings['shop_address'] || 'Alger Centre';

      const itemsHtml = items.map(item => `
        <div style="display: flex; justify-content: space-between; font-size: 10px; margin-bottom: 2px;">
          <span>${item.quantity}x ${item.name_fr || item.name_ar}</span>
          <span style="font-weight: bold;">${(item.total_price || 0).toLocaleString()} DZD</span>
        </div>
      `).join('');

      const html = `
        <div style="width: 72mm; font-family: monospace; font-size: 10px; text-align: center; margin: 0 auto; padding: 2mm;">
          <p style="font-size: 14px; font-weight: 900; margin: 0; text-transform: uppercase;">${shopName}</p>
          <p style="font-size: 8px; margin: 2px 0;">${shopAddress} • Tél: ${shopPhone}</p>
          <hr style="border-top: 1px dashed #000; margin: 4px 0;" />
          <div style="display: flex; justify-content: space-between; font-size: 9px; font-weight: bold;">
            <span>FACTURE #${s.sale_number}</span>
            <span>${s.created_at}</span>
          </div>
          <div style="display: flex; justify-content: space-between; font-size: 8px;">
            <span>Client: ${s.customer_name || 'Client Comptoir'}</span>
            <span>Caisse: ${s.user_name || 'Admin'}</span>
          </div>
          <hr style="border-top: 1px dashed #000; margin: 4px 0;" />
          <div style="text-align: left;">
            ${itemsHtml}
          </div>
          <hr style="border-top: 1px dashed #000; margin: 4px 0;" />
          <div style="display: flex; justify-content: space-between; font-size: 12px; font-weight: 900;">
            <span>TOTAL:</span>
            <span>${s.total_amount.toLocaleString()} DZD</span>
          </div>
          <div style="display: flex; justify-content: space-between; font-size: 10px;">
            <span>Payé (${(s.payment_method || 'cash').toUpperCase()}):</span>
            <span>${s.paid_amount.toLocaleString()} DZD</span>
          </div>
          ${s.change_amount > 0 ? `
            <div style="display: flex; justify-content: space-between; font-size: 9px;">
              <span>Rendu:</span>
              <span>${s.change_amount.toLocaleString()} DZD</span>
            </div>
          ` : ''}
          <hr style="border-top: 1px dashed #000; margin: 4px 0;" />
          <p style="font-size: 8px; margin: 0;">Merci de votre visite / شكراً لزيارتكم</p>
          <p style="font-size: 7px; color: #555; margin: 2px 0;">TitaouPOS • Dev: Titaou Bedreddine (0553444057)</p>
        </div>
      `;
      printHtmlDirectly(html, 'Receipt #' + s.sale_number);
    } catch (e: any) {
      alert('Error printing receipt: ' + (e.message || e));
    }
  }

  function promptProtectedDelete() {
    deleteError = '';
    adminPassword = '';
    isDeleteModalOpen = true;
  }

  // Re-open a completed sale in the POS cart for editing. The caller
  // (App.svelte) switches to the POS route; the sale stays in history until
  // the cashier deletes it via the protected delete.
  async function editSaleInPos(sale: Sale) {
    try {
      const items = await invoke<any[]>('get_sale_items', { saleId: sale.id });
      const mapped = items.map((i) => ({
        product_id: i.product_id,
        sku: i.sku || '',
        barcode: i.barcode || '',
        name_ar: i.name_ar || '',
        name_fr: i.name_fr || '',
        name_en: i.name_en || '',
        image_path: i.image_path,
        unit_price: i.unit_price,
        quantity: i.quantity,
        discount_amount: i.discount_amount || 0,
        tax_amount: i.tax_amount || 0,
        total_price: i.total_price,
        is_refund: i.is_refund || false,
      }));
      clearCart();
      $cartItems = mapped;
      if (sale.customer_id) $selectedCustomerId = sale.customer_id;
      isDetailModalOpen = false;
      onRequestPosRoute?.();
    } catch (e) {
      console.error('Failed to load sale for editing:', e);
    }
  }

  export let onRequestPosRoute: () => void = () => {};

  async function executeProtectedDelete() {
    if (!$currentUser) return;
    if ($currentUser.role_name !== 'admin' && !adminPassword) {
      deleteError = 'Admin password required to delete sale / كلمة المرور مطلوبة';
      return;
    }
    try {
      isDeleting = true;
      deleteError = '';
      if ($currentUser.role_name !== 'admin') {
        const ok = await invoke<boolean>('verify_admin_password', { password: adminPassword });
        if (!ok) {
          deleteError = 'Invalid password / كلمة المرور غير صحيحة';
          isDeleting = false;
          return;
        }
      }

      if (selectedSale) {
        await invoke('delete_sale', { saleId: selectedSale.id });
        isDeleteModalOpen = false;
        isDetailModalOpen = false;
        selectedSale = null;
        await loadSales();
      }
    } catch (e: any) {
      deleteError = typeof e === 'string' ? e : e.message || 'Failed to delete sale';
    } finally {
      isDeleting = false;
    }
  }
</script>

<div class="p-6 space-y-4 overflow-y-auto h-full select-none flex flex-col bg-pos-bg">
  <div class="flex items-center justify-between shrink-0">
    <div class="flex items-center gap-3">
      <div class="w-10 h-10 rounded-2xl bg-sky-100 dark:bg-sky-950 text-sky-600 flex items-center justify-center font-bold">
        <ShoppingBag class="w-5 h-5" />
      </div>
      <div>
        <h1 class="text-xl font-black text-pos-text tracking-tight">Sales History & Transactions / سجل المبيعات</h1>
        <p class="text-xs text-pos-muted">Itemized sale breakdown, thermal reprint, debt status, and protected refunds</p>
      </div>
    </div>
  </div>

  <!-- Top Statistics Cards -->
  <div class="grid grid-cols-2 md:grid-cols-4 gap-3 shrink-0">
    <div class="bg-pos-card border border-pos-border p-3 rounded-2xl shadow-xs flex items-center gap-3">
      <div class="w-9 h-9 rounded-xl bg-sky-50 dark:bg-sky-950 text-sky-600 flex items-center justify-center font-bold">
        <ShoppingBag class="w-4 h-4" />
      </div>
      <div>
        <p class="text-[10px] font-bold text-pos-muted uppercase">Transactions</p>
        <p class="text-base font-black font-mono text-pos-text">{totalSalesCount.toLocaleString()}</p>
      </div>
    </div>

    <div class="bg-pos-card border border-pos-border p-3 rounded-2xl shadow-xs flex items-center gap-3">
      <div class="w-9 h-9 rounded-xl bg-emerald-50 dark:bg-emerald-950 text-emerald-600 flex items-center justify-center font-bold">
        <DollarSign class="w-4 h-4" />
      </div>
      <div>
        <p class="text-[10px] font-bold text-pos-muted uppercase">Total Revenue</p>
        <p class="text-base font-black font-mono text-emerald-600">{totalGrossRevenue.toLocaleString()} DZD</p>
      </div>
    </div>

    <div class="bg-pos-card border border-pos-border p-3 rounded-2xl shadow-xs flex items-center gap-3">
      <div class="w-9 h-9 rounded-xl bg-blue-50 dark:bg-blue-950 text-blue-600 flex items-center justify-center font-bold">
        <CreditCard class="w-4 h-4" />
      </div>
      <div>
        <p class="text-[10px] font-bold text-pos-muted uppercase">Net Paid</p>
        <p class="text-base font-black font-mono text-blue-600">{totalNetPaid.toLocaleString()} DZD</p>
      </div>
    </div>

    <div class="bg-pos-card border border-pos-border p-3 rounded-2xl shadow-xs flex items-center gap-3">
      <div class="w-9 h-9 rounded-xl bg-amber-50 dark:bg-amber-950 text-amber-600 flex items-center justify-center font-bold">
        <Layers class="w-4 h-4" />
      </div>
      <div>
        <p class="text-[10px] font-bold text-pos-muted uppercase">Credit / Due</p>
        <p class="text-base font-black font-mono text-amber-600">{totalDueCredit.toLocaleString()} DZD</p>
      </div>
    </div>
  </div>

  <!-- Quick Date Presets -->
  <div class="flex items-center justify-between shrink-0">
    <DateQuickFilters bind:startDate bind:endDate onChange={loadSales} />
  </div>

  <!-- Filter Bar -->
  <div class="bg-pos-card border border-pos-border rounded-2xl p-3 shadow-xs grid grid-cols-1 md:grid-cols-5 gap-2.5 items-end shrink-0">
    <div>
      <label class="block text-[10px] font-bold text-pos-muted mb-1">Search Sale # or Customer</label>
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search: #, client, exact amount, or scan receipt QR..."
        class="w-full px-3 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none"
      />
    </div>

    <div>
      <label class="block text-[10px] font-bold text-pos-muted mb-1">From Date</label>
      <input type="date" bind:value={startDate} on:change={loadSales} class="w-full px-3 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none" />
    </div>

    <div>
      <label class="block text-[10px] font-bold text-pos-muted mb-1">To Date</label>
      <input type="date" bind:value={endDate} on:change={loadSales} class="w-full px-3 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none" />
    </div>

    <div>
      <label class="block text-[10px] font-bold text-pos-muted mb-1">Status Filter</label>
      <select bind:value={selectedStatus} class="w-full px-3 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none">
        <option value="all">All Statuses (الكل)</option>
        <option value="paid">Fully Paid (مدفوع بالكامل)</option>
        <option value="partial">Partial / Credit (غير مكتمل / دين)</option>
        <option value="cash">Cash Only (نقد)</option>
        <option value="tpe">TPE Card (بطاقة)</option>
        <option value="credit">Credit Only (دين)</option>
        <option value="refunded">Refunded (مسترجع)</option>
      </select>
    </div>

    <div>
      <label class="block text-[10px] font-bold text-pos-muted mb-1">Cashier</label>
      <select bind:value={selectedCashier} on:change={loadSales} class="w-full px-3 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none">
        <option value={null}>All Cashiers</option>
        {#each users as u}
          <option value={u.id}>{u.display_name || u.username}</option>
        {/each}
      </select>
    </div>
  </div>

  <!-- Sales Table -->
  <div class="bg-pos-card border border-pos-border rounded-2xl shadow-xs overflow-hidden flex-1 overflow-y-auto">
    <table class="w-full text-start text-xs border-collapse">
      <thead class="bg-slate-50 dark:bg-slate-800/60 border-b border-pos-border text-pos-muted font-bold sticky top-0 z-10">
        <tr>
          <th class="p-3 text-start">Sale #</th>
          <th class="p-3 text-start">Date & Time</th>
          <th class="p-3 text-start">Cashier</th>
          <th class="p-3 text-start">Customer</th>
          <th class="p-3 text-end">Total Amount</th>
          <th class="p-3 text-end">Paid Amount</th>
          <th class="p-3 text-center">Payment</th>
          <th class="p-3 text-center">Status</th>
          <th class="p-3 text-end">Actions</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-pos-border/40">
        {#if filteredSales.length === 0}
          <tr>
            <td colspan="9" class="p-8 text-center text-pos-muted">No sales found matching the current filters.</td>
          </tr>
        {:else}
          {#each filteredSales as s}
            <tr
              on:click={() => openSaleDetails(s)}
              class="hover:bg-slate-50 dark:hover:bg-slate-800/40 transition cursor-pointer"
            >
              <td class="p-3 font-mono font-bold text-sky-600">#{s.sale_number}</td>
              <td class="p-3 font-mono text-pos-muted">{s.created_at}</td>
              <td class="p-3 font-bold text-pos-text">{s.user_name || 'Admin'}</td>
              <td class="p-3 text-pos-muted">{s.customer_name || 'Client Comptoir'}</td>
              <td class="p-3 text-end font-mono font-black text-pos-text">{s.total_amount.toLocaleString()} DZD</td>
              <td class="p-3 text-end font-mono font-black text-emerald-600">{s.paid_amount.toLocaleString()} DZD</td>
              <td class="p-3 text-center">
                <span class="px-2 py-0.5 rounded-md text-[10px] font-bold uppercase {(s.payment_method || 'cash') === 'cash' ? 'bg-emerald-100 text-emerald-800 dark:bg-emerald-950 dark:text-emerald-300' : (s.payment_method || '') === 'tpe' ? 'bg-sky-100 text-sky-800' : 'bg-amber-100 text-amber-800'}">
                  {s.payment_method || 'cash'}
                </span>
              </td>
              <td class="p-3 text-center">
                <span class="px-2 py-0.5 rounded-full text-[10px] font-black uppercase {s.payment_status === 'paid' ? 'bg-emerald-100 text-emerald-800 dark:bg-emerald-950 dark:text-emerald-300' : 'bg-amber-100 text-amber-800'}">
                  {s.payment_status}
                </span>
              </td>
              <td class="p-3 text-end">
                <div class="flex items-center justify-end gap-1">
                  <button
                    type="button"
                    on:click={(e) => { e.stopPropagation(); printReceipt(s); }}
                    class="p-1.5 text-pos-muted hover:text-sky-600 rounded-lg cursor-pointer"
                    title="Reprint Thermal Receipt"
                  >
                    <Printer class="w-4 h-4" />
                  </button>
                  <button
                    type="button"
                    on:click={(e) => { e.stopPropagation(); openSaleDetails(s); }}
                    class="p-1.5 text-pos-muted hover:text-sky-600 rounded-lg cursor-pointer"
                    title="View Details"
                  >
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

<!-- Sale Details Modal -->
{#if isDetailModalOpen && selectedSale}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl w-full max-w-2xl max-h-[90vh] flex flex-col overflow-hidden animate-in zoom-in-95 duration-150">
      <!-- Modal Header -->
      <div class="flex items-center justify-between px-6 py-4 border-b border-pos-border bg-slate-50 dark:bg-slate-800/60">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-2xl bg-sky-600/10 text-sky-600 flex items-center justify-center font-bold">
            <ShoppingBag class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-black text-base text-pos-text">Sale Invoice #{selectedSale.sale_number}</h3>
            <p class="text-xs text-pos-muted">{selectedSale.created_at} • Cashier: {selectedSale.user_name || 'Admin'}</p>
          </div>
        </div>
        <button on:click={() => (isDetailModalOpen = false)} class="text-pos-muted hover:text-pos-text p-1.5 rounded-xl cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Modal Body -->
      <div class="p-6 overflow-y-auto space-y-4 flex-1">
        <!-- Receipt QR (scan to find this sale) -->
        <div class="flex items-center justify-center gap-4 p-3 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border">
          <img src={entityQrUrl(entityQrPayload('SALE', selectedSale.sale_number), 110)} alt="Sale QR" class="w-[110px] h-[110px]" />
          <div class="text-xs text-pos-muted font-bold">
            <p>Receipt QR / رمز الوصل</p>
            <p class="font-mono text-pos-text">{entityQrPayload('SALE', selectedSale.sale_number)}</p>
          </div>
        </div>

        <!-- Customer & Payment Summary -->
        <div class="grid grid-cols-2 md:grid-cols-4 gap-3 p-3 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border text-xs">
          <div>
            <span class="text-pos-muted font-bold block mb-0.5">Customer:</span>
            <span class="font-black text-pos-text">{selectedSale.customer_name || 'Client Comptoir'}</span>
          </div>
          <div>
            <span class="text-pos-muted font-bold block mb-0.5">Payment Method:</span>
            <span class="font-black capitalize text-sky-600">{selectedSale.payment_method || 'cash'}</span>
          </div>
          <div>
            <span class="text-pos-muted font-bold block mb-0.5">Status:</span>
            <span class="font-black capitalize text-emerald-600">{selectedSale.payment_status}</span>
          </div>
          <div>
            <span class="text-pos-muted font-bold block mb-0.5">Total Amount:</span>
            <span class="font-black font-mono text-pos-text">{selectedSale.total_amount.toLocaleString()} DZD</span>
          </div>
        </div>

        <!-- Items Table -->
        <div class="bg-pos-card border border-pos-border rounded-2xl overflow-hidden shadow-xs">
          <table class="w-full text-start text-xs border-collapse">
            <thead class="bg-slate-50 dark:bg-slate-800/60 border-b border-pos-border text-pos-muted font-bold">
              <tr>
                <th class="p-2.5 text-start">Item</th>
                <th class="p-2.5 text-center">Qty</th>
                <th class="p-2.5 text-end">Unit Price</th>
                <th class="p-2.5 text-end">Line Total</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-pos-border/40">
              {#if isLoadingItems}
                <tr>
                  <td colspan="4" class="p-6 text-center text-pos-muted">Loading item details...</td>
                </tr>
              {:else if saleItems.length === 0}
                <tr>
                  <td colspan="4" class="p-6 text-center text-pos-muted">No line items recorded for this sale.</td>
                </tr>
              {:else}
                {#each saleItems as item}
                  <tr class="hover:bg-slate-50 dark:hover:bg-slate-800/30">
                    <td class="p-2.5 font-bold text-pos-text">
                      <p>{item.name_fr || item.name_ar}</p>
                      <p class="text-[10px] text-pos-muted font-mono">{item.barcode || item.sku || '—'}</p>
                    </td>
                    <td class="p-2.5 text-center font-mono font-bold">{item.quantity}</td>
                    <td class="p-2.5 text-end font-mono text-pos-muted">{item.unit_price} DZD</td>
                    <td class="p-2.5 text-end font-mono font-black text-pos-text">{item.total_price} DZD</td>
                  </tr>
                {/each}
              {/if}
            </tbody>
          </table>
        </div>
      </div>

      <!-- Modal Footer -->
      <div class="px-6 py-4 border-t border-pos-border bg-slate-50 dark:bg-slate-800/60 flex items-center justify-between">
        <button
          type="button"
          on:click={promptProtectedDelete}
          class="px-4 py-2 bg-rose-100 hover:bg-rose-200 text-rose-800 dark:bg-rose-950/60 dark:text-rose-300 font-bold text-xs rounded-xl flex items-center gap-1.5 cursor-pointer"
        >
          <Trash2 class="w-4 h-4" />
          <span>Delete / Cancel Sale</span>
        </button>

        <div class="flex items-center gap-2">
          <button
            type="button"
            on:click={() => editSaleInPos(selectedSale)}
            class="px-4 py-2 bg-amber-100 hover:bg-amber-200 text-amber-800 dark:bg-amber-950/60 dark:text-amber-300 font-bold text-xs rounded-xl flex items-center gap-1.5 cursor-pointer"
            title="Re-open this sale in the POS cart"
          >
            <Pencil class="w-4 h-4" />
            <span>Edit in POS / تعديل</span>
          </button>
          <button on:click={() => (isDetailModalOpen = false)} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-pos-text font-bold text-xs rounded-xl cursor-pointer">
            Close
          </button>
          <button
            on:click={() => printReceipt(selectedSale)}
            class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-md"
          >
            <Printer class="w-4 h-4" />
            <span>Reprint Receipt</span>
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Protected Delete Modal (z-index above the sale-detail modal it opens from) -->
{#if isDeleteModalOpen && selectedSale}
  <div class="fixed inset-0 z-[60] bg-black/60 backdrop-blur-2xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-2xl shadow-2xl p-6 max-w-sm w-full space-y-4 animate-in zoom-in-95">
      <div class="flex items-center gap-3 text-rose-600">
        <ShieldAlert class="w-6 h-6 shrink-0" />
        <h3 class="font-black text-sm text-pos-text">Protected Sale Deletion</h3>
      </div>
      <p class="text-xs text-pos-muted">
        Are you sure you want to delete invoice <strong class="text-pos-text">#{selectedSale.sale_number}</strong>? This action updates cash drawer totals and returns stock.
      </p>

      {#if $currentUser?.role_name !== 'admin'}
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Enter Admin Authorization Password *</label>
          <input
            type="password"
            bind:value={adminPassword}
            placeholder="Password"
            class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 rounded-xl text-xs font-mono outline-none"
          />
        </div>
      {/if}

      {#if deleteError}
        <div class="p-2 bg-rose-100 text-rose-800 text-xs font-bold rounded-lg">{deleteError}</div>
      {/if}

      <div class="flex justify-end gap-2 pt-2 border-t border-pos-border">
        <button on:click={() => (isDeleteModalOpen = false)} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-xl cursor-pointer">
          Cancel
        </button>
        <button on:click={executeProtectedDelete} disabled={isDeleting} class="px-4 py-2 bg-rose-600 text-white text-xs font-black rounded-xl cursor-pointer shadow-md">
          {isDeleting ? 'Deleting...' : 'Confirm Delete'}
        </button>
      </div>
    </div>
  </div>
{/if}
