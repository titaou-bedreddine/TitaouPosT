<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { Product } from '../types';
  import { AlertTriangle, Plus, Link2, Search, X, Check, Package, QrCode, ArrowLeft, Tag } from 'lucide-svelte';

  export let isOpen = false;
  export let barcode = '';
  export let onClose: () => void;
  export let onAddNewWithBarcode: (barcode: string) => void;
  export let onEditProductWithBarcode: (product: Product, newBarcode: string) => void;
  export let onLinkedToProduct: (product: Product) => void;

  let mode: 'choose' | 'link' = 'choose';
  let searchQuery = '';
  let searchResults: Product[] = [];
  let isSearching = false;
  let searchTimeout: any = null;

  $: if (isOpen) {
    mode = 'choose';
    searchQuery = '';
    searchResults = [];
  }

  function handleSearchInput() {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(() => {
      executeSearch();
    }, 200);
  }

  async function executeSearch() {
    if (!searchQuery.trim()) {
      searchResults = [];
      return;
    }
    try {
      isSearching = true;
      searchResults = await invoke<Product[]>('search_products', {
        query: searchQuery.trim(),
        categoryId: null,
        searchType: 'all',
      });
    } catch (e) {
      console.error('Failed to search existing products:', e);
    } finally {
      isSearching = false;
    }
  }

  function chooseExistingProduct(p: Product) {
    // Open the product editor with the scanned barcode attached so the user
    // can review price/quantity and save consciously.
    if (!barcode) return;
    onEditProductWithBarcode(p, barcode);
    onClose();
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl w-full max-w-lg overflow-hidden animate-in zoom-in-95 duration-150 relative">
      <!-- Modal Header -->
      <div class="flex items-center justify-between px-6 py-4 border-b border-pos-border bg-slate-50 dark:bg-slate-800/60">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-2xl bg-amber-500/10 text-amber-500 flex items-center justify-center font-bold shrink-0">
            <AlertTriangle class="w-5 h-5" />
          </div>
          <div>
            <h3 class="font-black text-base text-pos-text">
              {mode === 'choose' ? 'Unknown Barcode / باركود غير مسجل' : 'Link to Existing Product / ربط بمنتج موجود'}
            </h3>
            <p class="text-xs text-pos-muted font-mono">
              Scanned: <strong class="text-amber-600 dark:text-amber-400 bg-amber-50 dark:bg-amber-950/60 px-2 py-0.5 rounded-md">{barcode}</strong>
            </p>
          </div>
        </div>
        <button on:click={onClose} class="text-pos-muted hover:text-pos-text p-1.5 rounded-lg cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>

      <!-- Modal Body -->
      <div class="p-6 space-y-4">
        {#if mode === 'choose'}
          <p class="text-xs text-pos-muted font-medium">
            This barcode is not registered to any product in your database. What would you like to do?
          </p>

          <div class="grid grid-cols-1 gap-3 pt-1">
            <!-- Option 1: Add as New Product -->
            <button
              type="button"
              on:click={() => {
                onAddNewWithBarcode(barcode);
                onClose();
              }}
              class="flex items-center gap-4 p-4 rounded-2xl border-2 border-sky-200 dark:border-sky-800/60 bg-sky-50/50 dark:bg-sky-950/30 hover:border-sky-500 hover:bg-sky-100/50 transition cursor-pointer text-start group shadow-xs"
            >
              <div class="w-12 h-12 rounded-2xl bg-sky-600 text-white flex items-center justify-center shrink-0 group-hover:scale-105 transition">
                <Plus class="w-6 h-6" />
              </div>
              <div class="flex-1">
                <h4 class="text-sm font-black text-pos-text group-hover:text-sky-600 transition">
                  Create New Product / إنشاء منتج جديد
                </h4>
                <p class="text-xs text-pos-muted mt-0.5">
                  Open new product creator with barcode <strong class="font-mono text-sky-600">{barcode}</strong> pre-filled.
                </p>
              </div>
            </button>

            <!-- Option 2: Link to Existing Product -->
            <button
              type="button"
              on:click={() => {
                mode = 'link';
                executeSearch();
              }}
              class="flex items-center gap-4 p-4 rounded-2xl border-2 border-indigo-200 dark:border-indigo-800/60 bg-indigo-50/50 dark:bg-indigo-950/30 hover:border-indigo-500 hover:bg-indigo-100/50 transition cursor-pointer text-start group shadow-xs"
            >
              <div class="w-12 h-12 rounded-2xl bg-indigo-600 text-white flex items-center justify-center shrink-0 group-hover:scale-105 transition">
                <Link2 class="w-6 h-6" />
              </div>
              <div class="flex-1">
                <h4 class="text-sm font-black text-pos-text group-hover:text-indigo-600 transition">
                  Link to Existing Product / ربط بمنتج موجود
                </h4>
                <p class="text-xs text-pos-muted mt-0.5">
                  Add this secondary barcode to an existing item in your catalog.
                </p>
              </div>
            </button>
          </div>

        {:else if mode === 'link'}
          <!-- Search Box -->
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <label class="block text-xs font-bold text-pos-muted">Search Product by Name or SKU / ابحث عن المنتج</label>
              <button
                type="button"
                on:click={() => (mode = 'choose')}
                class="text-xs font-bold text-sky-600 hover:underline flex items-center gap-1 cursor-pointer"
              >
                <ArrowLeft class="w-3.5 h-3.5" />
                <span>Back / رجوع</span>
              </button>
            </div>

            <div class="relative flex items-center">
              <Search class="w-4 h-4 text-pos-muted absolute start-3 pointer-events-none" />
              <input
                type="text"
                bind:value={searchQuery}
                on:input={handleSearchInput}
                placeholder="Type product name or SKU..."
                class="w-full ps-9 pe-4 py-2.5 bg-slate-100 dark:bg-slate-800 border border-pos-border rounded-xl text-xs font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500"
                autofocus
              />
            </div>
          </div>

          <!-- Search Results List -->
          <div class="space-y-1.5 max-h-60 overflow-y-auto pr-1">
            {#if isSearching}
              <div class="p-6 text-center text-xs text-pos-muted font-bold">
                Searching catalog...
              </div>
            {:else if searchResults.length === 0}
              <div class="p-6 text-center text-xs text-pos-muted font-bold">
                {searchQuery ? 'No matching products found.' : 'Type a name to search existing products.'}
              </div>
            {:else}
              {#each searchResults as p}
                <div
                  class="flex items-center justify-between p-3 rounded-xl border border-pos-border bg-pos-card hover:bg-slate-50 dark:hover:bg-slate-800/60 transition gap-3"
                >
                  <div class="min-w-0 flex-1">
                    <div class="font-black text-xs text-pos-text truncate">{p.name_fr || p.name_ar}</div>
                    {#if p.name_ar && p.name_fr}
                      <div class="text-[10px] text-pos-muted truncate">{p.name_ar}</div>
                    {/if}
                    <div class="flex items-center gap-2 text-[10px] text-pos-muted font-mono mt-0.5">
                      <span>SKU: {p.sku || 'N/A'}</span>
                      <span>•</span>
                      <span class="text-sky-600 font-bold">{p.sale_price.toLocaleString()} DZD</span>
                      <span>•</span>
                      <span>Stock: {p.current_stock}</span>
                    </div>
                  </div>

                  <button
                    type="button"
                    on:click={() => chooseExistingProduct(p)}
                    class="px-3 py-2 bg-indigo-600 hover:bg-indigo-700 text-white text-xs font-black rounded-xl cursor-pointer shadow-xs shrink-0 flex items-center gap-1.5 transition"
                  >
                    <Link2 class="w-3.5 h-3.5" />
                    <span>Link & Edit</span>
                  </button>
                </div>
              {/each}
            {/if}
          </div>
        {/if}
      </div>

      <!-- Modal Footer -->
      <div class="flex justify-end px-6 py-3 border-t border-pos-border bg-slate-50/50 dark:bg-slate-800/30">
        <button
          type="button"
          on:click={onClose}
          class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-pos-text text-xs font-bold rounded-xl cursor-pointer"
        >
          Cancel / إغلاق
        </button>
      </div>
    </div>
  </div>
{/if}
