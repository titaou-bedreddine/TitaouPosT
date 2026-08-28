<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Expense } from '../../lib/types';
  import { currentUser } from '../../lib/stores/auth';
  import { activeSession } from '../../lib/stores/session';
  import { printHtmlDirectly } from '../../lib/utils/printer';
  import {
    Plus, DollarSign, Trash2, Eye, Printer, X, Check,
    TrendingDown, Calendar, User, Receipt
  } from 'lucide-svelte';

  let expenses: Expense[] = [];
  let isAddOpen = false;
  let previewExpense: Expense | null = null;

  let amount = 0;
  let categoryId = 1;
  let paymentMethod = 'cash';
  let recipient = '';
  let receiptRef = '';
  let notes = '';

  let isSaving = false;
  let errorMsg = '';

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
    if (amount <= 0) {
      errorMsg = 'Please enter a valid expense amount / الرجاء إدخال مبلغ صحيح';
      return;
    }

    try {
      isSaving = true;
      errorMsg = '';
      const userId = $currentUser?.id || 1;
      const sessionId = paymentMethod === 'cash' ? ($activeSession?.id || null) : null;

      await invoke('add_expense', {
        categoryId,
        amount,
        paymentMethod,
        sessionId,
        userId,
        recipient: recipient || null,
        receiptReference: receiptRef || null,
        notes: notes || null,
      });

      isAddOpen = false;
      amount = 0;
      recipient = '';
      receiptRef = '';
      notes = '';
      await loadExpenses();
    } catch (e: any) {
      errorMsg = typeof e === 'string' ? e : e.message || 'Failed to record expense';
    } finally {
      isSaving = false;
    }
  }

  async function handleDeleteExpense(id: number) {
    if (!confirm('Are you sure you want to delete this expense voucher?')) return;
    try {
      await invoke('delete_expense', { expenseId: id });
      await loadExpenses();
    } catch (e) {
      console.error(e);
    }
  }

  function printExpenseVoucher(exp: Expense) {
    const html = `
      <div class="text-center pb-2 border-b-dashed">
        <h2 class="font-black text-sm uppercase">TitaouPOS Superette</h2>
        <p class="text-xxs">Rue Principale, Alger • Tél: 0553444057</p>
        <p class="font-black text-xs mt-1 bg-black text-white px-1">BON DE DECAISSEMENT / سند صرف</p>
        <p class="text-xxs mt-0.5">${exp.date || new Date().toLocaleString()}</p>
      </div>

      <div class="py-2 border-b-dashed text-xxs space-y-1">
        <div class="flex justify-between"><span>Numéro:</span><strong>#${exp.expense_number}</strong></div>
        <div class="flex justify-between"><span>Bénéficiaire (المستفيد):</span><strong>${exp.recipient || 'Divers'}</strong></div>
        <div class="flex justify-between"><span>Catégorie:</span><strong>${exp.category_name || 'Général'}</strong></div>
        <div class="flex justify-between"><span>Mode de paiement:</span><strong>${exp.payment_method.toUpperCase()}</strong></div>
        ${exp.notes ? `<div class="flex justify-between"><span>Motif:</span><strong>${exp.notes}</strong></div>` : ''}
      </div>

      <div class="py-3 border-b-dashed text-center">
        <p class="text-xs text-gray-600 font-bold">MONTANT SORTIE (المبلغ المصروف):</p>
        <p class="text-2xl font-black font-mono text-black mt-1">${exp.amount.toLocaleString()} DZD</p>
      </div>

      <div class="py-2 text-xxs flex justify-between">
        <span>Signature Caissier: _______</span>
        <span>Signature Bénéficiaire: _______</span>
      </div>

      <div class="text-center pt-2 border-t-dashed text-[8px] text-gray-400">
        TitaouPOS • Created by Titaou Bedreddine 0553444057
      </div>
    `;

    printHtmlDirectly(html, `Voucher - ${exp.expense_number}`);
  }
</script>

<div class="h-full flex flex-col bg-pos-bg p-4 overflow-hidden select-none">
  <!-- Header -->
  <div class="flex items-center justify-between pb-4 border-b border-pos-border shrink-0">
    <div class="flex items-center gap-3">
      <div class="w-10 h-10 rounded-2xl bg-rose-100 dark:bg-rose-950 text-rose-600 flex items-center justify-center font-bold">
        <TrendingDown class="w-5 h-5" />
      </div>
      <div>
        <h1 class="text-xl font-black text-pos-text tracking-tight">Expenses & Operating Costs / المصاريف والتكاليف</h1>
        <p class="text-xs text-pos-muted">Track store overhead and deduct directly from active cash drawer</p>
      </div>
    </div>

    <div class="flex items-center gap-2">
      <button
        type="button"
        on:click={() => { isAddOpen = true; errorMsg = ''; }}
        class="px-4 py-2 bg-rose-600 hover:bg-rose-700 text-white rounded-xl text-xs font-black transition shadow-xs flex items-center gap-2 cursor-pointer active:scale-95"
      >
        <Plus class="w-4 h-4" />
        <span>Add Expense (تسجيل مصروف جديد)</span>
      </button>
    </div>
  </div>

  <!-- Expenses Table -->
  <div class="flex-1 overflow-y-auto mt-4 bg-pos-card border border-pos-border rounded-2xl shadow-xs">
    <table class="w-full text-start text-xs border-collapse">
      <thead class="bg-slate-50 dark:bg-slate-800/60 border-b border-pos-border text-pos-muted font-bold sticky top-0 z-10">
        <tr>
          <th class="p-3 text-start">Expense #</th>
          <th class="p-3 text-start">Date</th>
          <th class="p-3 text-start">Category / الفئة</th>
          <th class="p-3 text-start">Recipient / المستفيد</th>
          <th class="p-3 text-start">Payment Method</th>
          <th class="p-3 text-end">Amount (المبلغ)</th>
          <th class="p-3 text-end">Actions</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-pos-border/40">
        {#if expenses.length === 0}
          <tr>
            <td colspan="7" class="p-8 text-center text-pos-muted">No expenses recorded yet.</td>
          </tr>
        {:else}
          {#each expenses as exp}
            <tr class="hover:bg-slate-50 dark:hover:bg-slate-800/40 transition">
              <td class="p-3 font-mono font-bold text-sky-600">#{exp.expense_number}</td>
              <td class="p-3 font-mono text-pos-muted">{exp.date}</td>
              <td class="p-3 font-bold text-pos-text">{exp.category_name || 'Général'}</td>
              <td class="p-3 text-pos-muted">{exp.recipient || '—'}</td>
              <td class="p-3 uppercase font-mono font-bold">{exp.payment_method}</td>
              <td class="p-3 text-end font-mono font-black text-rose-600">{exp.amount.toLocaleString()} DZD</td>
              <td class="p-3 text-end">
                <div class="flex items-center justify-end gap-1">
                  <button on:click={() => printExpenseVoucher(exp)} class="p-1.5 text-pos-muted hover:text-sky-600 rounded-lg cursor-pointer" title="Print Voucher">
                    <Printer class="w-4 h-4" />
                  </button>
                  <button on:click={() => (previewExpense = exp)} class="p-1.5 text-pos-muted hover:text-sky-600 rounded-lg cursor-pointer" title="Preview">
                    <Eye class="w-4 h-4" />
                  </button>
                  <button on:click={() => handleDeleteExpense(exp.id)} class="p-1.5 text-pos-muted hover:text-rose-600 rounded-lg cursor-pointer" title="Delete">
                    <Trash2 class="w-4 h-4" />
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

<!-- Modal: Add Expense -->
{#if isAddOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl w-full max-w-lg overflow-hidden animate-in zoom-in-95 duration-150 flex flex-col">
      <div class="flex items-center justify-between px-6 py-4 border-b border-pos-border bg-slate-50 dark:bg-slate-800/60">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-2xl bg-rose-600/10 text-rose-600 flex items-center justify-center font-bold">
            <TrendingDown class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-black text-base text-pos-text">Record Expense / تسجيل مصروف</h3>
            <p class="text-xs text-pos-muted">Deducts automatically from active cash drawer</p>
          </div>
        </div>
        <button on:click={() => (isAddOpen = false)} class="text-pos-muted hover:text-pos-text p-1.5 rounded-xl cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>

      {#if errorMsg}
        <div class="mx-6 mt-4 p-3 bg-rose-100 text-rose-800 text-xs font-bold rounded-xl">{errorMsg}</div>
      {/if}

      <div class="p-6 space-y-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Expense Amount / المبلغ (DZD) *</label>
          <input type="number" min="1" bind:value={amount} placeholder="Ex: 5000" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-base font-mono font-black text-rose-600 outline-none focus:ring-2 focus:ring-rose-500" />
        </div>

        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Category / الفئة</label>
            <select bind:value={categoryId} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none">
              <option value={1}>Loyer / Rent (إيجار)</option>
              <option value={2}>Électricité / Gaz (كهرباء وغاز)</option>
              <option value={3}>Transport / Livraison (نقل وشحن)</option>
              <option value={4}>Maintenance / Reparation (صيانة)</option>
              <option value={5}>Salaires / Payroll (رواتب)</option>
              <option value={6}>Divers / Other (مصاريف أخرى)</option>
            </select>
          </div>

          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Payment Method</label>
            <select bind:value={paymentMethod} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none">
              <option value="cash">Cash (الصندوق)</option>
              <option value="bank">Banque / Virement</option>
              <option value="check">Chèque</option>
            </select>
          </div>
        </div>

        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Beneficiary / Recipient (المستفيد)</label>
          <input type="text" bind:value={recipient} placeholder="Ex: Sonelgaz, Propriétaire, Chauffeur" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none" />
        </div>

        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Notes / Description (ملاحظات)</label>
          <input type="text" bind:value={notes} placeholder="Ex: Facture électricité Janvier" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none" />
        </div>
      </div>

      <div class="px-6 py-4 border-t border-pos-border bg-slate-50 dark:bg-slate-800/60 flex items-center justify-end gap-2">
        <button on:click={() => (isAddOpen = false)} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-pos-text font-bold text-xs rounded-xl cursor-pointer">Cancel</button>
        <button on:click={handleAddExpense} disabled={isSaving} class="px-6 py-2 bg-rose-600 hover:bg-rose-700 disabled:opacity-50 text-white font-black text-xs rounded-xl shadow-md cursor-pointer flex items-center gap-1.5">
          <Check class="w-4 h-4" />
          <span>{isSaving ? 'Saving...' : 'Save Expense (حفظ المصروف)'}</span>
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Modal: Preview Expense -->
{#if previewExpense}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl w-full max-w-md overflow-hidden animate-in zoom-in-95 duration-150 flex flex-col">
      <div class="flex items-center justify-between px-6 py-4 border-b border-pos-border bg-slate-50 dark:bg-slate-800/60">
        <h3 class="font-black text-base text-pos-text">Expense #{previewExpense.expense_number}</h3>
        <button on:click={() => (previewExpense = null)} class="text-pos-muted hover:text-pos-text p-1.5 rounded-xl cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="p-6 space-y-3 text-xs">
        <div class="flex justify-between"><span class="text-pos-muted">Date:</span><span class="font-mono font-bold text-pos-text">{previewExpense.date}</span></div>
        <div class="flex justify-between"><span class="text-pos-muted">Category:</span><span class="font-bold text-pos-text">{previewExpense.category_name || 'Général'}</span></div>
        <div class="flex justify-between"><span class="text-pos-muted">Recipient:</span><span class="font-bold text-pos-text">{previewExpense.recipient || '—'}</span></div>
        <div class="flex justify-between"><span class="text-pos-muted">Payment Method:</span><span class="uppercase font-mono">{previewExpense.payment_method}</span></div>
        <div class="flex justify-between"><span class="text-pos-muted">Notes:</span><span class="text-pos-text">{previewExpense.notes || '—'}</span></div>
        <div class="flex justify-between pt-2 border-t border-pos-border"><span class="font-bold text-pos-muted">Amount:</span><span class="text-lg font-black font-mono text-rose-600">{previewExpense.amount.toLocaleString()} DZD</span></div>
      </div>

      <div class="px-6 py-4 border-t border-pos-border bg-slate-50 dark:bg-slate-800/60 flex items-center justify-between">
        <button on:click={() => printExpenseVoucher(previewExpense)} class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-xs">
          <Printer class="w-4 h-4" />
          <span>Print Voucher (طباعة سند)</span>
        </button>
        <button on:click={() => (previewExpense = null)} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-pos-text font-bold text-xs rounded-xl cursor-pointer">Close</button>
      </div>
    </div>
  </div>
{/if}