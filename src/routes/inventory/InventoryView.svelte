<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Category, Product, Unit } from '../../lib/types';
  import TokenizedBarcodeInput from '../../lib/components/TokenizedBarcodeInput.svelte';
  import { Plus, Package, Search, Barcode, Layers } from 'lucide-svelte';

  let products: Product[] = [];
  let categories: Category[] = [];
  let units: Unit[] = [];

  let searchQuery = '';
  let isAddOpen = false;

  let sku = '';
  let nameAr = '';
  let nameFr = '';
  let nameEn = '';
  let categoryId: number | null = null;
  let unitId: number | null = null;
  let purchasePrice = 0;
  let salePrice = 0;
  let minSalePrice = 0;
  let currentStock = 0;
  let minStock = 5;
  let barcodes: string[] = [];
  let isBundle = false;

  onMount(async () => {
    await loadData();
  });

  async function loadData() {
    try {
      categories = await invoke<Category[]>('get_categories');
      units = await invoke<Unit[]>('get_units');
      products = await invoke<Product[]>('search_products', {
        query: searchQuery,
        categoryId: null,
        searchType: 'all',
      });
    } catch (e) {
      console.error(e);
    }
  }

  async function handleSaveProduct() {
    try {
      await invoke('save_product', {
        input: {
          sku: sku || null,
          name_ar: nameAr || nameEn,
          name_fr: nameFr || nameEn,
          name_en: nameEn || nameAr,
          category_id: categoryId,
          unit_id: unitId,
          purchase_price: purchasePrice,
          sale_price: salePrice,
          min_sale_price: minSalePrice,
          tax_rate: 0,
          current_stock: currentStock,
          min_stock: minStock,
          image_path: null,
          is_bundle: isBundle,
          barcodes,
          bundle_items: null,
        },
        productId: null,
      });

      isAddOpen = false;
      sku = '';
      nameAr = '';
      nameEn = '';
      purchasePrice = 0;
      salePrice = 0;
      currentStock = 0;
      barcodes = [];
      await loadData();
    } catch (e) {
      console.error(e);
    }
  }
</script>

<div class="p-6 space-y-4 overflow-y-auto h-full">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-black text-pos-text">Inventory & Product Catalog (المخزون والمنتجات)</h1>
      <p class="text-xs text-pos-muted mt-1">Multi-barcodes, units, bundles, and minimum threshold management</p>
    </div>
    <button
      on:click={() => isAddOpen = true}
      class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-lg transition flex items-center gap-1.5 shadow-xs cursor-pointer"
    >
      <Plus class="w-4 h-4" />
      <span>New Product</span>
    </button>
  </div>

  {#if isAddOpen}
    <div class="bg-pos-card border border-pos-border rounded-xl p-5 shadow-sm space-y-4 animate-in fade-in duration-150">
      <h3 class="font-black text-sm text-pos-text flex items-center gap-2">
        <Package class="w-4 h-4 text-sky-500" />
        <span>Create Product with Multi-Barcodes & Tokens</span>
      </h3>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Name (Arabic / العربية)</label>
          <input type="text" bind:value={nameAr} placeholder="مثال: حليب كامل الدسم 1ل" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs text-pos-text font-bold" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Name (French / Français)</label>
          <input type="text" bind:value={nameFr} placeholder="ex: Lait Entier 1L" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">SKU / Code</label>
          <input type="text" bind:value={sku} placeholder="SKU-1002" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs text-pos-text font-mono" />
        </div>
      </div>

      <!-- Tokenized Barcodes Input -->
      <div>
        <label class="block text-xs font-bold text-pos-muted mb-1">Product Barcodes (Tokenized Tag Input - Scan or Type & Press Enter)</label>
        <TokenizedBarcodeInput bind:barcodes />
      </div>

      <div class="grid grid-cols-2 md:grid-cols-5 gap-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Purchase Price (DZD)</label>
          <input type="number" bind:value={purchasePrice} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs font-mono text-pos-text font-bold" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Sale Price (DZD)</label>
          <input type="number" bind:value={salePrice} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs font-mono text-sky-600 font-extrabold" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Min Sale Price (DZD)</label>
          <input type="number" bind:value={minSalePrice} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Current Stock Qty</label>
          <input type="number" bind:value={currentStock} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs font-mono text-pos-text font-bold" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Min Stock Warning</label>
          <input type="number" bind:value={minStock} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs font-mono text-pos-text" />
        </div>
      </div>

      <div class="flex justify-end gap-2 pt-2 border-t border-pos-border/60">
        <button on:click={() => isAddOpen = false} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded">Cancel</button>
        <button on:click={handleSaveProduct} class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white text-xs font-bold rounded shadow-xs">Save Product</button>
      </div>
    </div>
  {/if}

  <div class="bg-pos-card border border-pos-border rounded-xl shadow-xs overflow-hidden">
    <table class="w-full text-start text-xs border-collapse">
      <thead>
        <tr class="border-b border-pos-border bg-slate-50 dark:bg-slate-800/60 text-pos-muted font-bold">
          <th class="p-3 text-start">SKU</th>
          <th class="p-3 text-start">Product Name</th>
          <th class="p-3 text-start">Barcodes</th>
          <th class="p-3 text-end">Cost Price</th>
          <th class="p-3 text-end">Sale Price</th>
          <th class="p-3 text-center">Stock Level</th>
        </tr>
      </thead>
      <tbody>
        {#each products as p}
          <tr class="border-b border-pos-border/60 hover:bg-slate-50 dark:hover:bg-slate-800/40">
            <td class="p-3 font-mono font-bold text-pos-muted">{p.sku || '-'}</td>
            <td class="p-3 font-bold text-pos-text">{p.name_ar || p.name_en}</td>
            <td class="p-3">
              <div class="flex flex-wrap gap-1">
                {#each p.barcodes as bc}
                  <span class="px-1.5 py-0.5 rounded bg-slate-100 dark:bg-slate-800 font-mono text-[10px] text-pos-muted">{bc}</span>
                {/each}
              </div>
            </td>
            <td class="p-3 text-end font-mono text-pos-muted">{p.purchase_price.toLocaleString()} DZD</td>
            <td class="p-3 text-end font-mono font-bold text-sky-600">{p.sale_price.toLocaleString()} DZD</td>
            <td class="p-3 text-center">
              {#if p.current_stock > p.min_stock}
                <span class="px-2 py-0.5 rounded-full text-[11px] font-bold bg-emerald-100 text-emerald-800">{p.current_stock} in stock</span>
              {:else if p.current_stock > 0}
                <span class="px-2 py-0.5 rounded-full text-[11px] font-bold bg-amber-100 text-amber-800">{p.current_stock} low</span>
              {:else}
                <span class="px-2 py-0.5 rounded-full text-[11px] font-bold bg-rose-100 text-rose-800">0 out</span>
              {/if}
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</div>