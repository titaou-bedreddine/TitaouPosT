<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { t, currentLocale } from '../../lib/i18n';
  import type { Category, Product, Supplier, Unit } from '../../lib/types';
  import { cartItems, cartGrandTotal, cartSubtotal, globalDiscountAmount, globalDiscountMode, globalDiscountValue, globalDiscountPercent, isRefundMode, addToCart, clearCart, cartItemOrder, qtyEditTarget, itemKey, stopQtyEdit, posMode, restoreActiveCart } from '../../lib/stores/cart';
  import { currentUser } from '../../lib/stores/auth';
  import { activeSession } from '../../lib/stores/session';
  import { printHtmlSilently, buildReceiptHtml } from '../../lib/utils/printer';
  import { normalizeBarcode } from '../../lib/utils/barcode';

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
  import VersementCustomerModal from '../../lib/components/VersementCustomerModal.svelte';
  import CheckoutModal from '../../lib/components/CheckoutModal.svelte';
  import OtherArticleModal from '../../lib/components/OtherArticleModal.svelte';
  import UnknownBarcodeModal from '../../lib/components/UnknownBarcodeModal.svelte';

  import { customers, selectedCustomerId, selectedCustomer, refreshCustomers, DEFAULT_WALKIN_CUSTOMER_ID } from '../../lib/stores/customers';
  import { suppliers, selectedSupplierId, selectedSupplier, refreshSuppliers } from '../../lib/stores/suppliers';

  import {
    ShoppingBag, ArrowRight, CheckCircle2, Settings2, Plus,
    Store, Sparkles, AlertCircle, ArrowUpDown, Tag, Percent, UserRound, ChevronDown, Truck
  } from 'lucide-svelte';

  let products: Product[] = [];
  let categories: Category[] = [];
  let units: Unit[] = [];

  let selectedCategory: number | null = null;
  let searchQuery = '';
  let searchType: 'all' | 'name' | 'barcode' | 'price' | 'qr' = 'all';
  let sortBy: 'name_asc' | 'name_desc' | 'price_asc' | 'price_desc' | 'stock' | 'best_sellers' | 'worst_sellers' = 'name_asc';

  let selectedPaymentMode: 'cash' | 'card' | 'credit' | 'versement' = 'cash';
  let autoPrintEnabled = true;
  let autoDrawerEnabled = true;
  let isFastCheckingOut = false;

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
  let isVersementOpen = false;
  let isCheckoutOpen = false;
  let isCustomerSelectorOpen = false;
  let isSupplierSelectorOpen = false;
  let isOtherArticleOpen = false;

  let isUnknownBarcodeModalOpen = false;
  let unknownScannedBarcode = '';
  let initialBarcodeForNewProduct = '';
  let editingProductWithExtraBarcode = '';

  let lastSaleSuccessNumber = '';
  let barcodeBuffer = '';
  let lastKeyTime = 0;

  let currentShopName = 'TitaouPOS';
  let currentTime = new Date().toLocaleTimeString();
  let currentDate = new Date().toLocaleDateString();
  let timeInterval: any;
  // POS rule: idle seconds before the search bar re-steals focus (0 = off).
  let autofocusTimerSeconds = 0;

  let cartContainerEl: HTMLDivElement;

  onMount(async () => {
    try {
      const s = await invoke<Record<string, string>>('get_all_settings');
      currentShopName = s['shop_name_fr'] || s['shop_name_ar'] || 'TitaouPOS';
      if (s['cart_item_order'] === 'top' || s['cart_item_order'] === 'bottom') {
        $cartItemOrder = s['cart_item_order'];
      }
      // Auto-focus search: enabled flag + idle timer in seconds.
      if (s['pos_autofocus_search'] === 'true') {
        autofocusTimerSeconds = parseInt(s['pos_autofocus_timer_seconds'] || '10', 10) || 10;
      }
    } catch (e) {
      console.warn(e);
    }

    timeInterval = setInterval(() => {
      currentTime = new Date().toLocaleTimeString();
      currentDate = new Date().toLocaleDateString();
    }, 1000);

    await loadCategories();
    await loadUnits();
    await loadProducts();
    await refreshCustomers();
    await refreshSuppliers();

    // Recover an interrupted sale (shutdown/crash) before it was checked out.
    await restoreActiveCart();

    window.addEventListener('keydown', handleGlobalKeyDown);
  });

  // Auto-scroll ONLY when a new item is actually added (not on every cart
  // mutation like qty edits, which caused the jumpy scrolling). The new line
  // scrolls into view at the top or bottom per the configured order.
  let lastCartCount = 0;
  $: if ($cartItems.length !== lastCartCount) {
    const grew = $cartItems.length > lastCartCount;
    lastCartCount = $cartItems.length;
    if (grew && cartContainerEl) {
      requestAnimationFrame(() => {
        if (!cartContainerEl) return;
        if ($cartItemOrder === 'top') {
          cartContainerEl.scrollTop = 0;
        } else {
          cartContainerEl.scrollTop = cartContainerEl.scrollHeight;
        }
      });
    }
  }

  onDestroy(() => {
    clearInterval(timeInterval);
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


  async function loadProducts() {
    try {
      const list = await invoke<Product[]>('search_products', {
        query: searchQuery,
        categoryId: selectedCategory,
        searchType: searchType === 'qr' ? 'barcode' : searchType,
      });

      // Sort products
      if (sortBy === 'name_asc') {
        list.sort((a, b) => ((a.name_fr || a.name_ar || '') as string).localeCompare((b.name_fr || b.name_ar || '') as string));
      } else if (sortBy === 'name_desc') {
        list.sort((a, b) => ((b.name_fr || b.name_ar || '') as string).localeCompare((a.name_fr || a.name_ar || '') as string));
      } else if (sortBy === 'price_asc') {
        list.sort((a, b) => a.sale_price - b.sale_price);
      } else if (sortBy === 'price_desc') {
        list.sort((a, b) => b.sale_price - a.sale_price);
      } else if (sortBy === 'stock') {
        list.sort((a, b) => b.current_stock - a.current_stock);
      } else if (sortBy === 'best_sellers') {
        list.sort((a, b) => (b.total_sold || 0) - (a.total_sold || 0));
      } else if (sortBy === 'worst_sellers') {
        list.sort((a, b) => (a.total_sold || 0) - (b.total_sold || 0));
      }

      products = list;

      if (searchQuery.trim().length >= 6 && searchType === 'barcode') {
        const queryCode = searchQuery.trim();
        const matched = list.find(p => p.barcodes?.includes(queryCode) || p.sku === queryCode);
        if (matched) {
          addToCart(matched, 1, $isRefundMode);
          searchQuery = '';
          await loadProducts();
        } else {
          unknownScannedBarcode = queryCode;
          isUnknownBarcodeModalOpen = true;
          searchQuery = '';
        }
      } else if (searchQuery.trim().length >= 8 && searchType === 'all' && products.length === 1) {
        const exactMatch = products[0].barcodes?.includes(searchQuery.trim()) || products[0].sku === searchQuery.trim();
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

  async function handleScannedBarcode(rawBarcode: string) {
    const code = normalizeBarcode(rawBarcode).trim();
    if (!code) return;

    try {
      const list = await invoke<Product[]>('search_products', {
        query: code,
        categoryId: null,
        searchType: 'barcode',
      });

      const matched = list.find(p => p.barcodes?.includes(code) || p.sku === code);
      if (matched) {
        addToCart(matched, 1, $isRefundMode);
        searchQuery = '';
        await loadProducts();
      } else {
        unknownScannedBarcode = code;
        isUnknownBarcodeModalOpen = true;
      }
    } catch (e) {
      console.error('Error looking up scanned barcode:', e);
    }
  }

  function handleCategoryClick(catId: number | null) {
    selectedCategory = catId;
    loadProducts();
  }

  function handleOpenEdit(p: Product) {
    editingProduct = p;
    initialBarcodeForNewProduct = '';
    editingProductWithExtraBarcode = '';
    isProductEditOpen = true;
  }

  // Link flow: open the editor for an existing product with the scanned
  // barcode pre-attached for review before saving.
  function handleEditProductWithBarcode(p: Product, newBarcode: string) {
    editingProduct = p;
    initialBarcodeForNewProduct = '';
    editingProductWithExtraBarcode = newBarcode;
    isProductEditOpen = true;
  }

  function handleOpenNewProduct() {
    editingProduct = null;
    initialBarcodeForNewProduct = '';
    editingProductWithExtraBarcode = '';
    isProductEditOpen = true;
  }

  async function handleProductSaved() {
    await loadProducts();
    // After creating a product from a scanned barcode (or linking that
    // barcode to an existing one), add it straight to the cart.
    const pendingBarcode = initialBarcodeForNewProduct || editingProductWithExtraBarcode;
    if (pendingBarcode) {
      try {
        const list = await invoke<Product[]>('search_products', {
          query: pendingBarcode,
          categoryId: null,
          searchType: 'barcode',
        });
        const created = list.find(p => p.barcodes?.includes(pendingBarcode));
        if (created) {
          addToCart(created, 1, $isRefundMode);
        }
      } catch (e) {
        console.warn('Auto add created product error:', e);
      }
      initialBarcodeForNewProduct = '';
      editingProductWithExtraBarcode = '';
    }
  }

  // Fast Checkout Action
  async function handleFastCheckout() {
    if ($cartItems.length === 0) return;

    if (!$activeSession) {
      isCashDrawerOpen = true;
      return;
    }

    // Purchase / Broken modes have their own checkout flows.
    if ($posMode === 'purchase') {
      await executeModeCheckout('purchase');
      return;
    }
    if ($posMode === 'broken') {
      await executeModeCheckout('broken');
      return;
    }

    if (selectedPaymentMode === 'credit') {
      isCreditCustomerOpen = true;
      return;
    }

    if (selectedPaymentMode === 'versement') {
      isVersementOpen = true;
      return;
    }

    // Cash or TPE Checkout directly
    await executeCheckout(null, undefined);
  }

  // Purchase & Broken mode checkouts: purchases add stock from the
  // supplier; broken writes quantity off and books an expense.
  async function executeModeCheckout(mode: 'purchase' | 'broken') {
    if ($cartItems.length === 0) return;
    try {
      const stamp = Date.now().toString().slice(-8);
      if (mode === 'purchase') {
        // Stock-in purchase from the selected supplier at cart line prices.
        const total = $cartItems.reduce((s, i) => s + Math.round(i.unit_price * i.quantity), 0);
        await invoke('create_purchase', {
          input: {
            invoiceNumber: 'ACH-' + stamp,
            supplierId: $selectedSupplierId ?? 1,
            userId: $currentUser?.id || 1,
            date: new Date().toISOString().split('T')[0],
            subtotal: total,
            discount: 0,
            tax: 0,
            total,
            paidAmount: total,
            paymentMethod: 'cash',
            items: $cartItems.map((i) => ({
              product_id: i.product_id,
              quantity: i.quantity,
              unit_cost: Math.round(i.unit_price),
              discount: 0,
              tax: 0,
              total: Math.round(i.unit_price * i.quantity),
            })),
            notes: 'POS Purchase Mode (شراء)',
          },
        });
      } else {
        // Broken: quantity leaves stock and the value becomes an expense
        // per line (negative-quantity purchase = write-off movement).
        const total = $cartItems.reduce((s, i) => s + Math.round(i.unit_price * i.quantity), 0);
        await invoke('create_purchase', {
          input: {
            invoiceNumber: 'BRK-' + stamp,
            supplierId: 1,
            userId: $currentUser?.id || 1,
            date: new Date().toISOString().split('T')[0],
            subtotal: total,
            discount: 0,
            tax: 0,
            total: 0,
            paidAmount: 0,
            paymentMethod: 'cash',
            items: $cartItems.map((i) => ({
              product_id: i.product_id,
              quantity: -i.quantity,
              unit_cost: Math.round(i.unit_price),
              discount: 0,
              tax: 0,
              total: Math.round(i.unit_price * i.quantity),
            })),
            notes: `Broken/Damaged write-off (تالف): ${$cartItems.map((i) => `${i.name_fr || i.name_ar} x${i.quantity}`).join(', ')}`,
          },
        });
      }
      lastSaleSuccessNumber = mode === 'purchase' ? 'Purchase saved' : 'Broken write-off saved';
      clearCart();
      await loadProducts();
      setTimeout(() => (lastSaleSuccessNumber = ''), 4000);
    } catch (err: any) {
      console.error('Mode checkout failed:', err);
      alert('Checkout Failed: ' + (err?.message || err));
    }
  }

  // Unified Checkout dialog (TopBar Checkout / F2): customer + amount +
  // reste/change; amount < total lets the cashier pick credit vs versement.
  function handleOpenCheckout() {
    if ($cartItems.length === 0) return;
    if (!$activeSession) {
      isCashDrawerOpen = true;
      return;
    }
    isCheckoutOpen = true;
  }

  function handleCheckoutConfirm(result: {
    customerId: number;
    customerName: string;
    paidAmount: number;
    mode: 'direct' | 'credit' | 'versement';
  }) {
    $selectedCustomerId = result.customerId;
    const change = Math.max(0, result.paidAmount - $cartGrandTotal);
    executeCheckout(result.customerId, result.customerName, {
      mode: result.mode,
      paidAmount: result.paidAmount,
      changeAmount: change,
    });
  }

  async function executeCheckout(
    customerId: number | null,
    customerName?: string,
    options?: {
      mode?: 'direct' | 'credit' | 'versement';
      paidAmount?: number;
      changeAmount?: number;
    }
  ) {
    try {
      const saleNumber = 'VTE-' + Date.now().toString().slice(-6);
      const saleDate = new Date().toLocaleString();
      const cashier = $currentUser?.display_name || 'Admin';

      const mode = options?.mode || 'direct';
      const paid = mode === 'direct' ? $cartGrandTotal : (options?.paidAmount ?? 0);
      const change = mode === 'direct' ? 0 : (options?.changeAmount ?? 0);
      const reste = Math.max(0, $cartGrandTotal - paid);
      const effectiveMethod = mode === 'versement' ? 'versement' : mode === 'credit' ? 'credit' : selectedPaymentMode;

      const saleInput = {
        session_id: $activeSession?.id || 1,
        customer_id: customerId,
        user_id: $currentUser?.id || 1,
        subtotal: $cartSubtotal,
        total_amount: $cartGrandTotal,
        paid_amount: paid,
        change_amount: change,
        tax_amount: 0,
        discount_amount: $globalDiscountAmount,
        discount_percentage: $globalDiscountPercent,
        payment_method: effectiveMethod,
        is_refund: $isRefundMode,
        notes:
          mode === 'versement'
            ? `Versement Sale to ${customerName} — Paid ${paid} DZD, Reste ${reste} DZD (goods at shop)`
            : mode === 'credit'
              ? `Credit Sale to ${customerName} — Paid ${paid} DZD, Reste ${reste} DZD`
              : customerName
                ? `Direct Sale to ${customerName}`
                : undefined,
        skip_stock: mode === 'versement',
        payments: [
          {
            // Record only what was actually handed over now; the remainder is
            // tracked on the customer's account for credit AND versement.
            payment_method: effectiveMethod,
            amount: paid,
            reference_code: null,
          }
        ],
        items: $cartItems.map((i) => ({
          product_id: i.product_id,
          sku: i.sku || '',
          barcode: i.barcode || '',
          name_fr: i.name_fr || '',
          name_ar: i.name_ar || '',
          name_en: i.name_en || '',
          quantity: i.quantity,
          unit_price: i.unit_price,
          discount_amount: i.discount_amount || 0,
          tax_amount: i.tax_amount || 0,
          total_price: i.total_price,
          is_refund: i.is_refund || false,
        })),
      };

      await invoke('create_sale', { input: saleInput });

      // Silent Auto-Print
      if (autoPrintEnabled) {
        let appSettings: Record<string, string> = {};
        try {
          appSettings = await invoke<Record<string, string>>('get_all_settings');
        } catch (e) {
          console.warn('Could not load settings for receipt:', e);
        }

        const shopName = appSettings['shop_name_fr'] || appSettings['shop_name_ar'] || 'TitaouPOS Superette';
        const shopAddress = appSettings['shop_address'] || 'Rue Principale, Alger';
        const shopPhone = appSettings['shop_phone'] || '0553444057';
        const shopRc = appSettings['shop_rc'] || undefined;
        const shopNif = appSettings['shop_nif'] || undefined;

        const receiptItems = $cartItems.map((i) => ({
          name: i.name_fr || i.name_ar,
          quantity: i.quantity,
          unitPrice: i.unit_price,
          totalPrice: i.total_price,
          discountPerUnit: i.discount_amount || 0,
          isRefund: i.is_refund || false,
        }));

        if (mode === 'credit') {
          // Print 2 Copies: Store Copy + Client Copy
          const creditReceiptOpts = {
            shopName,
            shopAddress,
            shopPhone,
            shopRc,
            shopNif,
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
          };
          printHtmlSilently(
            buildReceiptHtml({ ...creditReceiptOpts, copyLabel: 'COPIE MAGASIN / STORE COPY' }) +
              '<div style="page-break-after:always;"></div>' +
              buildReceiptHtml({ ...creditReceiptOpts, copyLabel: 'COPIE CLIENT / CUSTOMER COPY' }),
            'Credit Receipts'
          );
        } else if (mode === 'versement') {
          // Layaway ticket: deposit paid now, remainder tracked on the customer.
          const receipt = buildReceiptHtml({
            shopName,
            shopAddress,
            shopPhone,
            shopRc,
            shopNif,
            saleNumber,
            saleDate,
            cashierName: cashier,
            customerName: customerName || 'Client Versement',
            items: receiptItems,
            subtotal: $cartSubtotal,
            discount: $globalDiscountAmount,
            grandTotal: $cartGrandTotal,
            paymentMethod: 'VERSEMENT (تسبقة)',
            versementPaid: paid,
            versementRemaining: reste,
            copyLabel: 'VERSEMENT / تسبقة',
          });
          printHtmlSilently(receipt, 'Versement Receipt #' + saleNumber);
        } else {
          const receipt = buildReceiptHtml({
            shopName,
            shopAddress,
            shopPhone,
            shopRc,
            shopNif,
            saleNumber,
            saleDate,
            cashierName: cashier,
            customerName: customerName || undefined,
            items: receiptItems,
            subtotal: $cartSubtotal,
            discount: $globalDiscountAmount,
            grandTotal: $cartGrandTotal,
            paymentMethod: effectiveMethod.toUpperCase(),
          });
          printHtmlSilently(receipt, 'Sale Receipt #' + saleNumber);
        }
      }

      // Auto-kick cash drawer
      if (autoDrawerEnabled && mode === 'direct' && effectiveMethod === 'cash') {
        try {
          const settings = await invoke<Record<string, string>>('get_all_settings');
          const port = parseInt(settings['drawer_com_port'] || '1');
          const baud = parseInt(settings['drawer_baud_rate'] || '9600');
          await invoke('open_serial_cash_drawer', { comPort: port, baudRate: baud });
        } catch (drawerErr) {
          console.warn('Native drawer checkout trigger notice:', drawerErr);
        }
      }

      lastSaleSuccessNumber = saleNumber;
      clearCart();
      await loadProducts();

      setTimeout(() => {
        lastSaleSuccessNumber = '';
      }, 4000);
    } catch (err: any) {
      console.error('Failed to complete sale:', err);
      alert('Checkout Failed: ' + (err.message || err));
    }
  }

  // Quantity-edit mode navigation: start on the first cart line (F6), then
  // Enter jumps to the next line, ESC leaves the mode entirely.
  function advanceQtyEdit() {
    const keys = $cartItems.map(itemKey);
    if (keys.length === 0) return;
    const currentIdx = keys.indexOf($qtyEditTarget);
    const nextIdx = currentIdx < 0 ? 0 : currentIdx + 1;
    if (nextIdx < keys.length) {
      qtyEditTarget.set(keys[nextIdx]);
    } else {
      // Past the last line: exit edit mode.
      stopQtyEdit();
    }
  }

  // Global Keyboard Shortcuts
  function handleGlobalKeyDown(e: KeyboardEvent) {
    // While editing quantities, Enter advances to the next cart line and
    // ESC exits the mode. Swallow both so they never trigger other actions.
    if ($qtyEditTarget) {
      if (e.key === 'Enter') {
        e.preventDefault();
        e.stopPropagation();
        advanceQtyEdit();
        return;
      }
      if (e.key === 'Escape') {
        e.preventDefault();
        e.stopPropagation();
        stopQtyEdit();
        return;
      }
    }

    // Barcode scanner rapid input detection
    const now = Date.now();
    if (now - lastKeyTime > 100) {
      barcodeBuffer = '';
    }
    lastKeyTime = now;

    if (e.key === 'Enter') {
      if (barcodeBuffer.length >= 6) {
        e.preventDefault();
        const code = barcodeBuffer;
        barcodeBuffer = '';
        handleScannedBarcode(code);
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
      // Enter quantity-edit mode on the first cart line (or the next one if
      // already editing).
      if ($cartItems.length === 0) return;
      if ($qtyEditTarget) {
        advanceQtyEdit();
      } else {
        qtyEditTarget.set(itemKey($cartItems[0]));
      }
    } else if (e.key === 'F7') {
      e.preventDefault();
      // Cycle the POS mode directly: Sale -> Purchase -> Broken.
      if ($posMode === 'sale') $posMode = 'purchase';
      else if ($posMode === 'purchase') $posMode = 'broken';
      else $posMode = 'sale';
    } else if (e.key === 'F8') {
      e.preventDefault();
      isReturnDamagedOpen = true;
    } else if (e.key === 'F9') {
      e.preventDefault();
      isCashDrawerOpen = true;
    } else if (e.key === 'F10') {
      e.preventDefault();
      try {
        invoke('open_serial_cash_drawer', { comPort: 1, baudRate: 9600 });
      } catch (e) {
        console.warn(e);
      }
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
    onCheckout={handleOpenCheckout}
    onOpenCashDrawer={() => (isCashDrawerOpen = true)}
    onOpenRemise={() => (isRemiseOpen = true)}
    onOpenHeldSales={() => (isHeldSalesOpen = true)}
    onPrintReceipt={() => (isPrintReceiptOpen = true)}
    onOpenOtherArticle={() => (isOtherArticleOpen = true)}
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
            autofocusSeconds={autofocusTimerSeconds}
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
            <option value="best_sellers">Best Sellers (Most Sold)</option>
            <option value="worst_sellers">Least Sold</option>
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
            <div class="w-10 h-10 rounded-2xl bg-white p-1 flex items-center justify-center shadow-md shrink-0">
              <img src="/logo.png" alt="Logo" class="w-full h-full object-contain" />
            </div>
            <div>
              <h2 class="font-black text-base tracking-tight leading-none text-white drop-shadow-xs truncate max-w-[210px]">
                {currentShopName}
              </h2>
              <p class="text-[10px] text-sky-200 font-bold leading-none mt-1">
                {currentDate} • {currentTime}
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
      <div class="p-3 border-b border-pos-border bg-slate-50 dark:bg-slate-800/40 space-y-2">
        <div class="flex items-center justify-between">
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

        <!-- Customer Selector (defaults to Walk-in / Client Comptoir).
             In purchase mode the supplier selector replaces it. -->
        {#if $posMode !== 'purchase'}
        <div class="relative" class:z-30={isCustomerSelectorOpen}>
          <button
            type="button"
            on:click={() => (isCustomerSelectorOpen = !isCustomerSelectorOpen)}
            class="w-full flex items-center justify-between gap-2 px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-bold cursor-pointer hover:border-sky-400 transition {isCustomerSelectorOpen ? 'ring-2 ring-sky-400 border-sky-400' : ''}"
            title="Select customer for this sale"
          >
            <span class="flex items-center gap-2 min-w-0">
              <UserRound class="w-3.5 h-3.5 {$selectedCustomer?.id === DEFAULT_WALKIN_CUSTOMER_ID ? 'text-pos-muted' : 'text-sky-600'} shrink-0" />
              <span class="truncate text-pos-text">{$selectedCustomer?.name || 'Client Comptoir / زبون عادي'}</span>
              {#if $selectedCustomer && $selectedCustomer.balance > 0}
                <span class="text-[9px] font-mono font-black text-rose-600 bg-rose-50 dark:bg-rose-950/50 px-1.5 py-0.5 rounded-full shrink-0">
                  {$selectedCustomer.balance.toLocaleString()} DZD dette
                </span>
              {/if}
            </span>
            <ChevronDown class="w-3.5 h-3.5 text-pos-muted shrink-0 transition-transform {isCustomerSelectorOpen ? 'rotate-180' : ''}" />
          </button>

          {#if isCustomerSelectorOpen}
            <div class="absolute left-0 right-0 top-full mt-1 bg-pos-card border border-pos-border rounded-xl shadow-2xl overflow-hidden z-40 animate-in fade-in duration-100">
              <div class="max-h-56 overflow-y-auto p-1.5 space-y-1">
                {#each $customers as c}
                  <button
                    type="button"
                    on:click={() => { $selectedCustomerId = c.id; isCustomerSelectorOpen = false; }}
                    class="w-full flex items-center justify-between gap-2 px-2.5 py-2 rounded-lg text-start transition cursor-pointer {$selectedCustomerId === c.id ? 'bg-sky-100 dark:bg-sky-950 text-sky-700 dark:text-sky-300' : 'hover:bg-slate-100 dark:hover:bg-slate-800 text-pos-text'}"
                  >
                    <span class="min-w-0">
                      <span class="text-[11px] font-black block truncate">{c.name}</span>
                      <span class="text-[9px] text-pos-muted block truncate">{c.phone || 'No phone'}</span>
                    </span>
                    {#if c.balance > 0}
                      <span class="text-[9px] font-mono font-black text-rose-600 shrink-0">{c.balance.toLocaleString()}</span>
                    {/if}
                  </button>
                {/each}
              </div>
            </div>
          {/if}
        </div>
        {/if}
        <!-- Supplier Selector: purchase mode picks the source supplier -->
        {#if $posMode === 'purchase'}
          <div class="relative" class:z-30={isSupplierSelectorOpen}>
            <button
              type="button"
              on:click={() => (isSupplierSelectorOpen = !isSupplierSelectorOpen)}
              class="w-full flex items-center justify-between gap-2 px-3 py-2 bg-white dark:bg-slate-900 border border-sky-400 rounded-xl text-xs font-bold cursor-pointer transition {isSupplierSelectorOpen ? 'ring-2 ring-sky-400' : ''}"
              title="Select supplier for this purchase"
            >
              <span class="flex items-center gap-2 min-w-0">
                <Truck class="w-3.5 h-3.5 text-sky-600 shrink-0" />
                <span class="truncate text-pos-text">{$selectedSupplier?.name || 'Fournisseur Divers / مورد متنوع'}</span>
                {#if $selectedSupplier && $selectedSupplier.balance > 0}
                  <span class="text-[9px] font-mono font-black text-amber-600 bg-amber-50 dark:bg-amber-950/50 px-1.5 py-0.5 rounded-full shrink-0">
                    {$selectedSupplier.balance.toLocaleString()} DZD due
                  </span>
                {/if}
              </span>
              <ChevronDown class="w-3.5 h-3.5 text-pos-muted shrink-0 transition-transform {isSupplierSelectorOpen ? 'rotate-180' : ''}" />
            </button>

            {#if isSupplierSelectorOpen}
              <div class="absolute left-0 right-0 top-full mt-1 bg-pos-card border border-pos-border rounded-xl shadow-2xl overflow-hidden z-40 animate-in fade-in duration-100">
                <div class="max-h-56 overflow-y-auto p-1.5 space-y-1">
                  {#each $suppliers as s}
                    <button
                      type="button"
                      on:click={() => { $selectedSupplierId = s.id; isSupplierSelectorOpen = false; }}
                      class="w-full flex items-center justify-between gap-2 px-2.5 py-2 rounded-lg text-start transition cursor-pointer {$selectedSupplierId === s.id ? 'bg-sky-100 dark:bg-sky-950 text-sky-700 dark:text-sky-300' : 'hover:bg-slate-100 dark:hover:bg-slate-800 text-pos-text'}"
                    >
                      <span class="min-w-0">
                        <span class="text-[11px] font-black block truncate">{s.name}</span>
                        <span class="text-[9px] text-pos-muted block truncate">{s.phone || 'No phone'}</span>
                      </span>
                      {#if s.balance > 0}
                        <span class="text-[9px] font-mono font-black text-amber-600 shrink-0">{s.balance.toLocaleString()}</span>
                      {/if}
                    </button>
                  {/each}
                </div>
              </div>
            {/if}
          </div>
        {/if}
      </div>
      <div bind:this={cartContainerEl} class="flex-1 overflow-y-auto p-2.5 space-y-2">
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
        {#if $globalDiscountAmount > 0}
          <!-- Applied Remise row, above the total -->
          <div class="flex items-center justify-between p-2.5 bg-purple-50/70 dark:bg-purple-950/30 rounded-xl border border-purple-200/70 dark:border-purple-800/60">
            <div class="flex items-center gap-1.5">
              <Percent class="w-3.5 h-3.5 text-purple-600" />
              <span class="text-[10px] font-black text-pos-muted uppercase tracking-wider">
                Remise appliquée{$globalDiscountMode === 'percent' ? ` (${$globalDiscountValue}%)` : ''}
              </span>
            </div>
            <div class="text-end">
              <span class="text-base font-black font-mono text-purple-600 dark:text-purple-400">
                -{$globalDiscountAmount.toLocaleString()} <span class="text-[10px] font-bold">DZD</span>
              </span>
              <span class="text-[10px] font-bold text-pos-muted block">
                Sous-Total: {$cartSubtotal.toLocaleString()} DZD
              </span>
            </div>
          </div>
        {/if}

        <div class="flex items-center justify-between p-2.5 bg-sky-50/50 dark:bg-sky-950/30 rounded-xl border border-sky-200/60 dark:border-sky-800/60">
          <div>
            <span class="text-[10px] font-black text-pos-muted uppercase tracking-wider block">TOTAL PAYABLE</span>
            <span class="text-[11px] font-bold text-sky-600">{$cartItems.length} lines</span>
          </div>
          <span class="text-3xl lg:text-4xl font-black font-mono tracking-tight transition-all duration-200 hover:scale-105 {$cartGrandTotal < 0 ? 'text-amber-600' : 'text-sky-600 dark:text-sky-400'}">
            {$cartGrandTotal.toLocaleString()} <span class="text-sm font-bold">DZD</span>
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
    initialBarcode={initialBarcodeForNewProduct}
    extraBarcode={editingProductWithExtraBarcode}
    onClose={() => (isProductEditOpen = false)}
    onSaved={handleProductSaved}
  />

  <UnknownBarcodeModal
    isOpen={isUnknownBarcodeModalOpen}
    barcode={unknownScannedBarcode}
    onClose={() => (isUnknownBarcodeModalOpen = false)}
    onAddNewWithBarcode={(bc) => {
      editingProduct = null;
      initialBarcodeForNewProduct = bc;
      editingProductWithExtraBarcode = '';
      isProductEditOpen = true;
    }}
    onEditProductWithBarcode={handleEditProductWithBarcode}
    onLinkedToProduct={(p) => {
      addToCart(p, 1, $isRefundMode);
      loadProducts();
    }}
  />

  <QuickPurchaseModal
    isOpen={isQuickPurchaseOpen}
    suppliers={$suppliers}
    onClose={() => (isQuickPurchaseOpen = false)}
    onPurchaseCompleted={loadProducts}
  />

  <ReturnDamagedModal
    isOpen={isReturnDamagedOpen}
    suppliers={$suppliers}
    onClose={() => (isReturnDamagedOpen = false)}
    onReturnCompleted={loadProducts}
  />

  <CreditCustomerModal
    isOpen={isCreditCustomerOpen}
    totalAmount={$cartGrandTotal}
    onClose={() => (isCreditCustomerOpen = false)}
    onConfirmCredit={(cId, cName, paid, remaining) => {
      $selectedCustomerId = cId;
      executeCheckout(cId, cName, { mode: 'credit', paidAmount: paid, changeAmount: 0, });
    }}
  />

  <VersementCustomerModal
    isOpen={isVersementOpen}
    totalAmount={$cartGrandTotal}
    onClose={() => (isVersementOpen = false)}
    onConfirmVersement={(cId, cName, paid, remaining) => {
      // Versement reserves the cart for the customer: their selection follows
      // the sale so the remainder is tracked on their account.
      $selectedCustomerId = cId;
      executeCheckout(cId, cName, { mode: 'versement', paidAmount: paid, changeAmount: 0 });
    }}
  />

  <CheckoutModal
    isOpen={isCheckoutOpen}
    totalAmount={$cartGrandTotal}
    onClose={() => (isCheckoutOpen = false)}
    onConfirmCheckout={handleCheckoutConfirm}
  />

  <OtherArticleModal
    isOpen={isOtherArticleOpen}
    onClose={() => (isOtherArticleOpen = false)}
  />
</div>