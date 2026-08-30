<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { CashMovement, CashSession, Customer } from '../../lib/types';
  import { activeSession } from '../../lib/stores/session';
  import { currentUser } from '../../lib/stores/auth';
  import { DollarSign, ArrowDownCircle, ArrowUpCircle, Lock, RefreshCw, Plus, CheckCircle, Check, Search, Wallet, TrendingUp, ArrowDownRight, Layers, Banknote, Wallet as WalletIcon } from 'lucide-svelte';
  import DateQuickFilters from '../../lib/components/DateQuickFilters.svelte';

  let currentTab: 'current' | 'history' = 'current';
  let movements: CashMovement[] = [];
  let historySessions: CashSession[] = [];

  let isDepositOpen = false;
  let isWithdrawOpen = false;
  let isCloseOpen = false;
  let isStartupOpen = false;
  let startupAmount = 10000;
  let startupReason = 'Startup Cash / رصيد افتتاحي';

  let amount = 0;
  let reason = '';
  let countedCash = 0;
  let closeNotes = '';

  // History Filter
  let fromDate = '';
  let toDate = '';

  // Debt & versement KPIs for the register page
  let totalUnpaidDebt = 0;
  let totalPaidDebt = 0;
  let totalVersement = 0;

  async function loadDebtKpis() {
    try {
      const customers = await invoke<Customer[]>('list_customers');
      // Unpaid debt = sum of what customers still owe.
      totalUnpaidDebt = customers.reduce((s, c) => s + Math.max(0, c.balance || 0), 0);
      // Paid debt: debt repayments recorded in the active session.
      totalPaidDebt = $activeSession
        ? movements
            .filter((m: any) => m.type === 'customer_debt_payment')
            .reduce((s: number, m: any) => s + Math.abs(m.amount || 0), 0)
        : 0;
      // Versement: remaining unpaid layaway balances today.
      try {
        const sales = await invoke<any[]>('list_sales', {
          startDate: new Date().toISOString().split('T')[0],
          endDate: new Date().toISOString().split('T')[0],
          userId: null,
          limit: 500,
        });
        totalVersement = sales
          .filter((s: any) => s.payment_method === 'versement')
          .reduce((sum: number, s: any) => sum + Math.max(0, (s.total_amount || 0) - (s.paid_amount || 0)), 0);
      } catch {
        totalVersement = 0;
      }
    } catch (e) {
      console.warn('Debt KPIs unavailable:', e);
    }
  }

  onMount(async () => {
    await loadData();
    await loadDebtKpis();
  });

  async function loadData() {
    if (!$currentUser) return;
    try {
      const active = await invoke<CashSession | null>('get_active_cash_session', { userId: $currentUser.id });
      $activeSession = active;
      if (active) {
        movements = await invoke<CashMovement[]>('list_cash_movements', { sessionId: active.id });
      }
      historySessions = await invoke<CashSession[]>('list_session_history');
    } catch (e) {
      console.error(e);
    }
  }

  async function handleDeposit() {
    if (!$activeSession || !$currentUser || amount <= 0) return;
    try {
      await invoke('add_cash_movement', {
        sessionId: $activeSession.id,
        userId: $currentUser.id,
        movementType: 'cash_in',
        amount,
        reason: reason || 'Cash Deposit / إيداع نقدي',
      });
      isDepositOpen = false;
      amount = 0;
      reason = '';
      await loadData();
    } catch (e) {
      console.error(e);
    }
  }

  async function handleWithdraw() {
    if (!$activeSession || !$currentUser || amount <= 0) return;
    try {
      await invoke('add_cash_movement', {
        sessionId: $activeSession.id,
        userId: $currentUser.id,
        movementType: 'cash_out',
        amount,
        reason: reason || 'Cash Withdrawal / سحب نقدي',
      });
      isWithdrawOpen = false;
      amount = 0;
      reason = '';
      await loadData();
    } catch (e) {
      console.error(e);
    }
  }

  async function handleCloseSession() {
    if (!$activeSession) return;
    try {
      await invoke('close_cash_session', {
        sessionId: $activeSession.id,
        actualCash: countedCash,
        notes: closeNotes || null,
      });
      isCloseOpen = false;
      $activeSession = null;
      await loadData();
    } catch (e) {
      console.error(e);
    }
  }

  async function handleStartupSession() {
    if (!$currentUser) return;
    try {
      const session = await invoke<any>('open_cash_session', {
        userId: $currentUser.id,
        registerId: 1,
        openingAmount: startupAmount,
        notes: startupReason || 'Startup Cash / رصيد افتتاحي',
      });
      $activeSession = session;
      isStartupOpen = false;
      await loadData();
    } catch (e: any) {
      console.error(e);
      alert('Failed to open session: ' + (e.message || e));
    }
  }
</script>

<div class="p-6 space-y-6 overflow-y-auto h-full select-none">
  <!-- Header with Tab Switcher -->
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-black text-pos-text">Cash Register Management (إدارة الصندوق)</h1>
      <p class="text-xs text-pos-muted mt-1">Monitor daily cash sessions and register transactions</p>
    </div>

    <!-- Toggle Buttons matching screenshot -->
    <div class="flex items-center bg-slate-100 dark:bg-slate-800 p-1 rounded-xl border border-pos-border">
      <button
        on:click={() => currentTab = 'current'}
        class="px-4 py-2 rounded-lg text-xs font-bold transition cursor-pointer {currentTab === 'current' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:text-pos-text'}"
      >
        Current Session
      </button>
      <button
        on:click={() => currentTab = 'history'}
        class="px-4 py-2 rounded-lg text-xs font-bold transition cursor-pointer {currentTab === 'history' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:text-pos-text'}"
      >
        Session History
      </button>
    </div>
  </div>

  {#if currentTab === 'current'}
    {#if $activeSession}
      <!-- Active Session Banner -->
      <div class="bg-emerald-50 dark:bg-emerald-950/40 border border-emerald-300 dark:border-emerald-800 rounded-2xl p-4 flex items-center justify-between shadow-xs">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-full bg-emerald-500 text-white flex items-center justify-center font-bold">
            <CheckCircle class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-extrabold text-sm text-emerald-900 dark:text-emerald-200">
              Active Session #{$activeSession.id} — {$activeSession.user_name || 'Cashier'}
            </h3>
            <p class="text-xs text-emerald-800 dark:text-emerald-300 mt-0.5 font-medium">
              Opened Since {$activeSession.opened_at}
            </p>
          </div>
        </div>

        <button
          on:click={() => { countedCash = $activeSession?.expected_cash || 0; isCloseOpen = true; }}
          class="px-5 py-2.5 bg-rose-600 hover:bg-rose-700 text-white font-extrabold text-xs rounded-xl transition shadow-sm flex items-center gap-2 cursor-pointer"
        >
          <Lock class="w-4 h-4" />
          <span>Close Session</span>
        </button>
      </div>

      <!-- Metric Cards matching screenshot -->
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div class="bg-pos-card border border-pos-border rounded-2xl p-4 shadow-xs">
          <span class="text-xs font-bold text-pos-muted flex items-center gap-1.5 mb-2">
            <Wallet class="w-4 h-4 text-sky-500" />
            <span>Opening Balance</span>
          </span>
          <div class="text-2xl font-black font-mono text-pos-text">
            {$activeSession.opening_amount.toLocaleString()} DZD
          </div>
        </div>

        <div class="bg-pos-card border border-pos-border rounded-2xl p-4 shadow-xs">
          <span class="text-xs font-bold text-pos-muted flex items-center gap-1.5 mb-2">
            <TrendingUp class="w-4 h-4 text-emerald-500" />
            <span>Total Sales</span>
          </span>
          <div class="text-2xl font-black font-mono text-emerald-600">
            {($activeSession.total_sales || 0).toLocaleString()} DZD
          </div>
        </div>

        <div class="bg-pos-card border border-pos-border rounded-2xl p-4 shadow-xs">
          <span class="text-xs font-bold text-pos-muted flex items-center gap-1.5 mb-2">
            <ArrowDownRight class="w-4 h-4 text-rose-500" />
            <span>Total Expenses</span>
          </span>
          <div class="text-2xl font-black font-mono text-rose-600">
            {($activeSession.total_expenses || 0).toLocaleString()} DZD
          </div>
        </div>

        <div class="bg-sky-600 text-white rounded-2xl p-4 shadow-md">
          <span class="text-xs font-bold text-sky-100 flex items-center gap-1.5 mb-2">
            <DollarSign class="w-4 h-4 text-white" />
            <span>Current Balance</span>
          </span>
          <div class="text-2xl font-black font-mono">
            {$activeSession.expected_cash.toLocaleString()} DZD
          </div>
        </div>
      </div>

      <!-- Debt & Versement Cards -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div class="bg-pos-card border border-rose-200 dark:border-rose-800/60 rounded-2xl p-4 shadow-xs">
          <span class="text-xs font-bold text-pos-muted flex items-center gap-1.5 mb-2">
            <Layers class="w-4 h-4 text-rose-500" />
            <span>Unpaid Customer Debt (دين غير مسدد)</span>
          </span>
          <div class="text-2xl font-black font-mono text-rose-600">
            {totalUnpaidDebt.toLocaleString()} DZD
          </div>
        </div>

        <div class="bg-pos-card border border-emerald-200 dark:border-emerald-800/60 rounded-2xl p-4 shadow-xs">
          <span class="text-xs font-bold text-pos-muted flex items-center gap-1.5 mb-2">
            <CheckCircle class="w-4 h-4 text-emerald-500" />
            <span>Debt Paid This Session (تسديدات)</span>
          </span>
          <div class="text-2xl font-black font-mono text-emerald-600">
            {totalPaidDebt.toLocaleString()} DZD
          </div>
        </div>

        <div class="bg-pos-card border border-violet-200 dark:border-violet-800/60 rounded-2xl p-4 shadow-xs">
          <span class="text-xs font-bold text-pos-muted flex items-center gap-1.5 mb-2">
            <WalletIcon class="w-4 h-4 text-violet-500" />
            <span>Versement Remaining (تسبقة متبقية)</span>
          </span>
          <div class="text-2xl font-black font-mono text-violet-600">
            {totalVersement.toLocaleString()} DZD
          </div>
        </div>
      </div>

      <!-- Action Buttons Row matching screenshot -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
        <button
          on:click={() => isDepositOpen = true}
          class="p-3.5 bg-emerald-50 dark:bg-emerald-950/40 hover:bg-emerald-100 border border-emerald-300 dark:border-emerald-800 text-emerald-700 dark:text-emerald-300 rounded-2xl text-xs font-extrabold flex items-center justify-center gap-2 transition cursor-pointer shadow-xs"
        >
          <ArrowDownCircle class="w-4 h-4 text-emerald-600" />
          <span>Cash Deposit (إيداع نقدي)</span>
        </button>

        <button
          on:click={() => isWithdrawOpen = true}
          class="p-3.5 bg-amber-50 dark:bg-amber-950/40 hover:bg-amber-100 border border-amber-300 dark:border-amber-800 text-amber-700 dark:text-amber-300 rounded-2xl text-xs font-extrabold flex items-center justify-center gap-2 transition cursor-pointer shadow-xs"
        >
          <ArrowUpCircle class="w-4 h-4 text-amber-600" />
          <span>Cash Withdrawal (سحب نقدي)</span>
        </button>

        <button
          on:click={() => isWithdrawOpen = true}
          class="p-3.5 bg-rose-50 dark:bg-rose-950/40 hover:bg-rose-100 border border-rose-300 dark:border-rose-800 text-rose-700 dark:text-rose-300 rounded-2xl text-xs font-extrabold flex items-center justify-center gap-2 transition cursor-pointer shadow-xs"
        >
          <Plus class="w-4 h-4 text-rose-600" />
          <span>Add Expense (مصروف من الصندوق)</span>
        </button>
      </div>

      <!-- Transaction Log Table matching screenshot -->
      <div class="bg-pos-card border border-pos-border rounded-2xl shadow-xs overflow-hidden">
        <div class="p-4 border-b border-pos-border flex items-center justify-between bg-slate-50 dark:bg-slate-800/40">
          <h3 class="font-extrabold text-xs text-pos-text">Transaction Log</h3>
          <button on:click={loadData} class="p-1 text-pos-muted hover:text-pos-text cursor-pointer">
            <RefreshCw class="w-4 h-4" />
          </button>
        </div>

        <table class="w-full text-start text-xs border-collapse">
          <thead>
            <tr class="border-b border-pos-border text-pos-muted font-bold">
              <th class="p-3 text-start">Time</th>
              <th class="p-3 text-start">Type</th>
              <th class="p-3 text-start">Description</th>
              <th class="p-3 text-end">Amount</th>
            </tr>
          </thead>
          <tbody>
            {#if movements.length === 0}
              <tr>
                <td colspan="4" class="p-8 text-center text-pos-muted">لا توجد حركات بعد</td>
              </tr>
            {:else}
              {#each movements as m}
                <tr class="border-b border-pos-border/60 hover:bg-slate-50 dark:hover:bg-slate-800/40">
                  <td class="p-3 text-pos-muted font-mono">{m.created_at}</td>
                  <td class="p-3 font-bold uppercase text-xs">{m.type_name}</td>
                  <td class="p-3 text-pos-text font-semibold">{m.reason || '-'}</td>
                  <td class="p-3 text-end font-mono font-bold text-sm {m.amount < 0 ? 'text-rose-600' : 'text-emerald-600'}">
                    {m.amount.toLocaleString()} DZD
                  </td>
                </tr>
              {/each}
            {/if}
          </tbody>
        </table>
      </div>
    {:else}
      <div class="p-12 text-center bg-pos-card border border-pos-border rounded-2xl shadow-sm max-w-md mx-auto my-8">
        <div class="w-14 h-14 rounded-2xl bg-amber-50 dark:bg-amber-950/40 text-amber-600 flex items-center justify-center mx-auto mb-4 font-bold">
          <Lock class="w-7 h-7" />
        </div>
        <h3 class="text-base font-black text-pos-text mb-1">No Active Session (الصندوق مغلق)</h3>
        <p class="text-xs font-bold text-pos-muted mb-5">Open a new cash register session to begin registering sales and movements.</p>
        <button
          type="button"
          on:click={() => { startupAmount = 10000; startupReason = 'Startup Cash / رصيد افتتاحي'; isStartupOpen = true; }}
          class="px-6 py-3 bg-emerald-600 hover:bg-emerald-700 text-white font-black text-xs rounded-xl shadow-md cursor-pointer transition active:scale-95 flex items-center gap-2 mx-auto"
        >
          <Plus class="w-4 h-4" />
          <span>Open New Session (فتح صندوق جديد)</span>
        </button>
      </div>
    {/if}
  {:else}
    <!-- Session History matching screenshot -->
    <div class="bg-pos-card border border-pos-border rounded-2xl p-4 shadow-xs space-y-4">
      <DateQuickFilters bind:startDate={fromDate} bind:endDate={toDate} onChange={loadData} />
      <div class="grid grid-cols-1 md:grid-cols-4 gap-3 items-end">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">From</label>
          <input type="date" bind:value={fromDate} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs text-pos-text font-bold" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">To</label>
          <input type="date" bind:value={toDate} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs text-pos-text font-bold" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Employee</label>
          <select class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs text-pos-text font-bold">
            <option>جميع الموظفين</option>
          </select>
        </div>
        <button class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-lg flex items-center justify-center gap-1.5 cursor-pointer">
          <Search class="w-4 h-4" />
          <span>Search</span>
        </button>
      </div>

      <table class="w-full text-start text-xs border-collapse">
        <thead>
          <tr class="border-b border-pos-border text-pos-muted font-bold bg-slate-50 dark:bg-slate-800/40">
            <th class="p-3 text-start">#</th>
            <th class="p-3 text-start">Employee</th>
            <th class="p-3 text-start">Opened At</th>
            <th class="p-3 text-start">Closed At</th>
            <th class="p-3 text-end">Opening</th>
            <th class="p-3 text-end">Closing</th>
            <th class="p-3 text-end">Expected</th>
            <th class="p-3 text-end">Difference</th>
            <th class="p-3 text-center">Status</th>
          </tr>
        </thead>
        <tbody>
          {#each historySessions as s}
            <tr class="border-b border-pos-border/60 hover:bg-slate-50 dark:hover:bg-slate-800/40">
              <td class="p-3 font-mono font-bold text-pos-muted">{s.id}</td>
              <td class="p-3 font-bold text-pos-text">{s.user_name || 'Admin'}</td>
              <td class="p-3 font-mono text-pos-muted">{s.opened_at}</td>
              <td class="p-3 font-mono text-pos-muted">{s.closed_at || '-'}</td>
              <td class="p-3 text-end font-mono font-bold">{s.opening_amount.toLocaleString()}</td>
              <td class="p-3 text-end font-mono font-bold">{s.actual_cash !== null && s.actual_cash !== undefined ? s.actual_cash.toLocaleString() : '-'}</td>
              <td class="p-3 text-end font-mono font-bold">{s.expected_cash.toLocaleString()}</td>
              <td class="p-3 text-end font-mono font-bold {s.difference && s.difference < 0 ? 'text-rose-600' : 'text-emerald-600'}">
                {s.difference !== null && s.difference !== undefined ? s.difference.toLocaleString() : '-'}
              </td>
              <td class="p-3 text-center">
                <span class="px-2 py-0.5 rounded-full text-[11px] font-bold {s.status === 'open' ? 'bg-emerald-100 text-emerald-800' : 'bg-slate-200 text-slate-800'}">
                  {s.status === 'open' ? 'مفتوحة' : 'مغلقة'}
                </span>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}

  <!-- Modals -->
  {#if isDepositOpen}
    <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
      <div class="bg-pos-card border border-pos-border rounded-xl p-5 w-full max-w-sm space-y-3">
        <h3 class="font-extrabold text-sm text-pos-text">Cash Deposit (إيداع نقدي)</h3>
        <input type="number" bind:value={amount} on:focus={(e) => (e.target as HTMLInputElement).select()} placeholder="Amount DZD" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-lg font-mono font-bold text-pos-text" />
        <input type="text" bind:value={reason} placeholder="Reason / Notes" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs text-pos-text" />
        <div class="flex justify-end gap-2 pt-2">
          <button on:click={() => isDepositOpen = false} class="px-3 py-1.5 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded">Cancel</button>
          <button on:click={handleDeposit} class="px-4 py-1.5 bg-emerald-600 text-white text-xs font-bold rounded">Confirm</button>
        </div>
      </div>
    </div>
  {/if}

  {#if isWithdrawOpen}
    <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
      <div class="bg-pos-card border border-pos-border rounded-xl p-5 w-full max-w-sm space-y-3">
        <h3 class="font-extrabold text-sm text-pos-text">Cash Withdrawal (سحب نقدي / مصروف)</h3>
        <input type="number" bind:value={amount} on:focus={(e) => (e.target as HTMLInputElement).select()} placeholder="Amount DZD" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-lg font-mono font-bold text-pos-text" />
        <input type="text" bind:value={reason} placeholder="Reason / Notes" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs text-pos-text" />
        <div class="flex justify-end gap-2 pt-2">
          <button on:click={() => isWithdrawOpen = false} class="px-3 py-1.5 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded">Cancel</button>
          <button on:click={handleWithdraw} class="px-4 py-1.5 bg-amber-600 text-white text-xs font-bold rounded">Confirm</button>
        </div>
      </div>
    </div>
  {/if}

  {#if isCloseOpen}
    <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
      <div class="bg-pos-card border border-pos-border rounded-xl p-5 w-full max-w-sm space-y-3">
        <h3 class="font-extrabold text-sm text-pos-text">Close Cash Session (إغلاق الصندوق)</h3>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Counted Cash in Drawer (DZD)</label>
          <input type="number" bind:value={countedCash} on:focus={(e) => (e.target as HTMLInputElement).select()} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xl font-mono font-bold text-pos-text" />
        </div>
        <input type="text" bind:value={closeNotes} placeholder="Closing notes..." class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs text-pos-text" />
        <div class="flex justify-end gap-2 pt-2">
          <button on:click={() => isCloseOpen = false} class="px-3 py-1.5 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded">Cancel</button>
          <button on:click={handleCloseSession} class="px-4 py-1.5 bg-rose-600 text-white text-xs font-bold rounded">Close Register</button>
        </div>
      </div>
    </div>
  {/if}

  {#if isStartupOpen}
    <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
      <div class="bg-pos-card border border-pos-border rounded-2xl p-6 w-full max-w-sm space-y-4 shadow-2xl animate-in zoom-in-95 duration-150">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-xl bg-emerald-100 text-emerald-700 flex items-center justify-center font-bold">
            <DollarSign class="w-6 h-6" />
          </div>
          <div>
            <h3 class="font-black text-sm text-pos-text">Open New Cash Session</h3>
            <p class="text-[11px] text-pos-muted">افتتاح صندوق جديد</p>
          </div>
        </div>

        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Opening Cash Amount (DZD) *</label>
          <input
            type="number"
            bind:value={startupAmount}
            on:focus={(e) => (e.target as HTMLInputElement).select()}
            min="0"
            class="w-full px-3 py-2.5 bg-slate-100 dark:bg-slate-800 border-2 border-emerald-500/40 focus:border-emerald-500 rounded-xl text-lg font-mono font-black text-pos-text outline-none"
          />
        </div>

        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Notes / Reason (ملاحظات)</label>
          <input
            type="text"
            bind:value={startupReason}
            class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none"
          />
        </div>

        <div class="flex justify-end gap-2 pt-2 border-t border-pos-border">
          <button
            type="button"
            on:click={() => isStartupOpen = false}
            class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-xl cursor-pointer"
          >
            Cancel
          </button>
          <button
            type="button"
            on:click={handleStartupSession}
            class="px-5 py-2 bg-emerald-600 hover:bg-emerald-700 text-white text-xs font-black rounded-xl shadow-md cursor-pointer flex items-center gap-1.5"
          >
            <Check class="w-4 h-4" />
            <span>Open Session (فتح الصندوق)</span>
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>