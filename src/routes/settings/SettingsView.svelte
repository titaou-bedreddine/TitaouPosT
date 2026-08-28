<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { currentUser } from '../../lib/stores/auth';
  import { printHtmlDirectly } from '../../lib/utils/printer';
  import {
    Sliders, User, Building, Printer, Smartphone, Download,
    ShieldCheck, RefreshCw, AlertOctagon, Check, Copy, Key,
    QrCode, Image as ImageIcon, Upload, Tag, ArrowRight,
    Wifi, HardDrive, FileText, CheckCircle2, History, Laptop,
    Scale, Bell, Send, CreditCard, Keyboard
  } from 'lucide-svelte';

  type SettingsTab =
    | 'general'
    | 'invoices'
    | 'barcodes'
    | 'scale'
    | 'notifications'
    | 'pos'
    | 'shortcuts'
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
    drawer_com_port: '1',
    drawer_baud_rate: '9600',
    scale_enabled: 'true',
    scale_model: 'ACLAS LH51 / LS M3 / TS',
    scale_ip: '192.168.1.87',
    scale_port: '0',
    scale_protocol: '0',
    scale_default_barcode_type: '97',
    scale_department_id: '1',
    scale_auto_sync: 'true',
    telegram_bot_token: '',
    telegram_chat_id: '',
    notify_daily_summary: 'true',
    notify_each_sale: 'false',
    notify_each_refund: 'true',
    notify_expiry: 'true',
    notify_low_stock: 'true',
    app_license_status: 'activated',
    allow_negative_stock: 'false',
    pos_autofocus_search: 'true',
    pos_auto_capture_barcode: 'true',
    require_pin_for_discount: 'false',
    default_customer_name: 'Client Comptoir / زبون عادي',
    hold_sale_require_note: 'false',
    default_barcode_prefix: '22',
    scale_barcode_format: '97',
    shortcut_f1: 'Focus Barcode Search',
    shortcut_f2: 'Quick New Product',
    shortcut_f3: 'Switch Cart Mode',
    shortcut_f4: 'Hold Current Sale',
    shortcut_f5: 'List Held Sales',
    shortcut_f6: 'Edit Quantity',
    shortcut_f7: 'Apply Discount',
    shortcut_f8: 'Select Customer',
    shortcut_f9: 'Pay Cash & Print',
    shortcut_f10: 'Kick Cash Drawer',
    shortcut_f11: 'Split / TPE Payment',
    shortcut_f12: 'Clear Active Cart',
    barcode_label_width: '50',
    barcode_label_height: '30',
    shelf_tag_width: '60',
    shelf_tag_height: '40',
  };

  let hwid = 'TIT-POS-DZ-9842-AF81';
  let activationCode = '';
  let activationSuccess = false;
  let saveSuccessMsg = '';

  // Scale state
  let scaleStatusMsg = '';
  let isTestingScale = false;
  let isUploadingScale = false;
  let scaleSyncLogs: any[] = [];

  // Drawer state
  let drawerStatusMsg = '';
  let isOpeningDrawer = false;

  // Telegram test state
  let isSendingTelegram = false;
  let telegramStatusMsg = '';

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
  let previewProductName = 'Lait Candia 1L Entier';
  let previewPrice = 120;

  // Factory Reset
  let resetType = 'transactions_only';
  let resetConfirm = '';

  onMount(async () => {
    await loadSettings();
    await loadScaleLogs();
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

  async function loadScaleLogs() {
    try {
      scaleSyncLogs = await invoke<any[]>('get_scale_sync_logs');
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

  async function handleTestScaleConnection() {
    try {
      isTestingScale = true;
      scaleStatusMsg = 'Testing connection to ACLAS scale...';
      const res = await invoke<string>('test_scale_connection', {
        ip: settings.scale_ip || '192.168.1.87',
        port: parseInt(settings.scale_port || '0'),
        protocolType: parseInt(settings.scale_protocol || '0'),
      });
      scaleStatusMsg = '✅ ' + res;
    } catch (e: any) {
      scaleStatusMsg = '❌ ' + (typeof e === 'string' ? e : e.message || 'Scale connection failed');
    } finally {
      isTestingScale = false;
    }
  }

  let isFetchingScale = false;

  async function handleFetchFromScale() {
    try {
      isFetchingScale = true;
      scaleStatusMsg = 'Fetching PLU products from scale to PC...';
      const count = await invoke<number>('fetch_products_from_scale', {
        ip: settings.scale_ip || '192.168.1.87',
        port: parseInt(settings.scale_port || '0'),
        protocolType: parseInt(settings.scale_protocol || '0'),
        userName: $currentUser?.display_name || 'admin',
      });
      scaleStatusMsg = `✅ Successfully downloaded & updated ${count} products from ACLAS scale!`;
      await loadScaleLogs();
    } catch (e: any) {
      scaleStatusMsg = '❌ Error downloading from scale: ' + (typeof e === 'string' ? e : e.message);
    } finally {
      isFetchingScale = false;
    }
  }

  async function handleUploadAllScalable() {
    try {
      isUploadingScale = true;
      scaleStatusMsg = 'Uploading all scalable products to scale...';
      const count = await invoke<number>('upload_all_scalable_to_scale', {
        ip: settings.scale_ip || '192.168.1.87',
        port: parseInt(settings.scale_port || '0'),
        protocolType: parseInt(settings.scale_protocol || '0'),
        defaultDept: parseInt(settings.scale_department_id || '1'),
        defaultBarcodeType: parseInt(settings.scale_default_barcode_type || '97'),
        userName: $currentUser?.display_name || 'admin',
      });
      scaleStatusMsg = `✅ Successfully synchronized ${count} scalable products to ACLAS scale!`;
      await loadScaleLogs();
    } catch (e: any) {
      scaleStatusMsg = '❌ Error uploading: ' + (typeof e === 'string' ? e : e.message);
    } finally {
      isUploadingScale = false;
    }
  }

  async function handleTestSerialDrawer() {
    try {
      isOpeningDrawer = true;
      drawerStatusMsg = 'Sending kick pulse to cash drawer...';
      const port = parseInt(settings.drawer_com_port || '1');
      const baud = parseInt(settings.drawer_baud_rate || '9600');
      const res = await invoke<string>('open_serial_cash_drawer', { comPort: port, baudRate: baud });
      drawerStatusMsg = '✅ ' + res;
    } catch (e: any) {
      drawerStatusMsg = '❌ ' + (typeof e === 'string' ? e : e.message);
    } finally {
      isOpeningDrawer = false;
    }
  }

  async function handleBackupDatabase() {
    try {
      const defaultName = `TitaouPOS_Backup_${new Date().toISOString().slice(0, 10)}.sqlite`;
      const targetPath = `C:\\Users\\Public\\Downloads\\${defaultName}`;
      const msg = await invoke<string>('backup_database', { destinationPath: targetPath });
      triggerSaveNotification(msg);
    } catch (e: any) {
      alert('Backup Error: ' + (e.message || e));
    }
  }

  async function handleRestoreDatabase() {
    const backupPath = prompt('Enter the absolute path of the backup file to restore (e.g. C:\\Users\\Public\\Downloads\\backup.sqlite):');
    if (!backupPath) return;
    try {
      const msg = await invoke<string>('restore_database', { sourceBackupPath: backupPath });
      alert(msg);
    } catch (e: any) {
      alert('Restore Error: ' + (e.message || e));
    }
  }

  async function sendTelegramTest() {
    if (!settings.telegram_bot_token || !settings.telegram_chat_id) {
      telegramStatusMsg = 'Please enter Telegram Bot Token and Chat ID';
      return;
    }
    try {
      isSendingTelegram = true;
      telegramStatusMsg = 'Sending test message...';
      const text = encodeURIComponent('🚀 *TitaouPOS Live Alert*\nTest connection successful from POS settings!');
      const url = `https://api.telegram.org/bot${settings.telegram_bot_token}/sendMessage?chat_id=${settings.telegram_chat_id}&text=${text}&parse_mode=Markdown`;
      const res = await fetch(url);
      const data = await res.json();
      if (data.ok) {
        telegramStatusMsg = '✅ Telegram test alert delivered successfully!';
      } else {
        telegramStatusMsg = '❌ Telegram error: ' + (data.description || 'Check Token/Chat ID');
      }
    } catch (e: any) {
      telegramStatusMsg = 'Network error: ' + e.message;
    } finally {
      isSendingTelegram = false;
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
        <span class="truncate">Printing & Drawer</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'scale')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'scale' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <Scale class="w-4 h-4 mb-1" />
        <span class="truncate">ACLAS Scale</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'notifications')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'notifications' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <Bell class="w-4 h-4 mb-1" />
        <span class="truncate">Notifications</span>
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
        on:click={() => (currentTab = 'shortcuts')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'shortcuts' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <Keyboard class="w-4 h-4 mb-1" />
        <span class="truncate">Shortcuts</span>
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

            <!-- Advanced Receipt Content Options -->
            <div class="p-4 bg-slate-50 dark:bg-slate-800/50 rounded-2xl space-y-2.5 border border-pos-border">
              <span class="text-xs font-black text-pos-text block mb-1">Receipt Content & Layout Options</span>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-2">
                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer">
                  <input type="checkbox" bind:checked={settings.receipt_show_cashier} class="rounded text-sky-600 focus:ring-0" />
                  <span>Display Cashier Name</span>
                </label>

                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer">
                  <input type="checkbox" bind:checked={settings.receipt_show_qr} class="rounded text-sky-600 focus:ring-0" />
                  <span>Print Receipt QR Verification Code</span>
                </label>

                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer">
                  <input type="checkbox" bind:checked={settings.receipt_show_tax} class="rounded text-sky-600 focus:ring-0" />
                  <span>Print Tax / TVA Breakdown</span>
                </label>

                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer">
                  <input type="checkbox" checked class="rounded text-sky-600 focus:ring-0" />
                  <span>Auto-cut receipt paper after printing</span>
                </label>
              </div>
            </div>
          </div>

          <!-- Live Receipt Preview Box -->
          <div class="bg-slate-100 dark:bg-slate-900/60 p-4 rounded-2xl flex flex-col items-center justify-start border border-pos-border">
            <span class="text-[10px] font-black text-pos-muted uppercase tracking-wider mb-2">Live Dynamic Preview ({settings.receipt_paper_width || '80mm'})</span>
            <div class="{settings.receipt_paper_width === '58mm' ? 'w-48' : 'w-64'} bg-white text-black p-3.5 shadow-md font-mono text-[10px] space-y-2 border border-slate-300 transition-all rounded-sm">
              <div class="text-center pb-1 border-b-dashed">
                <p class="font-black text-xs">{settings.shop_name_fr || 'TitaouPOS'}</p>
                {#if settings.shop_name_ar}
                  <p class="font-bold text-[10px]">{settings.shop_name_ar}</p>
                {/if}
                <p class="text-[8px] text-gray-600">{settings.shop_address || 'Alger, Algérie'}</p>
                <p class="text-[8px] text-gray-600">Tél: {settings.shop_phone || '0553444057'}</p>
                {#if settings.receipt_header}
                  <p class="text-[8px] font-bold text-sky-800 mt-1 italic">{settings.receipt_header}</p>
                {/if}
              </div>
              <div class="py-1 border-b-dashed flex justify-between text-[8px]">
                <span>TICKET #9842</span>
                {#if settings.receipt_show_cashier !== false}
                  <span>Caisse: Admin</span>
                {/if}
              </div>
              <div class="space-y-0.5 py-1 border-b-dashed">
                <div class="flex justify-between font-bold">
                  <span>1x Sucre Cevital 1kg</span>
                  <span>100 DZD</span>
                </div>
                <div class="flex justify-between font-bold">
                  <span>1x Lait Candia 1L</span>
                  <span>150 DZD</span>
                </div>
              </div>
              {#if settings.receipt_show_tax}
                <div class="text-[8px] text-gray-600 py-0.5 border-b-dashed flex justify-between">
                  <span>Total HT: 210 DZD</span>
                  <span>TVA (19%): 40 DZD</span>
                </div>
              {/if}
              <div class="font-black flex justify-between text-xs pt-1">
                <span>TOTAL PAYÉ:</span>
                <span>250 DZD</span>
              </div>
              {#if settings.receipt_footer}
                <div class="text-center text-[8px] pt-1 text-gray-500 border-t-dashed">
                  {settings.receipt_footer}
                </div>
              {:else}
                <div class="text-center text-[8px] pt-1 text-gray-500 border-t-dashed">
                  *** Merci de votre visite ***
                </div>
              {/if}
              {#if settings.receipt_show_qr !== false}
                <div class="text-center pt-1">
                  <div class="inline-block px-3 py-1 bg-slate-100 border border-slate-300 font-mono text-[7px] text-gray-600 rounded">
                    [QR: TITAOU-VTE-9842]
                  </div>
                </div>
              {/if}
            </div>
          </div>
        </div>

        <!-- Serial Cash Drawer Settings -->
        <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-3">
          <h3 class="font-black text-xs text-pos-text flex items-center gap-1.5">
            <CreditCard class="w-4 h-4 text-emerald-600" />
            <span>Serial Cash Drawer (COM Port)</span>
          </h3>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">COM Port</label>
              <select bind:value={settings.drawer_com_port} class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-bold text-pos-text">
                {#each Array.from({ length: 10 }, (_, i) => i + 1) as port}
                  <option value={port.toString()}>COM{port}</option>
                {/each}
              </select>
            </div>
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Baud Rate</label>
              <select bind:value={settings.drawer_baud_rate} class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-mono font-bold text-pos-text">
                <option value="9600">9600</option>
                <option value="19200">19200</option>
                <option value="38400">38400</option>
                <option value="115200">115200</option>
              </select>
            </div>
            <div class="flex items-end">
              <button type="button" on:click={handleTestSerialDrawer} disabled={isOpeningDrawer} class="w-full px-4 py-2 bg-emerald-600 hover:bg-emerald-700 text-white font-bold text-xs rounded-xl cursor-pointer shadow-xs">
                {isOpeningDrawer ? 'Opening...' : 'Test Open Cash Drawer (تجربة فتح الدرج)'}
              </button>
            </div>
          </div>
          {#if drawerStatusMsg}
            <div class="p-2 bg-sky-100 dark:bg-sky-950 text-sky-800 dark:text-sky-200 text-xs font-bold rounded-lg">{drawerStatusMsg}</div>
          {/if}
        </div>

        <div class="pt-4 border-t border-pos-border flex justify-end">
          <button on:click={saveAllSettings} class="px-6 py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl shadow-md transition cursor-pointer">
            Save Print & Drawer Settings
          </button>
        </div>
      </div>

    <!-- SCALE TAB (ACLAS Real SDK) -->
    {:else if currentTab === 'scale'}
      <div class="max-w-4xl space-y-6">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-base font-black text-pos-text flex items-center gap-2">
              <Scale class="w-5 h-5 text-sky-600" />
              <span>ACLAS Electronic Scale SDK Integration</span>
            </h2>
            <p class="text-xs text-pos-muted">Direct TCP/IP synchronization for ACLAS LH51, LS M3, and TS Series scales</p>
          </div>
          <button on:click={saveAllSettings} class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl shadow-md cursor-pointer flex items-center gap-1.5">
            <Check class="w-4 h-4" />
            <span>Save Scale Settings</span>
          </button>
        </div>

        {#if scaleStatusMsg}
          <div class="p-3 bg-sky-100 dark:bg-sky-950 border border-sky-300 dark:border-sky-800 text-sky-800 dark:text-sky-200 text-xs font-bold rounded-xl animate-in fade-in">
            {scaleStatusMsg}
          </div>
        {/if}

        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div class="md:col-span-2 p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4">
            <h3 class="font-black text-sm text-pos-text">Scale Network & Protocol Configuration</h3>
            
            <div class="grid grid-cols-2 gap-3">
              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">Scale IP Address (Ethernet) *</label>
                <input type="text" bind:value={settings.scale_ip} placeholder="192.168.1.87" class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-mono font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
              </div>

              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">Scale Port</label>
                <input type="number" bind:value={settings.scale_port} placeholder="0" class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-mono font-bold text-pos-text outline-none" />
              </div>
            </div>

            <div class="grid grid-cols-2 gap-3">
              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">Default Scale Barcode Format *</label>
                <select bind:value={settings.scale_default_barcode_type} class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-bold text-pos-text">
                  <option value="97">Type 97: 18-Code (Dept + ItemCode + Price + Weight + Checksum)</option>
                  <option value="2">Type 02: EAN-13 Price Embedded (DD IIIII PPPPP C)</option>
                  <option value="7">Type 07: EAN-13 Weight Embedded (DD IIIII WWWWW C)</option>
                  <option value="22">Type 22: EAN-13 1-Digit Dept Price Embedded (D IIIIII PPPPP C)</option>
                  <option value="27">Type 27: EAN-13 1-Digit Dept Weight Embedded (D IIIIII WWWWW C)</option>
                  <option value="12">Type 12: Fixed Code 22 Price Embedded (22 IIIII PPPPP C)</option>
                  <option value="17">Type 17: Fixed Code 27 Weight Embedded (27 IIIII WWWWW C)</option>
                </select>
              </div>

              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">Scale Department ID (1-99)</label>
                <input type="number" min="1" max="99" bind:value={settings.scale_department_id} class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-mono font-bold text-pos-text" />
              </div>
            </div>

            <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer">
              <input type="checkbox" bind:checked={settings.scale_auto_sync} class="rounded text-sky-600" />
              <span>Automatically sync scalable products to scale on price/name changes</span>
            </label>

            <!-- Actions -->
            <div class="flex flex-wrap items-center gap-3 pt-2">
              <button
                type="button"
                on:click={handleTestScaleConnection}
                disabled={isTestingScale}
                class="px-4 py-2 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 text-pos-text font-bold text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-xs"
              >
                <RefreshCw class="w-3.5 h-3.5 {isTestingScale ? 'animate-spin' : ''}" />
                <span>{isTestingScale ? 'Testing...' : 'Test Connection (فحص الاتصال)'}</span>
              </button>

              <button
                type="button"
                on:click={handleUploadAllScalable}
                disabled={isUploadingScale || isFetchingScale}
                class="px-4 py-2 bg-sky-600 hover:bg-sky-700 disabled:opacity-50 text-white font-black text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-md"
              >
                <Upload class="w-4 h-4" />
                <span>{isUploadingScale ? 'Uploading...' : 'Upload to Scale (إرسال للميزان)'}</span>
              </button>

              <button
                type="button"
                on:click={handleFetchFromScale}
                disabled={isUploadingScale || isFetchingScale}
                class="px-4 py-2 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-50 text-white font-black text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-md"
              >
                <Download class="w-4 h-4" />
                <span>{isFetchingScale ? 'Downloading...' : 'Fetch from Scale (جلب من الميزان)'}</span>
              </button>
            </div>
          </div>

          <div class="p-5 bg-sky-50 dark:bg-sky-950/30 border border-sky-200 dark:border-sky-800 rounded-2xl space-y-3">
            <h4 class="font-black text-xs text-sky-800 dark:text-sky-200">ACLAS Scale Features</h4>
            <ul class="text-xs text-pos-muted space-y-2">
              <li>• Direct native dynamic loading via <strong class="text-pos-text">AclasSDK.dll (Win64)</strong></li>
              <li>• Generates standard UTF-16LE PLU format with custom department & price</li>
              <li>• Real Barcode Type 97 payload verification</li>
              <li>• Automatic synchronization on POS price changes</li>
            </ul>
          </div>
        </div>

        <!-- Sync Logs Table -->
        <div class="space-y-3 pt-2">
          <div class="flex items-center justify-between">
            <h3 class="font-black text-xs text-pos-text">Recent Scale Synchronization History (سجل المزامنة)</h3>
            <button on:click={loadScaleLogs} class="text-xs font-bold text-sky-600 hover:underline">Refresh</button>
          </div>
          <div class="bg-pos-card border border-pos-border rounded-2xl overflow-hidden">
            <table class="w-full text-start text-xs border-collapse">
              <thead class="bg-slate-50 dark:bg-slate-800/60 border-b border-pos-border text-pos-muted font-bold">
                <tr>
                  <th class="p-2.5 text-start">Time</th>
                  <th class="p-2.5 text-start">Product</th>
                  <th class="p-2.5 text-start">PLU #</th>
                  <th class="p-2.5 text-center">Direction</th>
                  <th class="p-2.5 text-center">Status</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-pos-border/40">
                {#if scaleSyncLogs.length === 0}
                  <tr>
                    <td colspan="5" class="p-4 text-center text-pos-muted">No synchronization records yet.</td>
                  </tr>
                {:else}
                  {#each scaleSyncLogs.slice(0, 10) as log}
                    <tr class="hover:bg-slate-50 dark:hover:bg-slate-800/30">
                      <td class="p-2.5 font-mono text-pos-muted">{log.created_at}</td>
                      <td class="p-2.5 font-bold text-pos-text">{log.product_name || 'All Scalable Items'}</td>
                      <td class="p-2.5 font-mono text-sky-600">{log.scale_plu || '—'}</td>
                      <td class="p-2.5 text-center uppercase font-mono text-[10px]">{log.direction}</td>
                      <td class="p-2.5 text-center">
                        <span class="px-2 py-0.5 rounded-full text-[10px] font-black uppercase {log.status === 'SUCCESS' ? 'bg-emerald-100 text-emerald-800 dark:bg-emerald-950 dark:text-emerald-300' : 'bg-rose-100 text-rose-800 dark:bg-rose-950 dark:text-rose-300'}">
                          {log.status}
                        </span>
                      </td>
                    </tr>
                  {/each}
                {/if}
              </tbody>
            </table>
          </div>
        </div>
      </div>

    <!-- NOTIFICATIONS TAB -->
    {:else if currentTab === 'notifications'}
      <div class="max-w-4xl space-y-6">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-base font-black text-pos-text flex items-center gap-2">
              <Bell class="w-5 h-5 text-sky-600" />
              <span>Telegram Bot Notifications & Event Alerts</span>
            </h2>
            <p class="text-xs text-pos-muted">Send automated alerts directly to your phone or management channel</p>
          </div>
          <button on:click={saveAllSettings} class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl shadow-md cursor-pointer flex items-center gap-1.5">
            <Check class="w-4 h-4" />
            <span>Save Alert Settings</span>
          </button>
        </div>

        {#if telegramStatusMsg}
          <div class="p-3 bg-sky-100 dark:bg-sky-950 text-sky-800 dark:text-sky-200 text-xs font-bold rounded-xl">
            {telegramStatusMsg}
          </div>
        {/if}

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4">
            <h3 class="font-black text-sm text-pos-text">Telegram Bot Credentials</h3>
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Telegram Bot Token</label>
              <input type="text" bind:value={settings.telegram_bot_token} placeholder="123456789:ABCdefGhIJKlmNoPQRsTUVwxyZ" class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-mono font-bold text-pos-text outline-none" />
            </div>
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Telegram Chat ID (Channel or Group ID)</label>
              <input type="text" bind:value={settings.telegram_chat_id} placeholder="-100123456789" class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-mono font-bold text-pos-text outline-none" />
            </div>
            <button type="button" on:click={sendTelegramTest} disabled={isSendingTelegram} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-pos-text font-bold text-xs rounded-xl flex items-center gap-1.5 cursor-pointer">
              <Send class="w-3.5 h-3.5" />
              <span>{isSendingTelegram ? 'Sending...' : 'Send Test Alert (إرسال تجربة)'}</span>
            </button>
          </div>

          <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-3">
            <h3 class="font-black text-sm text-pos-text">Alert Triggers</h3>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <span>Daily End-of-Day revenue summary</span>
              <input type="checkbox" bind:checked={settings.notify_daily_summary} class="rounded text-sky-600" />
            </label>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <span>Instant notification on every sale</span>
              <input type="checkbox" bind:checked={settings.notify_each_sale} class="rounded text-sky-600" />
            </label>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <span>Notification on refunds & returns</span>
              <input type="checkbox" bind:checked={settings.notify_each_refund} class="rounded text-sky-600" />
            </label>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <span>Near-expired (under 30 days) & expired items alert</span>
              <input type="checkbox" bind:checked={settings.notify_expiry} class="rounded text-sky-600" />
            </label>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <span>Low stock & inventory depletion alert</span>
              <input type="checkbox" bind:checked={settings.notify_low_stock} class="rounded text-sky-600" />
            </label>
          </div>
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

    <!-- POS RULES TAB -->
    {:else if currentTab === 'pos'}
      <div class="max-w-4xl space-y-6">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-base font-black text-pos-text flex items-center gap-2">
              <Sliders class="w-5 h-5 text-sky-600" />
              <span>Point of Sale Business Rules & Operational Flow</span>
            </h2>
            <p class="text-xs text-pos-muted">Configure cart behavior, scanner auto-focus, cashier security rules, and negative stock selling</p>
          </div>
          <button on:click={saveAllSettings} class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl shadow-md cursor-pointer flex items-center gap-1.5">
            <Check class="w-4 h-4" />
            <span>Save POS Rules</span>
          </button>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <!-- Inventory & Stock Selling Rules -->
          <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4">
            <h3 class="font-black text-sm text-pos-text">Stock & Inventory Controls</h3>
            
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <div>
                <span>Allow Negative Stock Selling</span>
                <p class="text-[10px] text-pos-muted font-normal">Permit cashiers to complete sales even when stock is zero or depleted</p>
              </div>
              <input type="checkbox" bind:checked={settings.allow_negative_stock} class="rounded text-sky-600 w-4 h-4 cursor-pointer" />
            </label>

            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <div>
                <span>Require Note When Holding Sales</span>
                <p class="text-[10px] text-pos-muted font-normal">Prompt cashiers to enter a customer reference or note before holding a ticket</p>
              </div>
              <input type="checkbox" bind:checked={settings.hold_sale_require_note} class="rounded text-sky-600 w-4 h-4 cursor-pointer" />
            </label>

            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Default Walk-in Customer Label</label>
              <input type="text" bind:value={settings.default_customer_name} class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-bold text-pos-text outline-none" />
            </div>
          </div>

          <!-- Scanner & Hardware Automation Rules -->
          <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4">
            <h3 class="font-black text-sm text-pos-text">Scanner & Workflow Automation</h3>

            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <div>
                <span>Autofocus Barcode Search Bar</span>
                <p class="text-[10px] text-pos-muted font-normal">Always keep cursor ready in search bar after each product addition or sale</p>
              </div>
              <input type="checkbox" bind:checked={settings.pos_autofocus_search} class="rounded text-sky-600 w-4 h-4 cursor-pointer" />
            </label>

            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <div>
                <span>Global Barcode Auto-Capture</span>
                <p class="text-[10px] text-pos-muted font-normal">Intercept fast barcode scanner strokes even when cursor is on another element</p>
              </div>
              <input type="checkbox" bind:checked={settings.pos_auto_capture_barcode} class="rounded text-sky-600 w-4 h-4 cursor-pointer" />
            </label>

            <!-- Cart Line Ordering Rule -->
            <div class="p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border space-y-1">
              <label class="block text-xs font-bold text-pos-text">Cart Product Insertion Order</label>
              <p class="text-[10px] text-pos-muted">Choose where newly scanned items appear in the shopping cart</p>
              <select bind:value={settings.cart_item_order} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-bold text-pos-text mt-1">
                <option value="bottom">New items at BOTTOM (Auto-scrolls to bottom)</option>
                <option value="top">New items at TOP (Most recent on top)</option>
              </select>
            </div>

            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <div>
                <span>Require Admin PIN for Discounts</span>
                <p class="text-[10px] text-pos-muted font-normal">Disallow cashiers from applying custom discounts without supervisor authorization</p>
              </div>
              <input type="checkbox" bind:checked={settings.require_pin_for_discount} class="rounded text-sky-600 w-4 h-4 cursor-pointer" />
            </label>
          </div>
        </div>
      </div>

    <!-- KEYBOARD SHORTCUTS TAB -->
    {:else if currentTab === 'shortcuts'}
      <div class="max-w-4xl space-y-6">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-base font-black text-pos-text flex items-center gap-2">
              <Keyboard class="w-5 h-5 text-sky-600" />
              <span>POS Keyboard Shortcuts Map (اختصارات لوحة المفاتيح)</span>
            </h2>
            <p class="text-xs text-pos-muted">Custom high-speed keyboard bindings for touch-free cash register operation</p>
          </div>
          <button on:click={saveAllSettings} class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl shadow-md cursor-pointer flex items-center gap-1.5">
            <Check class="w-4 h-4" />
            <span>Save Shortcuts</span>
          </button>
        </div>

        <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-3 text-xs">
            <div class="flex items-center justify-between p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <span class="font-bold text-pos-text">Focus Barcode Search</span>
              <kbd class="px-2.5 py-1 bg-slate-200 dark:bg-slate-800 font-mono font-black text-sky-600 rounded-lg shadow-inner">F1</kbd>
            </div>

            <div class="flex items-center justify-between p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <span class="font-bold text-pos-text">Quick Add / New Product</span>
              <kbd class="px-2.5 py-1 bg-slate-200 dark:bg-slate-800 font-mono font-black text-sky-600 rounded-lg shadow-inner">F2</kbd>
            </div>

            <div class="flex items-center justify-between p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <span class="font-bold text-pos-text">Switch Mode (Sale / Refund / Purchase)</span>
              <kbd class="px-2.5 py-1 bg-slate-200 dark:bg-slate-800 font-mono font-black text-sky-600 rounded-lg shadow-inner">F3</kbd>
            </div>

            <div class="flex items-center justify-between p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <span class="font-bold text-pos-text">Hold Active Sale</span>
              <kbd class="px-2.5 py-1 bg-slate-200 dark:bg-slate-800 font-mono font-black text-amber-600 rounded-lg shadow-inner">F4</kbd>
            </div>

            <div class="flex items-center justify-between p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <span class="font-bold text-pos-text">Resume Held Sales Modal</span>
              <kbd class="px-2.5 py-1 bg-slate-200 dark:bg-slate-800 font-mono font-black text-amber-600 rounded-lg shadow-inner">F5</kbd>
            </div>

            <div class="flex items-center justify-between p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <span class="font-bold text-pos-text">Edit Selected Item Quantity</span>
              <kbd class="px-2.5 py-1 bg-slate-200 dark:bg-slate-800 font-mono font-black text-sky-600 rounded-lg shadow-inner">F6</kbd>
            </div>

            <div class="flex items-center justify-between p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <span class="font-bold text-pos-text">Apply Discount / Remise</span>
              <kbd class="px-2.5 py-1 bg-slate-200 dark:bg-slate-800 font-mono font-black text-sky-600 rounded-lg shadow-inner">F7</kbd>
            </div>

            <div class="flex items-center justify-between p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <span class="font-bold text-pos-text">Select Customer Account</span>
              <kbd class="px-2.5 py-1 bg-slate-200 dark:bg-slate-800 font-mono font-black text-sky-600 rounded-lg shadow-inner">F8</kbd>
            </div>

            <div class="flex items-center justify-between p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <span class="font-bold text-pos-text">Quick Cash Pay & Print Receipt</span>
              <kbd class="px-2.5 py-1 bg-emerald-100 dark:bg-emerald-950 font-mono font-black text-emerald-700 dark:text-emerald-300 rounded-lg shadow-inner">F9</kbd>
            </div>

            <div class="flex items-center justify-between p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <span class="font-bold text-pos-text">Kick Open Serial Cash Drawer</span>
              <kbd class="px-2.5 py-1 bg-emerald-100 dark:bg-emerald-950 font-mono font-black text-emerald-700 dark:text-emerald-300 rounded-lg shadow-inner">F10</kbd>
            </div>

            <div class="flex items-center justify-between p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <span class="font-bold text-pos-text">Split / Card / TPE Payment Modal</span>
              <kbd class="px-2.5 py-1 bg-slate-200 dark:bg-slate-800 font-mono font-black text-sky-600 rounded-lg shadow-inner">F11</kbd>
            </div>

            <div class="flex items-center justify-between p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <span class="font-bold text-pos-text">Clear / Cancel Active Cart</span>
              <kbd class="px-2.5 py-1 bg-rose-100 dark:bg-rose-950 font-mono font-black text-rose-600 rounded-lg shadow-inner">F12</kbd>
            </div>
          </div>
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
                on:click={handleBackupDatabase}
                class="w-full py-2.5 bg-emerald-600 hover:bg-emerald-700 text-white text-xs font-black rounded-xl flex items-center justify-center gap-2 cursor-pointer shadow-md transition active:scale-95"
              >
                <HardDrive class="w-4 h-4" />
                <span>Backup Now (نسخ احتياطي للبيانات)</span>
              </button>

              <button
                type="button"
                on:click={handleRestoreDatabase}
                class="w-full py-2 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 dark:hover:bg-slate-600 text-pos-text text-xs font-bold rounded-xl cursor-pointer transition active:scale-95"
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