<script lang="ts">
  import { cartItems, cartGrandTotal, globalDiscountMode, globalDiscountValue } from '../stores/cart';
  import { currentUser } from '../stores/auth';
  import { printHtmlDirectly } from '../utils/printer';
  import { Printer, X, Check } from 'lucide-svelte';

  export let isOpen = false;
  export let onClose: () => void;

  let receiptContainer: HTMLDivElement;

  function triggerPrint() {
    if (!receiptContainer) return;
    printHtmlDirectly(receiptContainer.innerHTML, 'Ticket de Caisse - TitaouPosT');
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-2xl shadow-2xl w-full max-w-md overflow-hidden animate-in fade-in duration-150">
      <!-- Modal Header -->
      <div class="flex items-center justify-between px-5 py-3.5 border-b border-pos-border bg-slate-50 dark:bg-slate-800/50">
        <h3 class="font-black text-sm text-pos-text flex items-center gap-2">
          <Printer class="w-4 h-4 text-sky-500" />
          <span>Thermal Receipt Preview (معاينة الوصل)</span>
        </h3>
        <button on:click={onClose} class="text-pos-muted hover:text-pos-text p-1 rounded-lg cursor-pointer">
          <X class="w-4 h-4" />
        </button>
      </div>

      <!-- Printable Thermal Receipt Area -->
      <div class="p-6 bg-slate-100 dark:bg-slate-900/60 max-h-[65vh] overflow-y-auto flex justify-center">
        <div
          bind:this={receiptContainer}
          class="w-[320px] bg-white text-black p-5 shadow-md font-mono text-[11px] leading-relaxed select-text border border-slate-200"
        >
          <!-- Store Header -->
          <div class="text-center pb-2 border-b-dashed">
            <h2 class="font-black text-base tracking-tight">TitaouPosT Supermarché</h2>
            <p class="text-xxs text-gray-700">Lumina Retail POS System</p>
            <p class="text-xxs text-gray-700">Didouche Mourad, Alger Centre</p>
            <p class="text-xxs text-gray-700">Tél: 0550 12 34 56 / 021 65 43 21</p>
            <p class="text-xxs text-gray-700">RC: 16/00-0123456B22 | NIF: 001616012345678</p>
            <p class="text-xxs text-gray-700 mt-1">{new Date().toLocaleString()}</p>
          </div>

          <!-- Invoice / Cashier Info -->
          <div class="py-1 border-b-dashed text-xxs flex justify-between">
            <span>Ticket #: {Math.floor(Date.now() / 1000).toString().slice(-6)}</span>
            <span>Caisse: {$currentUser?.display_name || 'Admin'}</span>
          </div>

          <!-- Items Table -->
          <div class="py-2 border-b-dashed">
            <table class="w-full">
              <thead>
                <tr>
                  <th class="text-start">Article</th>
                  <th class="text-center">Qté</th>
                  <th class="text-end">P.U</th>
                  <th class="text-end">Total</th>
                </tr>
              </thead>
              <tbody>
                {#each $cartItems as item}
                  <tr>
                    <td class="text-start font-bold">
                      {item.name_ar || item.name_fr}
                      {#if item.is_refund}
                        <span class="text-xxs font-black text-red-600">[RETOUR]</span>
                      {/if}
                    </td>
                    <td class="text-center">{item.quantity}</td>
                    <td class="text-end">{item.unit_price}</td>
                    <td class="text-end font-bold">
                      {item.is_refund ? '-' : ''}{item.total_price}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>

          <!-- Totals -->
          <div class="py-2 space-y-1 text-xs">
            {#if $globalDiscountMode !== 'none' && $globalDiscountValue > 0}
              <div class="flex justify-between text-xxs text-gray-700">
                <span>Remise Globale:</span>
                <span>-{$globalDiscountMode === 'percent' ? `${$globalDiscountValue}%` : `${$globalDiscountValue} DZD`}</span>
              </div>
            {/if}

            <div class="flex justify-between font-black text-sm pt-1 border-t-dashed">
              <span>NET À PAYER (TOTAL):</span>
              <span>{$cartGrandTotal.toLocaleString()} DZD</span>
            </div>

            <div class="flex justify-between text-xxs text-gray-600 pt-0.5">
              <span>Dont TVA (19%):</span>
              <span>{Math.round(($cartGrandTotal * 19) / 119).toLocaleString()} DZD</span>
            </div>
          </div>

          <!-- Footer Greetings & QR Code -->
          <div class="text-center pt-3 border-t-dashed text-xxs text-gray-700 space-y-1">
            <p class="font-bold">*** شكراً لزيارتكم - Merci de votre visite ***</p>
            <p>Les articles retournés doivent être présentés sous 48h</p>
            <div class="font-mono text-[9px] text-gray-500 pt-1">
              * TitaouPosT Offline Desktop POS *
            </div>
          </div>
        </div>
      </div>

      <!-- Action Footer -->
      <div class="px-5 py-3.5 border-t border-pos-border bg-slate-50 dark:bg-slate-800/50 flex justify-end gap-2">
        <button on:click={onClose} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-xl cursor-pointer">
          Close
        </button>
        <button
          type="button"
          on:click={triggerPrint}
          class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-md transition active:scale-95"
        >
          <Printer class="w-4 h-4" />
          <span>Print Receipt (طباعة الوصل)</span>
        </button>
      </div>
    </div>
  </div>
{/if}