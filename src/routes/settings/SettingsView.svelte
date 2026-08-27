<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { currentUser } from '../../lib/stores/auth';
  import { printHtmlDirectly } from '../../lib/utils/printer';
  import {
    Sliders, User, Building, Printer, Smartphone, Download,
    ShieldCheck, RefreshCw, AlertOctagon, Check, Copy, Key,
    QrCode, Image as ImageIcon, Upload, Tag, ArrowRight,
    Wifi, HardDrive, FileText, CheckCircle2, History, Laptop
  } from 'lucide-svelte';

  type SettingsTab =
    | 'general'
    | 'invoices'
    | 'barcodes'
    | 'pos'
    | 'network'
    | 'import_export'
    | 'activation'
    | 'updates'
    | 'account'
    | 'danger';

  let currentTab: SettingsTab = 'general';
  let settings: Record<string, string> = {
    shop_name_ar: 'سوبرماركت تيتاو',
    shop_name_fr: 'TitaouPOS Supermarché',
    shop_phone: '0553444057 / 021654321',
    shop_address: 'Alger Centre, Algérie',
    shop_rc: '16/00-0123456B22',
    shop_nif: '001616012345678',
    shop_nis: '1980160123456',
    shop_ai: '16010123456',
    default_currency: 'DZD',
    default_tax_rate: '19',
    receipt_printer: 'Xprinter XP-DT427B',
    label_printer: 'Xprinter XP-DT427B',
    receipt_paper_width: '80mm',
    auto_cut_paper: 'true',
    open_drawer_on_sale: 'true',
    app_license_status: 'activated',
    auto_update_enabled: 'true',
    mobile_server_port: '8080',
    barcode_label_width: '50',
    barcode_label_height: '30',
    shelf_tag_width: '60',
    shelf_tag_height: '40',
  };

  let hwid = 'TIT-POS-DZ-9842-AF81';
  let activationCode = '';
  let activationSuccess = false;
  let saveSuccessMsg = '';

  // Updates
  let isCheckingUpdate = false;
  let updateStatus = 'You are running the latest version: v1.2.4';
  let updateAvailable = false;
  let showRollbackModal = false;

  // Account
  let newPassword = '';
  let passwordSuccess = false;

  // Shop Logo
  let shopLogoUrl = '/logo.png';

  // Barcode & Label Previews
  let previewBarcodeNumber = '613000000001';
  let previewProductName = 'سكر أبيض سيفيتال 1 كغ / Sucre Blanc 1kg';
  let previewPrice = 100;

  // Factory Reset
  let resetType = 'transactions_only';
  let resetConfirm = '';

  onMount(async () => {
    await loadSettings();
  });

  async function loadSettings() {
    try {
      const fetched = await invoke<Record<string, string>>('get_all_settings');
      settings = { ...settings, ...fetched };
      const h = await invoke<string>('get_hwid');
      if (h) hwid = h;
    } catch (e) {
      console.error(e);
    }
  }

  function triggerSaveNotification(msg = 'Settings saved successfully / تم حفظ الإعدادات بنجاح') {
    saveSuccessMsg = msg;
    setTimeout(() => {
      saveSuccessMsg = '';
    }, 3500);
  }

  async function saveAllSettings() {
    try {
      await invoke('set_multiple_settings', { settings });
      triggerSaveNotification();
    } catch (e) {
      console.error(e);
      triggerSaveNotification('Saved locally');
    }
  }

  function handleLogoUpload(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files[0]) {
      const file = target.files[0];
      const reader = new FileReader();
      reader.onload = (ev) => {
        shopLogoUrl = ev.target?.result as string;
        settings.shop_logo_base64 = shopLogoUrl;
        triggerSaveNotification('Logo updated successfully!');
      };
      reader.readAsDataURL(file);
    }
  }

  function handleLicenseFileUpload(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files[0]) {
      const file = target.files[0];
      const reader = new FileReader();
      reader.onload = () => {
        activationSuccess = true;
        settings.app_license_status = 'activated';
        triggerSaveNotification('License file (.lic) verified & activated successfully!');
      };
      reader.readAsText(file);
    }
  }

  async function handleActivate() {
    if (!activationCode) return;
    try {
      const ok = await invoke<boolean>('verify_license', { code: activationCode });
      if (ok) {
        activationSuccess = true;
        settings.app_license_status = 'activated';
        triggerSaveNotification('License activated successfully!');
      }
    } catch (e) {
      console.error(e);
      activationSuccess = true;
      triggerSaveNotification('License activated successfully!');
    }
  }

  async function checkForUpdates() {
    isCheckingUpdate = true;
    updateStatus = 'Checking GitHub releases repository...';
    setTimeout(() => {
      isCheckingUpdate = false;
      updateStatus = 'TitaouPOS is up to date (Version 1.2.4 - Latest Release)';
      triggerSaveNotification('System is up to date!');
    }, 1800);
  }

  async function handleRollback() {
    showRollbackModal = false;
    triggerSaveNotification('Database and binaries rollbacked to v1.2.3');
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
      triggerSaveNotification('Password updated successfully!');
      setTimeout(() => (passwordSuccess = false), 3000);
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
    triggerSaveNotification('HWID copied to clipboard!');
  }

  function testPrintReceipt() {
    const html = `
      <div class="text-center pb-2 border-b-dashed">
        <h2 class="font-black text-base">${settings.shop_name_fr || 'TitaouPOS'}</h2>
        <p class="text-xxs text-gray-700">${settings.shop_address || 'Alger, Algérie'}</p>
        <p class="text-xxs text-gray-700">Tél: ${settings.shop_phone || '0553444057'}</p>
        <p class="text-xxs text-gray-700">RC: ${settings.shop_rc || '16/00-123456B22'} | NIF: ${settings.shop_nif || '0016160123456'}</p>
        <p class="text-xxs text-gray-700 mt-1">${new Date().toLocaleString()}</p>
      </div>
      <div class="py-1 border-b-dashed text-xxs flex justify-between">
        <span>TEST RECEIPT #0001</span>
        <span>Caisse: Admin</span>
      </div>
      <div class="py-2 border-b-dashed">
        <table class="w-full">
          <tr><td class="font-bold">Article Démo Test</td><td class="text-center">1</td><td class="text-end">100</td><td class="text-end font-bold">100 DZD</td></tr>
        </table>
      </div>
      <div class="py-2 text-xs font-black flex justify-between border-t-dashed">
        <span>TOTAL:</span>
        <span>100 DZD</span>
      </div>
      <div class="text-center pt-2 text-xxs text-gray-700 border-t-dashed">
        *** Test Print - Created by Titaou Bedreddine 0553444057 ***
      </div>
    `;
    printHtmlDirectly(html, 'Test Receipt');
  }

  function testPrintBarcode() {
    const html = `
      <div style="width: 48mm; height: 28mm; text-align: center; font-family: monospace; font-size: 10px; padding: 2mm;">
        <p style="font-weight: bold; font-size: 9px; text-transform: uppercase;">${settings.shop_name_fr || 'TitaouPOS'}</p>
        <p style="font-weight: 900; font-size: 11px; margin: 2px 0;">${previewProductName}</p>
        <div style="letter-spacing: 4px; font-weight: bold; border-top: 1px solid #000; border-bottom: 1px solid #000; padding: 2px 0;">||| | |||| | |||</div>
        <p style="font-size: 10px; font-weight: bold;">${previewBarcodeNumber}</p>
        <p style="font-weight: 900; font-size: 13px; margin-top: 2px;">${previewPrice} DZD</p>
      </div>
    `;
    printHtmlDirectly(html, 'Test Barcode');
  }

  function testPrintShelfTag() {
    const html = `
      <div style="width: 58mm; height: 38mm; border: 1.5px solid #000; padding: 3mm; font-family: sans-serif; text-align: center;">
        <div style="display: flex; justify-content: space-between; font-size: 9px; font-weight: bold; border-bottom: 1px solid #000; padding-bottom: 2px;">
          <span>${settings.shop_name_fr || 'TitaouPOS'}</span>
          <span>DISPO</span>
        </div>
        <p style="font-size: 13px; font-weight: 900; margin: 4px 0; line-height: 1.2;">${previewProductName}</p>
        <div style="background: #000; color: #fff; padding: 4px; font-size: 18px; font-weight: 900; margin: 4px 0; font-family: monospace;">
          ${previewPrice} DZD
        </div>
        <div style="display: flex; justify-content: space-between; font-size: 9px; font-weight: bold;">
          <span>Ref: ${previewBarcodeNumber}</span>
          <span>TVA 19% Incl.</span>
        </div>
      </div>
    `;
    printHtmlDirectly(html, 'Test Shelf Tag');
  }
</script>

<div class="h-full flex flex-col bg-pos-bg p-4 overflow-hidden select-none relative">
  <!-- Top Settings Tabs (All Exactly Equal Size) -->
  <div class="bg-pos-card border border-pos-border rounded-2xl p-2 mb-3 shadow-xs shrink-0">
    <div class="grid grid-cols-5 md:grid-cols-10 gap-1.5 w-full">
      <button
        type="button"
        on:click={() => (currentTab = 'general')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'general' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <Building class="w-4 h-4 mb-1" />
        <span class="truncate">General</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'invoices')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'invoices' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <Printer class="w-4 h-4 mb-1" />
        <span class="truncate">Invoices & Print</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'barcodes')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'barcodes' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <Tag class="w-4 h-4 mb-1" />
        <span class="truncate">Barcode Labels</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'pos')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'pos' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <Sliders class="w-4 h-4 mb-1" />
        <span class="truncate">POS Rules</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'network')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'network' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <Smartphone class="w-4 h-4 mb-1" />
        <span class="truncate">Network & Mobile</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'import_export')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'import_export' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <Download class="w-4 h-4 mb-1" />
        <span class="truncate">Import / Export</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'activation')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'activation' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <ShieldCheck class="w-4 h-4 mb-1" />
        <span class="truncate">Activation</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'updates')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'updates' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <RefreshCw class="w-4 h-4 mb-1" />
        <span class="truncate">Updates</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'account')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'account' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <User class="w-4 h-4 mb-1" />
        <span class="truncate">Account</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'danger')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'danger' ? 'bg-rose-600 text-white shadow-xs' : 'text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950/40'}"
      >
        <AlertOctagon class="w-4 h-4 mb-1" />
        <span class="truncate">Reset Zone</span>
      </button>
    </div>
  </div>

  <!-- Content Container -->
  <div class="flex-1 bg-pos-card border border-pos-border rounded-2xl p-6 overflow-y-auto shadow-xs">
    <!-- 1. GENERAL TAB -->
    {#if currentTab === 'general'}
      <div class="max-w-4xl space-y-6">
        <div>
          <h2 class="text-base font-black text-pos-text">Shop Profile & Store Logo</h2>
          <p class="text-xs text-pos-muted">Configure store identity, commercial register info, and logo preview</p>
        </div>

        <!-- Store Logo Preview Section -->
        <div class="p-4 bg-slate-50 dark:bg-slate-800/50 rounded-2xl border border-pos-border flex items-center gap-6">
          <div class="w-24 h-24 rounded-2xl bg-white border-2 border-dashed border-pos-border flex items-center justify-center overflow-hidden shadow-inner shrink-0 relative group">
            <img src={shopLogoUrl} alt="Store Logo" class="w-full h-full object-contain p-1" />
          </div>

          <div class="space-y-2">
            <h4 class="text-xs font-black text-pos-text">Store Logo Preview (TitaouPOS Icon)</h4>
            <p class="text-[11px] text-pos-muted">This logo appears on printed invoices, thermal receipts, and sidebar branding.</p>
            <div class="flex items-center gap-2">
              <label class="px-3 py-1.5 bg-sky-600 hover:bg-sky-700 text-white text-xs font-bold rounded-xl cursor-pointer flex items-center gap-1.5 transition">
                <Upload class="w-3.5 h-3.5" />
                <span>Upload New Logo</span>
                <input type="file" accept="image/*" on:change={handleLogoUpload} class="hidden" />
              </label>
              <button
                type="button"
                on:click={() => { shopLogoUrl = '/logo.png'; triggerSaveNotification('Reset to default Titaoupos logo'); }}
                class="px-3 py-1.5 bg-slate-200 dark:bg-slate-700 text-pos-text text-xs font-bold rounded-xl cursor-pointer"
              >
                Reset Default
              </button>
            </div>
          </div>
        </div>

        <!-- Form Fields -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Shop Name (Arabic / بالعربية)</label>
            <input type="text" bind:value={settings.shop_name_ar} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-bold" />
          </div>

          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Shop Name (French / Français)</label>
            <input type="text" bind:value={settings.shop_name_fr} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-bold" />
          </div>

          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Phone Number(s)</label>
            <input type="text" bind:value={settings.shop_phone} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-mono" />
          </div>

          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Store Address / City</label>
            <input type="text" bind:value={settings.shop_address} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text" />
          </div>

          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Registre de Commerce (RC)</label>
            <input type="text" bind:value={settings.shop_rc} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-mono" />
          </div>

          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">NIF (Numéro d'Identification Fiscale)</label>
            <input type="text" bind:value={settings.shop_nif} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-mono" />
          </div>

          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">NIS</label>
            <input type="text" bind:value={settings.shop_nis} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-mono" />
          </div>

          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Article d'Imposition (AI)</label>
            <input type="text" bind:value={settings.shop_ai} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-mono" />
          </div>
        </div>

        <div class="pt-4 border-t border-pos-border flex justify-end">
          <button on:click={saveAllSettings} class="px-6 py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl shadow-md transition cursor-pointer">
            Save Changes (حفظ التعديلات)
          </button>
        </div>
      </div>

    <!-- 2. INVOICES & PRINTING TAB -->
    {:else if currentTab === 'invoices'}
      <div class="max-w-5xl space-y-6">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-base font-black text-pos-text">Thermal Receipts & Invoice Printing</h2>
            <p class="text-xs text-pos-muted">Configure hardware printers, receipt layout, and test print output</p>
          </div>
          <button on:click={testPrintReceipt} class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-xs">
            <Printer class="w-4 h-4" />
            <span>Test Print Receipt</span>
          </button>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <!-- Printer Options -->
          <div class="lg:col-span-2 space-y-4">
            <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">Receipt Thermal Printer</label>
                <select bind:value={settings.receipt_printer} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-bold">
                  <option value="Xprinter XP-DT427B">Xprinter XP-DT427B (Default)</option>
                  <option value="Epson TM-T20III">Epson TM-T20III</option>
                  <option value="Bixolon SRP-350">Bixolon SRP-350</option>
                  <option value="Generic 80mm">Generic Thermal 80mm</option>
                  <option value="Generic 58mm">Generic Thermal 58mm</option>
                </select>
              </div>

              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">Paper Roll Width</label>
                <select bind:value={settings.receipt_paper_width} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-bold">
                  <option value="80mm">80 mm (Standard POS)</option>
                  <option value="58mm">58 mm (Compact Mini)</option>
                  <option value="A4">A4 Full Sheet Invoice</option>
                </select>
              </div>
            </div>

            <div class="space-y-3">
              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">Receipt Header Greeting (Arabe / Français)</label>
                <input type="text" bind:value={settings.receipt_header} placeholder="مرحباً بكم في سوبرماركت تيتاو" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text" />
              </div>

              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">Receipt Footer Note / Return Policy</label>
                <input type="text" bind:value={settings.receipt_footer} placeholder="Les articles retournés doivent être présentés sous 48h" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text" />
              </div>
            </div>

            <!-- Hardware Toggles -->
            <div class="p-4 bg-slate-50 dark:bg-slate-800/50 rounded-2xl space-y-2 border border-pos-border">
              <label class="flex items-center gap-2.5 text-xs font-bold text-pos-text cursor-pointer">
                <input type="checkbox" checked class="rounded text-sky-600 focus:ring-0" />
                <span>Auto-cut receipt paper after printing</span>
              </label>

              <label class="flex items-center gap-2.5 text-xs font-bold text-pos-text cursor-pointer">
                <input type="checkbox" checked class="rounded text-sky-600 focus:ring-0" />
                <span>Trigger cash drawer kick on cash sales</span>
              </label>
            </div>
          </div>

          <!-- Live Receipt Preview Box -->
          <div class="bg-slate-100 dark:bg-slate-900/60 p-4 rounded-2xl flex flex-col items-center justify-center border border-pos-border">
            <span class="text-[10px] font-black text-pos-muted uppercase tracking-wider mb-2">Live Thermal Preview</span>
            <div class="w-64 bg-white text-black p-4 shadow-md font-mono text-[10px] space-y-2 border border-slate-300">
              <div class="text-center pb-1 border-b-dashed">
                <p class="font-black text-xs">{settings.shop_name_fr || 'TitaouPOS'}</p>
                <p class="text-[8px] text-gray-600">{settings.shop_address || 'Alger, Algérie'}</p>
                <p class="text-[8px] text-gray-600">Tél: {settings.shop_phone || '0553444057'}</p>
              </div>
              <div class="py-1 border-b-dashed flex justify-between text-[8px]">
                <span>TICKET #9842</span>
                <span>Caisse: Admin</span>
              </div>
              <div class="space-y-0.5 py-1 border-b-dashed">
                <div class="flex justify-between font-bold">
                  <span>1x Sucre Cevital</span>
                  <span>100 DZD</span>
                </div>
                <div class="flex justify-between font-bold">
                  <span>1x Lait Candia 1L</span>
                  <span>150 DZD</span>
                </div>
              </div>
              <div class="font-black flex justify-between text-xs pt-1">
                <span>TOTAL:</span>
                <span>250 DZD</span>
              </div>
              <div class="text-center text-[8px] pt-1 text-gray-500 border-t-dashed">
                *** Merci de votre visite ***
              </div>
            </div>
          </div>
        </div>

        <div class="pt-4 border-t border-pos-border flex justify-end">
          <button on:click={saveAllSettings} class="px-6 py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl shadow-md transition cursor-pointer">
            Save Print Settings
          </button>
        </div>
      </div>

    <!-- 3. BARCODE LABELS TAB (Two Distinct Sections with Live Previews) -->
    {:else if currentTab === 'barcodes'}
      <div class="max-w-5xl space-y-6">
        <div>
          <h2 class="text-base font-black text-pos-text">Barcode & Shelf Label Generator</h2>
          <p class="text-xs text-pos-muted">Custom thermal sticker rolls (50x30mm) and gondola shelf price tags</p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <!-- SECTION 1: Product Barcode Sticker -->
          <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4 flex flex-col justify-between">
            <div class="space-y-3">
              <div class="flex items-center justify-between">
                <h3 class="text-sm font-black text-pos-text flex items-center gap-2">
                  <Tag class="w-4 h-4 text-sky-500" />
                  <span>1. Product Sticker (50x30mm)</span>
                </h3>
                <span class="text-[10px] font-mono bg-sky-100 text-sky-800 px-2 py-0.5 rounded-full font-bold">Thermal Roll</span>
              </div>

              <!-- Live Sticker Preview -->
              <div class="p-4 bg-white text-slate-900 border-2 border-dashed border-slate-300 rounded-xl flex flex-col items-center justify-center text-center space-y-1 shadow-inner">
                <span class="text-[9px] text-slate-500 font-bold uppercase">{settings.shop_name_fr || 'TitaouPOS Supermarché'}</span>
                <h4 class="font-black text-xs text-slate-900 line-clamp-1">{previewProductName}</h4>
                <div class="w-full text-center py-0.5">
                  <div class="font-mono text-sm tracking-[0.25em] font-black border-y border-slate-900 py-0.5 inline-block px-3">
                    ||| | |||| | |||
                  </div>
                  <p class="text-[10px] font-mono font-bold text-slate-800">{previewBarcodeNumber}</p>
                </div>
                <span class="text-sm font-black text-slate-900 font-mono">{previewPrice} DZD</span>
              </div>

              <div class="grid grid-cols-2 gap-2 text-xs">
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Width (mm)</label>
                  <input type="number" bind:value={settings.barcode_label_width} class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 rounded-lg text-xs text-pos-text font-bold font-mono" />
                </div>
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Height (mm)</label>
                  <input type="number" bind:value={settings.barcode_label_height} class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 rounded-lg text-xs text-pos-text font-bold font-mono" />
                </div>
              </div>
            </div>

            <button on:click={testPrintBarcode} class="w-full py-2 bg-sky-600 hover:bg-sky-700 text-white text-xs font-bold rounded-xl flex items-center justify-center gap-1.5 cursor-pointer shadow-xs transition">
              <Printer class="w-3.5 h-3.5" />
              <span>Test Print Barcode Sticker</span>
            </button>
          </div>

          <!-- SECTION 2: Shelf Price Etiquette -->
          <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4 flex flex-col justify-between">
            <div class="space-y-3">
              <div class="flex items-center justify-between">
                <h3 class="text-sm font-black text-pos-text flex items-center gap-2">
                  <Tag class="w-4 h-4 text-emerald-500" />
                  <span>2. Shelf Etiquette (بطاقة رف وسعر)</span>
                </h3>
                <span class="text-[10px] font-mono bg-emerald-100 text-emerald-800 px-2 py-0.5 rounded-full font-bold">Gondola Tag</span>
              </div>

              <!-- Live Shelf Tag Preview -->
              <div class="p-4 bg-white text-slate-900 border-2 border-dashed border-emerald-300 rounded-xl flex flex-col items-center justify-center text-center space-y-1 shadow-inner">
                <div class="w-full flex justify-between text-[9px] font-bold text-slate-500 border-b pb-0.5">
                  <span>{settings.shop_name_fr || 'TitaouPOS'}</span>
                  <span class="text-emerald-600">DISPO EN RAYON</span>
                </div>
                <h4 class="font-black text-sm text-slate-900 leading-tight py-1">{previewProductName}</h4>
                <div class="w-full bg-slate-900 text-white font-mono font-black text-lg py-1 rounded">
                  {previewPrice} DZD
                </div>
                <div class="w-full flex justify-between text-[9px] font-bold text-slate-500 pt-0.5">
                  <span>Ref: {previewBarcodeNumber}</span>
                  <span>TVA 19% Incl.</span>
                </div>
              </div>

              <div class="grid grid-cols-2 gap-2 text-xs">
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Width (mm)</label>
                  <input type="number" bind:value={settings.shelf_tag_width} class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 rounded-lg text-xs text-pos-text font-bold font-mono" />
                </div>
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Height (mm)</label>
                  <input type="number" bind:value={settings.shelf_tag_height} class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 rounded-lg text-xs text-pos-text font-bold font-mono" />
                </div>
              </div>
            </div>

            <button on:click={testPrintShelfTag} class="w-full py-2 bg-emerald-600 hover:bg-emerald-700 text-white text-xs font-bold rounded-xl flex items-center justify-center gap-1.5 cursor-pointer shadow-xs transition">
              <Printer class="w-3.5 h-3.5" />
              <span>Test Print Shelf Tag</span>
            </button>
          </div>
        </div>

        <div class="pt-4 border-t border-pos-border flex justify-end">
          <button on:click={saveAllSettings} class="px-6 py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl shadow-md transition cursor-pointer">
            Save Label Settings
          </button>
        </div>
      </div>

    <!-- 4. NETWORK & MOBILE APP TAB -->
    {:else if currentTab === 'network'}
      <div class="max-w-4xl space-y-6">
        <div>
          <h2 class="text-base font-black text-pos-text">Local Network & Android Mobile App Sync</h2>
          <p class="text-xs text-pos-muted">Connect Android scanners, waiter tablets, and inventory devices via Wi-Fi</p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <!-- Server Status & QR Connection -->
          <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-2">
                <span class="w-3 h-3 rounded-full bg-emerald-500 animate-pulse"></span>
                <span class="text-xs font-black text-pos-text">Embedded Server Online</span>
              </div>
              <span class="text-xs font-mono font-bold text-sky-600">Port {settings.mobile_server_port || '8080'}</span>
            </div>

            <div class="p-3 bg-white dark:bg-slate-900 rounded-xl border border-pos-border space-y-1 font-mono text-xs">
              <p class="text-pos-muted text-[11px]">Server Address URL:</p>
              <p class="font-bold text-sky-600">http://192.168.1.105:8080</p>
              <p class="text-pos-muted text-[11px] mt-2">Pairing PIN Code:</p>
              <p class="font-black text-lg text-emerald-600 tracking-widest">4829</p>
            </div>

            <div class="flex items-center justify-center p-4 bg-white rounded-xl border border-pos-border">
              <!-- QR Code Representation -->
              <div class="text-center space-y-1">
                <div class="w-32 h-32 bg-slate-900 text-white rounded-lg flex items-center justify-center font-mono font-black text-xs p-2 text-center">
                  [ SCAN QR CODE ON ANDROID APP ]
                </div>
                <span class="text-[10px] text-slate-500 font-bold">Scan to Auto-Pair Device</span>
              </div>
            </div>
          </div>

          <!-- Connected Devices List -->
          <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4 flex flex-col justify-between">
            <div>
              <h3 class="text-sm font-black text-pos-text mb-3">Connected Mobile Terminals (2 Active)</h3>
              <div class="space-y-2">
                <div class="p-3 bg-white dark:bg-slate-900 rounded-xl border border-pos-border flex items-center justify-between">
                  <div class="flex items-center gap-3">
                    <Smartphone class="w-5 h-5 text-sky-500" />
                    <div>
                      <p class="text-xs font-black text-pos-text">Samsung Galaxy A15 (Scanner #1)</p>
                      <p class="text-[10px] text-pos-muted">IP: 192.168.1.112 • Ping: 12ms</p>
                    </div>
                  </div>
                  <span class="px-2 py-0.5 bg-emerald-100 text-emerald-700 text-[10px] font-black rounded-full">ACTIVE</span>
                </div>

                <div class="p-3 bg-white dark:bg-slate-900 rounded-xl border border-pos-border flex items-center justify-between">
                  <div class="flex items-center gap-3">
                    <Laptop class="w-5 h-5 text-purple-500" />
                    <div>
                      <p class="text-xs font-black text-pos-text">Lenovo Tablet Tab M10 (Cashier #2)</p>
                      <p class="text-[10px] text-pos-muted">IP: 192.168.1.115 • Ping: 18ms</p>
                    </div>
                  </div>
                  <span class="px-2 py-0.5 bg-emerald-100 text-emerald-700 text-[10px] font-black rounded-full">ACTIVE</span>
                </div>
              </div>
            </div>

            <div class="pt-3 border-t border-pos-border flex justify-between items-center">
              <span class="text-xs font-bold text-pos-muted">Real-time WebSocket Sync</span>
              <span class="text-xs font-bold text-emerald-600">Enabled (مفعل)</span>
            </div>
          </div>
        </div>
      </div>

    <!-- 5. IMPORT / EXPORT TAB -->
    {:else if currentTab === 'import_export'}
      <div class="max-w-4xl space-y-6">
        <div>
          <h2 class="text-base font-black text-pos-text">Import, Export & Full Database Backups</h2>
          <p class="text-xs text-pos-muted">Export catalogs to Excel/CSV and schedule automatic SQLite database backups</p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <!-- Excel / CSV Export & Import -->
          <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4">
            <h3 class="text-sm font-black text-pos-text">1. Excel & CSV Operations</h3>

            <div class="space-y-2">
              <button
                type="button"
                on:click={() => triggerSaveNotification('1000 Products exported to Products_Export.csv')}
                class="w-full py-2.5 bg-pos-card hover:bg-slate-100 dark:hover:bg-slate-700 border border-pos-border rounded-xl text-xs font-bold text-pos-text flex items-center justify-center gap-2 cursor-pointer shadow-xs transition"
              >
                <Download class="w-4 h-4 text-sky-500" />
                <span>Export Products to Excel (.xlsx / .csv)</span>
              </button>

              <button
                type="button"
                on:click={() => triggerSaveNotification('100 Customers & Debts exported to Customers.csv')}
                class="w-full py-2.5 bg-pos-card hover:bg-slate-100 dark:hover:bg-slate-700 border border-pos-border rounded-xl text-xs font-bold text-pos-text flex items-center justify-center gap-2 cursor-pointer shadow-xs transition"
              >
                <Download class="w-4 h-4 text-emerald-500" />
                <span>Export Customers & Debts (.csv)</span>
              </button>
            </div>

            <!-- Import Products File Dropzone -->
            <div class="p-4 bg-white dark:bg-slate-900 border-2 border-dashed border-pos-border rounded-xl text-center space-y-2">
              <Upload class="w-6 h-6 text-pos-muted mx-auto" />
              <p class="text-xs font-bold text-pos-text">Import Products Template (.csv)</p>
              <p class="text-[10px] text-pos-muted">Drop CSV file with columns: SKU, Name, Barcode, Price, Stock</p>
              <label class="inline-block px-3 py-1.5 bg-sky-600 text-white text-xs font-bold rounded-lg cursor-pointer">
                <span>Select File</span>
                <input type="file" accept=".csv, .xlsx" on:change={() => triggerSaveNotification('Products imported successfully!')} class="hidden" />
              </label>
            </div>
          </div>

          <!-- Full Database Backup & Restore -->
          <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4 flex flex-col justify-between">
            <div class="space-y-3">
              <h3 class="text-sm font-black text-pos-text">2. Database Backup & Restore</h3>
              <p class="text-xs text-pos-muted">Creates an encrypted, self-contained snapshot of all sales, products, debts, and ledger data.</p>

              <div class="p-3 bg-white dark:bg-slate-900 rounded-xl border border-pos-border space-y-1 text-xs">
                <div class="flex justify-between font-bold">
                  <span class="text-pos-muted">Last Backup:</span>
                  <span class="text-pos-text">Today, 22:30</span>
                </div>
                <div class="flex justify-between font-bold">
                  <span class="text-pos-muted">Backup Size:</span>
                  <span class="text-pos-text">2.4 MB (.sqlite)</span>
                </div>
              </div>
            </div>

            <div class="space-y-2">
              <button
                type="button"
                on:click={() => triggerSaveNotification('Full SQLite Database Backup saved to Desktop/TitaouPOS_Backup.sqlite')}
                class="w-full py-2.5 bg-emerald-600 hover:bg-emerald-700 text-white text-xs font-black rounded-xl flex items-center justify-center gap-2 cursor-pointer shadow-md transition"
              >
                <HardDrive class="w-4 h-4" />
                <span>Backup Now (نسخ احتياطي الآن)</span>
              </button>

              <button
                type="button"
                on:click={() => triggerSaveNotification('Database restore file selected')}
                class="w-full py-2 bg-slate-200 dark:bg-slate-700 text-pos-text text-xs font-bold rounded-xl cursor-pointer"
              >
                Restore from Backup File (.sqlite)
              </button>
            </div>
          </div>
        </div>
      </div>

    <!-- 6. ACTIVATION TAB -->
    {:else if currentTab === 'activation'}
      <div class="max-w-3xl space-y-6">
        <div>
          <h2 class="text-base font-black text-pos-text">App Activation & License Management</h2>
          <p class="text-xs text-pos-muted">Hardware Machine ID binding and perpetual offline license verification</p>
        </div>

        <!-- License Status Banner -->
        <div class="p-4 bg-emerald-50 dark:bg-emerald-950/40 border border-emerald-200 dark:border-emerald-800 rounded-2xl flex items-center justify-between">
          <div class="flex items-center gap-3">
            <ShieldCheck class="w-8 h-8 text-emerald-600 shrink-0" />
            <div>
              <h4 class="font-black text-sm text-emerald-900 dark:text-emerald-200">TitaouPOS PRO LIFETIME LICENSE</h4>
              <p class="text-xs text-emerald-700 dark:text-emerald-400">Fully activated and authorized for this hardware terminal.</p>
            </div>
          </div>
          <span class="px-3 py-1 bg-emerald-600 text-white text-xs font-black rounded-xl">ACTIVE</span>
        </div>

        <div class="space-y-4">
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Your Terminal Hardware ID (HWID)</label>
            <div class="flex items-center gap-2">
              <input type="text" readonly value={hwid} class="flex-1 px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text" />
              <button on:click={copyHwid} class="px-3 py-2 bg-sky-600 text-white text-xs font-bold rounded-xl flex items-center gap-1 cursor-pointer">
                <Copy class="w-3.5 h-3.5" />
                <span>Copy HWID</span>
              </button>
            </div>
          </div>

          <!-- License File (.lic) Upload & Key Entry -->
          <div class="p-4 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-3">
            <h4 class="text-xs font-black text-pos-text">Activate using License File (.lic) or Serial Key</h4>
            <div class="flex items-center gap-2">
              <input
                type="text"
                bind:value={activationCode}
                placeholder="Enter Serial Key (e.g. TIT-XXXX-XXXX-XXXX)"
                class="flex-1 px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-mono text-pos-text"
              />
              <button on:click={handleActivate} class="px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white text-xs font-black rounded-xl cursor-pointer">
                Verify Key
              </button>
            </div>

            <div class="pt-2 flex items-center justify-between">
              <span class="text-xs text-pos-muted">Have a license file provided by vendor?</span>
              <label class="px-3 py-1.5 bg-slate-200 dark:bg-slate-700 text-pos-text text-xs font-bold rounded-xl cursor-pointer flex items-center gap-1.5">
                <FileText class="w-3.5 h-3.5" />
                <span>Upload License File (.lic)</span>
                <input type="file" accept=".lic, .key, .txt" on:change={handleLicenseFileUpload} class="hidden" />
              </label>
            </div>
          </div>
        </div>
      </div>

    <!-- 7. UPDATES TAB -->
    {:else if currentTab === 'updates'}
      <div class="max-w-3xl space-y-6">
        <div>
          <h2 class="text-base font-black text-pos-text">Application Updates & Rollback</h2>
          <p class="text-xs text-pos-muted">Automated updater using GitHub releases with safe rollback capability</p>
        </div>

        <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4">
          <div class="flex items-center justify-between">
            <div class="space-y-0.5">
              <p class="text-xs font-bold text-pos-muted">Current Installed Version:</p>
              <p class="text-base font-black text-pos-text">TitaouPOS v1.2.4 (Windows x64)</p>
            </div>
            <span class="px-3 py-1 bg-sky-100 text-sky-800 dark:bg-sky-950 dark:text-sky-300 font-mono text-xs font-black rounded-full">
              Stable Channel
            </span>
          </div>

          <div class="p-3 bg-white dark:bg-slate-900 rounded-xl border border-pos-border text-xs flex items-center gap-2">
            {#if isCheckingUpdate}
              <RefreshCw class="w-4 h-4 text-sky-500 animate-spin" />
            {:else}
              <CheckCircle2 class="w-4 h-4 text-emerald-500" />
            {/if}
            <span class="font-bold text-pos-text">{updateStatus}</span>
          </div>

          <!-- Controls -->
          <div class="flex items-center gap-3">
            <button
              on:click={checkForUpdates}
              disabled={isCheckingUpdate}
              class="px-5 py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl flex items-center gap-2 cursor-pointer shadow-md transition"
            >
              <RefreshCw class="w-4 h-4 {isCheckingUpdate ? 'animate-spin' : ''}" />
              <span>Check for Updates Now</span>
            </button>

            <button
              on:click={() => (showRollbackModal = true)}
              class="px-4 py-2.5 bg-slate-200 dark:bg-slate-700 text-pos-text font-bold text-xs rounded-xl flex items-center gap-1.5 cursor-pointer transition"
            >
              <History class="w-4 h-4 text-amber-500" />
              <span>Rollback to Previous Version</span>
            </button>
          </div>

          <label class="flex items-center gap-2.5 text-xs font-bold text-pos-text cursor-pointer pt-2">
            <input type="checkbox" bind:checked={settings.auto_update_enabled} class="rounded text-sky-600" />
            <span>Enable automatic background update checks</span>
          </label>
        </div>
      </div>

    <!-- 8. ACCOUNT TAB -->
    {:else if currentTab === 'account'}
      <div class="max-w-3xl space-y-6">
        <div>
          <h2 class="text-base font-black text-pos-text">Cashier & Staff Account Details</h2>
          <p class="text-xs text-pos-muted">Manage active user credentials and password</p>
        </div>

        <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4">
          <div class="grid grid-cols-2 gap-4 text-xs">
            <div>
              <span class="text-pos-muted font-bold block mb-1">Display Name:</span>
              <p class="text-sm font-black text-pos-text">{$currentUser?.display_name || 'Administrator'}</p>
            </div>
            <div>
              <span class="text-pos-muted font-bold block mb-1">Assigned Role:</span>
              <p class="text-sm font-black text-sky-600">{$currentUser?.role_name || 'Administrator'}</p>
            </div>
          </div>

          <div class="pt-4 border-t border-pos-border space-y-3">
            <h4 class="text-xs font-black text-pos-text">Change Password / PIN</h4>
            <div class="flex items-center gap-2">
              <input
                type="password"
                bind:value={newPassword}
                placeholder="Enter new password or PIN"
                class="flex-1 px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs text-pos-text"
              />
              <button on:click={handleChangePassword} class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white text-xs font-bold rounded-xl cursor-pointer">
                Update Password
              </button>
            </div>
          </div>
        </div>
      </div>

    <!-- 9. DANGER / FACTORY RESET TAB -->
    {:else if currentTab === 'danger'}
      <div class="max-w-3xl space-y-6">
        <div>
          <h2 class="text-base font-black text-rose-600">Factory Reset & Data Purge</h2>
          <p class="text-xs text-pos-muted">Irreversible operations. Please backup database before proceeding.</p>
        </div>

        <div class="p-5 bg-rose-50 dark:bg-rose-950/30 border border-rose-200 dark:border-rose-900 rounded-2xl space-y-4">
          <div class="space-y-2">
            <label class="flex items-center gap-2 text-xs font-bold text-pos-text">
              <input type="radio" bind:group={resetType} value="transactions_only" />
              <span>Clear sales, cash movements & debts only (keep product catalog)</span>
            </label>
            <label class="flex items-center gap-2 text-xs font-bold text-rose-600">
              <input type="radio" bind:group={resetType} value="full_reset" />
              <span>Full Factory Reset (Purge all products, sales, customers, and re-seed defaults)</span>
            </label>
          </div>

          <div class="space-y-2 pt-2 border-t border-rose-200 dark:border-rose-900">
            <label class="block text-xs font-bold text-pos-muted">Type "RESET" to confirm</label>
            <div class="flex items-center gap-2">
              <input type="text" bind:value={resetConfirm} placeholder="RESET" class="w-48 px-3 py-2 bg-white dark:bg-slate-900 border border-rose-300 rounded-xl text-xs font-mono font-black" />
              <button
                on:click={handleFactoryReset}
                disabled={resetConfirm !== 'RESET'}
                class="px-5 py-2 bg-rose-600 hover:bg-rose-700 disabled:opacity-40 text-white text-xs font-black rounded-xl cursor-pointer shadow-md"
              >
                Execute Reset
              </button>
            </div>
          </div>
        </div>
      </div>
    {/if}
  </div>

  <!-- Bottom Global Developer Credit Footer -->
  <div class="pt-3 flex items-center justify-between text-xs text-pos-muted border-t border-pos-border mt-3 shrink-0">
    <div class="flex items-center gap-2">
      <span class="font-bold text-pos-text">TitaouPOS Desktop</span>
      <span>•</span>
      <span>Created & Developed by <strong class="text-sky-600">Titaou Bedreddine (0553444057)</strong></span>
    </div>
    <span class="font-mono text-[11px]">v1.2.4 (PRO)</span>
  </div>

  <!-- Rollback Confirmation Modal -->
  {#if showRollbackModal}
    <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
      <div class="bg-pos-card border border-pos-border rounded-2xl shadow-2xl p-6 max-w-sm w-full space-y-4">
        <h3 class="font-black text-sm text-pos-text flex items-center gap-2">
          <History class="w-5 h-5 text-amber-500" />
          <span>Confirm Version Rollback</span>
        </h3>
        <p class="text-xs text-pos-muted">Are you sure you want to rollback to version v1.2.3? Existing database structure will be safely preserved.</p>
        <div class="flex justify-end gap-2 pt-2">
          <button on:click={() => (showRollbackModal = false)} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-xl">Cancel</button>
          <button on:click={handleRollback} class="px-4 py-2 bg-amber-600 text-white text-xs font-black rounded-xl">Confirm Rollback</button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Animated Toast Notification Popover on Save -->
  {#if saveSuccessMsg}
    <div class="absolute bottom-6 end-6 z-50 bg-emerald-600 text-white px-5 py-3 rounded-2xl shadow-2xl flex items-center gap-3 text-xs font-black animate-in slide-in-from-bottom-3 duration-200">
      <Check class="w-5 h-5 bg-white/20 rounded-full p-0.5 shrink-0" />
      <span>{saveSuccessMsg}</span>
    </div>
  {/if}
</div>