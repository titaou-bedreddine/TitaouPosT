<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Supplier } from '../../lib/types';
  import { Truck, Plus, QrCode, Edit2, Trash2 } from 'lucide-svelte';

  let suppliers: Supplier[] = [];
  let isAddOpen = false;

  let name = '';
  let contactPerson = '';
  let phone = '';
  let email = '';
  let address = '';
  let rc = '';
  let nif = '';
  let nis = '';
  let ai = '';
  let notes = '';
  let editingId: number | null = null;

  onMount(async () => {
    await loadSuppliers();
  });

  async function loadSuppliers() {
    try {
      suppliers = await invoke<Supplier[]>('list_suppliers');
    } catch (e) {
      console.error(e);
    }
  }

  async function handleSave() {
    if (!name.trim()) return;
    try {
      await invoke('save_supplier', {
        name,
        contactPerson: contactPerson || null,
        phone: phone || null,
        email: email || null,
        address: address || null,
        rc: rc || null,
        nif: nif || null,
        nis: nis || null,
        ai: ai || null,
        notes: notes || null,
        supplierId: editingId,
      });
      isAddOpen = false;
      resetForm();
      await loadSuppliers();
    } catch (e) {
      console.error(e);
    }
  }

  function startEdit(s: Supplier) {
    editingId = s.id;
    name = s.name;
    contactPerson = s.contact_person || '';
    phone = s.phone || '';
    email = s.email || '';
    address = s.address || '';
    rc = s.rc || '';
    nif = s.nif || '';
    nis = s.nis || '';
    ai = s.ai || '';
    notes = s.notes || '';
    isAddOpen = true;
  }

  function resetForm() {
    editingId = null;
    name = '';
    contactPerson = '';
    phone = '';
    email = '';
    address = '';
    rc = '';
    nif = '';
    nis = '';
    ai = '';
    notes = '';
  }

  async function handleDelete(id: number) {
    try {
      await invoke('delete_supplier', { supplierId: id });
      await loadSuppliers();
    } catch (e) {
      console.error(e);
    }
  }
</script>

<div class="p-6 space-y-4 overflow-y-auto h-full select-none">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-black text-pos-text">Suppliers Directory (الموردون)</h1>
      <p class="text-xs text-pos-muted mt-1">Manage vendor profiles, RC, NIF, and purchase balances</p>
    </div>
    <button
      on:click={() => { resetForm(); isAddOpen = true; }}
      class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-xl transition flex items-center gap-1.5 shadow-xs cursor-pointer"
    >
      <Plus class="w-4 h-4" />
      <span>New Supplier</span>
    </button>
  </div>

  {#if isAddOpen}
    <div class="bg-pos-card border border-pos-border rounded-2xl p-5 shadow-sm space-y-4 animate-in fade-in duration-150">
      <h3 class="font-extrabold text-sm text-pos-text">{editingId ? 'Edit Supplier' : 'Add New Supplier'}</h3>
      <div class="grid grid-cols-1 md:grid-cols-4 gap-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Company / Supplier Name</label>
          <input type="text" bind:value={name} placeholder="e.g. Candia Algérie" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Contact Person</label>
          <input type="text" bind:value={contactPerson} placeholder="Karim Mehdi" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Phone Number</label>
          <input type="text" bind:value={phone} placeholder="021 XX XX XX" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Address</label>
          <input type="text" bind:value={address} placeholder="Zone Industrielle Rouiba" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs text-pos-text" />
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-4 gap-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">RC (السجل التجاري)</label>
          <input type="text" bind:value={rc} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">NIF</label>
          <input type="text" bind:value={nif} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">NIS</label>
          <input type="text" bind:value={nis} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">AI</label>
          <input type="text" bind:value={ai} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
      </div>

      <div class="flex justify-end gap-2 pt-2 border-t border-pos-border/60">
        <button on:click={() => isAddOpen = false} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-lg">Cancel</button>
        <button on:click={handleSave} class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white text-xs font-bold rounded-lg shadow-xs">Save Supplier</button>
      </div>
    </div>
  {/if}

  <!-- Suppliers Table -->
  <div class="bg-pos-card border border-pos-border rounded-2xl shadow-xs overflow-hidden">
    <table class="w-full text-start text-xs border-collapse">
      <thead>
        <tr class="border-b border-pos-border bg-slate-50 dark:bg-slate-800/40 text-pos-muted font-bold">
          <th class="p-3 text-start">QR Code</th>
          <th class="p-3 text-start">Supplier Name</th>
          <th class="p-3 text-start">Contact Person</th>
          <th class="p-3 text-start">Phone</th>
          <th class="p-3 text-start">RC / NIF</th>
          <th class="p-3 text-end">Our Debt Balance</th>
          <th class="p-3 text-center">Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each suppliers as s}
          <tr class="border-b border-pos-border/60 hover:bg-slate-50 dark:hover:bg-slate-800/40">
            <td class="p-3">
              <span class="inline-flex items-center gap-1 font-mono font-bold text-[11px] text-sky-600 bg-sky-50 dark:bg-sky-950 px-2 py-0.5 rounded">
                <QrCode class="w-3.5 h-3.5" />
                <span>{s.qr_code || 'SUPP'}</span>
              </span>
            </td>
            <td class="p-3 font-bold text-pos-text text-sm">{s.name}</td>
            <td class="p-3 text-pos-muted">{s.contact_person || '-'}</td>
            <td class="p-3 text-pos-muted font-mono">{s.phone || '-'}</td>
            <td class="p-3 text-pos-muted font-mono">{s.rc || s.nif || '-'}</td>
            <td class="p-3 text-end font-mono font-black text-sm {s.balance > 0 ? 'text-rose-600' : 'text-emerald-600'}">
              {s.balance.toLocaleString()} DZD
            </td>
            <td class="p-3 text-center">
              <div class="flex items-center justify-center gap-1.5">
                <button
                  type="button"
                  on:click={() => startEdit(s)}
                  class="p-1.5 text-sky-600 hover:bg-sky-50 dark:hover:bg-sky-950 rounded-lg cursor-pointer"
                >
                  <Edit2 class="w-3.5 h-3.5" />
                </button>
                <button
                  type="button"
                  on:click={() => handleDelete(s.id)}
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
</div>