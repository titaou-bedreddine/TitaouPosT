<script lang="ts">
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

  export let selectedPaymentMode: 'cash' | 'tpe' | 'credit' = 'cash';
  export let autoPrintEnabled: boolean = true;
  export let autoDrawerEnabled: boolean = true;

  async function handleNewSale() {
    if ($cartItems.length > 0) {
      await holdCurrentSale('Auto-held for New Sale');
    } else {
      clearCart();
    }
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

  function kickDrawer() {
    printHtmlDirectly('<div style="display:none"></div>', 'Kick Drawer');
  }
</script>

<header class="bg-pos-card border-b border-pos-border px-3 py-2 flex items-center justify-between shadow-xs select-none relative gap-2 overflow-x-auto">
  <div class="flex items-center gap-2 shrink-0">
    <!-- Group 1: Sale Lifecycle (F1 New, Del Clear, F3 Hold, F6 Held List) -->
    <div class="flex items-center gap-1 bg-slate-50 dark:bg-slate-800/60 p-1 rounded-xl border border-pos-border shadow-xs">
      <button
        type="button"
        on:click={handleNewSale}
        class="flex items-center gap-1.5 px-2.5 py-1.5 bg-sky-600 hover:bg-sky-700 text-white rounded-lg text-xs font-black transition shadow-xs cursor-pointer active:scale-95"
        title="F1 - New Sale (Auto-holds active cart)"
      >
        <PlusCircle class="w-3.5 h-3.5" />
        <span>{t('btn_new_sale')}</span>
        <span class="text-[9px] bg-black/20 px-1 py-0.2 rounded font-mono font-normal">F1</span>
      </button>

      <button
        type="button"
        on:click={clearCart}
        disabled={$cartItems.length === 0}
        class="flex items-center gap-1 px-2 py-1.5 bg-rose-600 hover:bg-rose-700 disabled:opacity-40 text-white rounded-lg text-xs font-bold transition shadow-xs cursor-pointer active:scale-95"
        title="Del - Clear Cart"
      >
        <Trash2 class="w-3.5 h-3.5" />
        <span class="text-[9px] bg-black/20 px-1 py-0.2 rounded font-mono font-normal">Del</span>
      </button>

      <button
        type="button"
        on:click={onOpenHeldSales}
        class="flex items-center gap-1.5 px-2.5 py-1.5 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-pos-text rounded-lg text-xs font-bold transition shadow-xs cursor-pointer active:scale-95 relative"
        title="F3 - Held Sales (المعلقة)"
      >
        <PauseCircle class="w-3.5 h-3.5 text-amber-500" />
        <span>{t('btn_held_sales')}</span>
        {#if $heldSalesList.length > 0}
          <span class="px-1.5 py-0.2 bg-amber-500 text-white font-mono text-[9px] rounded-full font-black">
            {$heldSalesList.length}
          </span>
        {/if}
        <span class="text-[9px] bg-black/10 dark:bg-white/10 px-1 py-0.2 rounded font-mono font-normal">F3</span>
      </button>
    </div>

    <!-- Vertical Divider -->
    <div class="h-6 w-px bg-pos-border shrink-0"></div>

    <!-- Group 2: Cart Modifiers (Refund, Remise) -->
    <div class="flex items-center gap-1 bg-slate-50 dark:bg-slate-800/60 p-1 rounded-xl border border-pos-border shadow-xs">
      <button
        type="button"
        on:click={handleRefundToggle}
        class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs font-bold transition shadow-xs cursor-pointer active:scale-95 {$isRefundMode ? 'bg-amber-600 text-white ring-2 ring-amber-300' : 'bg-slate-200 dark:bg-slate-700 text-pos-text hover:bg-slate-300'}"
        title="Toggle Refund on All Items"
      >
        <Undo2 class="w-3.5 h-3.5" />
        <span>{t('btn_refund')}</span>
      </button>

      <button
        type="button"
        on:click={handleRemiseToggle}
        class="flex items-center gap-1.5 px-2.5 py-1.5 rounded-lg text-xs font-bold transition shadow-xs cursor-pointer active:scale-95 {$globalDiscountMode !== 'none' ? 'bg-purple-600 text-white ring-2 ring-purple-300' : 'bg-slate-200 dark:bg-slate-700 text-pos-text hover:bg-slate-300'}"
        title="F4 - Global Remise (% or DZD)"
      >
        <Percent class="w-3.5 h-3.5" />
        <span>{t('btn_remise')}</span>
        <span class="text-[9px] bg-black/10 dark:bg-white/10 px-1 py-0.2 rounded font-mono font-normal">F4</span>
      </button>
    </div>

    <!-- Vertical Divider -->
    <div class="h-6 w-px bg-pos-border shrink-0"></div>

    <!-- Group 3: Payment Mode Toggles (Cash | TPE | Credit) -->
    <div class="flex items-center gap-1 bg-slate-50 dark:bg-slate-800/60 p-1 rounded-xl border border-pos-border shadow-xs">
      <button
        type="button"
        on:click={() => (selectedPaymentMode = 'cash')}
        class="flex items-center gap-1 px-2.5 py-1.5 rounded-lg text-xs font-bold transition cursor-pointer {selectedPaymentMode === 'cash' ? 'bg-emerald-600 text-white shadow-xs' : 'text-pos-muted hover:text-pos-text'}"
      >
        <Banknote class="w-3.5 h-3.5" />
        <span>Cash (نقد)</span>
      </button>

      <button
        type="button"
        on:click={() => (selectedPaymentMode = 'tpe')}
        class="flex items-center gap-1 px-2.5 py-1.5 rounded-lg text-xs font-bold transition cursor-pointer {selectedPaymentMode === 'tpe' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:text-pos-text'}"
      >
        <CreditCard class="w-3.5 h-3.5" />
        <span>TPE (بطاقة)</span>
      </button>

      <button
        type="button"
        on:click={() => (selectedPaymentMode = 'credit')}
        class="flex items-center gap-1 px-2.5 py-1.5 rounded-lg text-xs font-bold transition cursor-pointer {selectedPaymentMode === 'credit' ? 'bg-amber-600 text-white shadow-xs' : 'text-pos-muted hover:text-pos-text'}"
      >
        <Layers class="w-3.5 h-3.5" />
        <span>Credit (دين)</span>
      </button>
    </div>

    <!-- Vertical Divider -->
    <div class="h-6 w-px bg-pos-border shrink-0"></div>

    <!-- Group 4: Special Operations (Quick Purchase, Return/Loss, Drawer, Register) -->
    <div class="flex items-center gap-1 bg-slate-50 dark:bg-slate-800/60 p-1 rounded-xl border border-pos-border shadow-xs">
      <button
        type="button"
        on:click={onQuickPurchase}
        class="flex items-center gap-1 px-2 py-1.5 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-pos-text rounded-lg text-xs font-bold transition shadow-xs cursor-pointer active:scale-95"
        title="F7 - Quick Purchase (شراء سريع)"
      >
        <ShoppingBag class="w-3.5 h-3.5 text-sky-500" />
        <span>Quick Purchase</span>
        <span class="text-[9px] bg-black/10 dark:bg-white/10 px-1 py-0.2 rounded font-mono font-normal">F7</span>
      </button>

      <button
        type="button"
        on:click={onReturnDamaged}
        class="flex items-center gap-1 px-2 py-1.5 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-pos-text rounded-lg text-xs font-bold transition shadow-xs cursor-pointer active:scale-95"
        title="F8 - Return / Damaged Goods (إرجاع متلف)"
      >
        <AlertTriangle class="w-3.5 h-3.5 text-rose-500" />
        <span>Return / Loss</span>
        <span class="text-[9px] bg-black/10 dark:bg-white/10 px-1 py-0.2 rounded font-mono font-normal">F8</span>
      </button>

      <button
        type="button"
        on:click={onOpenCashDrawer}
        class="flex items-center gap-1 px-2 py-1.5 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-pos-text rounded-lg text-xs font-bold transition shadow-xs cursor-pointer active:scale-95"
        title="F9 - Cash Register Session (صندوق Caisse)"
      >
        <DollarSign class="w-3.5 h-3.5 text-emerald-500" />
        <span>Cash Register</span>
        <span class="text-[9px] bg-black/10 dark:bg-white/10 px-1 py-0.2 rounded font-mono font-normal">F9</span>
      </button>

      <button
        type="button"
        on:click={kickDrawer}
        class="flex items-center gap-1 px-2 py-1.5 bg-emerald-600 hover:bg-emerald-700 text-white rounded-lg text-xs font-bold transition shadow-xs cursor-pointer active:scale-95"
        title="F10 - Open Cash Drawer (فتح الدرج)"
      >
        <CreditCard class="w-3.5 h-3.5" />
        <span>Drawer</span>
        <span class="text-[9px] bg-black/20 px-1 py-0.2 rounded font-mono font-normal">F10</span>
      </button>
    </div>

    <!-- Group 5: Hardware Silent Print Toggle Switch -->
    <div class="flex items-center gap-1.5 bg-slate-50 dark:bg-slate-800/60 p-1.5 rounded-xl border border-pos-border shadow-xs">
      <button
        type="button"
        on:click={() => (autoPrintEnabled = !autoPrintEnabled)}
        class="flex items-center gap-1 text-xs font-bold px-2 py-1 rounded-lg transition cursor-pointer {autoPrintEnabled ? 'bg-sky-600 text-white' : 'bg-slate-200 dark:bg-slate-700 text-pos-muted'}"
        title="Toggle Auto Print Receipt on Checkout"
      >
        <Printer class="w-3.5 h-3.5" />
        <span>Auto-Print: {autoPrintEnabled ? 'ON' : 'OFF'}</span>
      </button>
    </div>
  </div>

  <!-- Right: 3-Language Selector Pill (Arabic, French, English) -->
  <div class="flex items-center gap-1 bg-slate-100 dark:bg-slate-800 p-1 rounded-xl border border-pos-border shrink-0">
    <button
      type="button"
      on:click={() => switchLanguage('ar')}
      class="px-2 py-1 rounded-lg text-xs font-bold transition cursor-pointer {$currentLocale === 'ar' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:text-pos-text'}"
    >
      عربي
    </button>
    <button
      type="button"
      on:click={() => switchLanguage('fr')}
      class="px-2 py-1 rounded-lg text-xs font-bold transition cursor-pointer {$currentLocale === 'fr' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:text-pos-text'}"
    >
      FR
    </button>
    <button
      type="button"
      on:click={() => switchLanguage('en')}
      class="px-2 py-1 rounded-lg text-xs font-bold transition cursor-pointer {$currentLocale === 'en' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:text-pos-text'}"
    >
      EN
    </button>
  </div>

  <!-- Auto-Hold Notification Banner Popover -->
  {#if $heldNotification}
    <div class="absolute top-14 start-4 z-50 bg-emerald-600 text-white px-4 py-2 rounded-xl shadow-xl flex items-center gap-2 text-xs font-bold animate-in slide-in-from-top-2 duration-150">
      <Check class="w-4 h-4" />
      <span>{$heldNotification}</span>
    </div>
  {/if}
</header>