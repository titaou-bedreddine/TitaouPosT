<script lang="ts">
  import type { Product } from '../types';
  import { t, currentLocale } from '../i18n';
  import { addToCart, isRefundMode } from '../stores/cart';
  import { Package, Plus } from 'lucide-svelte';

  export let product: Product;
  let isClicked = false;

  function handleClick() {
    isClicked = true;
    setTimeout(() => (isClicked = false), 250);
    addToCart(product, 1, $isRefundMode);
  }

  $: displayName =
    $currentLocale === 'ar'
      ? product.name_ar || product.name_fr
      : $currentLocale === 'fr'
      ? product.name_fr || product.name_en
      : product.name_en || product.name_fr || product.name_ar;
</script>

<button
  type="button"
  on:click={handleClick}
  class="flex flex-col text-start bg-pos-card border border-pos-border hover:border-sky-500 rounded-2xl p-2.5 transition-all duration-150 shadow-xs hover:shadow-md cursor-pointer group relative overflow-hidden focus:outline-none focus:ring-2 focus:ring-sky-500 active:scale-95 {isClicked ? 'ring-2 ring-emerald-500 bg-emerald-50/40 dark:bg-emerald-950/30' : ''}"
>
  <!-- Stock Status Pill -->
  <div class="absolute top-2 end-2 z-10">
    {#if product.current_stock > product.min_stock}
      <span class="inline-flex items-center gap-1 text-[10px] font-black px-2 py-0.5 rounded-full bg-emerald-100 text-emerald-800 dark:bg-emerald-950 dark:text-emerald-300">
        <span class="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></span>
        {product.current_stock}
      </span>
    {:else if product.current_stock > 0}
      <span class="inline-flex items-center gap-1 text-[10px] font-black px-2 py-0.5 rounded-full bg-amber-100 text-amber-800 dark:bg-amber-950 dark:text-amber-300">
        <span class="w-1.5 h-1.5 rounded-full bg-amber-500"></span>
        {product.current_stock}
      </span>
    {:else}
      <span class="inline-flex items-center gap-1 text-[10px] font-black px-2 py-0.5 rounded-full bg-rose-100 text-rose-800 dark:bg-rose-950 dark:text-rose-300">
        <span class="w-1.5 h-1.5 rounded-full bg-rose-500"></span>
        {t('stock_out')}
      </span>
    {/if}
  </div>

  <!-- Product Image or Placeholder -->
  <div class="w-full h-22 bg-slate-100 dark:bg-slate-800/80 rounded-xl flex items-center justify-center mb-2 overflow-hidden relative">
    {#if product.image_path}
      <img src={product.image_path} alt={displayName} class="w-full h-full object-cover group-hover:scale-105 transition duration-200" />
    {:else}
      <Package class="w-7 h-7 text-pos-muted/40 group-hover:text-sky-500 transition duration-200" />
    {/if}

    <!-- Click Overlay Animation -->
    {#if isClicked}
      <div class="absolute inset-0 bg-emerald-500/20 backdrop-blur-[1px] flex items-center justify-center animate-in zoom-in-50 duration-150">
        <div class="w-7 h-7 rounded-full bg-emerald-500 text-white flex items-center justify-center font-black text-xs shadow-md">
          +1
        </div>
      </div>
    {/if}
  </div>

  <!-- Product Title & Primary Barcode -->
  <div class="flex-1 min-h-[38px]">
    <h3 class="font-bold text-xs text-pos-text line-clamp-2 leading-tight group-hover:text-sky-600 transition">
      {displayName}
    </h3>
    {#if product.barcodes && product.barcodes.length > 0}
      <span class="text-[10px] text-pos-muted font-mono block mt-0.5">{product.barcodes[0]}</span>
    {/if}
  </div>

  <!-- Price Footer -->
  <div class="mt-2 pt-1 border-t border-pos-border/60 flex items-center justify-between w-full">
    <span class="text-[10px] text-pos-muted font-bold">{product.unit_name || 'Unit'}</span>
    <span class="text-sm font-black text-sky-600 dark:text-sky-400 font-mono">
      {product.sale_price.toLocaleString()} DZD
    </span>
  </div>
</button>