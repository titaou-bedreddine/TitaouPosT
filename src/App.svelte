<script lang="ts">
  import { onMount } from 'svelte';
  import { t } from './lib/i18n';
  import { currentUser, isAuthenticated, logout } from './lib/stores/auth';
  import { activeSession } from './lib/stores/session';
  import { invoke } from '@tauri-apps/api/core';
  import type { CashSession } from './lib/types';

  // Routes
  import PosView from './routes/pos/PosView.svelte';
  import SalesView from './routes/sales/SalesView.svelte';
  import CashRegisterView from './routes/cash/CashRegisterView.svelte';
  import PurchasesView from './routes/purchases/PurchasesView.svelte';
  import CustomersView from './routes/customers/CustomersView.svelte';
  import SuppliersView from './routes/suppliers/SuppliersView.svelte';
  import InventoryView from './routes/inventory/InventoryView.svelte';
  import ExpensesView from './routes/expenses/ExpensesView.svelte';
  import PayrollView from './routes/payroll/PayrollView.svelte';
  import DashboardView from './routes/dashboard/DashboardView.svelte';
  import SettingsView from './routes/settings/SettingsView.svelte';
  import LoginView from './routes/auth/LoginView.svelte';

  // Icons
  import {
    LayoutDashboard, ShoppingCart, Receipt, DollarSign,
    Package, TrendingDown, Users, Settings, LogOut,
    Truck, FileSpreadsheet, UserCheck, Wifi, Moon, Sun
  } from 'lucide-svelte';

  let currentRoute = 'pos';
  let isDarkMode = false;

  onMount(async () => {
    // Disable right-click context menu across Tauri POS desktop app
    window.addEventListener('contextmenu', (e) => e.preventDefault());

    // Auto-select text on input focus for fast barcode/number replacement
    window.addEventListener('focusin', (e) => {
      const target = e.target as HTMLElement;
      if (target && (target.tagName === 'INPUT' || target.tagName === 'TEXTAREA')) {
        const input = target as HTMLInputElement;
        if (input.type === 'number' || input.type === 'text') {
          setTimeout(() => input.select(), 10);
        }
      }
    });

    if ($currentUser) {
      try {
        const session = await invoke<CashSession | null>('get_active_cash_session', { userId: $currentUser.id });
        $activeSession = session;
      } catch (err) {
        console.error(err);
      }
    }
  });

  function toggleTheme() {
    isDarkMode = !isDarkMode;
    if (isDarkMode) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }

  function handleLogout() {
    logout();
  }
</script>

{#if !$isAuthenticated}
  <LoginView />
{:else}
  <div class="flex h-screen w-screen overflow-hidden bg-pos-bg text-pos-text">
    <!-- SIDEBAR -->
    <aside class="w-64 bg-pos-card border-e border-pos-border flex flex-col justify-between shrink-0 select-none shadow-xs">
      <!-- Top Brand Header -->
      <div class="p-4 border-b border-pos-border/60">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-xl bg-sky-600 flex items-center justify-center text-white font-black shadow-md text-lg">
            T
          </div>
          <div>
            <h1 class="font-black text-sm tracking-tight text-pos-text">TitaouPosT</h1>
            <p class="text-[10px] text-sky-600 font-bold uppercase tracking-wider">Lumina Point of Sale</p>
          </div>
        </div>
      </div>

      <!-- Navigation Menu Links -->
      <nav class="flex-1 overflow-y-auto p-2.5 space-y-1">
        <button
          type="button"
          on:click={() => currentRoute = 'pos'}
          class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-bold transition cursor-pointer {currentRoute === 'pos' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-pos-text'}"
        >
          <ShoppingCart class="w-4 h-4" />
          <span>Point of Sale (POS)</span>
        </button>

        <button
          type="button"
          on:click={() => currentRoute = 'sales'}
          class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-bold transition cursor-pointer {currentRoute === 'sales' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-pos-text'}"
        >
          <Receipt class="w-4 h-4" />
          <span>Sales History (المبيعات)</span>
        </button>

        <button
          type="button"
          on:click={() => currentRoute = 'cash'}
          class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-bold transition cursor-pointer {currentRoute === 'cash' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-pos-text'}"
        >
          <DollarSign class="w-4 h-4 text-emerald-500" />
          <span>Cash Register (الصندوق)</span>
        </button>

        <button
          type="button"
          on:click={() => currentRoute = 'purchases'}
          class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-bold transition cursor-pointer {currentRoute === 'purchases' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-pos-text'}"
        >
          <FileSpreadsheet class="w-4 h-4" />
          <span>Purchases (المشتريات)</span>
        </button>

        <button
          type="button"
          on:click={() => currentRoute = 'customers'}
          class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-bold transition cursor-pointer {currentRoute === 'customers' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-pos-text'}"
        >
          <Users class="w-4 h-4" />
          <span>Customers & Debt (الزبائن)</span>
        </button>

        <button
          type="button"
          on:click={() => currentRoute = 'suppliers'}
          class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-bold transition cursor-pointer {currentRoute === 'suppliers' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-pos-text'}"
        >
          <Truck class="w-4 h-4" />
          <span>Suppliers (الموردون)</span>
        </button>

        <button
          type="button"
          on:click={() => currentRoute = 'inventory'}
          class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-bold transition cursor-pointer {currentRoute === 'inventory' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-pos-text'}"
        >
          <Package class="w-4 h-4" />
          <span>Products (المخزون)</span>
        </button>

        <button
          type="button"
          on:click={() => currentRoute = 'expenses'}
          class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-bold transition cursor-pointer {currentRoute === 'expenses' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-pos-text'}"
        >
          <TrendingDown class="w-4 h-4" />
          <span>Expenses (المصاريف)</span>
        </button>

        <button
          type="button"
          on:click={() => currentRoute = 'payroll'}
          class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-bold transition cursor-pointer {currentRoute === 'payroll' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-pos-text'}"
        >
          <UserCheck class="w-4 h-4" />
          <span>Employees (الموظفون)</span>
        </button>

        <button
          type="button"
          on:click={() => currentRoute = 'dashboard'}
          class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-bold transition cursor-pointer {currentRoute === 'dashboard' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-pos-text'}"
        >
          <LayoutDashboard class="w-4 h-4" />
          <span>Analytics (الإحصائيات)</span>
        </button>

        <button
          type="button"
          on:click={() => currentRoute = 'settings'}
          class="w-full flex items-center gap-3 px-3.5 py-2.5 rounded-xl text-xs font-bold transition cursor-pointer {currentRoute === 'settings' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800 hover:text-pos-text'}"
        >
          <Settings class="w-4 h-4" />
          <span>Settings (الإعدادات)</span>
        </button>
      </nav>

      <!-- Bottom User Profile & Network Status (matching user request) -->
      <div class="p-3 border-t border-pos-border/60 bg-slate-50 dark:bg-slate-800/40 space-y-2.5">
        <!-- Local Network Online Badge -->
        <div class="flex items-center justify-between text-[11px] text-emerald-600 dark:text-emerald-400 font-bold px-1">
          <div class="flex items-center gap-1.5">
            <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
            <span>Local Network Online</span>
          </div>
          <Wifi class="w-3.5 h-3.5 text-emerald-500" />
        </div>

        <!-- Clean User Card (No double "admin (admin)") -->
        <div class="flex items-center justify-between p-2 bg-pos-card rounded-xl border border-pos-border shadow-xs">
          <div class="flex items-center gap-2 min-w-0">
            <div class="w-8 h-8 rounded-lg bg-sky-100 dark:bg-sky-950 text-sky-600 dark:text-sky-300 flex items-center justify-center font-bold text-xs shrink-0">
              {$currentUser?.display_name?.charAt(0) || 'U'}
            </div>
            <div class="min-w-0">
              <p class="text-xs font-black text-pos-text truncate">{$currentUser?.display_name || 'Admin'}</p>
              <p class="text-[10px] text-pos-muted capitalize truncate">{$currentUser?.role_name || 'Administrator'}</p>
            </div>
          </div>

          <div class="flex items-center gap-1">
            <button
              type="button"
              on:click={toggleTheme}
              class="p-1.5 text-pos-muted hover:text-pos-text rounded-lg cursor-pointer"
              title="Toggle Theme"
            >
              {#if isDarkMode}
                <Sun class="w-3.5 h-3.5 text-amber-400" />
              {:else}
                <Moon class="w-3.5 h-3.5" />
              {/if}
            </button>
            <button
              type="button"
              on:click={handleLogout}
              class="p-1.5 text-rose-500 hover:text-rose-700 rounded-lg cursor-pointer"
              title="Sign Out"
            >
              <LogOut class="w-3.5 h-3.5" />
            </button>
          </div>
        </div>
      </div>
    </aside>

    <!-- MAIN ROUTE CONTENT -->
    <main class="flex-1 overflow-hidden bg-pos-bg">
      {#if currentRoute === 'pos'}
        <PosView />
      {:else if currentRoute === 'sales'}
        <SalesView />
      {:else if currentRoute === 'cash'}
        <CashRegisterView />
      {:else if currentRoute === 'purchases'}
        <PurchasesView />
      {:else if currentRoute === 'customers'}
        <CustomersView />
      {:else if currentRoute === 'suppliers'}
        <SuppliersView />
      {:else if currentRoute === 'inventory'}
        <InventoryView />
      {:else if currentRoute === 'expenses'}
        <ExpensesView />
      {:else if currentRoute === 'payroll'}
        <PayrollView />
      {:else if currentRoute === 'dashboard'}
        <DashboardView />
      {:else if currentRoute === 'settings'}
        <SettingsView />
      {/if}
    </main>
  </div>
{/if}