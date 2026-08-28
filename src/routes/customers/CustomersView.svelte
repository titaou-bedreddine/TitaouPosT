<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Customer } from '../../lib/types';
  import { printHtmlDirectly } from '../../lib/utils/printer';
  import CustomerDebtModal from '../../lib/components/CustomerDebtModal.svelte';
  import UniversalSearchBar from '../../lib/components/UniversalSearchBar.svelte';
  import {
    Users, Plus, QrCode, DollarSign, Edit2, Trash2, Search,
    X, Check, Printer, FileText, Phone, MapPin, Building, History
  } from 'lucide-svelte';

  let customers: Customer[] = [];
  let searchQuery = '';
  let searchType: 'all' | 'name' | 'barcode' | 'price' | 'qr' = 'all';

  let isModalOpen = false;
  let isDebtModalOpen = false;
  let selectedCustomer: Customer | null = null;
  let previewCustomer: Customer | null = null;

  let name = '';
  let phone = '';
  let email = '';
  let address = '';
  let rc = '';
  let nif = '';
  let nis = '';
  let ai = '';
  let initialDebt = 0;
  let notes = '';
  let editingId: number | null = null;

  let isSaving = false;
  let errorMsg = '';

  onMount(async () => {
    await loadCustomers();
  });

  async function loadCustomers() {
    try {
      customers = await invoke<Customer[]>('list_customers');
    } catch (e) {
      console.error(e);
    }
  }

  $: filteredCustomers = customers.filter(c =>
    c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    (c.phone && c.phone.includes(searchQuery)) ||
    (c.rc && c.rc.includes(searchQuery))
  );

  function openAddModal() {
    editingId = null;
    name = '';
    phone = '';
    email = '';
    address = '';
    rc = '';
    nif = '';
    nis = '';
    ai = '';
    initialDebt = 0;
    notes = '';
    errorMsg = '';
    isModalOpen = true;
  }

  function openEditModal(c: Customer) {
    editingId = c.id;
    name = c.name;
    phone = c.phone || '';
    email = c.email || '';
    address = c.address || '';
    rc = c.rc || '';
    nif = c.nif || '';
    nis = c.nis || '';
    ai = c.ai || '';
    initialDebt = 0;
    notes = c.notes || '';
    errorMsg = '';
    isModalOpen = true;
  }

  async function handleSave() {
    if (!name.trim()) {
      errorMsg = 'Customer name is required / اسم العميل مطلوب';
      return;
    }
    try {
      isSaving = true;
      errorMsg = '';
      await invoke('save_customer', {
        name,
        phone: phone || null,
        email: email || null,
        address: address || null,
        rc: rc || null,
        nif: nif || null,
        nis: nis || null,
        ai: ai || null,
        initialDebt,
        notes: notes || null,
        customerId: editingId,
      });

      isModalOpen = false;
      await loadCustomers();
    } catch (e: any) {
      errorMsg = typeof e === 'string' ? e : e.message || 'Failed to save customer';
    } finally {
      isSaving = false;
    }
  }

  function printCustomerDebtRecap(c: Customer) {
    const qrData = encodeURIComponent(`CUST:${c.id}:BALANCE:${c.balance}`);
    const qrUrl = `https://api.qrserver.com/v1/create-qr-code/?size=100x100&data=${qrData}`;

    const html = `
      <div class="text-center pb-2 border-b-dashed">
        <h2 class="font-black text-sm uppercase">TitaouPOS Superette</h2>
        <p class="text-xxs">Rue Principale, Alger • Tél: 0553444057</p>
        <p class="font-black text-xs mt-1 bg-black text-white px-1">RECAPITULATIF DE DETTE / كشف حساب دين</p>
        <p class="text-xxs mt-0.5">${new Date().toLocaleString()}</p>
      </div>

      <div class="py-2 border-b-dashed text-xxs space-y-1">
        <div class="flex justify-between"><span>Client:</span><strong>${c.name}</strong></div>
        <div class="flex justify-between"><span>Téléphone:</span><strong>${c.phone || 'N/A'}</strong></div>
        ${c.rc ? `<div class="flex justify-between"><span>RC:</span><strong>${c.rc}</strong></div>` : ''}
        ${c.nif ? `<div class="flex justify-between"><span>NIF:</span><strong>${c.nif}</strong></div>` : ''}
      </div>

      <div class="py-3 border-b-dashed text-center">
        <p class="text-xs text-gray-600 font-bold">SOLDE ACTUEL DU (دين مستحق):</p>
        <p class="text-2xl font-black font-mono text-black mt-1">${c.balance.toLocaleString()} DZD</p>
      </div>

      <div class="text-center pt-2">
        <img src="${qrUrl}" alt="QR" class="qr-box" />
        <p class="text-xxs text-gray-500 mt-1">Code Client: #CUST-${c.id}</p>
        <p class="text-[8px] text-gray-400 mt-1">TitaouPOS • Created by Titaou Bedreddine 0553444057</p>
      </div>
    `;

    printHtmlDirectly(html, `Recap Dette - ${c.name}`);
  }
</script>

<div class="h-full flex flex-col bg-pos-bg p-4 overflow-hidden select-none">
  <!-- Header -->
  <div class="flex items-center justify-between pb-4 border-b border-pos-border shrink-0">
    <div class="flex items-center gap-3">
      <div class="w-10 h-10 rounded-2xl bg-sky-100 dark:bg-sky-950 text-sky-600 flex items-center justify-center font-bold">
        <Users class="w-5 h-5" />
      </div>
      <div>
        <h1 class="text-xl font-black text-pos-text tracking-tight">Customers & Debts / العملاء والديون</h1>
        <p class="text-xs text-pos-muted">Manage customer records, credit balances, and debt payment receipts</p>
      </div>
    </div>

    <div class="flex items-center gap-2">
      <button
        type="button"
        on:click={openAddModal}
        class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white rounded-xl text-xs font-black transition shadow-xs flex items-center gap-2 cursor-pointer active:scale-95"
      >
        <Plus class="w-4 h-4" />
        <span>Add Customer (إضافة عميل جديد)</span>
      </button>
    </div>
  </div>

  <!-- Search Filter Bar -->
  <div class="mt-4 mb-2">
    <div class="relative">
      <Search class="w-4 h-4 text-pos-muted absolute start-3 top-2.5" />
      <input
        type="text"
        bind:value={searchQuery}
        placeholder="Search by customer name, phone, RC, or NIF..."
        class="w-full ps-9 pe-3 py-2 bg-pos-card border border-pos-border rounded-xl text-xs font-bold text-pos-text outline-none focus:border-sky-500 shadow-xs"
      />
    </div>
  </div>

  <!-- Customers Table -->
  <div class="flex-1 overflow-y-auto bg-pos-card border border-pos-border rounded-2xl shadow-xs">
    <table class="w-full text-start text-xs border-collapse">
      <thead class="bg-slate-50 dark:bg-slate-800/60 border-b border-pos-border text-pos-muted font-bold sticky top-0 z-10">
        <tr>
          <th class="p-3 text-start">Customer Name / الاسم</th>
          <th class="p-3 text-start">Phone</th>
          <th class="p-3 text-start">RC / NIF</th>
          <th class="p-3 text-end">Current Debt (الديون)</th>
          <th class="p-3 text-center">QR Code</th>
          <th class="p-3 text-end">Actions</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-pos-border/40">
        {#if filteredCustomers.length === 0}
          <tr>
            <td colspan="6" class="p-8 text-center text-pos-muted">No customers found.</td>
          </tr>
        {:else}
          {#each filteredCustomers as c}
            <tr class="hover:bg-slate-50 dark:hover:bg-slate-800/40 transition">
              <td class="p-3 font-bold text-pos-text cursor-pointer" on:click={() => (previewCustomer = c)}>
                {c.name}
              </td>
              <td class="p-3 font-mono text-pos-muted">{c.phone || '—'}</td>
              <td class="p-3 font-mono text-pos-muted">{c.rc ? `${c.rc} / ${c.nif || ''}` : '—'}</td>
              <td class="p-3 text-end font-mono font-black {c.balance > 0 ? 'text-rose-600' : 'text-emerald-600'}">
                {c.balance.toLocaleString()} DZD
              </td>
              <td class="p-3 text-center">
                <button
                  type="button"
                  on:click={() => printCustomerDebtRecap(c)}
                  class="p-1 text-sky-600 hover:bg-sky-50 dark:hover:bg-sky-950 rounded-lg cursor-pointer transition"
                  title="Print Debt QR Recap"
                >
                  <QrCode class="w-4 h-4 mx-auto" />
                </button>
              </td>
              <td class="p-3 text-end">
                <div class="flex items-center justify-end gap-1">
                  {#if c.balance > 0}
                    <button
                      type="button"
                      on:click={() => { selectedCustomer = c; isDebtModalOpen = true; }}
                      class="px-2.5 py-1 bg-amber-600 hover:bg-amber-700 text-white rounded-lg text-[11px] font-bold cursor-pointer transition shadow-xs"
                    >
                      Regler Dette
                    </button>
                  {/if}
                  <button
                    type="button"
                    on:click={() => openEditModal(c)}
                    class="p-1.5 text-pos-muted hover:text-sky-600 rounded-lg cursor-pointer"
                    title="Edit"
                  >
                    <Edit2 class="w-3.5 h-3.5" />
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

<!-- Modal: Add / Edit Customer -->
{#if isModalOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl w-full max-w-xl overflow-hidden animate-in zoom-in-95 duration-150 flex flex-col">
      <div class="flex items-center justify-between px-6 py-4 border-b border-pos-border bg-slate-50 dark:bg-slate-800/60">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-2xl bg-sky-600/10 text-sky-600 flex items-center justify-center font-bold">
            <Users class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-black text-base text-pos-text">
              {editingId ? 'Edit Customer / تعديل عميل' : 'New Customer / إضافة عميل جديد'}
            </h3>
            <p class="text-xs text-pos-muted">Enter personal, business & legal details</p>
          </div>
        </div>
        <button on:click={() => (isModalOpen = false)} class="text-pos-muted hover:text-pos-text p-1.5 rounded-xl cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>

      {#if errorMsg}
        <div class="mx-6 mt-4 p-3 bg-rose-100 text-rose-800 text-xs font-bold rounded-xl">{errorMsg}</div>
      {/if}

      <div class="p-6 space-y-3 overflow-y-auto max-h-[70vh]">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Full Name / الاسم الكامل *</label>
            <input type="text" bind:value={name} placeholder="Ex: Mohamed Amine" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
          </div>
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Phone / الهاتف</label>
            <input type="text" bind:value={phone} placeholder="Ex: 0550 12 34 56" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
          </div>
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Email / البريد الإلكتروني</label>
            <input type="email" bind:value={email} placeholder="Ex: client@email.com" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none" />
          </div>
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Address / العنوان</label>
            <input type="text" bind:value={address} placeholder="Ex: Alger Centre" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none" />
          </div>
        </div>

        <div class="p-3 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-2">
          <span class="text-xs font-black text-pos-text block">Legal & Fiscal Details (بيانات السجل الجبائي)</span>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-2">
            <div>
              <label class="block text-[10px] font-bold text-pos-muted mb-0.5">RC</label>
              <input type="text" bind:value={rc} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs font-mono" />
            </div>
            <div>
              <label class="block text-[10px] font-bold text-pos-muted mb-0.5">NIF</label>
              <input type="text" bind:value={nif} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs font-mono" />
            </div>
            <div>
              <label class="block text-[10px] font-bold text-pos-muted mb-0.5">NIS</label>
              <input type="text" bind:value={nis} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs font-mono" />
            </div>
            <div>
              <label class="block text-[10px] font-bold text-pos-muted mb-0.5">AI</label>
              <input type="text" bind:value={ai} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs font-mono" />
            </div>
          </div>
        </div>

        {#if !editingId}
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Initial Starting Debt / الرصيد الافتتاحي السابق (DZD)</label>
            <input type="number" min="0" bind:value={initialDebt} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none" />
          </div>
        {/if}
      </div>

      <div class="px-6 py-4 border-t border-pos-border bg-slate-50 dark:bg-slate-800/60 flex items-center justify-end gap-2">
        <button on:click={() => (isModalOpen = false)} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-pos-text font-bold text-xs rounded-xl cursor-pointer">Cancel</button>
        <button on:click={handleSave} disabled={isSaving} class="px-6 py-2 bg-sky-600 hover:bg-sky-700 disabled:opacity-50 text-white font-black text-xs rounded-xl shadow-md cursor-pointer flex items-center gap-1.5">
          <Check class="w-4 h-4" />
          <span>{isSaving ? 'Saving...' : 'Save Customer (حفظ)'}</span>
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Modal: Customer Profile & Real QR Code -->
{#if previewCustomer}
  {@const qrData = encodeURIComponent(`CUST:${previewCustomer.id}:BALANCE:${previewCustomer.balance}`)}
  {@const qrUrl = `https://api.qrserver.com/v1/create-qr-code/?size=150x150&data=${qrData}`}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl w-full max-w-md overflow-hidden animate-in zoom-in-95 duration-150 flex flex-col">
      <div class="flex items-center justify-between px-6 py-4 border-b border-pos-border bg-slate-50 dark:bg-slate-800/60">
        <h3 class="font-black text-base text-pos-text">{previewCustomer.name}</h3>
        <button on:click={() => (previewCustomer = null)} class="text-pos-muted hover:text-pos-text p-1.5 rounded-xl cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="p-6 text-center space-y-4">
        <!-- Real QR Code -->
        <div class="w-36 h-36 bg-white p-2 rounded-2xl mx-auto shadow-md border border-pos-border flex items-center justify-center">
          <img src={qrUrl} alt="QR Code" class="w-full h-full object-contain" />
        </div>
        <p class="text-xs font-mono font-bold text-pos-muted">Customer Code: #CUST-{previewCustomer.id}</p>

        <div class="p-3 bg-slate-100 dark:bg-slate-800 rounded-2xl text-start text-xs space-y-1.5">
          <div class="flex justify-between"><span class="text-pos-muted">Phone:</span><span class="font-mono font-bold text-pos-text">{previewCustomer.phone || 'N/A'}</span></div>
          <div class="flex justify-between"><span class="text-pos-muted">Address:</span><span class="font-bold text-pos-text">{previewCustomer.address || 'N/A'}</span></div>
          <div class="flex justify-between"><span class="text-pos-muted">RC / NIF:</span><span class="font-mono text-pos-text">{previewCustomer.rc || '—'} / {previewCustomer.nif || '—'}</span></div>
          <div class="flex justify-between pt-1 border-t border-pos-border"><span class="font-bold text-pos-muted">Balance Due (الرصيد المستحق):</span><span class="font-mono font-black text-rose-600">{previewCustomer.balance.toLocaleString()} DZD</span></div>
        </div>
      </div>

      <div class="px-6 py-4 border-t border-pos-border bg-slate-50 dark:bg-slate-800/60 flex items-center justify-between">
        <button on:click={() => printCustomerDebtRecap(previewCustomer)} class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-xs">
          <Printer class="w-4 h-4" />
          <span>Print Statement (طباعة كشف)</span>
        </button>
        <button on:click={() => (previewCustomer = null)} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-pos-text font-bold text-xs rounded-xl cursor-pointer">Close</button>
      </div>
    </div>
  </div>
{/if}

<CustomerDebtModal
  isOpen={isDebtModalOpen}
  customer={selectedCustomer}
  onClose={() => (isDebtModalOpen = false)}
  onPaymentSuccess={loadCustomers}
/>