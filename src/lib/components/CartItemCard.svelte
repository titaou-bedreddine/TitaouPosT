<script lang="ts">
  import type { CartItem } from '../types';
  import { updateItemQuantity, removeFromCart } from '../stores/cart';
  import { Minus, Plus, Trash2, Undo2, Package } from 'lucide-svelte';

  export let item: CartItem;

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

  function remove() {
    removeFromCart(item.product_id, item.is_refund);
  }
</script>

<div class="bg-pos-card border rounded-lg p-2.5 shadow-xs transition flex items-center gap-3 {item.is_refund ? 'border-amber-500 bg-amber-500/5' : 'border-pos-border'}">
  <!-- Thumbnail Image -->
  <div class="w-12 h-12 rounded bg-slate-100 dark:bg-slate-800 flex items-center justify-center shrink-0 overflow-hidden">
    {#if item.image_path}
      <img src={item.image_path} alt={item.name_ar} class="w-full h-full object-cover" />
    {:else}
      <Package class="w-6 h-6 text-pos-muted/40" />
    {/if}
  </div>

  <!-- Item Details & Pricing -->
  <div class="flex-1 min-w-0">
    <div class="flex items-center gap-1.5 flex-wrap">
      <h4 class="font-bold text-sm text-pos-text truncate">
        {item.name_ar || item.name_fr || item.name_en}
      </h4>
      {#if item.is_refund}
        <span class="inline-flex items-center gap-0.5 text-[10px] font-extrabold bg-amber-500 text-white px-1.5 py-0.5 rounded">
          <Undo2 class="w-2.5 h-2.5" /> REFUND
        </span>
      {/if}
    </div>

    <div class="flex items-center gap-2 text-xs text-pos-muted mt-0.5">
      {#if item.barcode}
        <span class="font-mono">{item.barcode}</span>
        <span>•</span>
      {/if}
      <span class="font-mono">{item.unit_price.toLocaleString()} DZD</span>
      {#if item.discount_amount > 0}
        <span class="text-rose-500 font-bold font-mono">(-{item.discount_amount} Remise)</span>
      {/if}
    </div>
  </div>

  <!-- Quantity Stepper Controls with Direct Input -->
  <div class="flex items-center bg-slate-100 dark:bg-slate-800 rounded border border-pos-border p-0.5">
    <button
      type="button"
      on:click={decrement}
      class="p-1 hover:bg-slate-200 dark:hover:bg-slate-700 rounded text-pos-text transition cursor-pointer"
      title="Decrease"
    >
      <Minus class="w-3.5 h-3.5" />
    </button>
    <input
      type="number"
      value={item.quantity}
      on:change={handleQtyChange}
      min="0.1"
      step="1"
      class="w-11 text-center font-bold text-sm bg-transparent border-0 outline-none text-pos-text font-mono"
    />
    <button
      type="button"
      on:click={increment}
      class="p-1 hover:bg-slate-200 dark:hover:bg-slate-700 rounded text-pos-text transition cursor-pointer"
      title="Increase"
    >
      <Plus class="w-3.5 h-3.5" />
    </button>
  </div>

  <!-- Line Total & Delete Button -->
  <div class="text-end shrink-0 min-w-[75px]">
    <div class="font-extrabold text-sm text-pos-text font-mono">
      {item.is_refund ? '-' : ''}{item.total_price.toLocaleString()} DZD
    </div>
    <button
      type="button"
      on:click={remove}
      class="text-rose-500 hover:text-rose-700 text-xs inline-flex items-center gap-0.5 mt-0.5 cursor-pointer"
      title="Remove Item"
    >
      <Trash2 class="w-3 h-3" />
    </button>
  </div>
</div>