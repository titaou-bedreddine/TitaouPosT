<script lang="ts">
  // Interactive shortcut rebind editor: click a row, press the new key.
  // Duplicate keys are rejected; each row has a reset, plus reset-all.
  import { invoke } from '@tauri-apps/api/core';
  import { Keyboard, RotateCcw, X, Check } from 'lucide-svelte';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  export let bindings: Record<string, string> = {};

  // Canonical actions and their default keys (the POS preset).
  // NOTE: not exported — Svelte component scripts allow only let/function
  // exports; `export const` broke this component's instantiation.
  const DEFAULT_PRESET: Record<string, string> = {
    new_sale: 'F1',
    checkout_print: 'F2',
    hold_cart: 'F3',
    remise: 'F4',
    returns: 'F5',
    edit_qty: 'F6',
    toggle_products: 'F7',
    toggle_register: 'F8',
    toggle_sales: 'F9',
    cycle_mode: 'F10',
    cycle_payment: 'F11',
    quick_checkout: 'F12',
    open_drawer: 'Control',
  };

  const ACTION_LABELS: Record<string, string> = {
    new_sale: 'New Sale (holds current cart) / بيع جديد',
    checkout_print: 'Checkout + Print Receipt / دفع مع طباعة',
    hold_cart: 'Hold Current Cart / تعليق السلة',
    remise: 'Global Discount (Remise) / تخفيض',
    returns: 'Returns & Refunds / المرتجعات',
    edit_qty: 'Edit Line Quantity / تعديل الكمية',
    toggle_products: 'Products Page / صفحة المنتجات',
    toggle_register: 'Cash Register / الصندوق',
    toggle_sales: 'Sales History / سجل المبيعات',
    cycle_mode: 'Cycle Mode (Sale-Purchase-Broken) / النمط',
    cycle_payment: 'Cycle Payment (Cash-TPE-Credit) / الدفع',
    quick_checkout: 'Quick Checkout (no print) / دفع سريع',
    open_drawer: 'Open Cash Drawer / فتح الدرج',
  };

  let listeningFor: string | null = null;
  let toastMsg = '';

  // Which keys are legal to bind (F1..F12 plus the Control drawer kick).
  function isBindableKey(e: KeyboardEvent): boolean {
    return /^F([1-9]|1[0-2])$/.test(e.key) || e.key === 'Control';
  }

  function startListening(action: string) {
    listeningFor = action;
    toastMsg = '';
  }

  async function handleKey(e: KeyboardEvent) {
    if (!listeningFor) return;
    e.preventDefault();
    e.stopPropagation();

    if (e.key === 'Escape') {
      listeningFor = null;
      return;
    }
    if (!isBindableKey(e)) {
      toastMsg = 'Only F1–F12 or Ctrl can be bound / فقط F1-F12 أو Ctrl';
      return;
    }

    const newKey = e.key === 'Control' ? 'Control' : e.key;
    const owner = Object.entries(bindings).find(([a, k]) => k === newKey && a !== listeningFor);
    if (owner) {
      toastMsg = `${newKey} is already bound to "${ACTION_LABELS[owner[0]]}" / مفتاح مستعمل`;
      return;
    }

    bindings[listeningFor] = newKey;
    listeningFor = null;
    await persist();
  }

  async function resetOne(action: string) {
    bindings[action] = DEFAULT_PRESET[action];
    listeningFor = null;
    await persist();
  }

  async function resetAll() {
    bindings = { ...DEFAULT_PRESET };
    listeningFor = null;
    await persist();
  }

  async function persist() {
    try {
      await invoke('set_setting', {
        key: 'pos_shortcuts',
        value: JSON.stringify(bindings),
      });
      dispatch('change', bindings);
      toastMsg = 'Saved / تم الحفظ';
      setTimeout(() => (toastMsg = ''), 1500);
    } catch (e) {
      toastMsg = 'Save failed: ' + (typeof e === 'string' ? e : (e as any).message || e);
    }
  }
</script>

<div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-3">
  <div class="flex items-center justify-between">
    <p class="text-xs text-pos-muted font-bold">
      Click a shortcut, then press the new key. Duplicates are rejected automatically.
    </p>
    <button
      type="button"
      on:click={resetAll}
      class="px-3 py-1.5 bg-rose-100 hover:bg-rose-200 dark:bg-rose-950/60 dark:hover:bg-rose-900/60 text-rose-700 dark:text-rose-300 text-[10px] font-black rounded-lg cursor-pointer flex items-center gap-1.5"
    >
      <RotateCcw class="w-3 h-3" />
      <span>Reset All (استرجاع الكل)</span>
    </button>
  </div>

  {#if toastMsg}
    <div class="p-2 bg-sky-100 dark:bg-sky-950 text-sky-700 dark:text-sky-300 text-[11px] font-bold rounded-lg">{toastMsg}</div>
  {/if}

  <svelte:window on:keydown={handleKey} />

  <div class="grid grid-cols-1 md:grid-cols-2 gap-2.5">
    {#each Object.keys(ACTION_LABELS) as action}
      <div
        class="flex items-center justify-between p-2.5 bg-white dark:bg-slate-900 rounded-xl border transition {listeningFor === action ? 'border-sky-500 ring-2 ring-sky-500/40' : 'border-pos-border'}"
      >
        <span class="font-bold text-xs text-pos-text">{ACTION_LABELS[action]}</span>
        <div class="flex items-center gap-1.5">
          <button
            type="button"
            on:click={() => startListening(action)}
            class="px-2.5 py-1 bg-slate-200 dark:bg-slate-800 font-mono font-black text-xs text-sky-600 rounded-lg shadow-inner cursor-pointer hover:bg-sky-100 dark:hover:bg-sky-950 min-w-[54px] text-center transition"
            title="Click then press the new key"
          >
            {listeningFor === action ? '...' : bindings[action] || DEFAULT_PRESET[action]}
          </button>
          <button
            type="button"
            on:click={() => resetOne(action)}
            class="p-1 text-pos-muted hover:text-rose-500 rounded-lg cursor-pointer"
            title="Reset to preset"
          >
            <RotateCcw class="w-3.5 h-3.5" />
          </button>
        </div>
      </div>
    {/each}
  </div>
</div>
