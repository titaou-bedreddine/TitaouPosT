<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Category, Product, Unit } from '../../lib/types';
  import PrintLabelModal from '../../lib/components/PrintLabelModal.svelte';
  import { Package, Plus, Search, Edit2, Trash2, QrCode, Printer } from 'lucide-svelte';

  let products: Product[] = [];
  let categories: Category[] = [];
  let units: Unit[] = [];
  let searchQuery = '';
  let selectedCategory: number | null = null;

  let isAddOpen = false;
  let isPrintOpen = false;
  let selectedProductForPrint: Product | null = null;

  let nameAr = '';
  let nameFr = '';
  let sku = '';
  let barcodeInput = '';
  let purchasePrice = 0;
  let salePrice = 0;
  let minSalePrice = 0;
  let currentStock = 0;
  let minStock = 5;
  let categoryId: number | null = null;
  let unitId: number | null = 1;
  let imagePath = '';
  let editingId: number | null = null;

  onMount(async () => {
    await loadCategories();
    await loadUnits();
    await loadProducts();
  });

  async function loadCategories() {
    try {
      categories = await invoke<Category[]>('get_categories');
    } catch (e) {
      console.error(e);
    }
  }

  async function loadUnits() {
    try {
      units = await invoke<Unit[]>('get_units');
    } catch (e) {
      console.error(e);
    }
  }

  async function loadProducts() {
    try {
      products = await invoke<Product[]>('search_products', {
        query: searchQuery,
        categoryId: selectedCategory,
        searchType: 'all',
      });
    } catch (e) {
      console.error(e);
    }
  }

  async function handleSave() {
    if (!nameAr.trim() && !nameFr.trim()) return;
    try {
      const barcodes = barcodeInput
        .split(',')
        .map(b => b.trim())
        .filter(b => b.length > 0);

      await invoke('save_product', {
        input: {
          sku: sku || null,
          name_ar: nameAr || nameFr,
          name_fr: nameFr || nameAr,
          name_en: nameFr || nameAr,
          category_id: categoryId,
          unit_id: unitId,
          purchase_price: purchasePrice,
          sale_price: salePrice,
          min_sale_price: minSalePrice,
          tax_rate: 0,
          current_stock: currentStock,
          min_stock: minStock,
          image_path: imagePath || null,
          is_bundle: false,
          barcodes,
        },
        productId: editingId,
      });

      isAddOpen = false;
      resetForm();
      await loadProducts();
    } catch (e) {
      console.error(e);
    }
  }

  function startEdit(p: Product) {
    editingId = p.id;
    nameAr = p.name_ar;
    nameFr = p.name_fr;
    sku = p.sku || '';
    barcodeInput = p.barcodes.join(', ');
    purchasePrice = p.purchase_price;
    salePrice = p.sale_price;
    minSalePrice = p.min_sale_price;
    currentStock = p.current_stock;
    minStock = p.min_stock;
    categoryId = p.category_id || null;
    unitId = p.unit_id || 1;
    imagePath = p.image_path || '';
    isAddOpen = true;
  }

  function resetForm() {
    editingId = null;
    nameAr = '';
    nameFr = '';
    sku = '';
    barcodeInput = '';
    purchasePrice = 0;
    salePrice = 0;
    minSalePrice = 0;
    currentStock = 0;
    minStock = 5;
    imagePath = '';
  }

  async function handleDelete(id: number) {
    try {
      await invoke('delete_product', { productId: id });
      await loadProducts();
    } catch (e) {
      console.error(e);
    }
  }

  function openPrintLabel(p: Product) {
    selectedProductForPrint = p;
    isPrintOpen = true;
  }
</script>

<div class="p-6 space-y-4 overflow-y-auto h-full select-none">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-black text-pos-text">Inventory Catalog (المخزون والمنتجات)</h1>
      <p class="text-xs text-pos-muted mt-1">Manage product details, multiple barcodes, stock levels, and print price tags</p>
    </div>
    <button
      on:click={() => { resetForm(); isAddOpen = true; }}
      class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-xl transition flex items-center gap-1.5 shadow-xs cursor-pointer"
    >
      <Plus class="w-4 h-4" />
      <span>New Product</span>
    </button>
  </div>

  {#if isAddOpen}
    <div class="bg-pos-card border border-pos-border rounded-2xl p-5 shadow-sm space-y-4 animate-in fade-in duration-150">
      <h3 class="font-extrabold text-sm text-pos-text">{editingId ? 'Edit Product' : 'Add New Product'}</h3>
      <div class="grid grid-cols-1 md:grid-cols-4 gap-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Product Name (Arabic)</label>
          <input type="text" bind:value={nameAr} placeholder="e.g. مياه معدنية 1.5 لتر" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Product Name (French)</label>
          <input type="text" bind:value={nameFr} placeholder="e.g. Eau Minérale 1.5L" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">SKU / Reference</label>
          <input type="text" bind:value={sku} placeholder="PRD-001" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Barcodes (Comma Separated)</label>
          <input type="text" bind:value={barcodeInput} placeholder="6130001001, 6130001002" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-5 gap-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Category / Group</label>
          <select bind:value={categoryId} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text">
            <option value={null}>General (عام)</option>
            {#each categories as cat}
              <option value={cat.id}>{cat.name_ar || cat.name_fr}</option>
            {/each}
          </select>
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Purchase Cost (DZD)</label>
          <input type="number" bind:value={purchasePrice} on:focus={(e) => (e.target as HTMLInputElement).select()} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono font-bold text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Sale Price (DZD)</label>
          <input type="number" bind:value={salePrice} on:focus={(e) => (e.target as HTMLInputElement).select()} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono font-bold text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Current Stock Qty</label>
          <input type="number" bind:value={currentStock} on:focus={(e) => (e.target as HTMLInputElement).select()} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono font-bold text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Min Stock Alert</label>
          <input type="number" bind:value={minStock} on:focus={(e) => (e.target as HTMLInputElement).select()} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono font-bold text-pos-text" />
        </div>
      </div>

      <div class="flex justify-end gap-2 pt-2 border-t border-pos-border/60">
        <button on:click={() => isAddOpen = false} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-lg">Cancel</button>
        <button on:click={handleSave} class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white text-xs font-bold rounded-lg shadow-xs">Save Product</button>
      </div>
    </div>
  {/if}

  <!-- Products List Table -->
  <div class="bg-pos-card border border-pos-border rounded-2xl shadow-xs overflow-hidden">
    <table class="w-full text-start text-xs border-collapse">
      <thead>
        <tr class="border-b border-pos-border bg-slate-50 dark:bg-slate-800/40 text-pos-muted font-bold">
          <th class="p-3 text-start">Image</th>
          <th class="p-3 text-start">Product Name</th>
          <th class="p-3 text-start">Category</th>
          <th class="p-3 text-start">Barcodes</th>
          <th class="p-3 text-end">Purchase Cost</th>
          <th class="p-3 text-end">Sale Price</th>
          <th class="p-3 text-center">Stock</th>
          <th class="p-3 text-center">Actions</th>
        </tr>
      </thead>
      <tbody>
        {#each products as p}
          <tr class="border-b border-pos-border/60 hover:bg-slate-50 dark:hover:bg-slate-800/40">
            <td class="p-3">
              <div class="w-9 h-9 rounded-lg bg-slate-100 dark:bg-slate-800 flex items-center justify-center overflow-hidden border border-pos-border">
                {#if p.image_path}
                  <img src={p.image_path} alt={p.name_ar} class="w-full h-full object-cover" />
                {:else}
                  <Package class="w-4 h-4 text-pos-muted/40" />
                {/if}
              </div>
            </td>
            <td class="p-3 font-bold text-pos-text text-xs">{p.name_ar}</td>
            <td class="p-3 text-pos-muted">{p.category_name || '-'}</td>
            <td class="p-3 font-mono text-[11px] text-pos-muted">{p.barcodes.join(', ') || p.sku || '-'}</td>
            <td class="p-3 text-end font-mono text-pos-muted">{p.purchase_price.toLocaleString()} DZD</td>
            <td class="p-3 text-end font-mono font-black text-sm text-sky-600">{p.sale_price.toLocaleString()} DZD</td>
            <td class="p-3 text-center">
              <span class="px-2 py-0.5 rounded-full text-[11px] font-bold font-mono {p.current_stock <= 0 ? 'bg-rose-100 text-rose-800' : p.current_stock <= p.min_stock ? 'bg-amber-100 text-amber-800' : 'bg-emerald-100 text-emerald-800'}">
                {p.current_stock}
              </span>
            </td>
            <td class="p-3 text-center">
              <div class="flex items-center justify-center gap-1">
                <button
                  type="button"
                  on:click={() => openPrintLabel(p)}
                  class="p-1.5 text-sky-600 hover:bg-sky-50 dark:hover:bg-sky-950 rounded-lg cursor-pointer"
                  title="Print Barcode / Etiquette"
                >
                  <Printer class="w-3.5 h-3.5" />
                </button>
                <button
                  type="button"
                  on:click={() => startEdit(p)}
                  class="p-1.5 text-sky-600 hover:bg-sky-50 dark:hover:bg-sky-950 rounded-lg cursor-pointer"
                >
                  <Edit2 class="w-3.5 h-3.5" />
                </button>
                <button
                  type="button"
                  on:click={() => handleDelete(p.id)}
                  class="p-1.5 text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 rounded-lg cursor-pointer"
                >
                  <Trash2 class="w-3.5 h-3.5" />
                </button>
              </div>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>

  <PrintLabelModal
    isOpen={isPrintOpen}
    product={selectedProductForPrint}
    onClose={() => isPrintOpen = false}
  />
</div>