<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { t } from '../../lib/i18n';
  import type { Category, Product } from '../../lib/types';
  import { cartItems, cartGrandTotal, isRefundMode, addToCart, clearCart } from '../../lib/stores/cart';
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
  import { ShoppingBag, ArrowRight, CheckCircle2, Settings2, Plus } from 'lucide-svelte';

  let products: Product[] = [];
  let categories: Category[] = [];
  let selectedCategory: number | null = null;
  let searchQuery = '';
  let searchType: 'all' | 'name' | 'barcode' | 'price' | 'qr' = 'all';

  let isPaymentOpen = false;
  let isCashDrawerOpen = false;
  let isCategoryManagerOpen = false;
  let isHeldSalesOpen = false;
  let isRemiseOpen = false;
  let isPrintReceiptOpen = false;
  let lastSaleSuccessNumber = '';

  onMount(async () => {
    await loadCategories();
    await loadProducts();
  });

  async function loadCategories() {
    try {
      categories = await invoke<Category[]>('get_categories');
    } catch (e) {
      console.error(e);
    }
  }

  async function loadProducts() {
    try {
      products = await invoke<Product[]>('search_products', {
        query: searchQuery,
        categoryId: selectedCategory,
        searchType: searchType === 'qr' ? 'barcode' : searchType,
      });

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

  function handleSaleSuccess(saleNum: string) {
    lastSaleSuccessNumber = saleNum;
    loadProducts();
    setTimeout(() => {
      lastSaleSuccessNumber = '';
    }, 4000);
  }
</script>

<div class="flex flex-col h-full bg-pos-bg select-none">
  <!-- Top Persistent Single-Row Action Bar -->
  <TopActionBar
    onOpenPayment={() => isPaymentOpen = true}
    onOpenCashDrawer={() => isCashDrawerOpen = true}
    onOpenRemise={() => isRemiseOpen = true}
    onOpenHeldSales={() => isHeldSalesOpen = true}
    onPrintReceipt={() => isPrintReceiptOpen = true}
  />

  {#if lastSaleSuccessNumber}
    <div class="bg-emerald-600 text-white px-4 py-2 text-xs font-bold flex items-center justify-between shadow-md">
      <div class="flex items-center gap-2">
        <CheckCircle2 class="w-4 h-4" />
        <span>Sale Completed Successfully: #{lastSaleSuccessNumber}</span>
      </div>
      <button on:click={() => lastSaleSuccessNumber = ''} class="underline cursor-pointer">Dismiss</button>
    </div>
  {/if}

  <!-- Main POS Layout -->
  <div class="flex-1 flex overflow-hidden p-2.5 gap-2.5">
    <!-- LEFT PANEL: Omni-Search, Categories & Catalog -->
    <div class="flex-1 flex flex-col min-w-0 bg-pos-card border border-pos-border rounded-xl p-3 shadow-xs">
      <!-- Universal Omni-Search -->
      <div class="mb-2">
        <UniversalSearchBar
          bind:query={searchQuery}
          bind:searchType={searchType}
          onSearch={loadProducts}
        />
      </div>

      <!-- Category Filter Chips with Manage Button -->
      <div class="flex items-center gap-1.5 overflow-x-auto pb-1.5 mb-1.5 shrink-0">
        <button
          type="button"
          on:click={() => handleCategoryClick(null)}
          class="px-3 py-1.5 rounded-lg text-xs font-bold shrink-0 transition cursor-pointer {selectedCategory === null ? 'bg-sky-600 text-white shadow-xs' : 'bg-slate-100 dark:bg-slate-800 text-pos-muted hover:bg-slate-200'}"
        >
          {t('filter_all')}
        </button>
        {#each categories as cat}
          <button
            type="button"
            on:click={() => handleCategoryClick(cat.id)}
            class="px-3 py-1.5 rounded-lg text-xs font-bold shrink-0 transition flex items-center gap-1.5 cursor-pointer {selectedCategory === cat.id ? 'bg-sky-600 text-white shadow-xs' : 'bg-slate-100 dark:bg-slate-800 text-pos-muted hover:bg-slate-200'}"
          >
            <span class="w-2 h-2 rounded-full" style="background-color: {cat.color || '#0284c7'}"></span>
            <span>{cat.name_ar || cat.name_fr}</span>
          </button>
        {/each}

        <!-- Add / Manage Categories -->
        <button
          type="button"
          on:click={() => isCategoryManagerOpen = true}
          class="p-1.5 rounded-lg text-xs font-bold bg-slate-100 dark:bg-slate-800 text-pos-muted hover:text-sky-600 hover:bg-slate-200 shrink-0 transition cursor-pointer"
          title="Add / Edit Product Groups"
        >
          <Settings2 class="w-4 h-4" />
        </button>
      </div>

      <!-- Products Grid -->
      <div class="flex-1 overflow-y-auto pr-1">
        {#if products.length === 0}
          <div class="h-full flex flex-col items-center justify-center text-pos-muted gap-2">
            <ShoppingBag class="w-10 h-10 stroke-1 opacity-40" />
            <p class="text-xs font-semibold">No products found matching query.</p>
          </div>
        {:else}
          <div class="grid grid-cols-2 md:grid-cols-3 xl:grid-cols-4 gap-2.5">
            {#each products as product (product.id)}
              <ProductCard {product} />
            {/each}
          </div>
        {/if}
      </div>
    </div>

    <!-- RIGHT PANEL: Visual Card-Based Item Cart -->
    <div class="w-[410px] flex flex-col shrink-0 bg-pos-card border border-pos-border rounded-xl shadow-xs overflow-hidden">
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

      <!-- Totals & Checkout Footer (TOTAL in Big Bold Typography) -->
      <div class="p-3.5 border-t border-pos-border bg-slate-50 dark:bg-slate-800/40 space-y-3">
        <div class="flex items-center justify-between">
          <span class="text-xs font-black text-pos-text uppercase tracking-wider">TOTAL</span>
          <span class="text-3xl font-black font-mono {$cartGrandTotal < 0 ? 'text-amber-600' : 'text-sky-600 dark:text-sky-400'}">
            {$cartGrandTotal.toLocaleString()} DZD
          </span>
        </div>

        <button
          type="button"
          on:click={() => isPaymentOpen = true}
          disabled={$cartItems.length === 0}
          class="w-full py-3 px-4 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-40 text-white font-black text-sm rounded-xl transition shadow-md flex items-center justify-center gap-2 cursor-pointer"
        >
          <span>{t('complete_sale')}</span>
          <ArrowRight class="w-4 h-4" />
        </button>
      </div>
    </div>
  </div>

  <!-- Modals -->
  <PaymentModal
    isOpen={isPaymentOpen}
    onClose={() => isPaymentOpen = false}
    onSaleSuccess={handleSaleSuccess}
  />

  <CashDrawerModal
    isOpen={isCashDrawerOpen}
    onClose={() => isCashDrawerOpen = false}
  />

  <CategoryManagerModal
    isOpen={isCategoryManagerOpen}
    onClose={() => isCategoryManagerOpen = false}
    onCategoryChanged={() => { loadCategories(); loadProducts(); }}
  />

  <HeldSalesModal
    isOpen={isHeldSalesOpen}
    onClose={() => isHeldSalesOpen = false}
  />

  <RemiseModal
    isOpen={isRemiseOpen}
    onClose={() => isRemiseOpen = false}
  />

  <PrintReceiptModal
    isOpen={isPrintReceiptOpen}
    onClose={() => isPrintReceiptOpen = false}
  />
</div>