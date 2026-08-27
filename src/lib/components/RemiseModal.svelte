<script lang="ts">
  import { globalDiscountMode, globalDiscountValue, cartSubtotal } from '../stores/cart';
  import { Percent, DollarSign, X, Check } from 'lucide-svelte';

  export let isOpen = false;
  export let onClose: () => void;

  let mode: 'percent' | 'amount' = $globalDiscountMode === 'amount' ? 'amount' : 'percent';
  let value: number = $globalDiscountValue;

  function apply() {
    $globalDiscountMode = mode;
    $globalDiscountValue = value;
    onClose();
  }

  function removeDiscount() {
    $globalDiscountMode = 'none';
    $globalDiscountValue = 0;
    onClose();
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-xl shadow-2xl w-full max-w-sm overflow-hidden animate-in fade-in duration-150">
      <div class="flex items-center justify-between px-5 py-3.5 border-b border-pos-border bg-slate-50 dark:bg-slate-800/50">
        <h3 class="font-extrabold text-base text-pos-text flex items-center gap-2">
          <Percent class="w-5 h-5 text-indigo-500" />
          <span>Apply Whole-Cart Remise / الخصم</span>
        </h3>
        <button on:click={onClose} class="text-pos-muted hover:text-pos-text p-1 rounded cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="p-5 space-y-4">
        <!-- Mode Tabs -->
        <div class="grid grid-cols-2 gap-2">
          <button
            type="button"
            on:click={() => mode = 'percent'}
            class="p-2.5 rounded-lg border font-bold text-xs flex items-center justify-center gap-1.5 transition cursor-pointer {mode === 'percent' ? 'border-indigo-500 bg-indigo-50 dark:bg-indigo-950 text-indigo-600' : 'border-pos-border text-pos-muted'}"
          >
            <Percent class="w-4 h-4" />
            <span>Percentage (%)</span>
          </button>
          <button
            type="button"
            on:click={() => mode = 'amount'}
            class="p-2.5 rounded-lg border font-bold text-xs flex items-center justify-center gap-1.5 transition cursor-pointer {mode === 'amount' ? 'border-indigo-500 bg-indigo-50 dark:bg-indigo-950 text-indigo-600' : 'border-pos-border text-pos-muted'}"
          >
            <DollarSign class="w-4 h-4" />
            <span>Fixed Amount (DZD)</span>
          </button>
        </div>

        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">
            {mode === 'percent' ? 'Discount Percentage (%)' : 'Discount Amount (DZD)'}
          </label>
          <input
            type="number"
            bind:value={value}
            on:focus={(e) => (e.target as HTMLInputElement).select()}
            min="0"
            max={mode === 'percent' ? 100 : $cartSubtotal}
            class="w-full px-3 py-2 bg-pos-card border border-pos-border rounded-lg text-xl font-bold font-mono text-pos-text outline-none focus:ring-2 focus:ring-indigo-500"
          />
        </div>
      </div>

      <div class="px-5 py-3.5 border-t border-pos-border bg-slate-50 dark:bg-slate-800/50 flex justify-between">
        <button
          type="button"
          on:click={removeDiscount}
          class="px-3 py-1.5 text-rose-500 hover:text-rose-700 text-xs font-bold"
        >
          Reset / Clear
        </button>
        <div class="flex gap-2">
          <button on:click={onClose} class="px-3 py-1.5 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded">
            Cancel
          </button>
          <button on:click={apply} class="px-4 py-1.5 bg-indigo-600 hover:bg-indigo-700 text-white text-xs font-bold rounded flex items-center gap-1 cursor-pointer">
            <Check class="w-3.5 h-3.5" />
            <span>Apply Remise</span>
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}