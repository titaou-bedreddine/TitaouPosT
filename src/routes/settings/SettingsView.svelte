<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { currentUser } from '../../lib/stores/auth';
  import {
    Sliders, User, Building, Printer, Smartphone, Download,
    ShieldCheck, RefreshCw, AlertOctagon, Check, Copy, Key, QrCode
  } from 'lucide-svelte';

  type SettingsTab =
    | 'account'
    | 'general'
    | 'invoices'
    | 'pos'
    | 'barcodes'
    | 'network'
    | 'import_export'
    | 'activation'
    | 'updates'
    | 'danger';

  let currentTab: SettingsTab = 'general';
  let settings: Record<string, string> = {};
  let hwid = '';
  let activationCode = '';
  let activationSuccess = false;
  let saveSuccess = false;

  // Account
  let newPassword = '';
  let passwordSuccess = false;

  // Factory Reset
  let resetType = 'transactions_only';
  let resetConfirm = '';

  onMount(async () => {
    await loadSettings();
  });

  async function loadSettings() {
    try {
      settings = await invoke<Record<string, string>>('get_all_settings');
      hwid = await invoke<string>('get_hwid');
    } catch (e) {
      console.error(e);
    }
  }

  async function saveAllSettings() {
    try {
      await invoke('set_multiple_settings', { settings });
      saveSuccess = true;
      setTimeout(() => saveSuccess = false, 3000);
    } catch (e) {
      console.error(e);
    }
  }

  async function handleChangePassword() {
    if (!$currentUser || !newPassword) return;
    try {
      await invoke('change_user_password', {
        userId: $currentUser.id,
        newPassword,
      });
      newPassword = '';
      passwordSuccess = true;
      setTimeout(() => passwordSuccess = false, 3000);
    } catch (e) {
      console.error(e);
    }
  }

  async function handleActivate() {
    if (!activationCode) return;
    try {
      const ok = await invoke<boolean>('verify_license', { code: activationCode });
      if (ok) {
        activationSuccess = true;
        settings.app_license_status = 'activated';
      }
    } catch (e) {
      console.error(e);
    }
  }

  async function handleFactoryReset() {
    if (resetConfirm !== 'RESET') return;
    try {
      await invoke('factory_reset', { resetType });
      resetConfirm = '';
      alert('Reset completed successfully.');
      window.location.reload();
    } catch (e) {
      console.error(e);
    }
  }

  function copyHwid() {
    navigator.clipboard.writeText(hwid);
    alert('Hardware Device ID (HWID) copied to clipboard!');
  }
</script>

<div class="p-6 space-y-6 overflow-y-auto h-full select-none">
  <!-- Top Tab Bar matching photo_2026-08-27_18-51-56.jpg -->
  <div class="flex items-center gap-1.5 border-b border-pos-border pb-2.5 overflow-x-auto">
    <button
      on:click={() => currentTab = 'account'}
      class="px-3 py-2 rounded-xl text-xs font-bold transition flex items-center gap-1.5 cursor-pointer {currentTab === 'account' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted hover:text-pos-text'}"
    >
      <User class="w-3.5 h-3.5" />
      <span>Account</span>
    </button>

    <button
      on:click={() => currentTab = 'general'}
      class="px-3 py-2 rounded-xl text-xs font-bold transition flex items-center gap-1.5 cursor-pointer {currentTab === 'general' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted hover:text-pos-text'}"
    >
      <Building class="w-3.5 h-3.5" />
      <span>General Settings</span>
    </button>

    <button
      on:click={() => currentTab = 'invoices'}
      class="px-3 py-2 rounded-xl text-xs font-bold transition flex items-center gap-1.5 cursor-pointer {currentTab === 'invoices' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted hover:text-pos-text'}"
    >
      <Printer class="w-3.5 h-3.5" />
      <span>Invoices & Printing</span>
    </button>

    <button
      on:click={() => currentTab = 'pos'}
      class="px-3 py-2 rounded-xl text-xs font-bold transition flex items-center gap-1.5 cursor-pointer {currentTab === 'pos' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted hover:text-pos-text'}"
    >
      <Sliders class="w-3.5 h-3.5" />
      <span>POS Settings</span>
    </button>

    <button
      on:click={() => currentTab = 'barcodes'}
      class="px-3 py-2 rounded-xl text-xs font-bold transition flex items-center gap-1.5 cursor-pointer {currentTab === 'barcodes' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted hover:text-pos-text'}"
    >
      <QrCode class="w-3.5 h-3.5" />
      <span>Barcode Labels</span>
    </button>

    <button
      on:click={() => currentTab = 'network'}
      class="px-3 py-2 rounded-xl text-xs font-bold transition flex items-center gap-1.5 cursor-pointer {currentTab === 'network' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted hover:text-pos-text'}"
    >
      <Smartphone class="w-3.5 h-3.5" />
      <span>Network & Mobile</span>
    </button>

    <button
      on:click={() => currentTab = 'import_export'}
      class="px-3 py-2 rounded-xl text-xs font-bold transition flex items-center gap-1.5 cursor-pointer {currentTab === 'import_export' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted hover:text-pos-text'}"
    >
      <Download class="w-3.5 h-3.5" />
      <span>Import / Export</span>
    </button>

    <button
      on:click={() => currentTab = 'activation'}
      class="px-3 py-2 rounded-xl text-xs font-bold transition flex items-center gap-1.5 cursor-pointer {currentTab === 'activation' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted hover:text-pos-text'}"
    >
      <ShieldCheck class="w-3.5 h-3.5" />
      <span>App Activation</span>
    </button>

    <button
      on:click={() => currentTab = 'updates'}
      class="px-3 py-2 rounded-xl text-xs font-bold transition flex items-center gap-1.5 cursor-pointer {currentTab === 'updates' ? 'bg-sky-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-pos-muted hover:text-pos-text'}"
    >
      <RefreshCw class="w-3.5 h-3.5" />
      <span>Updates</span>
    </button>

    <button
      on:click={() => currentTab = 'danger'}
      class="px-3 py-2 rounded-xl text-xs font-bold transition flex items-center gap-1.5 cursor-pointer {currentTab === 'danger' ? 'bg-rose-600 text-white shadow-xs' : 'bg-pos-card border border-pos-border text-rose-500 hover:text-rose-700'}"
    >
      <AlertOctagon class="w-3.5 h-3.5" />
      <span>Danger Zone</span>
    </button>
  </div>

  {#if saveSuccess}
    <div class="p-3 bg-emerald-100 text-emerald-800 text-xs font-bold rounded-xl shadow-xs flex items-center gap-2">
      <Check class="w-4 h-4" />
      <span>Settings saved successfully!</span>
    </div>
  {/if}

  <!-- TAB CONTENTS -->
  {#if currentTab === 'general'}
    <!-- General Settings matching photo_2026-08-27_18-51-06.jpg -->
    <div class="bg-pos-card border border-pos-border rounded-2xl p-6 shadow-xs space-y-5">
      <h3 class="font-extrabold text-base text-pos-text">Store Identity & Legal Tax Information</h3>
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Store / Business Name</label>
          <input type="text" bind:value={settings.shop_name} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Store Description</label>
          <input type="text" bind:value={settings.shop_description} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Address</label>
          <input type="text" bind:value={settings.shop_address} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs text-pos-text" />
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Mobile Phone</label>
          <input type="text" bind:value={settings.shop_phone} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Landline Phone</label>
          <input type="text" bind:value={settings.shop_landline} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Email</label>
          <input type="text" bind:value={settings.shop_email} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs text-pos-text" />
        </div>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-5 gap-4 pt-2 border-t border-pos-border/60">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">RC (السجل التجاري)</label>
          <input type="text" bind:value={settings.shop_rc} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">NIF (الرقم الجبائي)</label>
          <input type="text" bind:value={settings.shop_nif} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">NIS (رقم الإحصاء)</label>
          <input type="text" bind:value={settings.shop_nis} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">AI (المادة الجبائية)</label>
          <input type="text" bind:value={settings.shop_ai} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">TVA (%)</label>
          <input type="text" bind:value={settings.shop_tva} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono text-pos-text" />
        </div>
      </div>

      <div class="flex justify-end pt-3">
        <button on:click={saveAllSettings} class="px-6 py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-extrabold text-xs rounded-xl shadow-xs cursor-pointer">
          Save General Settings
        </button>
      </div>
    </div>
  {:else if currentTab === 'pos'}
    <!-- POS Settings matching photo_2026-08-27_18-51-13.jpg -->
    <div class="bg-pos-card border border-pos-border rounded-2xl p-6 shadow-xs space-y-4">
      <h3 class="font-extrabold text-base text-pos-text">POS Operational Rules</h3>

      <div class="space-y-3">
        <label class="flex items-center justify-between p-3.5 bg-slate-50 dark:bg-slate-800/40 rounded-xl border border-pos-border cursor-pointer">
          <div>
            <p class="text-xs font-bold text-pos-text">Skip Quick Sale Confirm</p>
            <p class="text-[11px] text-pos-muted">Directly finalize cash sales without popup verification</p>
          </div>
          <input type="checkbox" checked={settings.skip_quick_sale_confirm === '1'} on:change={(e) => settings.skip_quick_sale_confirm = e.currentTarget.checked ? '1' : '0'} class="w-4 h-4 rounded text-sky-600" />
        </label>

        <label class="flex items-center justify-between p-3.5 bg-slate-50 dark:bg-slate-800/40 rounded-xl border border-pos-border cursor-pointer">
          <div>
            <p class="text-xs font-bold text-pos-text">Financial-Only Mode</p>
            <p class="text-[11px] text-pos-muted">Process transactions purely financially without strict inventory stock depletion</p>
          </div>
          <input type="checkbox" checked={settings.financial_only_mode === '1'} on:change={(e) => settings.financial_only_mode = e.currentTarget.checked ? '1' : '0'} class="w-4 h-4 rounded text-sky-600" />
        </label>

        <label class="flex items-center justify-between p-3.5 bg-slate-50 dark:bg-slate-800/40 rounded-xl border border-pos-border cursor-pointer">
          <div>
            <p class="text-xs font-bold text-pos-text">Allow Negative Stock (السماح بالمخزون السالب)</p>
            <p class="text-[11px] text-pos-muted">Allow selling products even when inventory count is zero or below</p>
          </div>
          <input type="checkbox" checked={settings.allow_negative_stock === '1'} on:change={(e) => settings.allow_negative_stock = e.currentTarget.checked ? '1' : '0'} class="w-4 h-4 rounded text-sky-600" />
        </label>

        <label class="flex items-center justify-between p-3.5 bg-slate-50 dark:bg-slate-800/40 rounded-xl border border-pos-border cursor-pointer">
          <div>
            <p class="text-xs font-bold text-pos-text">Droit de Timbre (حق الطابع الجبائي للجزائر)</p>
            <p class="text-[11px] text-pos-muted">Apply Algerian fiscal stamp on cash invoices exceeding legal threshold</p>
          </div>
          <input type="checkbox" checked={settings.droit_de_timbre === '1'} on:change={(e) => settings.droit_de_timbre = e.currentTarget.checked ? '1' : '0'} class="w-4 h-4 rounded text-sky-600" />
        </label>
      </div>

      <div class="flex justify-end pt-3">
        <button on:click={saveAllSettings} class="px-6 py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-extrabold text-xs rounded-xl shadow-xs cursor-pointer">
          Save POS Settings
        </button>
      </div>
    </div>
  {:else if currentTab === 'barcodes'}
    <!-- Barcode Label Settings matching photo_2026-08-27_18-51-18.jpg -->
    <div class="bg-pos-card border border-pos-border rounded-2xl p-6 shadow-xs space-y-5">
      <h3 class="font-extrabold text-base text-pos-text">Thermal Barcode Sticker Dimensions & Elements</h3>
      <div class="grid grid-cols-3 gap-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Label Width (mm)</label>
          <input type="number" bind:value={settings.barcode_label_width} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono font-bold text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Label Height (mm)</label>
          <input type="number" bind:value={settings.barcode_label_height} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono font-bold text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Barcode Height (mm)</label>
          <input type="number" bind:value={settings.barcode_label_barcode_height} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-mono font-bold text-pos-text" />
        </div>
      </div>

      <div class="grid grid-cols-2 md:grid-cols-3 gap-3 pt-2">
        <label class="flex items-center gap-2 text-xs font-bold text-pos-text">
          <input type="checkbox" checked={settings.barcode_opt_shop_name === '1'} on:change={(e) => settings.barcode_opt_shop_name = e.currentTarget.checked ? '1' : '0'} />
          <span>Shop Name</span>
        </label>
        <label class="flex items-center gap-2 text-xs font-bold text-pos-text">
          <input type="checkbox" checked={settings.barcode_opt_product_name === '1'} on:change={(e) => settings.barcode_opt_product_name = e.currentTarget.checked ? '1' : '0'} />
          <span>Product Name</span>
        </label>
        <label class="flex items-center gap-2 text-xs font-bold text-pos-text">
          <input type="checkbox" checked={settings.barcode_opt_price === '1'} on:change={(e) => settings.barcode_opt_price = e.currentTarget.checked ? '1' : '0'} />
          <span>Product Price</span>
        </label>
        <label class="flex items-center gap-2 text-xs font-bold text-pos-text">
          <input type="checkbox" checked={settings.barcode_opt_barcode_number === '1'} on:change={(e) => settings.barcode_opt_barcode_number = e.currentTarget.checked ? '1' : '0'} />
          <span>Barcode Digits</span>
        </label>
      </div>

      <div class="flex justify-end pt-3">
        <button on:click={saveAllSettings} class="px-6 py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-extrabold text-xs rounded-xl shadow-xs cursor-pointer">
          Save Barcode Settings
        </button>
      </div>
    </div>
  {:else if currentTab === 'activation'}
    <!-- App Activation matching photo_2026-08-27_18-51-43.jpg -->
    <div class="bg-pos-card border border-pos-border rounded-2xl p-6 shadow-xs space-y-5">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="font-extrabold text-base text-pos-text">App Activation & Device Licensing</h3>
          <p class="text-xs text-pos-muted mt-0.5">Activate full lifetime license for this workstation</p>
        </div>
        <span class="px-3 py-1 rounded-full text-xs font-black uppercase {settings.app_license_status === 'activated' ? 'bg-emerald-100 text-emerald-800' : 'bg-amber-100 text-amber-800'}">
          Status: {settings.app_license_status || 'Trial Mode'}
        </span>
      </div>

      <!-- HWID Card -->
      <div class="bg-slate-100 dark:bg-slate-800/60 p-4 rounded-xl border border-pos-border flex items-center justify-between">
        <div>
          <span class="text-[11px] font-bold text-pos-muted">Your Unique Hardware Device ID (HWID):</span>
          <div class="font-mono text-sm font-bold text-sky-600 mt-0.5">{hwid}</div>
        </div>
        <button on:click={copyHwid} class="px-3 py-1.5 bg-sky-600 text-white text-xs font-bold rounded-lg flex items-center gap-1 cursor-pointer">
          <Copy class="w-3.5 h-3.5" />
          <span>Copy HWID</span>
        </button>
      </div>

      <!-- Activation Code Form -->
      <div class="space-y-3 pt-2">
        <label class="block text-xs font-bold text-pos-muted">Enter Serial Key / Activation Code</label>
        <div class="flex gap-2">
          <input
            type="text"
            bind:value={activationCode}
            placeholder="LUM-XXXX-XXXX-XXXX"
            class="flex-1 px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-sm font-mono font-bold text-pos-text uppercase"
          />
          <button on:click={handleActivate} class="px-6 py-2 bg-emerald-600 hover:bg-emerald-700 text-white font-extrabold text-xs rounded-lg cursor-pointer">
            Activate License
          </button>
        </div>
      </div>
    </div>
  {:else if currentTab === 'account'}
    <!-- Account Settings -->
    <div class="bg-pos-card border border-pos-border rounded-2xl p-6 shadow-xs space-y-4 max-w-lg">
      <h3 class="font-extrabold text-base text-pos-text">Change Password</h3>
      {#if passwordSuccess}
        <div class="p-3 bg-emerald-100 text-emerald-800 text-xs font-bold rounded-lg">Password updated successfully!</div>
      {/if}
      <div>
        <label class="block text-xs font-bold text-pos-muted mb-1">New Password</label>
        <input type="password" bind:value={newPassword} placeholder="Enter new password" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text" />
      </div>
      <button on:click={handleChangePassword} class="px-5 py-2.5 bg-sky-600 hover:bg-sky-700 text-white text-xs font-bold rounded-xl cursor-pointer">
        Update Password
      </button>
    </div>
  {:else if currentTab === 'danger'}
    <!-- Factory Reset matching photo_2026-08-27_18-51-39.jpg -->
    <div class="bg-rose-50/20 border-2 border-rose-500/40 rounded-2xl p-6 shadow-xs space-y-5 max-w-xl">
      <div class="flex items-center gap-3">
        <AlertOctagon class="w-6 h-6 text-rose-600" />
        <h3 class="font-black text-base text-rose-600">Danger Zone: Data Clean & Factory Reset</h3>
      </div>
      <p class="text-xs text-pos-muted">
        Resetting will permanently purge records. This action cannot be reversed.
      </p>

      <div class="space-y-2">
        <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer">
          <input type="radio" bind:group={resetType} value="transactions_only" />
          <span>Clear Transactions Only (Sales, Expenses, Purchases, Cash Movements)</span>
        </label>
        <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer">
          <input type="radio" bind:group={resetType} value="full_reset" />
          <span>Full Factory Reset (Purge Products, Customers, Suppliers & All Data)</span>
        </label>
      </div>

      <div class="pt-2">
        <label class="block text-xs font-bold text-rose-600 mb-1">Type "RESET" to confirm:</label>
        <div class="flex gap-2">
          <input type="text" bind:value={resetConfirm} placeholder="RESET" class="px-3 py-2 bg-pos-card border border-rose-300 rounded-lg text-xs font-bold font-mono text-pos-text" />
          <button
            on:click={handleFactoryReset}
            disabled={resetConfirm !== 'RESET'}
            class="px-5 py-2 bg-rose-600 hover:bg-rose-700 disabled:opacity-40 text-white font-black text-xs rounded-lg cursor-pointer"
          >
            Execute Reset
          </button>
        </div>
      </div>
    </div>
  {:else}
    <!-- Other Tab Fallback -->
    <div class="bg-pos-card border border-pos-border rounded-2xl p-6 shadow-xs space-y-4">
      <h3 class="font-extrabold text-base text-pos-text capitalize">{currentTab.replace('_', ' ')}</h3>
      <p class="text-xs text-pos-muted">Configured and operating normally.</p>
    </div>
  {/if}
</div>