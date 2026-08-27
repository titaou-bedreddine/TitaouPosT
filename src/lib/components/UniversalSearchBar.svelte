<script lang="ts">
  import { t } from '../i18n';
  import { Search, X, QrCode } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { currentUser } from '../stores/auth';
  import type { User } from '../types';

  export let query = '';
  export let searchType: 'all' | 'name' | 'barcode' | 'price' | 'qr' = 'all';
  export let onSearch: () => void;

  const modes: Array<'all' | 'name' | 'barcode' | 'price' | 'qr'> = ['all', 'name', 'barcode', 'price', 'qr'];
  const labels: Record<string, string> = {
    all: 'الكل (All)',
    name: 'الاسم (Name)',
    barcode: 'الباركود (Barcode)',
    price: 'السعر (Price)',
    qr: 'QR Code',
  };

  function cycleSearchType() {
    const currentIndex = modes.indexOf(searchType);
    const nextIndex = (currentIndex + 1) % modes.length;
    searchType = modes[nextIndex];
    onSearch();
  }

  function clearQuery() {
    query = '';
    onSearch();
  }

  async function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      // Check if scanned an employee QR code (e.g. EMP-QR-01)
      if (query.trim().startsWith('EMP-QR-') || query.trim().startsWith('EMP_QR_')) {
        try {
          const user = await invoke<User | null>('get_user_by_qr', { qrCode: query.trim() });
          if (user) {
            $currentUser = user;
            query = '';
            return;
          }
        } catch (err) {
          console.error(err);
        }
      }
      onSearch();
    }
  }
</script>

<div class="flex items-center gap-2 bg-pos-card border border-pos-border rounded-xl p-1.5 shadow-xs">
  <!-- Sleek Toggle Pill Button instead of select dropdown -->
  <button
    type="button"
    on:click={cycleSearchType}
    class="px-3 py-1.5 bg-sky-50 dark:bg-sky-950/60 border border-sky-200 dark:border-sky-800 text-sky-700 dark:text-sky-300 text-xs font-extrabold rounded-lg shrink-0 transition hover:bg-sky-100 flex items-center gap-1 cursor-pointer"
    title="Click to toggle search mode"
  >
    {#if searchType === 'qr'}
      <QrCode class="w-3.5 h-3.5 text-sky-600" />
    {/if}
    <span>{labels[searchType]}</span>
  </button>

  <div class="relative flex-1 flex items-center">
    <Search class="w-4 h-4 text-pos-muted absolute start-2.5 pointer-events-none" />
    <input
      type="text"
      bind:value={query}
      on:input={onSearch}
      on:keydown={handleKeyDown}
      on:focus={(e) => (e.target as HTMLInputElement).select()}
      placeholder={t('search_placeholder')}
      class="w-full ps-8 pe-8 py-1.5 bg-transparent text-pos-text placeholder-pos-muted text-xs font-semibold outline-none"
    />
    {#if query}
      <button
        type="button"
        on:click={clearQuery}
        class="p-1 text-pos-muted hover:text-rose-500 rounded-full transition absolute end-2 cursor-pointer"
        title="Clear Search"
      >
        <X class="w-3.5 h-3.5" />
      </button>
    {/if}
  </div>
</div>