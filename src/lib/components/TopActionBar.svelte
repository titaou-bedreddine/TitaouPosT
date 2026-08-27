<script lang="ts">
  import { t } from '../i18n';
  import { clearCart, isRefundMode, toggleAllCartRefund, cartItems, globalDiscountMode, heldSalesList } from '../stores/cart';
  import { PlusCircle, Trash2, Undo2, Percent, CreditCard, PauseCircle, Printer, DollarSign } from 'lucide-svelte';

  export let onOpenPayment: () => void;
  export let onOpenCashDrawer: () => void;
  export let onOpenRemise: () => void;
  export let onOpenHeldSales: () => void;
  export let onPrintReceipt: () => void;

  function handleRefundToggle() {
    $isRefundMode = !$isRefundMode;
    toggleAllCartRefund();
  }

  function handleRemiseToggle() {
    onOpenRemise();
  }
</script>

<header class="bg-pos-card border-b border-pos-border px-3 py-2 flex items-center justify-between shadow-xs select-none">
  <!-- Top Action Buttons in ONE Single Row -->
  <div class="flex items-center gap-1.5 flex-wrap">
    <button
      type="button"
      on:click={clearCart}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-sky-600 hover:bg-sky-700 text-white rounded-lg text-xs font-bold transition shadow-xs cursor-pointer"
      title="F1 - New Sale"
    >
      <PlusCircle class="w-3.5 h-3.5" />
      <span>{t('btn_new_sale')}</span>
    </button>

    <button
      type="button"
      on:click={clearCart}
      disabled={$cartItems.length === 0}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-rose-600 hover:bg-rose-700 disabled:opacity-40 text-white rounded-lg text-xs font-bold transition shadow-xs cursor-pointer"
      title="Del - Clear Cart"
    >
      <Trash2 class="w-3.5 h-3.5" />
      <span>{t('btn_delete_cart')}</span>
    </button>

    <button
      type="button"
      on:click={handleRefundToggle}
      class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-bold transition shadow-xs cursor-pointer {$isRefundMode ? 'bg-amber-600 text-white ring-2 ring-amber-300' : 'bg-slate-100 dark:bg-slate-800 text-pos-text hover:bg-slate-200'}"
      title="F3 - Toggle Refund on Cart"
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
      class="flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-bold transition shadow-xs cursor-pointer {$globalDiscountMode !== 'none' ? 'bg-indigo-600 text-white ring-2 ring-indigo-300' : 'bg-slate-100 dark:bg-slate-800 text-pos-text hover:bg-slate-200'}"
      title="F6 - Apply Remise (% / DZD)"
    >
      <Percent class="w-3.5 h-3.5" />
      <span>{t('btn_remise')}</span>
      {#if $globalDiscountMode !== 'none'}
        <span class="text-[10px] bg-black/30 px-1 py-0.2 rounded font-mono">{$globalDiscountMode === 'percent' ? '%' : 'DZD'}</span>
      {/if}
    </button>

    <button
      type="button"
      on:click={onOpenPayment}
      disabled={$cartItems.length === 0}
      class="flex items-center gap-2 px-4 py-1.5 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-40 text-white rounded-lg text-xs font-black transition shadow-xs cursor-pointer"
      title="F7 / F10 - Payment Checkout"
    >
      <CreditCard class="w-4 h-4" />
      <span>{t('btn_payment_type')}</span>
    </button>

    <button
      type="button"
      on:click={onOpenHeldSales}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 text-pos-text rounded-lg text-xs font-bold transition shadow-xs cursor-pointer relative"
      title="F9 - Held Sales"
    >
      <PauseCircle class="w-3.5 h-3.5" />
      <span>{t('btn_held_sales')}</span>
      {#if $heldSalesList.length > 0}
        <span class="text-[10px] bg-amber-500 text-slate-950 font-black px-1.5 py-0.2 rounded-full font-mono">
          {$heldSalesList.length}
        </span>
      {/if}
    </button>

    <button
      type="button"
      on:click={onPrintReceipt}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 text-pos-text rounded-lg text-xs font-bold transition shadow-xs cursor-pointer"
      title="F11 - Print Receipt"
    >
      <Printer class="w-3.5 h-3.5" />
      <span>{t('btn_print')}</span>
    </button>

    <button
      type="button"
      on:click={onOpenCashDrawer}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-amber-500 hover:bg-amber-600 text-slate-950 rounded-lg text-xs font-extrabold transition shadow-xs cursor-pointer"
      title="F12 - Fast Cash Options"
    >
      <DollarSign class="w-3.5 h-3.5" />
      <span>{t('btn_drawer')}</span>
    </button>
  </div>
</header>