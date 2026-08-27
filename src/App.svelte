<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { t, setLanguage, getLanguage } from './lib/i18n';
  import type { User, CashSession } from './lib/types';
  import { currentUser } from './lib/stores/auth';
  import { activeSession } from './lib/stores/session';
  import PosView from './routes/pos/PosView.svelte';
  import DashboardView from './routes/dashboard/DashboardView.svelte';
  import SalesView from './routes/sales/SalesView.svelte';
  import InventoryView from './routes/inventory/InventoryView.svelte';
  import ExpensesView from './routes/expenses/ExpensesView.svelte';
  import PayrollView from './routes/payroll/PayrollView.svelte';
  import SettingsView from './routes/settings/SettingsView.svelte';
  import {
    ShoppingCart, LayoutDashboard, Receipt, Package, DollarSign,
    Users, Settings, LogOut, Lock, LogIn
  } from 'lucide-svelte';

  let currentTab: 'pos' | 'dashboard' | 'sales' | 'inventory' | 'expenses' | 'payroll' | 'settings' = 'pos';

  // Login Modal State
  let loginUsername = 'admin';
  let loginPassword = 'admin';
  let loginError = '';
  let isLoggingIn = false;

  onMount(async () => {
    // Initialize language & theme
    const savedLang = getLanguage();
    setLanguage(savedLang);

    const savedTheme = localStorage.getItem('pos_theme') || 'light';
    if (savedTheme === 'dark') {
      document.documentElement.classList.add('dark');
    }

    // Auto-login default admin for instant offline usability
    await handleLogin();
  });

  async function handleLogin() {
    try {
      isLoggingIn = true;
      loginError = '';
      const user = await invoke<User | null>('login', {
        username: loginUsername,
        password: loginPassword,
      });

      if (user) {
        $currentUser = user;
        // Check active cash session
        const session = await invoke<CashSession | null>('get_active_cash_session', {
          userId: user.id,
        });
        $activeSession = session;
      } else {
        loginError = 'Invalid credentials. Default: admin / admin';
      }
    } catch (e: any) {
      loginError = typeof e === 'string' ? e : e.message || 'Login error';
    } finally {
      isLoggingIn = false;
    }
  }

  function handleLogout() {
    $currentUser = null;
    $activeSession = null;
  }
</script>

{#if !$currentUser}
  <!-- Authentication Screen -->
  <div class="h-screen w-screen flex items-center justify-center bg-slate-900 p-4 select-none">
    <div class="bg-pos-card border border-pos-border rounded-2xl p-8 shadow-2xl w-full max-w-sm space-y-6">
      <div class="text-center space-y-1">
        <div class="w-12 h-12 bg-sky-600 rounded-xl flex items-center justify-center mx-auto text-white shadow-lg mb-3">
          <ShoppingCart class="w-6 h-6" />
        </div>
        <h2 class="text-xl font-black text-pos-text">TitaouPosT</h2>
        <p class="text-xs text-pos-muted">Offline-First Point of Sale</p>
      </div>

      {#if loginError}
        <div class="p-3 bg-rose-100 dark:bg-rose-950 text-rose-700 dark:text-rose-300 text-xs font-bold rounded-lg border border-rose-300 dark:border-rose-800">
          {loginError}
        </div>
      {/if}

      <form on:submit|preventDefault={handleLogin} class="space-y-4">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Username</label>
          <input
            type="text"
            bind:value={loginUsername}
            class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border border-pos-border rounded-lg text-sm font-semibold text-pos-text outline-none focus:ring-2 focus:ring-sky-500"
          />
        </div>

        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Password</label>
          <input
            type="password"
            bind:value={loginPassword}
            class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border border-pos-border rounded-lg text-sm font-semibold text-pos-text outline-none focus:ring-2 focus:ring-sky-500"
          />
        </div>

        <button
          type="submit"
          disabled={isLoggingIn}
          class="w-full py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-extrabold text-sm rounded-lg transition shadow-md flex items-center justify-center gap-2 cursor-pointer"
        >
          <LogIn class="w-4 h-4" />
          <span>Sign In</span>
        </button>
      </form>
    </div>
  </div>
{:else}
  <!-- Main Application Shell -->
  <div class="h-screen w-screen flex bg-pos-bg overflow-hidden select-none">
    <!-- Left Narrow Navigation Sidebar -->
    <aside class="w-16 md:w-56 bg-pos-card border-e border-pos-border flex flex-col justify-between shrink-0 shadow-xs z-20">
      <div class="p-3 space-y-4">
        <!-- Brand Header -->
        <div class="flex items-center gap-2.5 px-2 py-1">
          <div class="w-8 h-8 rounded-lg bg-sky-600 text-white flex items-center justify-center font-black shadow-xs shrink-0">
            P
          </div>
          <div class="hidden md:block">
            <h2 class="font-extrabold text-sm text-pos-text leading-tight">TitaouPosT</h2>
            <span class="text-[10px] text-pos-muted">v0.1.0 Offline</span>
          </div>
        </div>

        <!-- Navigation Tabs -->
        <nav class="space-y-1">
          <button
            on:click={() => currentTab = 'pos'}
            class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-xs font-bold transition cursor-pointer {currentTab === 'pos' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
            title="Point of Sale"
          >
            <ShoppingCart class="w-4 h-4 shrink-0" />
            <span class="hidden md:inline">{t('nav_pos')}</span>
          </button>

          <button
            on:click={() => currentTab = 'dashboard'}
            class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-xs font-bold transition cursor-pointer {currentTab === 'dashboard' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
            title="Dashboard"
          >
            <LayoutDashboard class="w-4 h-4 shrink-0" />
            <span class="hidden md:inline">{t('nav_dashboard')}</span>
          </button>

          <button
            on:click={() => currentTab = 'sales'}
            class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-xs font-bold transition cursor-pointer {currentTab === 'sales' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
            title="Sales"
          >
            <Receipt class="w-4 h-4 shrink-0" />
            <span class="hidden md:inline">{t('nav_sales')}</span>
          </button>

          <button
            on:click={() => currentTab = 'inventory'}
            class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-xs font-bold transition cursor-pointer {currentTab === 'inventory' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
            title="Inventory"
          >
            <Package class="w-4 h-4 shrink-0" />
            <span class="hidden md:inline">{t('nav_inventory')}</span>
          </button>

          <button
            on:click={() => currentTab = 'expenses'}
            class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-xs font-bold transition cursor-pointer {currentTab === 'expenses' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
            title="Expenses"
          >
            <DollarSign class="w-4 h-4 shrink-0" />
            <span class="hidden md:inline">{t('nav_expenses')}</span>
          </button>

          <button
            on:click={() => currentTab = 'payroll'}
            class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-xs font-bold transition cursor-pointer {currentTab === 'payroll' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
            title="Payroll"
          >
            <Users class="w-4 h-4 shrink-0" />
            <span class="hidden md:inline">{t('nav_payroll')}</span>
          </button>

          <button
            on:click={() => currentTab = 'settings'}
            class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-xs font-bold transition cursor-pointer {currentTab === 'settings' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
            title="Settings"
          >
            <Settings class="w-4 h-4 shrink-0" />
            <span class="hidden md:inline">{t('nav_settings')}</span>
          </button>
        </nav>
      </div>

      <!-- Logout / User Footer -->
      <div class="p-3 border-t border-pos-border">
        <button
          on:click={handleLogout}
          class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-xs font-bold text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950/40 transition cursor-pointer"
        >
          <LogOut class="w-4 h-4 shrink-0" />
          <span class="hidden md:inline">Sign Out</span>
        </button>
      </div>
    </aside>

    <!-- Main Content Area -->
    <main class="flex-1 flex flex-col min-w-0 overflow-hidden">
      {#if currentTab === 'pos'}
        <PosView />
      {:else if currentTab === 'dashboard'}
        <DashboardView />
      {:else if currentTab === 'sales'}
        <SalesView />
      {:else if currentTab === 'inventory'}
        <InventoryView />
      {:else if currentTab === 'expenses'}
        <ExpensesView />
      {:else if currentTab === 'payroll'}
        <PayrollView />
      {:else if currentTab === 'settings'}
        <SettingsView />
      {/if}
    </main>
  </div>
{/if}