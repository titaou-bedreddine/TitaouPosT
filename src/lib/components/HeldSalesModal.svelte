<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { cartItems, heldSalesList, refreshHeldSales, clearCart } from '../stores/cart';
  import { currentUser } from '../stores/auth';
  import type { HeldSale } from '../types';
  import { PauseCircle, Play, Trash2, X, Plus } from 'lucide-svelte';

  export let isOpen = false;
  export let onClose: () => void;

  let holdNote = '';
  let isSaving = false;

  $: if (isOpen) {
    refreshHeldSales();
  }

  async function handleHoldCurrentCart() {
    if (!$currentUser || $cartItems.length === 0) return;
    try {
      isSaving = true;
      await invoke('hold_sale', {
        userId: $currentUser.id,
        customerId: null,
        cartJson: JSON.stringify($cartItems),
        note: holdNote || null,
      });
      clearCart();
      holdNote = '';
      await refreshHeldSales();
      onClose();
    } catch (e) {
      console.error(e);
    } finally {
      isSaving = false;
    }
  }

  function handleResume(sale: HeldSale) {
    try {
      const items = JSON.parse(sale.cart_json);
      $cartItems = items;
      handleDelete(sale.id);
      onClose();
    } catch (e) {
      console.error(e);
    }
  }

  async function handleDelete(id: number) {
    try {
      await invoke('delete_held_sale', { heldId: id });
      await refreshHeldSales();
    } catch (e) {
      console.error(e);
    }
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-xl shadow-2xl w-full max-w-lg overflow-hidden animate-in fade-in duration-150">
      <div class="flex items-center justify-between px-5 py-3.5 border-b border-pos-border bg-slate-50 dark:bg-slate-800/50">
        <h3 class="font-extrabold text-base text-pos-text flex items-center gap-2">
          <PauseCircle class="w-5 h-5 text-indigo-500" />
          <span>Held Sales / Ventes en Attente ({$heldSalesList.length})</span>
        </h3>
        <button on:click={onClose} class="text-pos-muted hover:text-pos-text p-1 rounded cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="p-5 space-y-4">
        {#if $cartItems.length > 0}
          <div class="p-3 bg-indigo-50 dark:bg-indigo-950/50 border border-indigo-200 dark:border-indigo-800 rounded-xl space-y-2">
            <span class="text-xs font-bold text-indigo-900 dark:text-indigo-200">Hold Active Cart ({$cartItems.length} items):</span>
            <div class="flex gap-2">
              <input
                type="text"
                bind:value={holdNote}
                placeholder="Optional customer name or table note..."
                class="flex-1 px-3 py-1.5 bg-pos-card border border-pos-border rounded text-xs text-pos-text outline-none"
              />
              <button
                type="button"
                on:click={handleHoldCurrentCart}
                disabled={isSaving}
                class="px-4 py-1.5 bg-indigo-600 hover:bg-indigo-700 text-white font-bold text-xs rounded transition flex items-center gap-1 cursor-pointer"
              >
                <span>Hold Now</span>
              </button>
            </div>
          </div>
        {/if}

        <!-- List of Held Sales -->
        <div class="max-h-64 overflow-y-auto space-y-2">
          {#if $heldSalesList.length === 0}
            <div class="p-8 text-center text-pos-muted text-xs">
              No held sales currently in queue.
            </div>
          {:else}
            {#each $heldSalesList as sale}
              <div class="flex items-center justify-between p-3 bg-pos-card border border-pos-border rounded-lg hover:border-indigo-400 transition">
                <div>
                  <div class="flex items-center gap-2">
                    <span class="font-bold text-xs text-indigo-600 font-mono">{sale.sale_reference}</span>
                    <span class="text-[11px] text-pos-muted">{sale.created_at}</span>
                  </div>
                  {#if sale.note}
                    <p class="text-xs font-semibold text-pos-text mt-0.5">{sale.note}</p>
                  {/if}
                </div>
                <div class="flex items-center gap-1.5">
                  <button
                    type="button"
                    on:click={() => handleResume(sale)}
                    class="px-3 py-1.5 bg-emerald-600 hover:bg-emerald-700 text-white text-xs font-bold rounded flex items-center gap-1 cursor-pointer"
                  >
                    <Play class="w-3 h-3" />
                    <span>Resume</span>
                  </button>
                  <button
                    type="button"
                    on:click={() => handleDelete(sale.id)}
                    class="p-1.5 text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 rounded cursor-pointer"
                  >
                    <Trash2 class="w-3.5 h-3.5" />
                  </button>
                </div>
              </div>
            {/each}
          {/if}
        </div>
      </div>

      <div class="px-5 py-3 border-t border-pos-border bg-slate-50 dark:bg-slate-800/50 flex justify-end">
        <button on:click={onClose} class="px-4 py-1.5 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded">
          Close
        </button>
      </div>
    </div>
  </div>
{/if}