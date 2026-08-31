<script lang="ts">
  import type { Product } from '../types';
  import { invoke } from '@tauri-apps/api/core';
  import { X, Check, PackagePlus, Percent } from 'lucide-svelte';

  export let isOpen = false;
  export let product: Product | null = null;
  export let onClose: () => void;
  // Confirm receives the final purchase price; the caller uses it as the
  // cart line's unit_price (POS purchase mode buys at cost).
  export let onConfirm: (price: number, salePrice: number, qty?: number) => void = () => {};

  let purchasePrice = 0;
  // Units-in-packaging: pick 1 carton = 24 pcs instead of typing 24.
  interface Packaging {
    id: number;
    name: string;
    unitsPerPackage: number;
    salePrice: number;
  }
  let packagings: Packaging[] = [];
  let selectedPackaging: number | null = null; // packaging id
  let packagingCount = 1;

  $: if (product) {
    // Load packaging definitions for this product.
    invoke<any[]>('list_packagings', { productId: product.id })
      .then((list) => {
        packagings = list.map((p) => ({
          id: p.id,
          name: p.name,
          unitsPerPackage: p.units_per_package,
          salePrice: p.sale_price,
        }));
        selectedPackaging = null;
        packagingCount = 1;
      })
      .catch(() => (packagings = []));
  }

  // Effective base-unit quantity: packagings multiply.
  $: effectiveQty = selectedPackaging
    ? packagingCount * (packagings.find((p) => p.id === selectedPackaging)?.unitsPerPackage || 1)
    : 1;
  let salePrice = 0;
  let marginPercent = 0;
  let errorMsg = '';

  $: if (product) {
    // Prefill with the product's existing cost; the cashier confirms or
    // types the new one from the supplier's invoice.
    purchasePrice = product.purchase_price || 0;
    salePrice = product.sale_price || 0;
    errorMsg = '';
  }

  $: if (purchasePrice > 0) {
    marginPercent = Math.round(((salePrice - purchasePrice) / purchasePrice) * 100);
  } else {
    marginPercent = 0;
  }

  // Typing a target margin back-calculates the sale price.
  function applyMargin(percent: number) {
    if (!product || purchasePrice <= 0) return;
    salePrice = Math.round(purchasePrice * (1 + percent / 100));
  }

  function confirm() {
    if (!product) return;
    if (!purchasePrice || purchasePrice <= 0) {
      errorMsg = 'Purchase price must be greater than 0 / سعر الشراء إجباري';
      return;
    }
    // Persist the new costs on the product so stock value stays true.
    try {
      invoke('save_product', {
        input: {
          sku: product.sku || undefined,
          name_ar: product.name_ar,
          name_fr: product.name_fr || product.name_ar,
          name_en: product.name_en || product.name_fr || product.name_ar,
          category_id: product.category_id,
          unit_id: product.unit_id,
          purchase_price: purchasePrice,
          sale_price: salePrice || product.sale_price,
          min_sale_price: product.min_sale_price || 0,
          tax_rate: product.tax_rate || 19,
          current_stock: product.current_stock || 0,
          min_stock: product.min_stock || 5,
          image_path: product.image_path || undefined,
          expiry_date: product.expiry_date || undefined,
          is_scalable: false,
          is_bundle: false,
          barcodes: product.barcodes || [],
        },
        productId: product.id,
      }).catch(() => {
        // Cost update is best-effort; the purchase line itself must proceed.
      });
    } catch {
      // Never block the purchase flow on the side update.
    }
    onConfirm(purchasePrice, salePrice || product.sale_price, effectiveQty);
    onClose();
  }
</script>

{#if isOpen && product}
  <div class="fixed inset-0 z-[70] bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-2xl shadow-2xl w-full max-w-md p-6 space-y-4 animate-in zoom-in-95">
      <div class="flex items-start justify-between">
        <div class="flex items-center gap-3">
          <div class="w-11 h-11 rounded-xl bg-amber-50 dark:bg-amber-950/60 text-amber-600 flex items-center justify-center shrink-0">
            <PackagePlus class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-black text-sm text-pos-text">Purchase Price (سعر الشراء)</h3>
            <p class="text-xs text-pos-muted truncate max-w-[220px]">{product.name_fr || product.name_ar}</p>
          </div>
        </div>
        <button on:click={onClose} class="p-1.5 text-pos-muted hover:text-pos-text rounded-lg cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>

      {#if errorMsg}
        <div class="p-2.5 bg-rose-100 text-rose-700 text-xs font-bold rounded-lg">{errorMsg}</div>
      {/if}

      <div class="grid grid-cols-2 gap-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Purchase Cost (DZD)</label>
          <input
            type="number"
            inputmode="numeric"
            min="0"
            bind:value={purchasePrice}
            on:focus={(e) => (e.target as HTMLInputElement).select()}
            class="w-full px-3 py-2.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-lg font-mono font-black text-pos-text outline-none focus:ring-2 focus:ring-amber-500"
          />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Sale Price (DZD)</label>
          <input
            type="number"
            inputmode="numeric"
            min="0"
            bind:value={salePrice}
            on:focus={(e) => (e.target as HTMLInputElement).select()}
            class="w-full px-3 py-2.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-lg font-mono font-black text-sky-600 outline-none focus:ring-2 focus:ring-sky-500"
          />
        </div>
      </div>

      {#if packagings.length > 0}
        <div class="space-y-1.5">
          <span class="text-[10px] font-bold text-pos-muted block">Packaging (التغليف):</span>
          <div class="flex items-center gap-1.5 flex-wrap">
            <button
              type="button"
              on:click={() => (selectedPackaging = null)}
              class="px-2 py-1 rounded-lg text-[10px] font-black cursor-pointer {selectedPackaging === null ? 'bg-amber-500 text-white' : 'bg-slate-100 dark:bg-slate-800 text-pos-muted'}"
            >
              1 unit
            </button>
            {#each packagings as pk}
              <button
                type="button"
                on:click={() => (selectedPackaging = pk.id)}
                class="px-2 py-1 rounded-lg text-[10px] font-black cursor-pointer {selectedPackaging === pk.id ? 'bg-amber-500 text-white' : 'bg-slate-100 dark:bg-slate-800 text-pos-muted'}"
              >
                1 {pk.name} = {pk.unitsPerPackage}u
              </button>
            {/each}
            {#if selectedPackaging}
              <div class="flex items-center gap-1 ms-1">
                <span class="text-[10px] font-bold text-pos-muted">×</span>
                <input
                  type="number"
                  min="1"
                  inputmode="numeric"
                  bind:value={packagingCount}
                  class="w-16 px-2 py-1 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono font-black text-pos-text outline-none"
                />
                <span class="text-[10px] font-black text-emerald-600">= {effectiveQty} u</span>
              </div>
            {/if}
          </div>
        </div>
      {/if}

      <div class="flex items-center gap-1.5 flex-wrap">
        <span class="text-[10px] font-bold text-pos-muted flex items-center gap-1">
          <Percent class="w-3 h-3" />
          Quick margin:
        </span>
        {#each [5, 10, 15, 20, 25, 30, 50] as pct}
          <button
            type="button"
            on:click={() => applyMargin(pct)}
            class="px-2 py-0.5 bg-slate-100 dark:bg-slate-800 text-[10px] font-black rounded-lg cursor-pointer hover:bg-amber-100 dark:hover:bg-amber-950/60 hover:text-amber-600 transition"
          >
            +{pct}%
          </button>
        {/each}
        {#if purchasePrice > 0}
          <span class="text-[10px] font-black {marginPercent >= 0 ? 'text-emerald-600' : 'text-rose-600'} ms-1">
            (margin: {marginPercent}%)
          </span>
        {/if}
      </div>

      <div class="flex justify-end gap-2 pt-2 border-t border-pos-border">
        <button on:click={onClose} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-xl cursor-pointer">
          Cancel
        </button>
        <button
          type="button"
          on:click={confirm}
          class="px-5 py-2 bg-amber-600 hover:bg-amber-700 text-white text-xs font-black rounded-xl cursor-pointer shadow-md flex items-center gap-1.5"
        >
          <Check class="w-4 h-4" />
          <span>Use Price (اعتماد)</span>
        </button>
      </div>
    </div>
  </div>
{/if}
