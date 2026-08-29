<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { t, currentLocale } from '../i18n';
  import type { Category, Product, ProductInput, Unit } from '../types';
  import PrintLabelModal from './PrintLabelModal.svelte';
  import {
    X, Check, Plus, Trash2, Edit2, Tag, Upload, Calendar,
    DollarSign, Package, History, Layers, AlertTriangle, Scale, RefreshCw, Send,
    Copy, Percent, Sparkles, FolderPlus, QrCode, Printer
  } from 'lucide-svelte';

  export let isOpen = false;
  export let product: Product | null = null;
  export let categories: Category[] = [];
  export let units: Unit[] = [];
  export let initialBarcode: string = '';
  export let onClose: () => void;
  export let onSaved: () => void;

  let activeTab: 'details' | 'scalable' | 'history' = 'details';

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
  let currentStock = 0;
  let minStock = 5;
  let imagePath = '';
  let expiryDate = '';
  let isBundle = false;

  // Scale Integration
  let isScalable = false;
  let scaleCode = '';
  let scalePlu = 1;
  let scaleBarcodeType = 97;
  let scaleDepartmentId = 1;
  let scaleSyncStatus = 'pending';
  let isSyncingScale = false;
  let scaleSyncMsg = '';

  // Barcode tokenizer
  let barcodeTokens: string[] = [];
  let currentBarcodeTyped = '';
  let editingTokenIndex: number | null = null;

  // Profit / Margin Mode
  let marginMode: 'percent' | 'amount' = 'percent';
  let profitMarginPercent = 20;
  let profitMarginAmount = 0;

  // Quick Add Family Modal
  let isQuickFamilyOpen = false;
  let quickFamilyNameFr = '';
  let quickFamilyNameAr = '';
  let quickFamilyColor = '#0284c7';
  let isSavingFamily = false;

  // Quick Add Unit Modal
  let isQuickUnitOpen = false;
  let quickUnitName = '';
  let quickUnitShort = '';
  let isSavingUnit = false;

  // Print Label Modal Integration
  let isPrintLabelOpen = false;
  let printLabelInitialType: 'barcode' | 'etiquette' = 'barcode';
  let printLabelInitialQty = 1;
  let printProductObj: Product | null = null;

  function generateRandomSku() {
    sku = 'PRD-' + Math.floor(1000 + Math.random() * 9000);
  }

  function getTransientProduct(): Product {
    const bCodes = [...barcodeTokens];
    if (currentBarcodeTyped.trim() && !bCodes.includes(currentBarcodeTyped.trim())) {
      bCodes.push(currentBarcodeTyped.trim());
    }
    return {
      id: product ? product.id : 0,
      sku: sku || undefined,
      name_ar: nameAr.trim() || nameFr.trim() || 'Produit',
      name_fr: nameFr.trim() || nameAr.trim() || 'Produit',
      name_en: nameEn.trim() || nameFr.trim() || 'Product',
      category_id: categoryId,
      category_name: categories.find(c => c.id === categoryId)?.name_fr || '',
      unit_id: unitId,
      unit_name: units.find(u => u.id === unitId)?.name || '',
      purchase_price: Number(purchasePrice) || 0,
      sale_price: Number(salePrice) || 0,
      min_sale_price: Number(minSalePrice) || 0,
      tax_rate: Number(taxRate) || 19,
      current_stock: Number(currentStock) || 0,
      min_stock: Number(minStock) || 5,
      image_path: imagePath || undefined,
      expiry_date: expiryDate || undefined,
      is_scalable: isScalable,
      scale_code: scaleCode || undefined,
      scale_plu: isScalable ? scalePlu : undefined,
      scale_barcode_type: isScalable ? scaleBarcodeType : undefined,
      scale_department_id: isScalable ? scaleDepartmentId : undefined,
      scale_sync_status: scaleSyncStatus,
      is_bundle: isBundle,
      barcodes: bCodes,
    };
  }

  function openPrintSticker() {
    printProductObj = getTransientProduct();
    printLabelInitialType = 'barcode';
    printLabelInitialQty = Number(currentStock) > 0 ? Number(currentStock) : 1;
    isPrintLabelOpen = true;
  }

  function openPrintShelf() {
    printProductObj = getTransientProduct();
    printLabelInitialType = 'etiquette';
    printLabelInitialQty = 1;
    isPrintLabelOpen = true;
  }

  let isSaving = false;
  let errorMsg = '';
  let copyFeedback = '';

  function handlePurchasePriceChange() {
    if (marginMode === 'percent') {
      salePrice = Math.round(purchasePrice * (1 + profitMarginPercent / 100));
      profitMarginAmount = salePrice - purchasePrice;
    } else {
      salePrice = purchasePrice + profitMarginAmount;
      if (purchasePrice > 0) {
        profitMarginPercent = Math.round((profitMarginAmount / purchasePrice) * 100);
      }
    }
  }

  function handleMarginPercentChange() {
    if (purchasePrice > 0) {
      salePrice = Math.round(purchasePrice * (1 + profitMarginPercent / 100));
      profitMarginAmount = salePrice - purchasePrice;
    }
  }

  function handleMarginAmountChange() {
    salePrice = purchasePrice + profitMarginAmount;
    if (purchasePrice > 0) {
      profitMarginPercent = Math.round((profitMarginAmount / purchasePrice) * 100);
    }
  }

  function handleSalePriceChange() {
    if (purchasePrice > 0) {
      profitMarginAmount = salePrice - purchasePrice;
      profitMarginPercent = Math.round(((salePrice - purchasePrice) / purchasePrice) * 100);
    }
  }

  function generateValidEan13() {
    // Prefix 613 (Algeria GS1) + 9 random digits = 12 digits
    let base = '613' + Math.floor(100000000 + Math.random() * 900000000).toString().slice(0, 9);
    // Calculate Modulo 10 Checksum
    let sum = 0;
    for (let i = 0; i < 12; i++) {
      const digit = parseInt(base[i]);
      sum += i % 2 === 0 ? digit * 1 : digit * 3;
    }
    const checksum = (10 - (sum % 10)) % 10;
    const ean13 = base + checksum.toString();
    if (!barcodeTokens.includes(ean13)) {
      barcodeTokens = [...barcodeTokens, ean13];
    }
    currentBarcodeTyped = '';
  }

  function generateScalePluCode() {
    scalePlu = Math.floor(100 + Math.random() * 899);
    scaleCode = '95' + Math.floor(1000 + Math.random() * 8999).toString();
  }

  function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text);
    copyFeedback = 'Copied!';
    setTimeout(() => (copyFeedback = ''), 2000);
  }

  let lastIsOpen = false;
  let lastProductId: number | null | undefined = undefined;
  let lastInitialBarcode = '';

  function initializeModal() {
    if (product) {
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
      isScalable = product.is_scalable || false;
      scaleCode = product.scale_code || '';
      scalePlu = product.scale_plu || (product.id || 1);
      scaleBarcodeType = product.scale_barcode_type || 97;
      scaleDepartmentId = product.scale_department_id || 1;
      scaleSyncStatus = product.scale_sync_status || 'pending';
      barcodeTokens = product.barcodes ? [...product.barcodes] : [];
      currentBarcodeTyped = '';
      editingTokenIndex = null;
      handleSalePriceChange();
      activeTab = 'details';
    } else {
      sku = '';
      nameAr = '';
      nameFr = '';
      nameEn = '';
      categoryId = categories.length > 0 ? categories[0].id : 1;
      unitId = units.length > 0 ? units[0].id : 1;
      purchasePrice = 0;
      salePrice = 0;
      minSalePrice = 0;
      taxRate = 19;
      currentStock = 0;
      minStock = 5;
      imagePath = '';
      expiryDate = '';
      isBundle = false;
      isScalable = false;
      scaleCode = '';
      scalePlu = 1;
      scaleBarcodeType = 97;
      scaleDepartmentId = 1;
      scaleSyncStatus = 'pending';
      barcodeTokens = initialBarcode ? [initialBarcode.trim()] : [];
      currentBarcodeTyped = '';
      editingTokenIndex = null;
      profitMarginPercent = 20;
      profitMarginAmount = 0;
      activeTab = 'details';
    }
  }

  $: if (isOpen && (!lastIsOpen || (product ? product.id : null) !== lastProductId || initialBarcode !== lastInitialBarcode)) {
    lastIsOpen = true;
    lastProductId = product ? product.id : null;
    lastInitialBarcode = initialBarcode;
    initializeModal();
  } else if (!isOpen) {
    lastIsOpen = false;
    lastProductId = undefined;
    lastInitialBarcode = '';
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

  async function handleSaveQuickFamily() {
    if (!quickFamilyNameFr && !quickFamilyNameAr) return;
    try {
      isSavingFamily = true;
      const newId = await invoke<number>('save_category', {
        nameAr: quickFamilyNameAr || quickFamilyNameFr,
        nameFr: quickFamilyNameFr || quickFamilyNameAr,
        nameEn: quickFamilyNameFr || quickFamilyNameAr,
        color: quickFamilyColor || '#0284c7',
        categoryId: null,
      });
      categories = await invoke<Category[]>('get_categories');
      categoryId = newId;
      isQuickFamilyOpen = false;
      quickFamilyNameFr = '';
      quickFamilyNameAr = '';
    } catch (e: any) {
      alert('Failed to add category: ' + (e.message || e));
    } finally {
      isSavingFamily = false;
    }
  }

  async function handleSaveQuickUnit() {
    if (!quickUnitName) return;
    try {
      isSavingUnit = true;
      const newId = await invoke<number>('save_unit', {
        name: quickUnitName,
        shortName: quickUnitShort || quickUnitName.slice(0, 3),
        allowDecimals: false,
        unitId: null,
      });
      units = await invoke<Unit[]>('get_units');
      unitId = newId;
      isQuickUnitOpen = false;
      quickUnitName = '';
      quickUnitShort = '';
    } catch (e: any) {
      alert('Failed to add unit: ' + (e.message || e));
    } finally {
      isSavingUnit = false;
    }
  }

  async function handleDirectScaleSync() {
    if (!product?.id) {
      scaleSyncMsg = 'Save product first before synchronizing to scale';
      return;
    }
    try {
      isSyncingScale = true;
      scaleSyncMsg = 'Sending PLU data to ACLAS scale...';
      const settings = await invoke<Record<string, string>>('get_all_settings');
      const ip = settings['scale_ip'] || '192.168.1.87';
      const port = parseInt(settings['scale_port'] || '0');
      const protocol = parseInt(settings['scale_protocol'] || '0');

      await invoke('upload_product_to_scale', {
        productId: product.id,
        ip,
        port,
        protocolType: protocol,
        defaultDept: scaleDepartmentId,
        defaultBarcodeType: scaleBarcodeType,
        userName: 'admin',
      });
      scaleSyncStatus = 'synced';
      scaleSyncMsg = '✅ Synchronized successfully with scale!';
      setTimeout(() => (scaleSyncMsg = ''), 4000);
    } catch (e: any) {
      scaleSyncMsg = '❌ Scale error: ' + (typeof e === 'string' ? e : e.message);
    } finally {
      isSyncingScale = false;
    }
  }

  async function handleSave() {
    if (!nameFr.trim() && !nameAr.trim()) {
      errorMsg = 'Product name is required / اسم المنتج إجباري';
      return;
    }

    try {
      isSaving = true;
      errorMsg = '';

      if (currentBarcodeTyped.trim()) {
        addBarcodeToken();
      }

      const input: ProductInput = {
        sku: sku || undefined,
        name_ar: nameAr.trim() || nameFr.trim(),
        name_fr: nameFr.trim() || nameAr.trim(),
        name_en: nameEn.trim() || nameFr.trim(),
        category_id: categoryId,
        unit_id: unitId,
        purchase_price: Number(purchasePrice) || 0,
        sale_price: Number(salePrice) || 0,
        min_sale_price: Number(minSalePrice) || 0,
        tax_rate: Number(taxRate) || 19,
        current_stock: Number(currentStock) || 0,
        min_stock: Number(minStock) || 5,
        image_path: imagePath || undefined,
        expiry_date: expiryDate || undefined,
        is_scalable: isScalable,
        scale_code: scaleCode || undefined,
        scale_plu: isScalable ? scalePlu : undefined,
        scale_barcode_type: isScalable ? scaleBarcodeType : undefined,
        scale_department_id: isScalable ? scaleDepartmentId : undefined,
        scale_sync_status: scaleSyncStatus,
        is_bundle: isBundle,
        barcodes: barcodeTokens,
      };

      const savedId = await invoke<number>('save_product', {
        input,
        productId: product ? product.id : null,
      });

      // Auto-sync if enabled
      if (isScalable) {
        try {
          const settings = await invoke<Record<string, string>>('get_all_settings');
          if (settings['scale_auto_sync'] === 'true') {
            const ip = settings['scale_ip'] || '192.168.1.87';
            const port = parseInt(settings['scale_port'] || '0');
            const protocol = parseInt(settings['scale_protocol'] || '0');
            await invoke('upload_product_to_scale', {
              productId: savedId,
              ip,
              port,
              protocolType: protocol,
              defaultDept: scaleDepartmentId,
              defaultBarcodeType: scaleBarcodeType,
              userName: 'admin',
            });
          }
        } catch (scaleErr) {
          console.warn('Auto scale sync notice:', scaleErr);
        }
      }

      onSaved();
      onClose();
    } catch (e: any) {
      errorMsg = typeof e === 'string' ? e : e.message || 'Failed to save product';
    } finally {
      isSaving = false;
    }
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <!-- Strict Fixed Modal Geometry to Prevent Any Jumps/Resizing on Tab Switches -->
    <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl w-full max-w-3xl h-[88vh] flex flex-col overflow-hidden animate-in zoom-in-95 duration-150 relative">
      <!-- Modal Header (Fixed Height) -->
      <div class="flex items-center justify-between px-6 py-4 border-b border-pos-border bg-slate-50 dark:bg-slate-800/60 shrink-0">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-2xl bg-sky-600/10 text-sky-600 flex items-center justify-center font-bold shrink-0">
            <Package class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-black text-base text-pos-text">
              {product ? `Edit Product: ${product.name_fr || product.name_ar}` : 'Add New Product (إضافة منتج جديد)'}
            </h3>
            <p class="text-xs text-pos-muted">Multi-barcode tokens, scalable ACLAS scale integration, and margin controls</p>
          </div>
        </div>

        <!-- Fixed Tab Bar -->
        <div class="flex items-center bg-slate-200 dark:bg-slate-700 p-1 rounded-xl gap-1 shrink-0">
          <button
            type="button"
            on:click={() => (activeTab = 'details')}
            class="px-3 py-1 rounded-lg text-xs font-bold transition {activeTab === 'details' ? 'bg-white dark:bg-slate-800 text-sky-600 shadow-xs' : 'text-pos-muted'}"
          >
            Details
          </button>
          <button
            type="button"
            on:click={() => (activeTab = 'scalable')}
            class="px-3 py-1 rounded-lg text-xs font-bold transition flex items-center gap-1 {activeTab === 'scalable' ? 'bg-white dark:bg-slate-800 text-sky-600 shadow-xs' : 'text-pos-muted'}"
          >
            <Scale class="w-3.5 h-3.5" />
            <span>Scale</span>
          </button>
          {#if product}
            <button
              type="button"
              on:click={() => (activeTab = 'history')}
              class="px-3 py-1 rounded-lg text-xs font-bold transition {activeTab === 'history' ? 'bg-white dark:bg-slate-800 text-sky-600 shadow-xs' : 'text-pos-muted'}"
            >
              Price History
            </button>
          {/if}
          <button on:click={onClose} class="text-pos-muted hover:text-pos-text p-1.5 rounded-lg cursor-pointer">
            <X class="w-5 h-5" />
          </button>
        </div>
      </div>

      {#if errorMsg}
        <div class="mx-6 mt-3 p-3 bg-rose-100 dark:bg-rose-950 text-rose-800 dark:text-rose-200 text-xs font-bold rounded-xl border border-rose-300 shrink-0">
          {errorMsg}
        </div>
      {/if}

      <!-- Modal Body (Strictly Scrollable Interior) -->
      <div class="p-6 overflow-y-auto space-y-4 flex-1">
        {#if activeTab === 'details'}
          <!-- Images and Names Grid -->
          <div class="grid grid-cols-1 md:grid-cols-4 gap-4 items-start">
            <!-- Image upload box -->
            <div class="flex flex-col items-center">
              <div class="w-28 h-28 rounded-2xl border-2 border-dashed border-pos-border bg-slate-50 dark:bg-slate-800 flex flex-col items-center justify-center relative overflow-hidden group">
                {#if imagePath}
                  <img src={imagePath} alt="Product" class="w-full h-full object-cover" />
                {:else}
                  <Tag class="w-8 h-8 text-pos-muted mb-1" />
                  <span class="text-[10px] text-pos-muted font-bold">Photo</span>
                {/if}
                <label class="absolute inset-0 bg-black/40 text-white text-[10px] font-bold flex flex-col items-center justify-center opacity-0 group-hover:opacity-100 transition cursor-pointer">
                  <Upload class="w-4 h-4 mb-1" />
                  <span>Change</span>
                  <input type="file" accept="image/*" on:change={handleImageUpload} class="hidden" />
                </label>
              </div>
            </div>

            <!-- Names -->
            <div class="md:col-span-3 space-y-2.5">
              <div class="grid grid-cols-2 gap-3">
                <div>
                  <label class="block text-xs font-bold text-pos-muted mb-1">Product Name (Français) *</label>
                  <input type="text" bind:value={nameFr} placeholder="Ex: Lait Candia 1L Entier" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
                </div>
                <div>
                  <label class="block text-xs font-bold text-pos-muted mb-1">Product Name (العربية)</label>
                  <input type="text" bind:value={nameAr} placeholder="Ex: حليب كانديا 1 لتر كامل" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
                </div>
              </div>

              <div class="grid grid-cols-3 gap-3">
                <div>
                  <div class="flex items-center justify-between mb-1">
                    <label class="block text-xs font-bold text-pos-muted">SKU / Reference</label>
                    <button type="button" on:click={generateRandomSku} class="text-[10px] font-black text-sky-600 hover:underline flex items-center gap-0.5" title="Auto-generate SKU">
                      <Sparkles class="w-3 h-3" />
                      <span>Auto</span>
                    </button>
                  </div>
                  <input type="text" bind:value={sku} placeholder="Optional (PRD-001)" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
                </div>

                <!-- Category / Family with Quick Add Trigger -->
                <div>
                  <div class="flex items-center justify-between mb-1">
                    <label class="block text-xs font-bold text-pos-muted">Family / الفئة</label>
                    <button type="button" on:click={() => (isQuickFamilyOpen = true)} class="text-[10px] font-black text-sky-600 hover:underline flex items-center gap-0.5">
                      <Plus class="w-3 h-3" />
                      <span>Add</span>
                    </button>
                  </div>
                  <select bind:value={categoryId} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none">
                    {#each categories as cat}
                      <option value={cat.id}>{cat.name_fr} / {cat.name_ar}</option>
                    {/each}
                  </select>
                </div>

                <!-- Unit with Quick Add Trigger -->
                <div>
                  <div class="flex items-center justify-between mb-1">
                    <label class="block text-xs font-bold text-pos-muted">Unit / الوحدة</label>
                    <button type="button" on:click={() => (isQuickUnitOpen = true)} class="text-[10px] font-black text-sky-600 hover:underline flex items-center gap-0.5">
                      <Plus class="w-3 h-3" />
                      <span>Add</span>
                    </button>
                  </div>
                  <select bind:value={unitId} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text outline-none">
                    {#each units as u}
                      <option value={u.id}>{u.name} ({u.short_name})</option>
                    {/each}
                  </select>
                </div>
              </div>
            </div>
          </div>

          <!-- Barcode Tokenizer Section & Generator -->
          <div class="bg-slate-50 dark:bg-slate-800/40 p-3.5 rounded-2xl border border-pos-border space-y-2">
            <div class="flex items-center justify-between">
              <label class="block text-xs font-bold text-pos-text">
                Barcodes & Tokens (الباركودات المتعددة)
                <span class="text-[10px] text-pos-muted font-normal block">Type barcode & press Enter or comma (,) to add multiple</span>
              </label>
              <button
                type="button"
                on:click={generateValidEan13}
                class="px-2.5 py-1 bg-sky-100 dark:bg-sky-950 text-sky-700 dark:text-sky-300 hover:bg-sky-200 border border-sky-300 dark:border-sky-800 text-[10px] font-bold rounded-lg flex items-center gap-1 cursor-pointer transition shadow-2xs"
              >
                <Sparkles class="w-3 h-3" />
                <span>+ Generate EAN-13 (توليد باركود)</span>
              </button>
            </div>

            <div class="flex flex-wrap gap-1.5 items-center p-2 bg-pos-card border border-pos-border rounded-xl min-h-[44px]">
              {#each barcodeTokens as token, idx}
                <div class="flex items-center gap-1 px-2.5 py-1 bg-sky-100 dark:bg-sky-950 text-sky-800 dark:text-sky-200 border border-sky-300 dark:border-sky-800 rounded-lg text-xs font-mono font-bold">
                  <span on:dblclick={() => startEditToken(idx)} class="cursor-pointer" title="Double click to edit">{token}</span>
                  <button type="button" on:click={() => copyToClipboard(token)} class="text-sky-600 hover:text-sky-900 ml-0.5" title="Copy"><Copy class="w-3 h-3" /></button>
                  <button type="button" on:click={() => removeBarcodeToken(idx)} class="text-rose-500 hover:text-rose-700 ml-0.5"><X class="w-3 h-3" /></button>
                </div>
              {/each}
              <input
                type="text"
                bind:value={currentBarcodeTyped}
                on:keydown={handleBarcodeKeyDown}
                placeholder={barcodeTokens.length === 0 ? "Scan or type barcode & Enter..." : "+ Add barcode..."}
                class="flex-1 min-w-[140px] bg-transparent border-0 text-xs font-mono font-bold text-pos-text outline-none px-2"
              />
            </div>
            {#if copyFeedback}
              <p class="text-[10px] text-emerald-600 font-bold">{copyFeedback}</p>
            {/if}
          </div>

          <!-- Pricing, Margin Toggle & Stock Grid -->
          <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Purchase Cost (DZD)</label>
              <input
                type="number"
                min="0"
                bind:value={purchasePrice}
                on:input={handlePurchasePriceChange}
                class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none"
              />
            </div>

            <!-- Margin Toggle: % vs DZD Amount -->
            <div>
              <div class="flex items-center justify-between mb-1">
                <label class="block text-xs font-bold text-pos-muted">Margin ({marginMode === 'percent' ? '%' : 'DZD'})</label>
                <div class="flex items-center bg-slate-200 dark:bg-slate-700 rounded-md p-0.5 text-[9px] font-bold">
                  <button
                    type="button"
                    on:click={() => { marginMode = 'percent'; handleMarginPercentChange(); }}
                    class="px-1.5 py-0.5 rounded {marginMode === 'percent' ? 'bg-sky-600 text-white' : 'text-pos-muted'}"
                  >%</button>
                  <button
                    type="button"
                    on:click={() => { marginMode = 'amount'; handleMarginAmountChange(); }}
                    class="px-1.5 py-0.5 rounded {marginMode === 'amount' ? 'bg-sky-600 text-white' : 'text-pos-muted'}"
                  >DZD</button>
                </div>
              </div>

              {#if marginMode === 'percent'}
                <input
                  type="number"
                  bind:value={profitMarginPercent}
                  on:input={handleMarginPercentChange}
                  class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-emerald-600 outline-none"
                />
              {:else}
                <input
                  type="number"
                  bind:value={profitMarginAmount}
                  on:input={handleMarginAmountChange}
                  class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-emerald-600 outline-none"
                />
              {/if}
            </div>

            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Sale Price (DZD) *</label>
              <input
                type="number"
                min="0"
                bind:value={salePrice}
                on:input={handleSalePriceChange}
                class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-black text-sky-600 outline-none"
              />
            </div>

            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Current Stock</label>
              <input
                type="number"
                min="0"
                bind:value={currentStock}
                class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none"
              />
            </div>
          </div>

          <!-- Expiry Date & Min Stock -->
          <div class="grid grid-cols-2 gap-3">
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Expiry Date / تاريخ الصلاحية</label>
              <input type="date" bind:value={expiryDate} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none" />
            </div>
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Min Stock Alert Level</label>
              <input type="number" min="1" bind:value={minStock} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none" />
            </div>
          </div>
        {:else if activeTab === 'scalable'}
          <!-- Scale Tab -->
          <div class="space-y-4">
            <div class="flex items-center justify-between p-4 bg-sky-50 dark:bg-sky-950/40 border border-sky-200 dark:border-sky-800 rounded-2xl">
              <div class="flex items-center gap-3">
                <Scale class="w-6 h-6 text-sky-600" />
                <div>
                  <h4 class="font-black text-sm text-pos-text">Scalable Product (منتج يباع بالوزن على الميزان)</h4>
                  <p class="text-xs text-pos-muted">Enable to synchronize name and pricing to ACLAS electronic barcode scales</p>
                </div>
              </div>
              <label class="relative inline-flex items-center cursor-pointer">
                <input type="checkbox" bind:checked={isScalable} class="sr-only peer" />
                <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-sky-600"></div>
              </label>
            </div>

            {#if isScalable}
              <div class="grid grid-cols-2 md:grid-cols-3 gap-3 p-4 bg-pos-card border border-pos-border rounded-2xl">
                <div>
                  <div class="flex items-center justify-between mb-1">
                    <label class="block text-xs font-bold text-pos-muted">Scale PLU Number</label>
                    <button type="button" on:click={generateScalePluCode} class="text-[10px] text-sky-600 font-bold hover:underline">Regenerate</button>
                  </div>
                  <input type="number" min="1" max="999999" bind:value={scalePlu} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text" />
                </div>

                <div>
                  <label class="block text-xs font-bold text-pos-muted mb-1">Scale Item Code (6 digits)</label>
                  <input type="text" bind:value={scaleCode} placeholder="Ex: 950001" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text" />
                </div>

                <div>
                  <label class="block text-xs font-bold text-pos-muted mb-1">Scale Department ID</label>
                  <input type="number" min="1" max="99" bind:value={scaleDepartmentId} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text" />
                </div>

                <div class="md:col-span-2">
                  <label class="block text-xs font-bold text-pos-muted mb-1">Scale Barcode Type Format</label>
                  <select bind:value={scaleBarcodeType} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text">
                    <option value={97}>Type 97: 18-Code (Dept + ItemCode + Price + Weight + Checksum)</option>
                    <option value={2}>Type 02: EAN-13 Price Embedded (DD IIIII PPPPP C)</option>
                    <option value={7}>Type 07: EAN-13 Weight Embedded (DD IIIII WWWWW C)</option>
                    <option value={22}>Type 22: EAN-13 1-Digit Dept Price Embedded (D IIIIII PPPPP C)</option>
                    <option value={27}>Type 27: EAN-13 1-Digit Dept Weight Embedded (D IIIIII WWWWW C)</option>
                    <option value={12}>Type 12: Fixed Code 22 Price Embedded (22 IIIII PPPPP C)</option>
                    <option value={17}>Type 17: Fixed Code 27 Weight Embedded (27 IIIII WWWWW C)</option>
                  </select>
                </div>

                <div>
                  <label class="block text-xs font-bold text-pos-muted mb-1">Sync Status</label>
                  <div class="px-3 py-2 bg-slate-100 dark:bg-slate-800 rounded-xl text-xs font-bold capitalize text-sky-600">
                    {scaleSyncStatus}
                  </div>
                </div>
              </div>

              {#if scaleSyncMsg}
                <div class="p-3 bg-sky-100 dark:bg-sky-950 text-sky-800 dark:text-sky-200 text-xs font-bold rounded-xl">
                  {scaleSyncMsg}
                </div>
              {/if}

              {#if product}
                <div class="flex justify-end">
                  <button
                    type="button"
                    on:click={handleDirectScaleSync}
                    disabled={isSyncingScale}
                    class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-xl flex items-center gap-2 cursor-pointer shadow-xs"
                  >
                    <Send class="w-4 h-4" />
                    <span>{isSyncingScale ? 'Sending to Scale...' : 'Sync to ACLAS Scale Now (إرسال للميزان)'}</span>
                  </button>
                </div>
              {/if}
            {/if}
          </div>
        {:else if activeTab === 'history'}
          <div class="p-4 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border text-xs text-pos-muted text-center">
            Price modifications are automatically logged in SQLite `product_price_history`.
          </div>
        {/if}
      </div>

      <!-- Modal Footer (Fixed Height) -->
      <div class="px-6 py-4 border-t border-pos-border bg-slate-50 dark:bg-slate-800/60 flex items-center justify-between shrink-0">
        <div class="flex items-center gap-2">
          <button on:click={onClose} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 text-pos-text font-bold text-xs rounded-xl transition cursor-pointer">
            Cancel (إلغاء)
          </button>

          <button
            type="button"
            on:click={openPrintSticker}
            class="px-3 py-2 bg-sky-50 hover:bg-sky-100 dark:bg-sky-950/60 dark:hover:bg-sky-900/60 text-sky-700 dark:text-sky-300 border border-sky-200 dark:border-sky-800 font-bold text-xs rounded-xl transition cursor-pointer flex items-center gap-1.5 shadow-2xs"
            title="Print Product Sticker"
          >
            <QrCode class="w-3.5 h-3.5" />
            <span>Print Sticker ({Number(currentStock) > 0 ? currentStock : 1})</span>
          </button>

          <button
            type="button"
            on:click={openPrintShelf}
            class="px-3 py-2 bg-emerald-50 hover:bg-emerald-100 dark:bg-emerald-950/60 dark:hover:bg-emerald-900/60 text-emerald-700 dark:text-emerald-300 border border-emerald-200 dark:border-emerald-800 font-bold text-xs rounded-xl transition cursor-pointer flex items-center gap-1.5 shadow-2xs"
            title="Print Shelf Tag"
          >
            <Tag class="w-3.5 h-3.5" />
            <span>Shelf Tag (1)</span>
          </button>
        </div>

        <button on:click={handleSave} disabled={isSaving} class="px-6 py-2 bg-sky-600 hover:bg-sky-700 disabled:opacity-50 text-white font-black text-xs rounded-xl shadow-md transition cursor-pointer flex items-center gap-1.5">
          <Check class="w-4 h-4" />
          <span>{isSaving ? 'Saving...' : 'Save Product (حفظ المنتج)'}</span>
        </button>
      </div>
    </div>
  </div>
{/if}

<!-- Embedded Print Label Modal -->
<PrintLabelModal
  isOpen={isPrintLabelOpen}
  product={printProductObj}
  initialType={printLabelInitialType}
  initialQty={printLabelInitialQty}
  onClose={() => (isPrintLabelOpen = false)}
/>

<!-- Sub-Modal: Quick Add Family -->
{#if isQuickFamilyOpen}
  <div class="fixed inset-0 z-[70] bg-black/70 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl p-6 max-w-sm w-full space-y-4 animate-in zoom-in-95">
      <div class="flex items-center justify-between border-b border-pos-border pb-2">
        <h4 class="font-black text-xs text-pos-text flex items-center gap-1.5">
          <FolderPlus class="w-4 h-4 text-sky-600" />
          <span>Quick Add Family / إضافة فئة جديدة</span>
        </h4>
        <button on:click={() => (isQuickFamilyOpen = false)} class="text-pos-muted hover:text-pos-text"><X class="w-4 h-4" /></button>
      </div>

      <div class="space-y-3 text-xs">
        <div>
          <label class="block font-bold text-pos-muted mb-1">Family Name (Français) *</label>
          <input type="text" bind:value={quickFamilyNameFr} placeholder="Ex: Boissons" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 rounded-xl font-bold text-pos-text outline-none" />
        </div>
        <div>
          <label class="block font-bold text-pos-muted mb-1">Family Name (العربية)</label>
          <input type="text" bind:value={quickFamilyNameAr} placeholder="Ex: مشروبات" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 rounded-xl font-bold text-pos-text outline-none" />
        </div>
        <div>
          <label class="block font-bold text-pos-muted mb-1">Category Color</label>
          <div class="flex items-center gap-2">
            <input type="color" bind:value={quickFamilyColor} class="w-8 h-8 rounded-lg border-0 cursor-pointer" />
            <span class="font-mono text-pos-muted">{quickFamilyColor}</span>
          </div>
        </div>
      </div>

      <div class="flex justify-end gap-2 pt-2 border-t border-pos-border">
        <button on:click={() => (isQuickFamilyOpen = false)} class="px-3 py-1.5 bg-slate-200 dark:bg-slate-700 text-pos-text text-xs font-bold rounded-xl">Cancel</button>
        <button on:click={handleSaveQuickFamily} disabled={isSavingFamily} class="px-4 py-1.5 bg-sky-600 text-white text-xs font-black rounded-xl">Save & Select</button>
      </div>
    </div>
  </div>
{/if}

<!-- Sub-Modal: Quick Add Unit -->
{#if isQuickUnitOpen}
  <div class="fixed inset-0 z-[70] bg-black/70 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl p-6 max-w-sm w-full space-y-4 animate-in zoom-in-95">
      <div class="flex items-center justify-between border-b border-pos-border pb-2">
        <h4 class="font-black text-xs text-pos-text flex items-center gap-1.5">
          <Plus class="w-4 h-4 text-sky-600" />
          <span>Quick Add Unit / إضافة وحدة جديدة</span>
        </h4>
        <button on:click={() => (isQuickUnitOpen = false)} class="text-pos-muted hover:text-pos-text"><X class="w-4 h-4" /></button>
      </div>

      <div class="space-y-3 text-xs">
        <div>
          <label class="block font-bold text-pos-muted mb-1">Unit Full Name *</label>
          <input type="text" bind:value={quickUnitName} placeholder="Ex: Kilogramme, Litre, Pièce" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 rounded-xl font-bold text-pos-text outline-none" />
        </div>
        <div>
          <label class="block font-bold text-pos-muted mb-1">Unit Short Code</label>
          <input type="text" bind:value={quickUnitShort} placeholder="Ex: kg, L, pcs" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 rounded-xl font-mono font-bold text-pos-text outline-none" />
        </div>
      </div>

      <div class="flex justify-end gap-2 pt-2 border-t border-pos-border">
        <button on:click={() => (isQuickUnitOpen = false)} class="px-3 py-1.5 bg-slate-200 dark:bg-slate-700 text-pos-text text-xs font-bold rounded-xl">Cancel</button>
        <button on:click={handleSaveQuickUnit} disabled={isSavingUnit} class="px-4 py-1.5 bg-sky-600 text-white text-xs font-black rounded-xl">Save & Select</button>
      </div>
    </div>
  </div>
{/if}
