<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { DashboardStats } from '../../lib/types';
  import { TrendingUp, ShoppingBag, AlertTriangle, ArrowDownRight, DollarSign, Wallet, Trophy, RefreshCw, Layers } from 'lucide-svelte';
  import DateQuickFilters from '../../lib/components/DateQuickFilters.svelte';

  let stats: DashboardStats | null = null;
  let fromDate = new Date().toISOString().split('T')[0];
  let toDate = new Date().toISOString().split('T')[0];
  let selectedTab: 'financial' | 'debts' | 'inventory' | 'expenses' | 'caisse' = 'financial';

  onMount(async () => {
    await loadStats();
  });

  async function loadStats() {
    try {
      stats = await invoke<DashboardStats>('get_dashboard_stats', {
        startDate: fromDate,
        endDate: toDate,
      });
    } catch (e) {
      console.error(e);
    }
  }
</script>

<div class="p-6 space-y-6 overflow-y-auto h-full select-none">
  <!-- Header with Date Filters matching screenshot -->
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-black text-pos-text">Statistics & Business Analytics (الإحصائيات)</h1>
      <p class="text-xs text-pos-muted mt-1">Real-time overview of revenue, profits, margins, and inventory performance</p>
    </div>

    <div class="flex items-center gap-2 bg-pos-card border border-pos-border p-1.5 rounded-xl shadow-xs">
      <input type="date" bind:value={fromDate} on:change={loadStats} class="px-2.5 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text" />
      <span class="text-xs text-pos-muted">to</span>
      <input type="date" bind:value={toDate} on:change={loadStats} class="px-2.5 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text" />
      <button on:click={loadStats} class="p-1.5 bg-sky-600 text-white rounded-lg hover:bg-sky-700 cursor-pointer">
        <RefreshCw class="w-4 h-4" />
      </button>
    </div>
  </div>

  <!-- Quick Date Presets -->
  <DateQuickFilters bind:startDate={fromDate} bind:endDate={toDate} onChange={loadStats} />

  {#if stats}
    <!-- Top 6 Metric Cards matching photo_2026-08-27_18-52-00.jpg -->
    <div class="grid grid-cols-2 md:grid-cols-3 xl:grid-cols-6 gap-3">
      <div class="bg-pos-card border border-pos-border rounded-xl p-3.5 shadow-xs">
        <span class="text-[11px] font-bold text-pos-muted">Revenue (المداخيل)</span>
        <div class="text-lg font-black font-mono text-sky-600 mt-1">{stats.today_sales.toLocaleString()} DZD</div>
      </div>

      <div class="bg-pos-card border border-pos-border rounded-xl p-3.5 shadow-xs">
        <span class="text-[11px] font-bold text-pos-muted">Returns (المرتجعات)</span>
        <div class="text-lg font-black font-mono text-amber-600 mt-1">{stats.returns_amount.toLocaleString()} DZD</div>
      </div>

      <div class="bg-pos-card border border-pos-border rounded-xl p-3.5 shadow-xs">
        <span class="text-[11px] font-bold text-pos-muted">Net Revenue (الصافي)</span>
        <div class="text-lg font-black font-mono text-pos-text mt-1">{stats.net_revenue.toLocaleString()} DZD</div>
      </div>

      <div class="bg-pos-card border border-pos-border rounded-xl p-3.5 shadow-xs">
        <span class="text-[11px] font-bold text-pos-muted">Cost of Goods Sold</span>
        <div class="text-lg font-black font-mono text-slate-500 mt-1">{stats.cost_of_goods.toLocaleString()} DZD</div>
      </div>

      <div class="bg-pos-card border border-pos-border rounded-xl p-3.5 shadow-xs">
        <span class="text-[11px] font-bold text-pos-muted">Gross Profit (الهامش)</span>
        <div class="text-lg font-black font-mono text-emerald-600 mt-1">{stats.gross_profit.toLocaleString()} DZD</div>
      </div>

      <div class="bg-pos-card border border-pos-border rounded-xl p-3.5 shadow-xs">
        <span class="text-[11px] font-bold text-pos-muted">Average Basket (السلة)</span>
        <div class="text-lg font-black font-mono text-indigo-600 mt-1">{stats.average_basket.toLocaleString()} DZD</div>
      </div>
    </div>

    <!-- Category Tabs matching screenshot -->
    <div class="flex items-center gap-2 border-b border-pos-border pb-2 overflow-x-auto">
      <button
        on:click={() => selectedTab = 'financial'}
        class="px-4 py-2 rounded-xl text-xs font-bold transition cursor-pointer {selectedTab === 'financial' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted'}"
      >
        Financial Summary
      </button>
      <button
        on:click={() => selectedTab = 'debts'}
        class="px-4 py-2 rounded-xl text-xs font-bold transition cursor-pointer {selectedTab === 'debts' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted'}"
      >
        Debts & Balances
      </button>
      <button
        on:click={() => selectedTab = 'inventory'}
        class="px-4 py-2 rounded-xl text-xs font-bold transition cursor-pointer {selectedTab === 'inventory' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted'}"
      >
        Inventory Analytics
      </button>
      <button
        on:click={() => selectedTab = 'expenses'}
        class="px-4 py-2 rounded-xl text-xs font-bold transition cursor-pointer {selectedTab === 'expenses' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted'}"
      >
        Expenses
      </button>
      <button
        on:click={() => selectedTab = 'caisse'}
        class="px-4 py-2 rounded-xl text-xs font-bold transition cursor-pointer {selectedTab === 'caisse' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted'}"
      >
        Caisse Transactions
      </button>
    </div>

    <!-- Main Analytics Content matching photo_2026-08-27_18-52-04.jpg -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Left Column: Net Profit & Cash Flow -->
      <div class="space-y-4">
        <h3 class="font-extrabold text-sm text-pos-text">Net Profit & Cash Flow</h3>

        <div class="bg-pos-card border-2 border-sky-500/40 rounded-2xl p-5 shadow-xs space-y-2">
          <span class="text-xs font-extrabold text-sky-600 uppercase tracking-wider">Final Net Profit</span>
          <div class="text-3xl font-black font-mono text-sky-600">{stats.today_profit.toLocaleString()} DZD</div>
          <span class="inline-block text-[11px] font-bold text-sky-700 bg-sky-50 dark:bg-sky-950 px-2 py-0.5 rounded">
            Profit Margin: {stats.today_sales > 0 ? Math.round((stats.today_profit / stats.today_sales) * 100) : 0}%
          </span>
        </div>

        <div class="bg-emerald-50 dark:bg-emerald-950/40 border border-emerald-300 dark:border-emerald-800 rounded-2xl p-4 flex items-center justify-between">
          <div>
            <span class="text-[11px] font-black text-emerald-800 uppercase">Cash In (المدخول)</span>
            <div class="text-xl font-black font-mono text-emerald-700 mt-1">{stats.today_sales.toLocaleString()} DZD</div>
          </div>
          <div class="w-9 h-9 rounded-full bg-emerald-500 text-white flex items-center justify-center">
            <TrendingUp class="w-4 h-4" />
          </div>
        </div>

        <div class="bg-rose-50 dark:bg-rose-950/40 border border-rose-300 dark:border-rose-800 rounded-2xl p-4 flex items-center justify-between">
          <div>
            <span class="text-[11px] font-black text-rose-800 uppercase">Cash Out (المصروفات)</span>
            <div class="text-xl font-black font-mono text-rose-700 mt-1">{stats.today_expenses.toLocaleString()} DZD</div>
          </div>
          <div class="w-9 h-9 rounded-full bg-rose-500 text-white flex items-center justify-center">
            <ArrowDownRight class="w-4 h-4" />
          </div>
        </div>
      </div>

      <!-- Right Column: Top Profitable Products matching screenshot -->
      <div class="lg:col-span-2 bg-pos-card border border-pos-border rounded-2xl shadow-xs overflow-hidden flex flex-col">
        <div class="p-4 border-b border-pos-border flex items-center justify-between bg-slate-50 dark:bg-slate-800/40">
          <h3 class="font-extrabold text-xs text-pos-text flex items-center gap-2">
            <Trophy class="w-4 h-4 text-amber-500" />
            <span>Top Profitable Products (المنتجات الأكثر ربحية)</span>
          </h3>
          <span class="text-[11px] font-bold text-amber-600 bg-amber-50 dark:bg-amber-950 px-2 py-0.5 rounded">
            Top 15 Products
          </span>
        </div>

        <table class="w-full text-start text-xs border-collapse">
          <thead>
            <tr class="border-b border-pos-border text-pos-muted font-bold">
              <th class="p-3 text-start">Product</th>
              <th class="p-3 text-start">Category</th>
              <th class="p-3 text-center">Sold Qty</th>
              <th class="p-3 text-end">Revenue</th>
              <th class="p-3 text-end">Cost</th>
              <th class="p-3 text-end">Profit</th>
            </tr>
          </thead>
          <tbody>
            {#if stats.top_products.length === 0}
              <tr>
                <td colspan="6" class="p-12 text-center text-pos-muted font-semibold">
                  No sales data available for the specified period.
                </td>
              </tr>
            {:else}
              {#each stats.top_products as tp}
                <tr class="border-b border-pos-border/60 hover:bg-slate-50 dark:hover:bg-slate-800/40">
                  <td class="p-3 font-bold text-pos-text">{tp.product_name}</td>
                  <td class="p-3 text-pos-muted">{tp.category_name}</td>
                  <td class="p-3 text-center font-mono font-bold">{tp.sold_qty}</td>
                  <td class="p-3 text-end font-mono font-bold text-sky-600">{tp.revenue.toLocaleString()} DZD</td>
                  <td class="p-3 text-end font-mono text-pos-muted">{tp.cost.toLocaleString()} DZD</td>
                  <td class="p-3 text-end font-mono font-black text-emerald-600">{tp.profit.toLocaleString()} DZD</td>
                </tr>
              {/each}
            {/if}
          </tbody>
        </table>
      </div>
    </div>
  {/if}
</div>