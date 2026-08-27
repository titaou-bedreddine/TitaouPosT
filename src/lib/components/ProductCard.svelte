<script lang="ts">
  import type { Product } from '../types';
  import { t } from '../i18n';
  import { addToCart, isRefundMode } from '../stores/cart';
  import { Package } from 'lucide-svelte';

  export let product: Product;

  function handleClick() {
    addToCart(product, 1, $isRefundMode);
  }
</script>

<button
  type="button"
  on:click={handleClick}
  class="flex flex-col text-start bg-pos-card border border-pos-border hover:border-sky-500 rounded-lg p-2.5 transition shadow-xs hover:shadow-md cursor-pointer group relative overflow-hidden focus:outline-none focus:ring-2 focus:ring-sky-500"
>
  <!-- Stock Status Pill -->
  <div class="absolute top-2 end-2 z-10">
    {#if product.current_stock > product.min_stock}
      <span class="inline-flex items-center gap-1 text-[11px] font-bold px-2 py-0.5 rounded-full bg-emerald-100 text-emerald-800 dark:bg-emerald-950 dark:text-emerald-300">
        <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span>
        {product.current_stock} {t('stock_in')}
      </span>
    {:else if product.current_stock > 0}
      <span class="inline-flex items-center gap-1 text-[11px] font-bold px-2 py-0.5 rounded-full bg-amber-100 text-amber-800 dark:bg-amber-950 dark:text-amber-300">
        <span class="w-1.5 h-1.5 rounded-full bg-amber-500"></span>
        {product.current_stock} {t('stock_low')}
      </span>
    {:else}
      <span class="inline-flex items-center gap-1 text-[11px] font-bold px-2 py-0.5 rounded-full bg-rose-100 text-rose-800 dark:bg-rose-950 dark:text-rose-300">
        <span class="w-1.5 h-1.5 rounded-full bg-rose-500"></span>
        {t('stock_out')}
      </span>
    {/if}
  </div>

  <!-- Product Image or Placeholder -->
  <div class="w-full h-24 bg-slate-100 dark:bg-slate-800 rounded flex items-center justify-center mb-2 overflow-hidden">
    {#if product.image_path}
      <img src={product.image_path} alt={product.name_ar} class="w-full h-full object-cover group-hover:scale-105 transition" />
    {:else}
      <Package class="w-8 h-8 text-pos-muted/50 group-hover:text-sky-500 transition" />
    {/if}
  </div>

  <!-- Product Title & Primary Barcode -->
  <div class="flex-1">
    <h3 class="font-bold text-sm text-pos-text line-clamp-1 group-hover:text-sky-600 transition">
      {product.name_ar || product.name_fr || product.name_en}
    </h3>
    {#if product.barcodes.length > 0}
      <span class="text-[11px] text-pos-muted font-mono">{product.barcodes[0]}</span>
    {/if}
  </div>

  <!-- Price -->
  <div class="mt-2 pt-1 border-t border-pos-border/50 flex items-center justify-between w-full">
    <span class="text-xs text-pos-muted font-semibold">{product.unit_name || 'Piece'}</span>
    <span class="text-base font-extrabold text-sky-600 dark:text-sky-400 font-mono">
      {product.sale_price.toLocaleString()} DZD
    </span>
  </div>
</button>