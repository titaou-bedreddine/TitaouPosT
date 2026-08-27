<script lang="ts">
  import { t } from '../i18n';
  import { Search } from 'lucide-svelte';

  export let query = '';
  export let searchType: 'all' | 'name' | 'barcode' | 'price' = 'all';
  export let onSearch: () => void;

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      onSearch();
    }
  }
</script>

<div class="flex items-center gap-2 bg-pos-card border border-pos-border rounded-lg p-1.5 shadow-xs">
  <select
    bind:value={searchType}
    on:change={onSearch}
    class="bg-slate-100 dark:bg-slate-800 text-pos-text text-sm font-semibold rounded px-2.5 py-2 border-0 outline-none cursor-pointer focus:ring-1 focus:ring-sky-500"
  >
    <option value="all">{t('filter_all')}</option>
    <option value="name">{t('filter_name')}</option>
    <option value="barcode">{t('filter_barcode')}</option>
    <option value="price">{t('filter_price')}</option>
  </select>

  <div class="relative flex-1">
    <Search class="w-4 h-4 text-pos-muted absolute start-3 top-1/2 -translate-y-1/2" />
    <input
      type="text"
      bind:value={query}
      on:input={onSearch}
      on:keydown={handleKeyDown}
      placeholder={t('search_placeholder')}
      class="w-full ps-9 pe-3 py-2 bg-transparent text-pos-text placeholder-pos-muted text-sm outline-none font-medium"
      autofocus
    />
  </div>
</div>