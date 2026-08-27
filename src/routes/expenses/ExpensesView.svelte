<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Expense } from '../../lib/types';
  import { currentUser } from '../../lib/stores/auth';
  import { activeSession } from '../../lib/stores/session';
  import { Plus, DollarSign } from 'lucide-svelte';

  let expenses: Expense[] = [];
  let isAddOpen = false;

  let amount = 0;
  let categoryId = 1;
  let paymentMethod = 'cash';
  let recipient = '';
  let receiptRef = '';
  let notes = '';

  onMount(async () => {
    await loadExpenses();
  });

  async function loadExpenses() {
    try {
      expenses = await invoke<Expense[]>('list_expenses');
    } catch (e) {
      console.error(e);
    }
  }

  async function handleAddExpense() {
    if (!$currentUser) return;
    try {
      await invoke('add_expense', {
        categoryId,
        amount,
        paymentMethod,
        sessionId: paymentMethod === 'cash' ? $activeSession?.id : null,
        userId: $currentUser.id,
        recipient: recipient || null,
        receiptReference: receiptRef || null,
        notes: notes || null,
      });
      isAddOpen = false;
      amount = 0;
      recipient = '';
      notes = '';
      await loadExpenses();
    } catch (e) {
      console.error(e);
    }
  }
</script>

<div class="p-6 space-y-4 overflow-y-auto h-full">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-black text-pos-text">Expenses & Operating Costs (المصاريف)</h1>
      <p class="text-xs text-pos-muted mt-1">Track store overhead and deduct directly from active cash drawer</p>
    </div>
    <button
      on:click={() => isAddOpen = true}
      class="px-4 py-2 bg-rose-600 hover:bg-rose-700 text-white font-bold text-xs rounded-lg transition flex items-center gap-1.5 shadow-xs cursor-pointer"
    >
      <Plus class="w-4 h-4" />
      <span>Add Expense</span>
    </button>
  </div>

  {#if isAddOpen}
    <div class="bg-pos-card border border-pos-border rounded-xl p-4 shadow-sm space-y-3">
      <h3 class="font-bold text-sm text-pos-text">New Expense Entry</h3>
      <div class="grid grid-cols-1 md:grid-cols-4 gap-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Category</label>
          <select bind:value={categoryId} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs text-pos-text">
            <option value={1}>Rent (إيجار)</option>
            <option value={2}>Utilities (الكهرباء والماء)</option>
            <option value={3}>Maintenance (الصيانة)</option>
            <option value={4}>Transport (النقل)</option>
            <option value={5}>Supplies (اللوازم)</option>
            <option value={6}>Other (أخرى)</option>
          </select>
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Amount (DZD)</label>
          <input type="number" bind:value={amount} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs font-bold font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Payment Method</label>
          <select bind:value={paymentMethod} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs text-pos-text">
            <option value="cash">Cash (From Register Drawer)</option>
            <option value="bank_transfer">Bank Transfer / Other</option>
          </select>
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Recipient / Supplier</label>
          <input type="text" bind:value={recipient} placeholder="e.g. Landlord or Electricity Company" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs text-pos-text" />
        </div>
      </div>
      <div class="flex justify-end gap-2 pt-2">
        <button on:click={() => isAddOpen = false} class="px-3 py-1.5 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded">Cancel</button>
        <button on:click={handleAddExpense} class="px-4 py-1.5 bg-rose-600 text-white text-xs font-bold rounded">Save Expense</button>
      </div>
    </div>
  {/if}

  <div class="bg-pos-card border border-pos-border rounded-xl shadow-xs overflow-hidden">
    <table class="w-full text-start text-xs border-collapse">
      <thead>
        <tr class="border-b border-pos-border bg-slate-50 dark:bg-slate-800/60 text-pos-muted font-bold">
          <th class="p-3 text-start">Expense #</th>
          <th class="p-3 text-start">Category</th>
          <th class="p-3 text-start">Date</th>
          <th class="p-3 text-start">Payment Source</th>
          <th class="p-3 text-start">Recipient</th>
          <th class="p-3 text-end">Amount</th>
        </tr>
      </thead>
      <tbody>
        {#each expenses as exp}
          <tr class="border-b border-pos-border/60 hover:bg-slate-50 dark:hover:bg-slate-800/40">
            <td class="p-3 font-mono font-bold text-rose-600">{exp.expense_number}</td>
            <td class="p-3 font-bold text-pos-text">{exp.category_name || 'Expense'}</td>
            <td class="p-3 text-pos-muted">{exp.date}</td>
            <td class="p-3 font-semibold uppercase">{exp.payment_method}</td>
            <td class="p-3 text-pos-muted">{exp.recipient || '-'}</td>
            <td class="p-3 text-end font-mono font-bold text-sm text-rose-600">
              {exp.amount.toLocaleString()} DZD
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>