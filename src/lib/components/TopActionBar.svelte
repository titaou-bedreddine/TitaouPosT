<script lang="ts">
  import { t } from '../i18n';
  import { clearCart, isRefundMode, cartItems } from '../stores/cart';
  import { currentUser } from '../stores/auth';
  import { PlusCircle, Trash2, Undo2, Percent, CreditCard, PauseCircle, Printer, DollarSign, RefreshCw } from 'lucide-svelte';

  export let onOpenPayment: () => void;
  export let onOpenCashDrawer: () => void;
  export let onOpenRemise: () => void;
  export let onOpenHeldSales: () => void;
  export let onPrintReceipt: () => void;

  function toggleRefund() {
    $isRefundMode = !$isRefundMode;
  }
</script>

<header class="bg-pos-card border-b border-pos-border px-4 py-2 flex items-center justify-between shadow-sm select-none">
  <!-- Left Side: Top Action Buttons in ONE Single Row -->
  <div class="flex items-center gap-2 flex-wrap">
    <button
      on:click={clearCart}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-sky-600 hover:bg-sky-700 text-white rounded text-sm font-semibold transition shadow-xs"
      title="F1 - New Sale"
    >
      <PlusCircle class="w-4 h-4" />
      <span>{t('btn_new_sale')}</span>
    </button>

    <button
      on:click={clearCart}
      disabled={$cartItems.length === 0}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-red-600 hover:bg-red-700 disabled:opacity-40 text-white rounded text-sm font-semibold transition shadow-xs"
      title="Del - Clear Cart"
    >
      <Trash2 class="w-4 h-4" />
      <span>{t('btn_delete_cart')}</span>
    </button>

    <button
      on:click={toggleRefund}
      class="flex items-center gap-1.5 px-3 py-1.5 rounded text-sm font-semibold transition shadow-xs {$isRefundMode ? 'bg-amber-600 text-white ring-2 ring-amber-300' : 'bg-slate-200 dark:bg-slate-700 text-slate-800 dark:text-slate-200 hover:bg-slate-300'}"
      title="F3 - Toggle Refund Mode"
    >
      <Undo2 class="w-4 h-4" />
      <span>{t('btn_refund')}</span>
      {#if $isRefundMode}
        <span class="text-xs bg-black/30 px-1.5 py-0.5 rounded">ON</span>
      {/if}
    </button>

    <button
      on:click={onOpenRemise}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-slate-800 dark:text-slate-200 rounded text-sm font-semibold transition shadow-xs"
      title="F6 - Apply Remise"
    >
      <Percent class="w-4 h-4" />
      <span>{t('btn_remise')}</span>
    </button>

    <button
      on:click={onOpenPayment}
      disabled={$cartItems.length === 0}
      class="flex items-center gap-1.5 px-3.5 py-1.5 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-40 text-white rounded text-sm font-bold transition shadow-xs"
      title="F7 / F10 - Payment Checkout"
    >
      <CreditCard class="w-4 h-4" />
      <span>{t('btn_payment_type')}</span>
    </button>

    <button
      on:click={onOpenHeldSales}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-slate-800 dark:text-slate-200 rounded text-sm font-semibold transition shadow-xs"
      title="F9 - Held Sales"
    >
      <PauseCircle class="w-4 h-4" />
      <span>{t('btn_held_sales')}</span>
    </button>

    <button
      on:click={onPrintReceipt}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-slate-800 dark:text-slate-200 rounded text-sm font-semibold transition shadow-xs"
      title="F11 - Print Receipt"
    >
      <Printer class="w-4 h-4" />
      <span>{t('btn_print')}</span>
    </button>

    <button
      on:click={onOpenCashDrawer}
      class="flex items-center gap-1.5 px-3 py-1.5 bg-amber-500 hover:bg-amber-600 text-slate-900 rounded text-sm font-semibold transition shadow-xs"
      title="F12 - Cash Drawer (الصندوق)"
    >
      <DollarSign class="w-4 h-4" />
      <span>{t('btn_drawer')}</span>
    </button>
  </div>

  <!-- Right Side: User Status & Connection Status -->
  <div class="flex items-center gap-4 text-xs">
    <div class="flex items-center gap-1.5 text-emerald-600 dark:text-emerald-400 font-medium">
      <span class="w-2.5 h-2.5 rounded-full bg-emerald-500 animate-pulse"></span>
      <span>{t('online_status')}</span>
    </div>

    {#if $currentUser}
      <div class="flex items-center gap-2 bg-slate-100 dark:bg-slate-800 px-2.5 py-1 rounded border border-pos-border">
        <span class="font-semibold text-pos-text">{$currentUser.display_name}</span>
        <span class="text-pos-muted">({$currentUser.role_name || 'Cashier'})</span>
      </div>
    {/if}
  </div>
</header>