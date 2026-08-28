<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { t, currentLocale } from '../i18n';
  import type { Category, Product, ProductInput, Unit } from '../types';
  import {
    X, Check, Plus, Trash2, Edit2, Tag, Upload, Calendar,
    DollarSign, Package, History, Layers, AlertTriangle
  } from 'lucide-svelte';

  export let isOpen = false;
  export let product: Product | null = null;
  export let categories: Category[] = [];
  export let units: Unit[] = [];
  export let onClose: () => void;
  export let onSaved: () => void;

  let activeTab: 'details' | 'history' = 'details';

  let sku = '';
  let nameAr = '';
  let nameFr = '';
  let nameEn = '';
  let categoryId: number | null = 1;
  let unitId: number | null = 1;
  let purchasePrice = 0;
  let salePrice = 0;
  let minSalePrice = 0;
  let taxRate = 19;
  let currentStock = 100;
  let minStock = 5;
  let imagePath = '';
  let expiryDate = '';
  let isBundle = false;

  let barcodeTokens: string[] = [];
  let currentBarcodeTyped = '';
  let editingTokenIndex: number | null = null;

  let isSaving = false;
  let errorMsg = '';

  $: if (isOpen && product) {
    sku = product.sku || '';
    nameAr = product.name_ar || '';
    nameFr = product.name_fr || '';
    nameEn = product.name_en || '';
    categoryId = product.category_id || (categories.length > 0 ? categories[0].id : 1);
    unitId = product.unit_id || (units.length > 0 ? units[0].id : 1);
    purchasePrice = product.purchase_price || 0;
    salePrice = product.sale_price || 0;
    minSalePrice = product.min_sale_price || 0;
    taxRate = product.tax_rate || 19;
    currentStock = product.current_stock || 0;
    minStock = product.min_stock || 5;
    imagePath = product.image_path || '';
    expiryDate = product.expiry_date || '';
    isBundle = product.is_bundle || false;
    barcodeTokens = product.barcodes ? [...product.barcodes] : [];
    currentBarcodeTyped = '';
    editingTokenIndex = null;
    activeTab = 'details';
  } else if (isOpen && !product) {
    sku = 'PRD-' + Math.floor(1000 + Math.random() * 9000);
    nameAr = '';
    nameFr = '';
    nameEn = '';
    categoryId = categories.length > 0 ? categories[0].id : 1;
    unitId = units.length > 0 ? units[0].id : 1;
    purchasePrice = 0;
    salePrice = 0;
    minSalePrice = 0;
    taxRate = 19;
    currentStock = 50;
    minStock = 5;
    imagePath = '';
    expiryDate = '';
    isBundle = false;
    barcodeTokens = ['613' + Math.floor(100000000 + Math.random() * 900000000)];
    currentBarcodeTyped = '';
    editingTokenIndex = null;
    activeTab = 'details';
  }

  function addBarcodeToken() {
    const raw = currentBarcodeTyped.trim().replace(/,/g, '');
    if (!raw) return;
    if (editingTokenIndex !== null) {
      barcodeTokens[editingTokenIndex] = raw;
      editingTokenIndex = null;
    } else {
      if (!barcodeTokens.includes(raw)) {
        barcodeTokens = [...barcodeTokens, raw];
      }
    }
    currentBarcodeTyped = '';
  }

  function handleBarcodeKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ',') {
      e.preventDefault();
      addBarcodeToken();
    }
  }

  function removeBarcodeToken(index: number) {
    barcodeTokens = barcodeTokens.filter((_, i) => i !== index);
  }

  function startEditToken(index: number) {
    editingTokenIndex = index;
    currentBarcodeTyped = barcodeTokens[index];
  }

  function handleImageUpload(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files[0]) {
      const file = target.files[0];
      const reader = new FileReader();
      reader.onload = (ev) => {
        imagePath = ev.target?.result as string;
      };
      reader.readAsDataURL(file);
    }
  }

  async function handleSave() {
    if (!nameFr && !nameAr && !nameEn) {
      errorMsg = 'Please provide a product name / الرجاء إدخال اسم المنتج';
      return;
    }

    if (currentBarcodeTyped.trim()) {
      addBarcodeToken();
    }

    try {
      isSaving = true;
      errorMsg = '';
      const input: ProductInput = {
        sku: sku || null,
        name_ar: nameAr || nameFr || nameEn,
        name_fr: nameFr || nameAr || nameEn,
        name_en: nameEn || nameFr || nameAr,
        category_id: categoryId,
        unit_id: unitId,
        purchase_price: purchasePrice,
        sale_price: salePrice,
        min_sale_price: minSalePrice,
        tax_rate: taxRate,
        current_stock: currentStock,
        min_stock: minStock,
        image_path: imagePath || null,
        expiry_date: expiryDate || null,
        is_bundle: isBundle,
        barcodes: barcodeTokens,
      };

      await invoke('save_product', {
        input,
        productId: product ? product.id : null,
      });

      onSaved();
      onClose();
    } catch (err: any) {
      errorMsg = typeof err === 'string' ? err : err.message || 'Failed to save product';
    } finally {
      isSaving = false;
    }
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl w-full max-w-2xl overflow-hidden animate-in zoom-in-95 duration-150 flex flex-col max-h-[90vh]">
      <div class="flex items-center justify-between px-6 py-4 border-b border-pos-border bg-slate-50 dark:bg-slate-800/60">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-2xl bg-sky-600/10 text-sky-600 flex items-center justify-center font-bold">
            <Package class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-black text-base text-pos-text">
              {product ? 'Edit Product / تعديل منتج' : 'Add New Product / إضافة منتج جديد'}
            </h3>
            <p class="text-xs text-pos-muted">SKU: {sku}</p>
          </div>
        </div>

        <div class="flex items-center gap-2">
          {#if product}
            <div class="flex items-center bg-slate-200 dark:bg-slate-700 p-1 rounded-xl text-xs font-bold">
              <button
                type="button"
                on:click={() => (activeTab = 'details')}
                class="px-3 py-1 rounded-lg transition cursor-pointer {activeTab === 'details' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted'}"
              >
                Details
              </button>
              <button
                type="button"
                on:click={() => (activeTab = 'history')}
                class="px-3 py-1 rounded-lg transition cursor-pointer {activeTab === 'history' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted'}"
              >
                History
              </button>
            </div>
          {/if}

          <button on:click={onClose} class="text-pos-muted hover:text-pos-text p-1.5 rounded-xl cursor-pointer">
            <X class="w-5 h-5" />
          </button>
        </div>
      </div>

      {#if errorMsg}
        <div class="mx-6 mt-4 p-3 bg-rose-100 text-rose-800 text-xs font-bold rounded-xl">
          {errorMsg}
        </div>
      {/if}

      <div class="p-6 overflow-y-auto flex-1 space-y-4">
        {#if activeTab === 'details'}
          <div class="flex flex-col md:flex-row gap-4 items-start">
            <div class="flex flex-col items-center gap-2 shrink-0">
              <div class="w-24 h-24 rounded-2xl bg-slate-100 dark:bg-slate-800 border-2 border-dashed border-pos-border flex items-center justify-center overflow-hidden relative group">
                {#if imagePath}
                  <img src={imagePath} alt="Product" class="w-full h-full object-cover" />
                {:else}
                  <Package class="w-8 h-8 text-pos-muted/40" />
                {/if}
              </div>
              <label class="px-2.5 py-1 bg-sky-600 hover:bg-sky-700 text-white text-[11px] font-bold rounded-lg cursor-pointer flex items-center gap-1">
                <Upload class="w-3 h-3" />
                <span>Upload</span>
                <input type="file" accept="image/*" on:change={handleImageUpload} class="hidden" />
              </label>
            </div>

            <div class="flex-1 grid grid-cols-1 md:grid-cols-2 gap-3 w-full">
              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">Product Name (Français / French)</label>
                <input type="text" bind:value={nameFr} placeholder="Ex: Sucre Blanc 1kg" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
              </div>
              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">Product Name (العربية / Arabic)</label>
                <input type="text" bind:value={nameAr} placeholder="مثال: سكر أبيض 1 كغ" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
              </div>
              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">Product Name (English)</label>
                <input type="text" bind:value={nameEn} placeholder="Ex: White Sugar 1kg" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
              </div>
              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">SKU / Code Reference</label>
                <input type="text" bind:value={sku} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
              </div>
            </div>
          </div>

          <div class="p-4 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-2">
            <div class="flex items-center justify-between">
              <label class="block text-xs font-black text-pos-text flex items-center gap-1.5">
                <Tag class="w-3.5 h-3.5 text-sky-500" />
                <span>Barcodes / رموز الباركود</span>
              </label>
              <span class="text-[10px] text-pos-muted">Press Enter or Comma (,) to add</span>
            </div>

            <div class="flex flex-wrap gap-1.5 min-h-[34px] p-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl">
              {#each barcodeTokens as token, idx}
                <div class="inline-flex items-center gap-1 px-2.5 py-1 bg-sky-100 dark:bg-sky-950 text-sky-800 dark:text-sky-300 rounded-lg text-xs font-mono font-bold shadow-xs">
                  <span on:dblclick={() => startEditToken(idx)} class="cursor-pointer">{token}</span>
                  <button type="button" on:click={() => startEditToken(idx)} class="p-0.5 hover:text-sky-600 cursor-pointer">
                    <Edit2 class="w-2.5 h-2.5" />
                  </button>
                  <button type="button" on:click={() => removeBarcodeToken(idx)} class="p-0.5 hover:text-rose-600 cursor-pointer">
                    <X class="w-3 h-3" />
                  </button>
                </div>
              {/each}

              <input
                type="text"
                bind:value={currentBarcodeTyped}
                on:keydown={handleBarcodeKeyDown}
                placeholder={barcodeTokens.length === 0 ? "Scan or type barcode, then hit Enter..." : "+ Add barcode..."}
                class="flex-1 min-w-[140px] bg-transparent border-0 text-xs font-mono text-pos-text outline-none px-1"
              />
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Category / الفئة</label>
              <select bind:value={categoryId} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none">
                {#each categories as cat}
                  <option value={cat.id}>{cat.name_fr} / {cat.name_ar}</option>
                {/each}
              </select>
            </div>

            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Unit of Measure / وحدة القياس</label>
              <select bind:value={unitId} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none">
                {#each units as u}
                  <option value={u.id}>{u.name}</option>
                {/each}
              </select>
            </div>
          </div>

          <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Purchase Price (DZD)</label>
              <input type="number" min="0" bind:value={purchasePrice} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
            </div>

            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Sale Price (DZD)</label>
              <input type="number" min="0" bind:value={salePrice} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-black text-sky-600 outline-none focus:ring-2 focus:ring-sky-500" />
            </div>

            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Min Sale Price (DZD)</label>
              <input type="number" min="0" bind:value={minSalePrice} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
            </div>

            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Expiry Date / تاريخ الصلاحية</label>
              <input type="date" bind:value={expiryDate} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
            </div>
          </div>

          <div class="grid grid-cols-2 gap-3">
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Current Stock / المخزون الحالي</label>
              <input type="number" min="0" bind:value={currentStock} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none" />
            </div>
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Min Stock Alert / حد التنبيه</label>
              <input type="number" min="0" bind:value={minStock} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none" />
            </div>
          </div>
        {:else}
          <div class="space-y-4">
            <h4 class="font-black text-sm text-pos-text flex items-center gap-2">
              <History class="w-4 h-4 text-sky-500" />
              <span>Product Movements & Activity Logs</span>
            </h4>

            <div class="grid grid-cols-3 gap-3">
              <div class="p-3 bg-slate-100 dark:bg-slate-800 rounded-xl text-center">
                <span class="text-xs text-pos-muted font-bold block">Total Sold</span>
                <span class="text-base font-black text-emerald-600 font-mono">142 pcs</span>
              </div>
              <div class="p-3 bg-slate-100 dark:bg-slate-800 rounded-xl text-center">
                <span class="text-xs text-pos-muted font-bold block">Total Purchased</span>
                <span class="text-base font-black text-sky-600 font-mono">200 pcs</span>
              </div>
              <div class="p-3 bg-slate-100 dark:bg-slate-800 rounded-xl text-center">
                <span class="text-xs text-pos-muted font-bold block">Total Returned</span>
                <span class="text-base font-black text-amber-600 font-mono">2 pcs</span>
              </div>
            </div>

            <div class="p-4 bg-slate-50 dark:bg-slate-800/40 rounded-xl border border-pos-border text-xs text-pos-muted text-center">
              All transactions for this product are recorded in the central database ledger.
            </div>
          </div>
        {/if}
      </div>

      <div class="px-6 py-4 border-t border-pos-border bg-slate-50 dark:bg-slate-800/60 flex items-center justify-end gap-3">
        <button on:click={onClose} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-pos-text font-bold text-xs rounded-xl cursor-pointer">
          Cancel
        </button>
        <button on:click={handleSave} disabled={isSaving} class="px-6 py-2 bg-sky-600 hover:bg-sky-700 disabled:opacity-50 text-white font-black text-xs rounded-xl shadow-md cursor-pointer flex items-center gap-1.5">
          <Check class="w-4 h-4" />
          <span>{isSaving ? 'Saving...' : 'Save Product (حفظ)'}</span>
        </button>
      </div>
    </div>
  </div>
{/if}