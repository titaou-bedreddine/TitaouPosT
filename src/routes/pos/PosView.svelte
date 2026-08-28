<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { t, currentLocale } from '../../lib/i18n';
  import type { Category, Product, Supplier, Unit, SaleInput } from '../../lib/types';
  import { cartItems, cartGrandTotal, cartSubtotal, globalDiscountAmount, isRefundMode, addToCart, clearCart } from '../../lib/stores/cart';
  import { currentUser } from '../../lib/stores/auth';
  import { activeSession } from '../../lib/stores/session';
  import { printHtmlDirectly, buildReceiptHtml } from '../../lib/utils/printer';

  import TopActionBar from '../../lib/components/TopActionBar.svelte';
  import UniversalSearchBar from '../../lib/components/UniversalSearchBar.svelte';
  import ProductCard from '../../lib/components/ProductCard.svelte';
  import CartItemCard from '../../lib/components/CartItemCard.svelte';
  import PaymentModal from '../../lib/components/PaymentModal.svelte';
  import CashDrawerModal from '../../lib/components/CashDrawerModal.svelte';
  import CategoryManagerModal from '../../lib/components/CategoryManagerModal.svelte';
  import HeldSalesModal from '../../lib/components/HeldSalesModal.svelte';
  import RemiseModal from '../../lib/components/RemiseModal.svelte';
  import PrintReceiptModal from '../../lib/components/PrintReceiptModal.svelte';
  import ProductEditModal from '../../lib/components/ProductEditModal.svelte';
  import QuickPurchaseModal from '../../lib/components/QuickPurchaseModal.svelte';
  import ReturnDamagedModal from '../../lib/components/ReturnDamagedModal.svelte';
  import CreditCustomerModal from '../../lib/components/CreditCustomerModal.svelte';

  import {
    ShoppingBag, ArrowRight, CheckCircle2, Settings2, Plus,
    Store, Sparkles, AlertCircle, ArrowUpDown, Tag
  } from 'lucide-svelte';

  let products: Product[] = [];
  let categories: Category[] = [];
  let units: Unit[] = [];
  let suppliers: Supplier[] = [];

  let selectedCategory: number | null = null;
  let searchQuery = '';
  let searchType: 'all' | 'name' | 'barcode' | 'price' | 'qr' = 'all';
  let sortBy: 'default' | 'name_asc' | 'name_desc' | 'price_asc' | 'price_desc' | 'stock' = 'default';

  // Checkout and Mode State
  let selectedPaymentMode: 'cash' | 'tpe' | 'credit' = 'cash';
  let autoPrintEnabled = true;
  let autoDrawerEnabled = true;

  // Modals
  let isPaymentOpen = false;
  let isCashDrawerOpen = false;
  let isCategoryManagerOpen = false;
  let isHeldSalesOpen = false;
  let isRemiseOpen = false;
  let isPrintReceiptOpen = false;
  let isProductEditOpen = false;
  let editingProduct: Product | null = null;
  let isQuickPurchaseOpen = false;
  let isReturnDamagedOpen = false;
  let isCreditCustomerOpen = false;

  let lastSaleSuccessNumber = '';
  let barcodeBuffer = '';
  let lastKeyTime = 0;

  onMount(async () => {
    await loadCategories();
    await loadUnits();
    await loadSuppliers();
    await loadProducts();

    window.addEventListener('keydown', handleGlobalKeyDown);
  });

  onDestroy(() => {
    window.removeEventListener('keydown', handleGlobalKeyDown);
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

  async function loadSuppliers() {
    try {
      suppliers = await invoke<Supplier[]>('list_suppliers');
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

      // Sort products
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

      if (searchQuery.trim().length >= 8 && searchType === 'all' && products.length === 1) {
        const exactMatch = products[0].barcodes.includes(searchQuery.trim()) || products[0].sku === searchQuery.trim();
        if (exactMatch) {
          addToCart(products[0], 1, $isRefundMode);
          searchQuery = '';
          await loadProducts();
        }
      }
    } catch (e) {
      console.error(e);
    }
  }

  function handleCategoryClick(catId: number | null) {
    selectedCategory = catId;
    loadProducts();
  }

  function handleOpenEdit(p: Product) {
    editingProduct = p;
    isProductEditOpen = true;
  }

  function handleOpenNewProduct() {
    editingProduct = null;
    isProductEditOpen = true;
  }

  // Fast Checkout Action
  async function handleFastCheckout() {
    if ($cartItems.length === 0) return;

    if (!$activeSession) {
      isCashDrawerOpen = true;
      return;
    }

    if (selectedPaymentMode === 'credit') {
      isCreditCustomerOpen = true;
      return;
    }

    // Cash or TPE Checkout directly
    await executeCheckout(null, undefined);
  }

  async function executeCheckout(customerId: number | null, customerName?: string) {
    try {
      const saleNumber = 'VTE-' + Date.now().toString().slice(-6);
      const saleDate = new Date().toLocaleString();
      const cashier = $currentUser?.display_name || 'Admin';

      const saleInput: SaleInput = {
        session_id: $activeSession?.id || 1,
        customer_id: customerId,
        user_id: $currentUser?.id || 1,
        total_amount: $cartGrandTotal,
        tax_amount: 0,
        discount_amount: $globalDiscountAmount,
        final_amount: $cartGrandTotal,
        payment_method: selectedPaymentMode,
        is_refund: $isRefundMode,
        notes: customerName ? `Credit Sale to ${customerName}` : undefined,
        items: $cartItems.map((i) => ({
          product_id: i.product_id,
          sku: i.sku,
          barcode: i.barcode,
          quantity: i.quantity,
          unit_price: i.unit_price,
          discount_amount: i.discount_amount,
          tax_amount: i.tax_amount,
          total_price: i.total_price,
          is_refund: i.is_refund,
        })),
      };

      await invoke('create_sale', { input: saleInput });

      // Silent Auto-Print
      if (autoPrintEnabled) {
        const receiptItems = $cartItems.map((i) => ({
          name: i.name_fr || i.name_ar,
          quantity: i.quantity,
          unitPrice: i.unit_price,
          totalPrice: i.total_price,
        }));

        if (selectedPaymentMode === 'credit') {
          // Print 2 Copies: Store Copy + Client Copy
          const copy1 = buildReceiptHtml({
            shopName: 'TitaouPOS Superette',
            shopAddress: 'Rue Principale, Alger',
            shopPhone: '0553444057',
            saleNumber,
            saleDate,
            cashierName: cashier,
            customerName: customerName || 'Client Crédit',
            items: receiptItems,
            subtotal: $cartSubtotal,
            discount: $globalDiscountAmount,
            grandTotal: $cartGrandTotal,
            paymentMethod: 'CREDIT (دين)',
            isCredit: true,
            copyLabel: 'COPIE MAGASIN / STORE COPY',
          });

          const copy2 = buildReceiptHtml({
            shopName: 'TitaouPOS Superette',
            shopAddress: 'Rue Principale, Alger',
            shopPhone: '0553444057',
            saleNumber,
            saleDate,
            cashierName: cashier,
            customerName: customerName || 'Client Crédit',
            items: receiptItems,
            subtotal: $cartSubtotal,
            discount: $globalDiscountAmount,
            grandTotal: $cartGrandTotal,
            paymentMethod: 'CREDIT (دين)',
            isCredit: true,
            copyLabel: 'COPIE CLIENT / CUSTOMER COPY',
          });

          printHtmlDirectly(copy1 + '<div style="page-break-after:always;"></div>' + copy2, 'Credit Receipts');
        } else {
          const receipt = buildReceiptHtml({
            shopName: 'TitaouPOS Superette',
            shopAddress: 'Rue Principale, Alger',
            shopPhone: '0553444057',
            saleNumber,
            saleDate,
            cashierName: cashier,
            items: receiptItems,
            subtotal: $cartSubtotal,
            discount: $globalDiscountAmount,
            grandTotal: $cartGrandTotal,
            paymentMethod: selectedPaymentMode.toUpperCase(),
          });
          printHtmlDirectly(receipt, 'Sale Receipt #' + saleNumber);
        }
      }

      // Auto-kick cash drawer
      if (autoDrawerEnabled && selectedPaymentMode === 'cash') {
        printHtmlDirectly('<div style="display:none"></div>', 'Kick Drawer');
      }

      lastSaleSuccessNumber = saleNumber;
      clearCart();
      await loadProducts();

      setTimeout(() => {
        lastSaleSuccessNumber = '';
      }, 4000);
    } catch (err: any) {
      console.error('Failed to complete sale:', err);
    }
  }

  // Global Keyboard Shortcuts
  function handleGlobalKeyDown(e: KeyboardEvent) {
    // Barcode scanner rapid input detection
    const now = Date.now();
    if (now - lastKeyTime > 100) {
      barcodeBuffer = '';
    }
    lastKeyTime = now;

    if (e.key === 'Enter') {
      if (barcodeBuffer.length >= 6) {
        e.preventDefault();
        searchQuery = barcodeBuffer;
        searchType = 'barcode';
        loadProducts();
        barcodeBuffer = '';
        return;
      }
    } else if (e.key.length === 1) {
      barcodeBuffer += e.key;
    }

    // Function keys shortcuts
    if (e.key === 'F1') {
      e.preventDefault();
      clearCart();
    } else if (e.key === 'F2') {
      e.preventDefault();
      handleFastCheckout();
    } else if (e.key === 'F3') {
      e.preventDefault();
      isHeldSalesOpen = true;
    } else if (e.key === 'F4') {
      e.preventDefault();
      isRemiseOpen = true;
    } else if (e.key === 'F6') {
      e.preventDefault();
      // Focus quantity on first cart item input
      const qtyInput = document.querySelector('input[type="number"]') as HTMLInputElement;
      if (qtyInput) {
        qtyInput.focus();
        qtyInput.select();
      }
    } else if (e.key === 'F7') {
      e.preventDefault();
      isQuickPurchaseOpen = true;
    } else if (e.key === 'F8') {
      e.preventDefault();
      isReturnDamagedOpen = true;
    } else if (e.key === 'F9') {
      e.preventDefault();
      isCashDrawerOpen = true;
    } else if (e.key === 'F10') {
      e.preventDefault();
      printHtmlDirectly('<div style="display:none"></div>', 'Kick Drawer');
    }
  }
</script>

<div class="flex flex-col h-full bg-pos-bg select-none">
  <!-- Top Persistent Grouped Action Cards Bar -->
  <TopActionBar
    bind:selectedPaymentMode
    bind:autoPrintEnabled
    bind:autoDrawerEnabled
    onOpenPayment={handleFastCheckout}
    onOpenCashDrawer={() => (isCashDrawerOpen = true)}
    onOpenRemise={() => (isRemiseOpen = true)}
    onOpenHeldSales={() => (isHeldSalesOpen = true)}
    onPrintReceipt={() => (isPrintReceiptOpen = true)}
    onQuickPurchase={() => (isQuickPurchaseOpen = true)}
    onReturnDamaged={() => (isReturnDamagedOpen = true)}
  />

  {#if lastSaleSuccessNumber}
    <div class="bg-emerald-600 text-white px-4 py-2 text-xs font-bold flex items-center justify-between shadow-md animate-in slide-in-from-top-2">
      <div class="flex items-center gap-2">
        <CheckCircle2 class="w-4 h-4" />
        <span>Sale Completed Successfully: #{lastSaleSuccessNumber}</span>
      </div>
      <button on:click={() => (lastSaleSuccessNumber = '')} class="underline cursor-pointer">Dismiss</button>
    </div>
  {/if}

  <!-- Main POS Workspace -->
  <div class="flex-1 flex overflow-hidden p-2.5 gap-2.5">
    <!-- LEFT PANEL: Search, Category Pills, Catalog Grid -->
    <div class="flex-1 flex flex-col min-w-0 bg-pos-card border border-pos-border rounded-2xl p-3 shadow-xs">
      <!-- Search & Sort Row -->
      <div class="flex items-center gap-2 mb-2">
        <div class="flex-1">
          <UniversalSearchBar
            bind:query={searchQuery}
            bind:searchType
            onSearch={loadProducts}
          />
        </div>

        <!-- Sort dropdown -->
        <div class="flex items-center gap-1 bg-slate-100 dark:bg-slate-800 px-2.5 py-1.5 rounded-xl border border-pos-border text-xs">
          <ArrowUpDown class="w-3.5 h-3.5 text-pos-muted" />
          <select bind:value={sortBy} on:change={loadProducts} class="bg-transparent text-pos-text font-bold outline-none cursor-pointer">
            <option value="default">Default Sort</option>
            <option value="name_asc">Name (A-Z)</option>
            <option value="name_desc">Name (Z-A)</option>
            <option value="price_asc">Price (Low → High)</option>
            <option value="price_desc">Price (High → Low)</option>
            <option value="stock">Most Stock</option>
          </select>
        </div>
      </div>

      <!-- Categories Pills with Wrap -->
      <div class="flex items-center gap-1.5 flex-wrap pb-1.5 mb-1.5 shrink-0">
        <button
          type="button"
          on:click={() => handleCategoryClick(null)}
          class="px-3 py-1.5 rounded-xl text-xs font-bold shrink-0 transition cursor-pointer {selectedCategory === null ? 'bg-sky-600 text-white shadow-xs' : 'bg-slate-100 dark:bg-slate-800 text-pos-muted hover:bg-slate-200'}"
        >
          {t('filter_all')}
        </button>

        {#each categories as cat}
          <button
            type="button"
            on:click={() => handleCategoryClick(cat.id)}
            class="px-3 py-1.5 rounded-xl text-xs font-bold shrink-0 transition flex items-center gap-1.5 cursor-pointer {selectedCategory === cat.id ? 'bg-sky-600 text-white shadow-xs' : 'bg-slate-100 dark:bg-slate-800 text-pos-muted hover:bg-slate-200'}"
          >
            <span class="w-2.5 h-2.5 rounded-full" style="background-color: {cat.color || '#0284c7'}"></span>
            <span>{cat.name_ar || cat.name_fr}</span>
          </button>
        {/each}

        <!-- Add / Manage Categories -->
        <button
          type="button"
          on:click={() => (isCategoryManagerOpen = true)}
          class="p-1.5 rounded-xl text-xs font-bold bg-slate-100 dark:bg-slate-800 text-pos-muted hover:text-sky-600 hover:bg-slate-200 shrink-0 transition cursor-pointer"
          title="Manage Groups"
        >
          <Settings2 class="w-4 h-4" />
        </button>
      </div>

      <!-- Products Grid -->
      <div class="flex-1 overflow-y-auto pr-1">
        <div class="grid grid-cols-2 md:grid-cols-3 xl:grid-cols-4 gap-2.5">
          <!-- CARD 1: Interactive "+ Quick Add Product" Card -->
          <button
            type="button"
            on:click={handleOpenNewProduct}
            class="flex flex-col items-center justify-center p-4 bg-sky-50/50 dark:bg-sky-950/20 border-2 border-dashed border-sky-400 hover:border-sky-600 rounded-2xl text-sky-600 dark:text-sky-400 cursor-pointer transition hover:scale-[1.02] active:scale-95 group shadow-xs min-h-[160px]"
          >
            <div class="w-12 h-12 rounded-2xl bg-sky-100 dark:bg-sky-900/60 flex items-center justify-center group-hover:rotate-90 transition duration-200 mb-2">
              <Plus class="w-6 h-6" />
            </div>
            <span class="font-black text-xs text-center">Add Product</span>
            <span class="text-[10px] text-pos-muted font-bold">إضافة منتج جديد</span>
          </button>

          <!-- Product Catalog Cards with Category Borders and Pen Icon -->
          {#each products as product (product.id)}
            {@const cat = categories.find(c => c.id === product.category_id)}
            <ProductCard
              {product}
              categoryColor={cat?.color || '#0284c7'}
              onEditProduct={handleOpenEdit}
            />
          {/each}
        </div>
      </div>
    </div>

    <!-- RIGHT PANEL: Shopping Cart with Animated Store Title -->
    <div class="w-[410px] flex flex-col shrink-0 bg-pos-card border border-pos-border rounded-2xl shadow-xs overflow-hidden">
      <!-- Big Animated Store Header -->
      <div class="p-3 bg-gradient-to-r from-sky-600 via-indigo-600 to-sky-700 text-white shadow-xs">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2.5">
            <div class="w-9 h-9 rounded-xl bg-white p-1 flex items-center justify-center shadow-md">
              <img src="/logo.png" alt="Logo" class="w-full h-full object-contain" />
            </div>
            <div>
              <h2 class="font-black text-base tracking-tight leading-none text-white drop-shadow-xs">
                TitaouPOS Market
              </h2>
              <p class="text-[10px] text-sky-200 font-bold leading-none mt-1">
                Point de Vente & Caisse Moderne
              </p>
            </div>
          </div>
          <div class="flex items-center gap-1 bg-white/20 px-2 py-0.5 rounded-full text-[10px] font-bold">
            <span class="w-2 h-2 rounded-full bg-emerald-400 animate-ping"></span>
            <span>Live</span>
          </div>
        </div>
      </div>

      <!-- Closed Cash Register Warning Alert Banner -->
      {#if !$activeSession}
        <div class="p-2.5 bg-amber-500/15 border-b border-amber-500/30 flex items-center justify-between text-amber-800 dark:text-amber-300 text-xs">
          <div class="flex items-center gap-1.5 font-bold">
            <AlertCircle class="w-4 h-4 text-amber-500 shrink-0" />
            <span>Cash session is closed / الصندوق مغلق</span>
          </div>
          <button
            on:click={() => (isCashDrawerOpen = true)}
            class="px-2.5 py-1 bg-amber-600 hover:bg-amber-700 text-white font-black text-[11px] rounded-lg cursor-pointer"
          >
            Open Session
          </button>
        </div>
      {/if}

      <!-- Cart Header -->
      <div class="p-3 border-b border-pos-border bg-slate-50 dark:bg-slate-800/40 flex items-center justify-between">
        <div class="flex items-center gap-2">
          <ShoppingBag class="w-4 h-4 text-sky-600" />
          <span class="font-extrabold text-xs text-pos-text">Shopping Cart</span>
          <span class="text-[11px] font-bold bg-sky-100 dark:bg-sky-950 text-sky-700 dark:text-sky-300 px-2 py-0.2 rounded-full font-mono">
            {$cartItems.length} items
          </span>
        </div>
        {#if $cartItems.length > 0}
          <button
            on:click={clearCart}
            class="text-[11px] text-rose-500 hover:text-rose-700 font-bold cursor-pointer"
          >
            {t('btn_delete_cart')}
          </button>
        {/if}
      </div>

      <!-- Cart Item Cards List -->
      <div class="flex-1 overflow-y-auto p-2.5 space-y-2">
        {#if $cartItems.length === 0}
          <div class="h-full flex flex-col items-center justify-center text-pos-muted p-4 text-center">
            <ShoppingBag class="w-10 h-10 stroke-1 mb-2 opacity-20" />
            <p class="text-xs font-medium">{t('cart_empty')}</p>
          </div>
        {:else}
          {#each $cartItems as item (item.product_id + (item.is_refund ? '_ref' : ''))}
            <CartItemCard {item} />
          {/each}
        {/if}
      </div>

      <!-- Totals & Checkout Footer -->
      <div class="p-3.5 border-t border-pos-border bg-slate-50 dark:bg-slate-800/40 space-y-3">
        <div class="flex items-center justify-between">
          <span class="text-xs font-black text-pos-text uppercase tracking-wider">TOTAL</span>
          <span class="text-3xl font-black font-mono {$cartGrandTotal < 0 ? 'text-amber-600' : 'text-sky-600 dark:text-sky-400'}">
            {$cartGrandTotal.toLocaleString()} DZD
          </span>
        </div>

        <button
          type="button"
          on:click={handleFastCheckout}
          disabled={$cartItems.length === 0}
          class="w-full py-3.5 px-4 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-40 text-white font-black text-sm rounded-xl transition shadow-md flex items-center justify-center gap-2 cursor-pointer active:scale-98"
        >
          <span>Checkout [{selectedPaymentMode.toUpperCase()}]</span>
          <ArrowRight class="w-4 h-4" />
        </button>
      </div>
    </div>
  </div>

  <!-- Modals -->
  <PaymentModal
    isOpen={isPaymentOpen}
    onClose={() => (isPaymentOpen = false)}
    onSaleSuccess={(num) => { lastSaleSuccessNumber = num; loadProducts(); }}
  />

  <CashDrawerModal
    isOpen={isCashDrawerOpen}
    onClose={() => (isCashDrawerOpen = false)}
  />

  <CategoryManagerModal
    isOpen={isCategoryManagerOpen}
    onClose={() => (isCategoryManagerOpen = false)}
    onCategoryChanged={() => { loadCategories(); loadProducts(); }}
  />

  <HeldSalesModal
    isOpen={isHeldSalesOpen}
    onClose={() => (isHeldSalesOpen = false)}
  />

  <RemiseModal
    isOpen={isRemiseOpen}
    onClose={() => (isRemiseOpen = false)}
  />

  <PrintReceiptModal
    isOpen={isPrintReceiptOpen}
    onClose={() => (isPrintReceiptOpen = false)}
  />

  <ProductEditModal
    isOpen={isProductEditOpen}
    product={editingProduct}
    categories={categories}
    units={units}
    onClose={() => (isProductEditOpen = false)}
    onSaved={loadProducts}
  />

  <QuickPurchaseModal
    isOpen={isQuickPurchaseOpen}
    suppliers={suppliers}
    onClose={() => (isQuickPurchaseOpen = false)}
    onPurchaseCompleted={loadProducts}
  />

  <ReturnDamagedModal
    isOpen={isReturnDamagedOpen}
    suppliers={suppliers}
    onClose={() => (isReturnDamagedOpen = false)}
    onReturnCompleted={loadProducts}
  />

  <CreditCustomerModal
    isOpen={isCreditCustomerOpen}
    totalAmount={$cartGrandTotal}
    onClose={() => (isCreditCustomerOpen = false)}
    onConfirmCredit={(cId, cName) => executeCheckout(cId, cName)}
  />
</div>