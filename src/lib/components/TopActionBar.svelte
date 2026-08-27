<script lang="ts">
  import { t, currentLocale, setLocale, type Language } from '../i18n';
  import {
    clearCart, isRefundMode, toggleAllCartRefund,
    cartItems, globalDiscountMode, heldSalesList,
    holdCurrentSale, heldNotification
  } from '../stores/cart';
  import {
    PlusCircle, Trash2, Undo2, Percent, CreditCard,
    PauseCircle, Printer, DollarSign, Languages, Check
  } from 'lucide-svelte';

  export let onOpenPayment: () => void;
  export let onOpenCashDrawer: () => void;
  export let onOpenRemise: () => void;
  export let onOpenHeldSales: () => void;
  export let onPrintReceipt: () => void;

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
</script>

<header class="bg-pos-card border-b border-pos-border px-3 py-2 flex items-center justify-between shadow-xs select-none relative">
  <!-- Top Action Buttons in ONE Single Row -->
  <div class="flex items-center gap-1.5 flex-wrap">
    <button
      type="button"
      on:click={handleNewSale}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-sky-600 hover:bg-sky-700 text-white rounded-xl text-xs font-black transition shadow-xs cursor-pointer active:scale-95"
      title="F1 - New Sale (Auto-holds active cart)"
    >
      <PlusCircle class="w-3.5 h-3.5" />
      <span>{t('btn_new_sale')}</span>
    </button>

    <button
      type="button"
      on:click={clearCart}
      disabled={$cartItems.length === 0}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-rose-600 hover:bg-rose-700 disabled:opacity-40 text-white rounded-xl text-xs font-bold transition shadow-xs cursor-pointer active:scale-95"
      title="Del - Clear Cart"
    >
      <Trash2 class="w-3.5 h-3.5" />
      <span>{t('btn_delete_cart')}</span>
    </button>

    <button
      type="button"
      on:click={handleRefundToggle}
      class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-bold transition shadow-xs cursor-pointer active:scale-95 {$isRefundMode ? 'bg-amber-600 text-white ring-2 ring-amber-300' : 'bg-slate-100 dark:bg-slate-800 text-pos-text hover:bg-slate-200'}"
      title="F3 - Toggle Refund on All Items"
    >
      <Undo2 class="w-3.5 h-3.5" />
      <span>{t('btn_refund')}</span>
      {#if $isRefundMode}
        <span class="text-[10px] bg-black/30 px-1 py-0.2 rounded font-mono">ALL</span>
      {/if}
    </button>

    <button
      type="button"
      on:click={handleRemiseToggle}
      class="flex items-center gap-1.5 px-3 py-1.5 rounded-xl text-xs font-bold transition shadow-xs cursor-pointer active:scale-95 {$globalDiscountMode !== 'none' ? 'bg-purple-600 text-white ring-2 ring-purple-300' : 'bg-slate-100 dark:bg-slate-800 text-pos-text hover:bg-slate-200'}"
      title="F4 - Global Remise (% or DZD)"
    >
      <Percent class="w-3.5 h-3.5" />
      <span>{t('btn_remise')}</span>
    </button>

    <button
      type="button"
      on:click={onOpenPayment}
      disabled={$cartItems.length === 0}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-40 text-white rounded-xl text-xs font-black transition shadow-xs cursor-pointer active:scale-95"
      title="F2 - Pay Cash / TPE / Credit"
    >
      <CreditCard class="w-3.5 h-3.5" />
      <span>{t('btn_payment_type')}</span>
    </button>

    <button
      type="button"
      on:click={onOpenHeldSales}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 text-pos-text rounded-xl text-xs font-bold transition shadow-xs cursor-pointer active:scale-95 relative"
      title="F6 - Held Sales (المعلقة)"
    >
      <PauseCircle class="w-3.5 h-3.5 text-amber-500" />
      <span>{t('btn_held_sales')}</span>
      {#if $heldSalesList.length > 0}
        <span class="px-1.5 py-0.2 bg-amber-500 text-white font-mono text-[10px] rounded-full font-black">
          {$heldSalesList.length}
        </span>
      {/if}
    </button>

    <button
      type="button"
      on:click={onPrintReceipt}
      disabled={$cartItems.length === 0}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 disabled:opacity-40 text-pos-text rounded-xl text-xs font-bold transition shadow-xs cursor-pointer active:scale-95"
      title="F8 - Print Receipt"
    >
      <Printer class="w-3.5 h-3.5 text-sky-500" />
      <span>{t('btn_print')}</span>
    </button>

    <button
      type="button"
      on:click={onOpenCashDrawer}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 text-pos-text rounded-xl text-xs font-bold transition shadow-xs cursor-pointer active:scale-95"
      title="F9 - Open / Adjust Drawer"
    >
      <DollarSign class="w-3.5 h-3.5 text-emerald-500" />
      <span>{t('btn_drawer')}</span>
    </button>
  </div>

  <!-- Right: 3-Language Selector Pill (Arabic, French, English) -->
  <div class="flex items-center gap-1 bg-slate-100 dark:bg-slate-800 p-1 rounded-xl border border-pos-border">
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
    <div class="absolute top-12 start-4 z-50 bg-emerald-600 text-white px-4 py-2 rounded-xl shadow-xl flex items-center gap-2 text-xs font-bold animate-in slide-in-from-top-2 duration-150">
      <Check class="w-4 h-4" />
      <span>{$heldNotification}</span>
    </div>
  {/if}
</header>