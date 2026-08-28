<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { t, currentLocale, setLocale, type Language } from '../i18n';
  import {
    clearCart, isRefundMode, toggleAllCartRefund,
    cartItems, globalDiscountMode, heldSalesList,
    holdCurrentSale, heldNotification
  } from '../stores/cart';
  import { printHtmlDirectly } from '../utils/printer';
  import {
    PlusCircle, Trash2, Undo2, Percent, CreditCard,
    PauseCircle, Printer, DollarSign, Languages, Check,
    ShoppingBag, RefreshCw, AlertTriangle, Layers, Banknote, ShieldAlert
  } from 'lucide-svelte';

  export let onOpenPayment: () => void;
  export let onOpenCashDrawer: () => void;
  export let onOpenRemise: () => void;
  export let onOpenHeldSales: () => void;
  export let onPrintReceipt: () => void;
  export let onQuickPurchase: () => void;
  export let onReturnDamaged: () => void;
  export let onOpenOtherArticle: () => void = () => {};

  export let selectedPaymentMode: 'cash' | 'tpe' | 'credit' = 'cash';
  export let autoPrintEnabled: boolean = true;
  export let autoDrawerEnabled: boolean = true;

  async function handleNewSale() {
    if ($cartItems.length > 0) {
      await holdCurrentSale('New Sale held cart');
    } else {
      clearCart();
    }
  }

  async function handleHoldClick() {
    if ($cartItems.length > 0) {
      await holdCurrentSale('User held cart');
    } else {
      onOpenHeldSales();
    }
  }

  function cyclePaymentMode() {
    if (selectedPaymentMode === 'cash') selectedPaymentMode = 'tpe';
    else if (selectedPaymentMode === 'tpe') selectedPaymentMode = 'credit';
    else selectedPaymentMode = 'cash';
  }

  function handleRefundToggle() {
    $isRefundMode = !$isRefundMode;
    toggleAllCartRefund();
  }

  function handleRemiseToggle() {
    onOpenRemise();
  }

  function switchLanguage(lang: Language) {
    setLocale(lang);
  }

  async function kickDrawer() {
    try {
      const settings = await invoke<Record<string, string>>('get_all_settings');
      const port = parseInt(settings['drawer_com_port'] || '1');
      const baud = parseInt(settings['drawer_baud_rate'] || '9600');
      await invoke('open_serial_cash_drawer', { comPort: port, baudRate: baud });
    } catch (e) {
      console.warn('Native drawer open status:', e);
    }
  }
</script>

<header class="bg-pos-card border-b border-pos-border px-3 py-2 flex items-center justify-between shadow-xs select-none relative gap-2 shrink-0">
  <div class="flex items-center gap-2 flex-wrap lg:flex-nowrap">
    <!-- Group 1: Sale Lifecycle (F1 New, Del Clear, F3 Held List) -->
    <div class="flex items-center gap-1.5 bg-slate-50 dark:bg-slate-800/60 p-1.5 rounded-2xl border border-pos-border shadow-xs">
      <button
        type="button"
        on:click={handleNewSale}
        class="flex flex-col items-center justify-center w-20 h-14 bg-sky-600 hover:bg-sky-700 text-white rounded-xl text-center transition shadow-xs cursor-pointer active:scale-95 shrink-0"
        title="F1 - New Sale (Auto-holds active cart)"
      >
        <PlusCircle class="w-4 h-4 mb-0.5" />
        <span class="text-[10px] font-black leading-tight">{t('btn_new_sale')}</span>
        <span class="text-[8px] bg-black/25 px-1 rounded font-mono font-normal mt-0.5">F1</span>
      </button>

      <button
        type="button"
        on:click={clearCart}
        disabled={$cartItems.length === 0}
        class="flex flex-col items-center justify-center w-20 h-14 bg-rose-600 hover:bg-rose-700 disabled:opacity-40 text-white rounded-xl text-center transition shadow-xs cursor-pointer active:scale-95 shrink-0"
        title="Del - Clear Cart"
      >
        <Trash2 class="w-4 h-4 mb-0.5" />
        <span class="text-[10px] font-black leading-tight">{t('btn_delete_cart') || 'Clear'}</span>
        <span class="text-[8px] bg-black/25 px-1 rounded font-mono font-normal mt-0.5">Del</span>
      </button>

      <button
        type="button"
        on:click={handleHoldClick}
        class="flex flex-col items-center justify-center w-20 h-14 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-pos-text rounded-xl text-center transition shadow-xs cursor-pointer active:scale-95 relative shrink-0"
        title="F3 - {$cartItems.length > 0 ? 'Click to Hold Active Cart' : 'Open Held Sales List'}"
      >
        <PauseCircle class="w-4 h-4 text-amber-500 mb-0.5" />
        <span class="text-[10px] font-black leading-tight">{$cartItems.length > 0 ? 'Hold Cart' : t('btn_held_sales')}</span>
        {#if $heldSalesList.length > 0}
          <span class="absolute top-1 end-1 px-1.5 py-0.2 bg-amber-500 text-white font-mono text-[8px] rounded-full font-black">
            {$heldSalesList.length}
          </span>
        {/if}
        <span class="text-[8px] bg-black/10 dark:bg-white/10 px-1 rounded font-mono font-normal mt-0.5">F3</span>
      </button>
    </div>

    <!-- Vertical Divider -->
    <div class="h-8 w-px bg-pos-border shrink-0 hidden sm:block"></div>

    <!-- Group 2: Cart Modifiers (Refund, Remise, Other Article) -->
    <div class="flex items-center gap-1.5 bg-slate-50 dark:bg-slate-800/60 p-1.5 rounded-2xl border border-pos-border shadow-xs">
      <button
        type="button"
        on:click={handleRefundToggle}
        class="flex flex-col items-center justify-center w-20 h-14 rounded-xl text-center transition shadow-xs cursor-pointer active:scale-95 shrink-0 {$isRefundMode ? 'bg-amber-600 text-white ring-2 ring-amber-300' : 'bg-slate-200 dark:bg-slate-700 text-pos-text hover:bg-slate-300'}"
        title="Toggle Refund on All Items"
      >
        <Undo2 class="w-4 h-4 mb-0.5" />
        <span class="text-[10px] font-black leading-tight">{t('btn_refund')}</span>
        <span class="text-[8px] bg-black/10 dark:bg-white/10 px-1 rounded font-mono font-normal mt-0.5">F5</span>
      </button>

      <button
        type="button"
        on:click={handleRemiseToggle}
        class="flex flex-col items-center justify-center w-20 h-14 rounded-xl text-center transition shadow-xs cursor-pointer active:scale-95 shrink-0 {$globalDiscountMode !== 'none' ? 'bg-purple-600 text-white ring-2 ring-purple-300' : 'bg-slate-200 dark:bg-slate-700 text-pos-text hover:bg-slate-300'}"
        title="F4 - Global Remise (% or DZD)"
      >
        <Percent class="w-4 h-4 mb-0.5" />
        <span class="text-[10px] font-black leading-tight">{t('btn_remise')}</span>
        <span class="text-[8px] bg-black/10 dark:bg-white/10 px-1 rounded font-mono font-normal mt-0.5">F4</span>
      </button>

      <!-- Other Article (Custom Price / Divers) -->
      <button
        type="button"
        on:click={onOpenOtherArticle}
        class="flex flex-col items-center justify-center w-20 h-14 bg-amber-50 dark:bg-amber-950/40 border border-amber-300 dark:border-amber-700 hover:bg-amber-100 text-amber-900 dark:text-amber-200 rounded-xl text-center transition shadow-xs cursor-pointer active:scale-95 shrink-0"
        title="Other Article / مقال حر (Divers - Fast Price Entry)"
      >
        <DollarSign class="w-4 h-4 text-amber-600 mb-0.5" />
        <span class="text-[10px] font-black leading-tight">Divers</span>
        <span class="text-[8px] bg-amber-500/20 px-1 rounded font-mono font-normal mt-0.5">+ Price</span>
      </button>
    </div>

    <!-- Vertical Divider -->
    <div class="h-8 w-px bg-pos-border shrink-0 hidden sm:block"></div>

    <!-- Group 3: Payment Mode Single Multi-Toggle Button -->
    <div class="flex items-center bg-slate-50 dark:bg-slate-800/60 p-1.5 rounded-2xl border border-pos-border shadow-xs">
      <button
        type="button"
        on:click={cyclePaymentMode}
        class="flex flex-col items-center justify-center w-24 h-14 rounded-xl text-center transition cursor-pointer shrink-0 shadow-sm active:scale-95 {selectedPaymentMode === 'cash' ? 'bg-emerald-600 text-white ring-2 ring-emerald-400' : selectedPaymentMode === 'tpe' ? 'bg-sky-600 text-white ring-2 ring-sky-400' : 'bg-amber-600 text-white ring-2 ring-amber-400'}"
        title="Click to Toggle: Cash -> TPE -> Credit"
      >
        {#if selectedPaymentMode === 'cash'}
          <Banknote class="w-4 h-4 mb-0.5" />
          <span class="text-[10px] font-black leading-tight">Cash (نقد)</span>
        {:else if selectedPaymentMode === 'tpe'}
          <CreditCard class="w-4 h-4 mb-0.5" />
          <span class="text-[10px] font-black leading-tight">TPE (بطاقة)</span>
        {:else}
          <Layers class="w-4 h-4 mb-0.5" />
          <span class="text-[10px] font-black leading-tight">Credit (دين)</span>
        {/if}
        <span class="text-[8px] opacity-80 font-mono mt-0.5">Toggle ↻</span>
      </button>
    </div>

    <!-- Vertical Divider -->
    <div class="h-8 w-px bg-pos-border shrink-0 hidden sm:block"></div>

    <!-- Group 4: Operations (Purchases, Return/Loss, Drawer, Register) -->
    <div class="flex items-center gap-1.5 bg-slate-50 dark:bg-slate-800/60 p-1.5 rounded-2xl border border-pos-border shadow-xs">
      <button
        type="button"
        on:click={onQuickPurchase}
        class="flex flex-col items-center justify-center w-20 h-14 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-pos-text rounded-xl text-center transition shadow-xs cursor-pointer active:scale-95 shrink-0"
        title="F7 - Purchases Mode"
      >
        <ShoppingBag class="w-4 h-4 text-sky-500 mb-0.5" />
        <span class="text-[10px] font-black leading-tight">Purchases</span>
        <span class="text-[8px] bg-black/10 dark:bg-white/10 px-1 rounded font-mono font-normal mt-0.5">F7</span>
      </button>

      <button
        type="button"
        on:click={onReturnDamaged}
        class="flex flex-col items-center justify-center w-20 h-14 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-pos-text rounded-xl text-center transition shadow-xs cursor-pointer active:scale-95 shrink-0"
        title="F8 - Return / Damaged Goods (إرجاع متلف)"
      >
        <AlertTriangle class="w-4 h-4 text-rose-500 mb-0.5" />
        <span class="text-[10px] font-black leading-tight">Return</span>
        <span class="text-[8px] bg-black/10 dark:bg-white/10 px-1 rounded font-mono font-normal mt-0.5">F8</span>
      </button>

      <button
        type="button"
        on:click={onOpenCashDrawer}
        class="flex flex-col items-center justify-center w-20 h-14 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-pos-text rounded-xl text-center transition shadow-xs cursor-pointer active:scale-95 shrink-0"
        title="F9 - Cash Register (صندوق Caisse)"
      >
        <DollarSign class="w-4 h-4 text-emerald-500 mb-0.5" />
        <span class="text-[10px] font-black leading-tight">Register</span>
        <span class="text-[8px] bg-black/10 dark:bg-white/10 px-1 rounded font-mono font-normal mt-0.5">F9</span>
      </button>

      <button
        type="button"
        on:click={kickDrawer}
        class="flex flex-col items-center justify-center w-20 h-14 bg-emerald-600 hover:bg-emerald-700 text-white rounded-xl text-center transition shadow-xs cursor-pointer active:scale-95 shrink-0"
        title="F10 - Open Cash Drawer (فتح الدرج)"
      >
        <CreditCard class="w-4 h-4 mb-0.5" />
        <span class="text-[10px] font-black leading-tight">Drawer</span>
        <span class="text-[8px] bg-black/25 px-1 rounded font-mono font-normal mt-0.5">F10</span>
      </button>
    </div>

    <!-- Vertical Divider -->
    <div class="h-8 w-px bg-pos-border shrink-0 hidden sm:block"></div>

    <!-- Group 5: Silent Print Toggle Card -->
    <div class="flex items-center bg-slate-50 dark:bg-slate-800/60 p-1.5 rounded-2xl border border-pos-border shadow-xs">
      <button
        type="button"
        on:click={() => (autoPrintEnabled = !autoPrintEnabled)}
        class="flex flex-col items-center justify-center w-20 h-14 rounded-xl text-center transition cursor-pointer shrink-0 {autoPrintEnabled ? 'bg-sky-600 text-white' : 'bg-slate-200 dark:bg-slate-700 text-pos-muted'}"
        title="Toggle Auto Print Receipt on Checkout"
      >
        <Printer class="w-4 h-4 mb-0.5" />
        <span class="text-[9px] font-black uppercase">Auto-Print</span>
        <span class="text-[8px] font-mono font-bold mt-0.5">{autoPrintEnabled ? 'ON' : 'OFF'}</span>
      </button>
    </div>
  </div>

  <!-- Auto-Hold Notification Banner Popover -->
  {#if $heldNotification}
    <div class="absolute top-14 start-4 z-50 bg-emerald-600 text-white px-4 py-2 rounded-xl shadow-xl flex items-center gap-2 text-xs font-bold animate-in slide-in-from-top-2 duration-150">
      <Check class="w-4 h-4" />
      <span>{$heldNotification}</span>
    </div>
  {/if}
</header>