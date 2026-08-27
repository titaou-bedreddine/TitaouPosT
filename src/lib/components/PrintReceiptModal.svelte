<script lang="ts">
  import { cartItems, cartGrandTotal } from '../stores/cart';
  import { currentUser } from '../stores/auth';
  import { Printer, X } from 'lucide-svelte';

  export let isOpen = false;
  export let onClose: () => void;

  function triggerPrint() {
    window.print();
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-xl shadow-2xl w-full max-w-sm overflow-hidden animate-in fade-in duration-150">
      <div class="flex items-center justify-between px-5 py-3 border-b border-pos-border bg-slate-50 dark:bg-slate-800/50">
        <h3 class="font-extrabold text-sm text-pos-text flex items-center gap-2">
          <Printer class="w-4 h-4 text-sky-500" />
          <span>Receipt Print Preview</span>
        </h3>
        <button on:click={onClose} class="text-pos-muted hover:text-pos-text p-1 rounded cursor-pointer">
          <X class="w-4 h-4" />
        </button>
      </div>

      <!-- Thermal Printable Receipt Area -->
      <div class="p-6 bg-white text-slate-900 font-mono text-xs space-y-3 max-h-[70vh] overflow-y-auto">
        <div class="text-center space-y-0.5 border-b border-dashed border-slate-300 pb-2">
          <h2 class="font-black text-sm">لومينا ديجيتال سيرفيس</h2>
          <p class="text-[10px] text-slate-600">Didouche Mourad St., Algiers</p>
          <p class="text-[10px] text-slate-600">Phone: 0555 00 11 22</p>
          <p class="text-[10px] text-slate-500">{new Date().toLocaleString()}</p>
        </div>

        <div class="space-y-1.5 py-1 border-b border-dashed border-slate-300">
          {#each $cartItems as item}
            <div class="flex justify-between items-start">
              <div>
                <p class="font-bold">{item.name_ar || item.name_fr}</p>
                <p class="text-[10px] text-slate-500">{item.quantity} x {item.unit_price} DZD</p>
              </div>
              <span class="font-bold">{item.total_price} DZD</span>
            </div>
          {/each}
        </div>

        <div class="space-y-1 pt-1 font-bold">
          <div class="flex justify-between text-base font-black">
            <span>TOTAL:</span>
            <span>{$cartGrandTotal.toLocaleString()} DZD</span>
          </div>
          <div class="text-[10px] text-slate-500 flex justify-between pt-1">
            <span>Cashier: {$currentUser?.display_name || 'Admin'}</span>
            <span>RC: 16/00-1234567B22</span>
          </div>
        </div>

        <div class="text-center pt-2 text-[10px] text-slate-500 border-t border-dashed border-slate-300">
          *** شكراً لزيارتكم - Merci de votre visite ***
        </div>
      </div>

      <div class="px-5 py-3 border-t border-pos-border bg-slate-50 dark:bg-slate-800/50 flex justify-end gap-2">
        <button on:click={onClose} class="px-3 py-1.5 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded">
          Close
        </button>
        <button on:click={triggerPrint} class="px-4 py-1.5 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded flex items-center gap-1 cursor-pointer">
          <Printer class="w-3.5 h-3.5" />
          <span>Print Receipt</span>
        </button>
      </div>
    </div>
  </div>
{/if}