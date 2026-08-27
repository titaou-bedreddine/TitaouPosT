<script lang="ts">
  import type { Product } from '../types';
  import { printHtmlDirectly } from '../utils/printer';
  import { Printer, QrCode, X } from 'lucide-svelte';

  export let isOpen = false;
  export let product: Product | null = null;
  export let onClose: () => void;

  let labelType: 'barcode' | 'etiquette' = 'barcode';
  let widthMm = 50;
  let heightMm = 30;
  let showShopName = true;
  let showPrice = true;
  let showBarcode = true;
  let copies = 1;
  let previewContainer: HTMLDivElement;

  function triggerPrint() {
    if (!previewContainer) return;
    let combinedHtml = '';
    for (let i = 0; i < copies; i++) {
      combinedHtml += `<div style="page-break-after: always; padding: 2mm; text-align: center; border: 1px dashed #ccc; margin-bottom: 2mm;">${previewContainer.innerHTML}</div>`;
    }
    printHtmlDirectly(combinedHtml, 'Barcode Label');
  }
</script>

{#if isOpen && product}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-2xl shadow-2xl w-full max-w-md overflow-hidden animate-in fade-in duration-150">
      <div class="flex items-center justify-between px-5 py-3.5 border-b border-pos-border bg-slate-50 dark:bg-slate-800/50">
        <h3 class="font-black text-sm text-pos-text flex items-center gap-2">
          <QrCode class="w-4 h-4 text-sky-500" />
          <span>Print Product Label / Price Etiquette</span>
        </h3>
        <button on:click={onClose} class="text-pos-muted hover:text-pos-text p-1 rounded-lg cursor-pointer">
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="p-5 space-y-4">
        <!-- Label Type Tabs -->
        <div class="grid grid-cols-2 gap-2">
          <button
            type="button"
            on:click={() => (labelType = 'barcode')}
            class="p-2.5 rounded-xl border font-bold text-xs transition cursor-pointer {labelType === 'barcode' ? 'border-sky-500 bg-sky-50 dark:bg-sky-950 text-sky-600' : 'border-pos-border text-pos-muted'}"
          >
            Barcode Label (ملصق باركود)
          </button>
          <button
            type="button"
            on:click={() => (labelType = 'etiquette')}
            class="p-2.5 rounded-xl border font-bold text-xs transition cursor-pointer {labelType === 'etiquette' ? 'border-sky-500 bg-sky-50 dark:bg-sky-950 text-sky-600' : 'border-pos-border text-pos-muted'}"
          >
            Shelf Etiquette (بطاقة رف وسعر)
          </button>
        </div>

        <!-- Preview Card -->
        <div
          bind:this={previewContainer}
          class="p-4 bg-white text-slate-900 border-2 border-dashed border-slate-300 rounded-xl flex flex-col items-center justify-center text-center space-y-1 shadow-xs"
        >
          {#if showShopName}
            <span class="text-[10px] text-slate-500 font-bold uppercase tracking-wider">TitaouPosT Supermarché</span>
          {/if}
          <h4 class="font-black text-sm text-slate-900 leading-tight">{product.name_ar || product.name_fr}</h4>
          {#if showBarcode && product.barcodes && product.barcodes.length > 0}
            <div class="w-full text-center py-1">
              <!-- Render clean barcode representation -->
              <div class="font-mono text-base tracking-[0.25em] font-black border-y border-slate-900 py-1 inline-block px-4">
                ||| | |||| | |||
              </div>
              <p class="text-[11px] font-mono font-bold text-slate-800 tracking-wider mt-0.5">{product.barcodes[0]}</p>
            </div>
          {/if}
          {#if showPrice}
            <span class="text-base font-black text-slate-900 font-mono">
              {product.sale_price.toLocaleString()} DZD
            </span>
          {/if}
        </div>

        <!-- Print Parameters Form -->
        <div class="grid grid-cols-3 gap-2">
          <div>
            <label for="label-copies" class="block text-[11px] font-bold text-pos-muted mb-1">Copies</label>
            <input id="label-copies" type="number" bind:value={copies} min="1" class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-bold font-mono outline-none" />
          </div>
          <div>
            <label for="label-width" class="block text-[11px] font-bold text-pos-muted mb-1">Width (mm)</label>
            <input id="label-width" type="number" bind:value={widthMm} class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-bold font-mono outline-none" />
          </div>
          <div>
            <label for="label-height" class="block text-[11px] font-bold text-pos-muted mb-1">Height (mm)</label>
            <input id="label-height" type="number" bind:value={heightMm} class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-bold font-mono outline-none" />
          </div>
        </div>
      </div>

      <!-- Action Footer -->
      <div class="px-5 py-3.5 border-t border-pos-border bg-slate-50 dark:bg-slate-800/50 flex justify-end gap-2">
        <button on:click={onClose} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-xl cursor-pointer">
          Cancel
        </button>
        <button
          type="button"
          on:click={triggerPrint}
          class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-md transition active:scale-95"
        >
          <Printer class="w-4 h-4" />
          <span>Print Labels ({copies})</span>
        </button>
      </div>
    </div>
  </div>
{/if}