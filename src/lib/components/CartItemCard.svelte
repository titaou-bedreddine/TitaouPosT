<script lang="ts">
  import type { CartItem } from '../types';
  import { updateItemQuantity, applyItemDiscount, toggleItemRefund, removeFromCart } from '../stores/cart';
  import { Minus, Plus, Trash2, Undo2, Percent, Package } from 'lucide-svelte';

  export let item: CartItem;

  let showDiscountInput = false;
  let lineDiscountValue = item.discount_amount;

  function decrement() {
    updateItemQuantity(item.product_id, item.is_refund, item.quantity - 1);
  }

  function increment() {
    updateItemQuantity(item.product_id, item.is_refund, item.quantity + 1);
  }

  function handleQtyChange(e: Event) {
    const val = parseFloat((e.target as HTMLInputElement).value);
    if (!isNaN(val) && val > 0) {
      updateItemQuantity(item.product_id, item.is_refund, val);
    }
  }

  function toggleRefund() {
    toggleItemRefund(item.product_id, item.is_refund);
  }

  function applyDiscount() {
    applyItemDiscount(item.product_id, item.is_refund, lineDiscountValue);
    showDiscountInput = false;
  }
</script>

<div class="bg-pos-card border rounded-xl p-2.5 shadow-xs transition flex flex-col gap-2 {item.is_refund ? 'border-amber-500 bg-amber-500/5' : 'border-pos-border'}">
  <div class="flex items-center gap-3">
    <!-- Thumbnail Image -->
    <div class="w-11 h-11 rounded-lg bg-slate-100 dark:bg-slate-800 flex items-center justify-center shrink-0 overflow-hidden border border-pos-border/40">
      {#if item.image_path}
        <img src={item.image_path} alt={item.name_ar} class="w-full h-full object-cover" />
      {:else}
        <Package class="w-5 h-5 text-pos-muted/40" />
      {/if}
    </div>

    <!-- Title & Barcode -->
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-1.5">
        <h4 class="font-bold text-xs text-pos-text truncate">
          {item.name_ar || item.name_fr || item.name_en}
        </h4>
        {#if item.is_refund}
          <span class="inline-flex items-center gap-0.5 text-[9px] font-black bg-amber-500 text-white px-1.5 py-0.2 rounded font-mono">
            REFUND
          </span>
        {/if}
      </div>

      <div class="flex items-center gap-2 text-[11px] text-pos-muted mt-0.5">
        <span class="font-mono">{item.unit_price.toLocaleString()} DZD</span>
        {#if item.discount_amount > 0}
          <span class="text-rose-500 font-bold font-mono">(-{item.discount_amount} Remise)</span>
        {/if}
      </div>
    </div>

    <!-- Line Total -->
    <div class="text-end shrink-0">
      <span class="text-sm font-black font-mono {item.is_refund ? 'text-amber-600' : 'text-pos-text'}">
        {item.is_refund ? '-' : ''}{item.total_price.toLocaleString()} DZD
      </span>
    </div>
  </div>

  <!-- Stepper Controls & Quick Action Buttons -->
  <div class="flex items-center justify-between pt-1 border-t border-pos-border/40 text-xs">
    <div class="flex items-center gap-1">
      <!-- Line Refund Toggle -->
      <button
        type="button"
        on:click={toggleRefund}
        class="p-1 rounded text-xs font-bold transition cursor-pointer {item.is_refund ? 'bg-amber-500 text-white' : 'bg-slate-100 dark:bg-slate-800 text-pos-muted hover:text-amber-500'}"
        title="Toggle Line Refund"
      >
        <Undo2 class="w-3.5 h-3.5" />
      </button>

      <!-- Line Discount Toggle -->
      <button
        type="button"
        on:click={() => showDiscountInput = !showDiscountInput}
        class="p-1 rounded text-xs font-bold transition cursor-pointer {item.discount_amount > 0 ? 'bg-indigo-600 text-white' : 'bg-slate-100 dark:bg-slate-800 text-pos-muted hover:text-indigo-600'}"
        title="Set Line Remise"
      >
        <Percent class="w-3.5 h-3.5" />
      </button>

      <!-- Delete Button -->
      <button
        type="button"
        on:click={() => removeFromCart(item.product_id, item.is_refund)}
        class="p-1 text-rose-500 hover:text-rose-700 rounded transition cursor-pointer"
        title="Remove Item"
      >
        <Trash2 class="w-3.5 h-3.5" />
      </button>
    </div>

    <!-- Stepper Qty -->
    <div class="flex items-center bg-slate-100 dark:bg-slate-800 rounded-lg border border-pos-border p-0.5">
      <button
        type="button"
        on:click={decrement}
        class="p-1 hover:bg-slate-200 dark:hover:bg-slate-700 rounded text-pos-text transition cursor-pointer"
      >
        <Minus class="w-3 h-3" />
      </button>
      <input
        type="number"
        value={item.quantity}
        on:change={handleQtyChange}
        on:focus={(e) => (e.target as HTMLInputElement).select()}
        min="0.1"
        step="1"
        class="w-10 text-center font-bold text-xs bg-transparent border-0 outline-none text-pos-text font-mono"
      />
      <button
        type="button"
        on:click={increment}
        class="p-1 hover:bg-slate-200 dark:hover:bg-slate-700 rounded text-pos-text transition cursor-pointer"
      >
        <Plus class="w-3 h-3" />
      </button>
    </div>
  </div>

  <!-- Line Discount Input Popover -->
  {#if showDiscountInput}
    <div class="flex items-center gap-1.5 p-1.5 bg-indigo-50 dark:bg-indigo-950/50 rounded-lg border border-indigo-200 dark:border-indigo-800 animate-in fade-in duration-100">
      <input
        type="number"
        bind:value={lineDiscountValue}
        on:focus={(e) => (e.target as HTMLInputElement).select()}
        placeholder="Discount DZD"
        class="flex-1 px-2 py-1 bg-pos-card border border-pos-border rounded text-xs font-mono text-pos-text outline-none"
      />
      <button
        type="button"
        on:click={applyDiscount}
        class="px-2.5 py-1 bg-indigo-600 text-white font-bold text-xs rounded cursor-pointer"
      >
        Apply
      </button>
    </div>
  {/if}
</div>