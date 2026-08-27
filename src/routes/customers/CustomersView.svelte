<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Customer } from '../../lib/types';
  import CustomerDebtModal from '../../lib/components/CustomerDebtModal.svelte';
  import { Users, Plus, QrCode, DollarSign, Edit2, Trash2, Search } from 'lucide-svelte';

  let customers: Customer[] = [];
  let isAddOpen = false;
  let isDebtModalOpen = false;
  let selectedCustomer: Customer | null = null;

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

  async function handleSave() {
    if (!name.trim()) return;
    try {
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
      isAddOpen = false;
      resetForm();
      await loadCustomers();
    } catch (e) {
      console.error(e);
    }
  }

  function startEdit(c: Customer) {
    editingId = c.id;
    name = c.name;
    phone = c.phone || '';
    email = c.email || '';
    address = c.address || '';
    rc = c.rc || '';
    nif = c.nif || '';
    nis = c.nis || '';
    ai = c.ai || '';
    initialDebt = c.initial_debt || 0;
    notes = c.notes || '';
    isAddOpen = true;
  }

  function resetForm() {
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
  }

  async function handleDelete(id: number) {
    try {
      await invoke('delete_customer', { customerId: id });
      await loadCustomers();
    } catch (e) {
      console.error(e);
    }
  }

  function openDebtModal(c: Customer) {
    selectedCustomer = c;
    isDebtModalOpen = true;
  }
</script>

<div class="p-6 space-y-4 overflow-y-auto h-full select-none">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-black text-pos-text">Customers & Debt Ledger (الزبائن والديون)</h1>
      <p class="text-xs text-pos-muted mt-1">Manage customer profiles, RC, NIF, debt balances, and repayments</p>
    </div>
    <button
      on:click={() => { resetForm(); isAddOpen = true; }}
      class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-xl transition flex items-center gap-1.5 shadow-xs cursor-pointer"
    >
      <Plus class="w-4 h-4" />
      <span>New Customer</span>
    </button>
  </div>

  {#if isAddOpen}
    <div class="bg-pos-card border border-pos-border rounded-2xl p-5 shadow-sm space-y-4 animate-in fade-in duration-150">
      <h3 class="font-extrabold text-sm text-pos-text">{editingId ? 'Edit Customer' : 'Add New Customer'}</h3>
      <div class="grid grid-cols-1 md:grid-cols-4 gap-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Customer / Company Name</label>
          <input type="text" bind:value={name} placeholder="e.g. Ets Boualem" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Phone Number</label>
          <input type="text" bind:value={phone} placeholder="0555 XX XX XX" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Email</label>
          <input type="email" bind:value={email} placeholder="contact@example.dz" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Address</label>
          <input type="text" bind:value={address} placeholder="Algiers, Algeria" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs text-pos-text" />
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-5 gap-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">RC (السجل التجاري)</label>
          <input type="text" bind:value={rc} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">NIF (الرقم الجبائي)</label>
          <input type="text" bind:value={nif} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">NIS (رقم الإحصاء)</label>
          <input type="text" bind:value={nis} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">AI (المادة الجبائية)</label>
          <input type="text" bind:value={ai} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Initial Debt (DZD)</label>
          <input type="number" bind:value={initialDebt} on:focus={(e) => (e.target as HTMLInputElement).select()} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono font-bold text-pos-text" />
        </div>
      </div>

      <div class="flex justify-end gap-2 pt-2 border-t border-pos-border/60">
        <button on:click={() => isAddOpen = false} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-lg">Cancel</button>
        <button on:click={handleSave} class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white text-xs font-bold rounded-lg shadow-xs">Save Customer</button>
      </div>
    </div>
  {/if}

  <!-- Customers Table -->
  <div class="bg-pos-card border border-pos-border rounded-2xl shadow-xs overflow-hidden">
    <table class="w-full text-start text-xs border-collapse">
      <thead>
        <tr class="border-b border-pos-border bg-slate-50 dark:bg-slate-800/40 text-pos-muted font-bold">
          <th class="p-3 text-start">QR Code</th>
          <th class="p-3 text-start">Customer Name</th>
          <th class="p-3 text-start">Phone</th>
          <th class="p-3 text-start">RC / NIF</th>
          <th class="p-3 text-end">Total Purchases</th>
          <th class="p-3 text-end">Debt Balance</th>
          <th class="p-3 text-center">Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each customers as c}
          <tr class="border-b border-pos-border/60 hover:bg-slate-50 dark:hover:bg-slate-800/40">
            <td class="p-3">
              <span class="inline-flex items-center gap-1 font-mono font-bold text-[11px] text-sky-600 bg-sky-50 dark:bg-sky-950 px-2 py-0.5 rounded">
                <QrCode class="w-3.5 h-3.5" />
                <span>{c.qr_code || 'QR'}</span>
              </span>
            </td>
            <td class="p-3 font-bold text-pos-text text-sm">{c.name}</td>
            <td class="p-3 text-pos-muted font-mono">{c.phone || '-'}</td>
            <td class="p-3 text-pos-muted font-mono">{c.rc || c.nif || '-'}</td>
            <td class="p-3 text-end font-mono font-bold text-sky-600">{(c.total_purchases || 0).toLocaleString()} DZD</td>
            <td class="p-3 text-end font-mono font-black text-sm {c.balance > 0 ? 'text-rose-600' : 'text-emerald-600'}">
              {c.balance.toLocaleString()} DZD
            </td>
            <td class="p-3 text-center">
              <div class="flex items-center justify-center gap-1.5">
                <button
                  type="button"
                  on:click={() => openDebtModal(c)}
                  class="px-2.5 py-1 bg-amber-500 hover:bg-amber-600 text-slate-950 rounded-lg text-xs font-bold flex items-center gap-1 cursor-pointer"
                  title="Profile & Debt Payment"
                >
                  <DollarSign class="w-3.5 h-3.5" />
                  <span>Repay</span>
                </button>
                <button
                  type="button"
                  on:click={() => startEdit(c)}
                  class="p-1.5 text-sky-600 hover:bg-sky-50 dark:hover:bg-sky-950 rounded-lg cursor-pointer"
                >
                  <Edit2 class="w-3.5 h-3.5" />
                </button>
                <button
                  type="button"
                  on:click={() => handleDelete(c.id)}
                  class="p-1.5 text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 rounded-lg cursor-pointer"
                >
                  <Trash2 class="w-3.5 h-3.5" />
                </button>
              </div>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <CustomerDebtModal
    isOpen={isDebtModalOpen}
    customer={selectedCustomer}
    onClose={() => isDebtModalOpen = false}
    onPaymentRecorded={loadCustomers}
  />
</div>