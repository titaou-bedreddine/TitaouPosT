<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Category, Product, Unit } from '../../lib/types';
  import ProductEditModal from '../../lib/components/ProductEditModal.svelte';
  import PrintLabelModal from '../../lib/components/PrintLabelModal.svelte';
  import UniversalSearchBar from '../../lib/components/UniversalSearchBar.svelte';
  import { printHtmlDirectly } from '../../lib/utils/printer';
  import {
    Package, Plus, Search, Edit2, Trash2, QrCode, Printer,
    ArrowUpDown, AlertTriangle, AlertOctagon, Tag
  } from 'lucide-svelte';

  let products: Product[] = [];
  let categories: Category[] = [];
  let units: Unit[] = [];

  let searchQuery = '';
  let searchType: 'all' | 'name' | 'barcode' | 'price' | 'qr' = 'all';
  let selectedCategory: number | null = null;
  let sortBy: 'default' | 'name_asc' | 'name_desc' | 'price_asc' | 'price_desc' | 'stock' = 'default';

  let isProductEditOpen = false;
  let editingProduct: Product | null = null;
  let isPrintOpen = false;
  let selectedProductForPrint: Product | null = null;

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
      const list = await invoke<Product[]>('search_products', {
        query: searchQuery,
        categoryId: selectedCategory,
        searchType: searchType === 'qr' ? 'barcode' : searchType,
      });

      if (sortBy === 'name_asc') {
        list.sort((a, b) => (a.name_fr || a.name_ar).localeCompare(b.name_fr || b.name_ar));
      } else if (sortBy === 'name_desc') {
        list.sort((a, b) => (b.name_fr || b.name_ar).localeCompare(a.name_fr || a.name_ar));
      } else if (sortBy === 'price_asc') {
        list.sort((a, b) => a.sale_price - b.sale_price);
      } else if (sortBy === 'price_desc') {
        list.sort((a, b) => b.sale_price - a.sale_price);
      } else if (sortBy === 'stock') {
        list.sort((a, b) => b.current_stock - a.current_stock);
      }

      products = list;
    } catch (e) {
      console.error(e);
    }
  }

  function openAddNew() {
    editingProduct = null;
    isProductEditOpen = true;
  }

  function openEdit(p: Product) {
    editingProduct = p;
    isProductEditOpen = true;
  }

  async function handleDelete(id: number) {
    if (!confirm('Are you sure you want to delete this product?')) return;
    try {
      await invoke('delete_product', { productId: id });
      await loadProducts();
    } catch (e) {
      console.error(e);
    }
  }

  function printShelfTag(p: Product) {
    const barcode = (p.barcodes && p.barcodes[0]) || p.sku || '12345678';
    const html = `
      <div style="width: 40mm; height: 20mm; padding: 2mm; border: 1px solid #000; text-align: center; display: flex; flex-direction: column; justify-content: space-between;">
        <p style="font-size: 8px; font-weight: bold; margin: 0; overflow: hidden; white-space: nowrap;">${p.name_fr || p.name_ar}</p>
        <p style="font-size: 11px; font-weight: 900; margin: 0;">${p.sale_price.toLocaleString()} DZD</p>
        <p style="font-size: 7px; font-family: monospace; margin: 0;">${barcode}</p>
      </div>
    `;
    printHtmlDirectly(html, `Shelf Tag - ${p.sku}`);
  }
</script>

<div class="h-full flex flex-col bg-pos-bg p-4 overflow-hidden select-none space-y-3">
  <!-- Header -->
  <div class="flex items-center justify-between pb-3 border-b border-pos-border shrink-0">
    <div class="flex items-center gap-3">
      <div class="w-10 h-10 rounded-2xl bg-sky-100 dark:bg-sky-950 text-sky-600 flex items-center justify-center font-bold">
        <Package class="w-5 h-5" />
      </div>
      <div>
        <h1 class="text-xl font-black text-pos-text tracking-tight">Inventory Catalog / إدارة المنتجات والمخزون</h1>
        <p class="text-xs text-pos-muted">Manage product prices, barcodes tokenizer, expiry dates, and shelf labels</p>
      </div>
    </div>

    <div class="flex items-center gap-2">
      <button
        type="button"
        on:click={openAddNew}
        class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white rounded-xl text-xs font-black transition shadow-xs flex items-center gap-2 cursor-pointer active:scale-95"
      >
        <Plus class="w-4 h-4" />
        <span>Add Product (إضافة منتج جديد)</span>
      </button>
    </div>
  </div>

  <!-- Filter & Sort Row -->
  <div class="flex items-center gap-2 shrink-0">
    <div class="flex-1">
      <UniversalSearchBar
        bind:query={searchQuery}
        bind:searchType
        onSearch={loadProducts}
      />
    </div>

    <!-- Category selector -->
    <div class="bg-pos-card border border-pos-border rounded-xl px-3 py-1.5 text-xs font-bold text-pos-text">
      <select bind:value={selectedCategory} on:change={loadProducts} class="bg-transparent outline-none cursor-pointer">
        <option value={null}>All Categories (كل الفئات)</option>
        {#each categories as cat}
          <option value={cat.id}>{cat.name_fr} / {cat.name_ar}</option>
        {/each}
      </select>
    </div>

    <!-- Sort selector -->
    <div class="flex items-center gap-1 bg-pos-card border border-pos-border rounded-xl px-3 py-1.5 text-xs font-bold text-pos-text">
      <ArrowUpDown class="w-3.5 h-3.5 text-pos-muted" />
      <select bind:value={sortBy} on:change={loadProducts} class="bg-transparent outline-none cursor-pointer">
        <option value="default">Default Sort</option>
        <option value="name_asc">Name (A-Z)</option>
        <option value="name_desc">Name (Z-A)</option>
        <option value="price_asc">Price (Low → High)</option>
        <option value="price_desc">Price (High → Low)</option>
        <option value="stock">Highest Stock</option>
      </select>
    </div>
  </div>

  <!-- Products Table -->
  <div class="flex-1 overflow-y-auto bg-pos-card border border-pos-border rounded-2xl shadow-xs">
    <table class="w-full text-start text-xs border-collapse">
      <thead class="bg-slate-50 dark:bg-slate-800/60 border-b border-pos-border text-pos-muted font-bold sticky top-0 z-10">
        <tr>
          <th class="p-3 text-start">SKU</th>
          <th class="p-3 text-start">Product Name</th>
          <th class="p-3 text-start">Category</th>
          <th class="p-3 text-start">Barcodes</th>
          <th class="p-3 text-end">Cost (DZD)</th>
          <th class="p-3 text-end">Sale Price (DZD)</th>
          <th class="p-3 text-center">Stock</th>
          <th class="p-3 text-center">Expiry</th>
          <th class="p-3 text-end">Actions</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-pos-border/40">
        {#if products.length === 0}
          <tr>
            <td colspan="9" class="p-8 text-center text-pos-muted">No products found matching filters.</td>
          </tr>
        {:else}
          {#each products as p}
            <tr class="hover:bg-slate-50 dark:hover:bg-slate-800/40 transition">
              <td class="p-3 font-mono font-bold text-sky-600">{p.sku || '—'}</td>
              <td class="p-3 font-bold text-pos-text">
                <p>{p.name_fr || p.name_ar}</p>
                {#if p.name_ar && p.name_fr}
                  <p class="text-[10px] text-pos-muted">{p.name_ar}</p>
                {/if}
              </td>
              <td class="p-3 text-pos-muted">{p.category_name || 'Général'}</td>
              <td class="p-3">
                <div class="flex flex-wrap gap-1">
                  {#each (p.barcodes || []) as b}
                    <span class="px-1.5 py-0.2 bg-slate-100 dark:bg-slate-800 text-[10px] font-mono rounded font-bold">{b}</span>
                  {/each}
                </div>
              </td>
              <td class="p-3 text-end font-mono text-pos-muted">{p.purchase_price.toLocaleString()} DZD</td>
              <td class="p-3 text-end font-mono font-black text-sky-600">{p.sale_price.toLocaleString()} DZD</td>
              <td class="p-3 text-center">
                <span class="px-2 py-0.5 rounded-full text-[10px] font-black font-mono {p.current_stock > p.min_stock ? 'bg-emerald-100 text-emerald-800 dark:bg-emerald-950 dark:text-emerald-300' : 'bg-rose-100 text-rose-800 dark:bg-rose-950 dark:text-rose-300'}">
                  {p.current_stock}
                </span>
              </td>
              <td class="p-3 text-center font-mono text-[10px]">
                {#if p.expiry_date}
                  <span class="text-pos-muted">{p.expiry_date}</span>
                {:else}
                  <span class="text-pos-muted/40">—</span>
                {/if}
              </td>
              <td class="p-3 text-end">
                <div class="flex items-center justify-end gap-1">
                  <button on:click={() => printShelfTag(p)} class="p-1.5 text-pos-muted hover:text-sky-600 rounded-lg cursor-pointer" title="Print Shelf Tag">
                    <Tag class="w-3.5 h-3.5" />
                  </button>
                  <button on:click={() => openEdit(p)} class="p-1.5 text-pos-muted hover:text-sky-600 rounded-lg cursor-pointer" title="Edit">
                    <Edit2 class="w-3.5 h-3.5" />
                  </button>
                  <button on:click={() => handleDelete(p.id)} class="p-1.5 text-pos-muted hover:text-rose-600 rounded-lg cursor-pointer" title="Delete">
                    <Trash2 class="w-3.5 h-3.5" />
                  </button>
                </div>
              </td>
            </tr>
          {/each}
        {/if}
      </tbody>
    </table>
  </div>
</div>

<ProductEditModal
  isOpen={isProductEditOpen}
  product={editingProduct}
  categories={categories}
  units={units}
  onClose={() => (isProductEditOpen = false)}
  onSaved={loadProducts}
/>

<PrintLabelModal
  isOpen={isPrintOpen}
  product={selectedProductForPrint}
  onClose={() => (isPrintOpen = false)}
/>