<script lang="ts">
  import { t } from '../i18n';
  import { invoke } from '@tauri-apps/api/core';
  import { activeSession } from '../stores/session';
  import { currentUser } from '../stores/auth';
  import { DollarSign, ArrowDownCircle, ArrowUpCircle, Lock, X, Check } from 'lucide-svelte';

  export let isOpen = false;
  export let onClose: () => void;

  let mode: 'startup' | 'in' | 'out' | 'close' = 'in';
  let amount = 0;
  let reason = '';
  let countedCash = 0;
  let errorMsg = '';
  let isSubmitting = false;

  async function handleAction() {
    if (!$currentUser) return;
    try {
      isSubmitting = true;
      errorMsg = '';

      if (mode === 'startup') {
        const session = await invoke<any>('open_cash_session', {
          userId: $currentUser.id,
          registerId: 1,
          openingAmount: amount,
          notes: reason || null,
        });
        $activeSession = session;
        onClose();
      } else if (mode === 'in' || mode === 'out') {
        if (!$activeSession) return;
        await invoke('add_cash_movement', {
          sessionId: $activeSession.id,
          userId: $currentUser.id,
          movementType: mode === 'in' ? 'cash_in' : 'cash_out',
          amount,
          reason: reason || (mode === 'in' ? 'Cash In' : 'Cash Out'),
        });
        // Refresh session
        const updated = await invoke<any>('get_active_cash_session', { userId: $currentUser.id });
        $activeSession = updated;
        onClose();
      } else if (mode === 'close') {
        if (!$activeSession) return;
        await invoke('close_cash_session', {
          sessionId: $activeSession.id,
          actualCash: countedCash,
          notes: reason || null,
        });
        $activeSession = null;
        onClose();
      }
    } catch (err: any) {
      errorMsg = typeof err === 'string' ? err : err.message || 'Operation failed';
    } finally {
      isSubmitting = false;
    }
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-xl shadow-2xl w-full max-w-md overflow-hidden">
      <!-- Header -->
      <div class="flex items-center justify-between px-5 py-3.5 border-b border-pos-border bg-slate-50 dark:bg-slate-800/50">
        <h3 class="font-extrabold text-base text-pos-text flex items-center gap-2">
          <DollarSign class="w-5 h-5 text-amber-500" />
          <span>{t('btn_drawer')} ({$activeSession ? 'Session #' + $activeSession.id : 'No Active Session'})</span>
        </h3>
        <button on:click={onClose} class="text-pos-muted hover:text-pos-text p-1 rounded">
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Content -->
      <div class="p-5 space-y-4">
        {#if errorMsg}
          <div class="p-3 bg-rose-100 text-rose-700 text-xs font-bold rounded">{errorMsg}</div>
        {/if}

        {#if !$activeSession}
          <!-- Prompt Startup Money -->
          <div class="p-3 bg-amber-50 dark:bg-amber-950/40 border border-amber-300 dark:border-amber-800 rounded text-xs text-amber-800 dark:text-amber-300">
            Open cash session to start sales. Enter opening startup balance.
          </div>
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">{t('startup_cash')} (DZD)</label>
            <input
              type="number"
              bind:value={amount}
              class="w-full px-3 py-2 bg-pos-card border border-pos-border rounded text-xl font-bold font-mono text-pos-text outline-none focus:ring-2 focus:ring-amber-500"
            />
          </div>
        {:else}
          <!-- Mode Tabs -->
          <div class="grid grid-cols-3 gap-2">
            <button
              type="button"
              on:click={() => { mode = 'in'; amount = 0; }}
              class="p-2.5 rounded border text-xs font-bold flex flex-col items-center gap-1 transition cursor-pointer {mode === 'in' ? 'border-emerald-500 bg-emerald-50 dark:bg-emerald-950/40 text-emerald-600' : 'border-pos-border text-pos-muted'}"
            >
              <ArrowDownCircle class="w-4 h-4" />
              <span>{t('cash_in')}</span>
            </button>

            <button
              type="button"
              on:click={() => { mode = 'out'; amount = 0; }}
              class="p-2.5 rounded border text-xs font-bold flex flex-col items-center gap-1 transition cursor-pointer {mode === 'out' ? 'border-rose-500 bg-rose-50 dark:bg-rose-950/40 text-rose-600' : 'border-pos-border text-pos-muted'}"
            >
              <ArrowUpCircle class="w-4 h-4" />
              <span>{t('cash_out')}</span>
            </button>

            <button
              type="button"
              on:click={() => { mode = 'close'; countedCash = $activeSession?.expected_cash || 0; }}
              class="p-2.5 rounded border text-xs font-bold flex flex-col items-center gap-1 transition cursor-pointer {mode === 'close' ? 'border-amber-500 bg-amber-50 dark:bg-amber-950/40 text-amber-600' : 'border-pos-border text-pos-muted'}"
            >
              <Lock class="w-4 h-4" />
              <span>{t('close_session')}</span>
            </button>
          </div>

          <!-- Expected Cash Display -->
          <div class="bg-slate-100 dark:bg-slate-800 p-3 rounded-lg border border-pos-border flex items-center justify-between">
            <span class="text-xs font-bold text-pos-muted">{t('expected_cash')}</span>
            <span class="text-xl font-black font-mono text-pos-text">{$activeSession.expected_cash.toLocaleString()} DZD</span>
          </div>

          {#if mode === 'in' || mode === 'out'}
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Amount (DZD)</label>
              <input
                type="number"
                bind:value={amount}
                class="w-full px-3 py-2 bg-pos-card border border-pos-border rounded text-lg font-bold font-mono text-pos-text outline-none focus:ring-2 focus:ring-sky-500"
              />
            </div>
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Reason / Notes</label>
              <input
                type="text"
                bind:value={reason}
                placeholder="e.g. Change replenishment or petty expense"
                class="w-full px-3 py-2 bg-pos-card border border-pos-border rounded text-xs text-pos-text outline-none focus:ring-2 focus:ring-sky-500"
              />
            </div>
          {:else if mode === 'close'}
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">{t('actual_cash')} (Counted in Drawer)</label>
              <input
                type="number"
                bind:value={countedCash}
                class="w-full px-3 py-2 bg-pos-card border border-pos-border rounded text-xl font-bold font-mono text-pos-text outline-none focus:ring-2 focus:ring-amber-500"
              />
            </div>
            <div class="flex items-center justify-between p-2.5 bg-slate-100 dark:bg-slate-800 rounded text-xs font-bold">
              <span>{t('difference')}</span>
              <span class="font-mono {countedCash - $activeSession.expected_cash < 0 ? 'text-rose-500' : 'text-emerald-500'}">
                {(countedCash - $activeSession.expected_cash).toLocaleString()} DZD
              </span>
            </div>
          {/if}
        {/if}
      </div>

      <!-- Footer -->
      <div class="px-5 py-3.5 border-t border-pos-border bg-slate-50 dark:bg-slate-800/50 flex justify-end gap-2">
        <button
          type="button"
          on:click={onClose}
          class="px-4 py-2 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 text-pos-text font-bold text-xs rounded transition"
        >
          Cancel
        </button>
        <button
          type="button"
          on:click={handleAction}
          disabled={isSubmitting}
          class="px-5 py-2 bg-amber-500 hover:bg-amber-600 text-slate-950 font-extrabold text-xs rounded transition flex items-center gap-1.5 cursor-pointer shadow-xs"
        >
          <Check class="w-4 h-4" />
          <span>Confirm</span>
        </button>
      </div>
    </div>
  </div>
{/if}