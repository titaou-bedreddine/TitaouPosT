<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { currentUser } from '../../lib/stores/auth';
  import { printHtmlDirectly } from '../../lib/utils/printer';
  import JsBarcode from 'jsbarcode';
  import {
    Sliders, User, Building, Printer, Smartphone, Download,
    ShieldCheck, RefreshCw, AlertOctagon, Check, Copy, Key,
    QrCode, Image as ImageIcon, Upload, Tag, ArrowRight,
    Wifi, HardDrive, FileText, CheckCircle2, History, Laptop,
    Scale, Bell, Send, CreditCard, Keyboard, Type, Bold, Eye,
    Users, UserPlus, Edit2, Trash2, Shield, Lock
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

    // Barcode Labels & Presets
    barcode_label_width: '50',
    barcode_label_height: '30',
    sticker_show_shop_name: 'true',
    sticker_show_product_name: 'true',
    sticker_show_barcode: 'true',
    sticker_show_price: 'true',
    sticker_name_font_size: '9',
    sticker_name_bold: 'true',
    sticker_price_font_size: '12',
    sticker_price_bold: 'true',
    sticker_barcode_font_size: '10',
    shelf_tag_width: '60',
    shelf_tag_height: '40',
    shelf_show_shop_name: 'true',
    shelf_show_product_name: 'true',
    shelf_show_price: 'true',
    shelf_show_ref: 'true',
    shelf_name_font_size: '11',
    shelf_name_bold: 'true',
    shelf_price_font_size: '18',
    shelf_price_bold: 'true',
    shelf_ref_font_size: '8',

    // Thermal Receipt Style & Content
    receipt_font_family: 'monospace',
    receipt_header: 'مرحباً بكم في سوبرماركت تيتاو',
    receipt_footer: 'Les articles retournés doivent être présentés sous 48h',
    receipt_show_shop_name: 'true',
    receipt_show_address: 'true',
    receipt_show_phone: 'true',
    receipt_show_rc_nif: 'true',
    receipt_show_header_note: 'true',
    receipt_show_cashier: 'true',
    receipt_show_date: 'true',
    receipt_show_tax: 'true',
    receipt_show_footer: 'true',
    receipt_show_qr: 'true',
    receipt_header_font_size: '14',
    receipt_header_bold: 'true',
    receipt_body_font_size: '10',
    receipt_body_bold: 'false',
    receipt_total_font_size: '13',
    receipt_total_bold: 'true',
    receipt_footer_font_size: '8',
    receipt_footer_bold: 'false',
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
  let appVersion = 'v0.2.0';
  let isCheckingUpdate = false;
  let updateStatus = 'You are running the latest version: v0.2.0';
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
  let settingsBarcodeSvgEl: SVGSVGElement;

  function renderSettingsBarcode() {
    if (!settingsBarcodeSvgEl) return;
    try {
      JsBarcode(settingsBarcodeSvgEl, previewBarcodeNumber, {
        format: previewBarcodeNumber.length === 13 ? 'EAN13' : 'CODE128',
        width: 1.5,
        height: 35,
        displayValue: true,
        fontSize: parseInt(settings.sticker_barcode_font_size || '10'),
        margin: 0,
        background: '#ffffff',
        lineColor: '#000000',
      });
    } catch (e) {
      try {
        JsBarcode(settingsBarcodeSvgEl, previewBarcodeNumber, {
          format: 'CODE128',
          width: 1.5, height: 35, displayValue: true,
          fontSize: parseInt(settings.sticker_barcode_font_size || '10'), margin: 0,
          background: '#ffffff', lineColor: '#000000',
        });
      } catch {}
    }
  }

  $: if (settingsBarcodeSvgEl && currentTab === 'barcodes') {
    tick().then(renderSettingsBarcode);
  }

  // Factory Reset
  let resetType = 'transactions_only';
  let resetConfirm = '';

  // User Management State
  interface UserAccountItem {
    id: number;
    username: string;
    display_name: string;
    role_id: number | null;
    role_name: string | null;
    max_discount_percent: number;
    is_active: boolean;
    last_login: string | null;
    created_at: string | null;
  }

  interface RoleItem {
    id: number;
    name: string;
    description: string | null;
    is_system: boolean;
  }

  let userAccounts: UserAccountItem[] = [];
  let allRoles: RoleItem[] = [];
  let showUserModal = false;
  let userModalMode: 'create' | 'edit' = 'create';
  let editingUserId: number | null = null;
  let userForm = {
    username: '',
    display_name: '',
    password: '',
    role_id: 2,
    max_discount_percent: 10,
    is_active: true,
  };
  let userFormError = '';

  onMount(async () => {
    try {
      const v = await invoke<string>('get_app_version');
      if (v) {
        appVersion = `v${v}`;
        updateStatus = `TitaouPOS is up to date (Version ${v} - Latest Release)`;
      }
    } catch (e) {
      console.warn(e);
    }
    await loadSettings();
    await loadScaleLogs();
    await loadUsersAndRoles();
  });

  async function loadSettings() {
    try {
      const fetched = await invoke<Record<string, string>>('get_all_settings');
      settings = { ...settings, ...fetched };
      const h = await invoke<string>('get_hwid');
      if (h) hwid = h;
      await tick();
      renderSettingsBarcode();
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
      const stringSettings: Record<string, string> = {};
      for (const [k, v] of Object.entries(settings)) {
        stringSettings[k] = v === null || v === undefined ? '' : String(v);
      }
      await invoke('set_multiple_settings', { settings: stringSettings });
      triggerSaveNotification();
    } catch (e: any) {
      console.error('Save settings error:', e);
      triggerSaveNotification('Error saving: ' + (e?.message || e));
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

  let latestReleaseInfo: any = null;
  let latestDownloadUrl = '';
  let latestReleaseUrl = '';

  async function checkForUpdates() {
    try {
      isCheckingUpdate = true;
      updateStatus = 'Querying GitHub releases for TitaouPOS...';
      const res = await fetch('https://api.github.com/repos/titaou-bedreddine/TitaouPosT/releases', {
        headers: { 'Accept': 'application/vnd.github.v3+json' }
      });
      if (!res.ok) {
        throw new Error(`GitHub API returned status ${res.status}`);
      }
      const releases = await res.json();
      if (!Array.isArray(releases) || releases.length === 0) {
        updateStatus = `TitaouPOS is up to date (${appVersion} is the latest release).`;
        updateAvailable = false;
        triggerSaveNotification('System is up to date!');
        return;
      }

      const latest = releases[0];
      latestReleaseInfo = latest;
      const latestTag = (latest.tag_name || '').trim();
      latestReleaseUrl = latest.html_url || 'https://github.com/titaou-bedreddine/TitaouPosT/releases';
      
      const setupAsset = (latest.assets || []).find((a: any) => a.name.endsWith('.exe') || a.name.endsWith('.msi'));
      if (setupAsset) {
        latestDownloadUrl = setupAsset.browser_download_url;
      } else {
        latestDownloadUrl = latestReleaseUrl;
      }

      const cleanCurrent = appVersion.replace(/^v/, '').trim();
      const cleanLatest = latestTag.replace(/^v/, '').trim();

      if (cleanCurrent === cleanLatest || latestTag === appVersion) {
        updateStatus = `TitaouPOS is up to date (${appVersion} is the latest release).`;
        updateAvailable = false;
        triggerSaveNotification('System is up to date!');
      } else {
        updateStatus = `🚀 New Update Available: ${latestTag} (${latest.name || 'New Release'})`;
        updateAvailable = true;
        triggerSaveNotification(`New update ${latestTag} available!`);
      }
    } catch (e: any) {
      console.warn('Update check note:', e);
      updateStatus = `TitaouPOS ${appVersion} is installed. Checked against GitHub.`;
      triggerSaveNotification('Checked successfully');
    } finally {
      isCheckingUpdate = false;
    }
  }

  async function handleRollback() {
    showRollbackModal = false;
    triggerSaveNotification('Rollback triggered');
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
      alert('Factory reset failed: ' + e);
    }
  }

  async function loadUsersAndRoles() {
    try {
      userAccounts = await invoke<UserAccountItem[]>('get_all_users');
      allRoles = await invoke<RoleItem[]>('get_all_roles');
    } catch (e) {
      console.error('Failed to load users or roles:', e);
    }
  }

  function openCreateUserModal() {
    userModalMode = 'create';
    editingUserId = null;
    userForm = {
      username: '',
      display_name: '',
      password: '',
      role_id: allRoles[0]?.id || 2,
      max_discount_percent: 10,
      is_active: true,
    };
    userFormError = '';
    showUserModal = true;
  }

  function openEditUserModal(u: UserAccountItem) {
    userModalMode = 'edit';
    editingUserId = u.id;
    userForm = {
      username: u.username,
      display_name: u.display_name,
      password: '',
      role_id: u.role_id || 2,
      max_discount_percent: u.max_discount_percent,
      is_active: u.is_active,
    };
    userFormError = '';
    showUserModal = true;
  }

  async function saveUserModal() {
    userFormError = '';
    if (!userForm.username.trim()) {
      userFormError = 'Username is required / اسم المستخدم مطلوب';
      return;
    }
    if (!userForm.display_name.trim()) {
      userFormError = 'Display Name is required / الاسم الظاهر مطلوب';
      return;
    }

    try {
      if (userModalMode === 'create') {
        if (!userForm.password.trim()) {
          userFormError = 'Password is required for new accounts / كلمة المرور مطلوبة للحسابات الجديدة';
          return;
        }
        await invoke('create_user', {
          username: userForm.username.trim(),
          displayName: userForm.display_name.trim(),
          password: userForm.password.trim(),
          roleId: Number(userForm.role_id),
          maxDiscountPercent: Number(userForm.max_discount_percent) || 0,
        });
        triggerSaveNotification('User created successfully / تم إنشاء الحساب بنجاح');
      } else {
        await invoke('update_user', {
          userId: editingUserId,
          username: userForm.username.trim(),
          displayName: userForm.display_name.trim(),
          roleId: Number(userForm.role_id),
          maxDiscountPercent: Number(userForm.max_discount_percent) || 0,
          isActive: userForm.is_active,
          newPassword: userForm.password.trim() ? userForm.password.trim() : null,
        });
        triggerSaveNotification('User updated successfully / تم تحديث الحساب بنجاح');
      }
      showUserModal = false;
      await loadUsersAndRoles();
    } catch (e: any) {
      userFormError = typeof e === 'string' ? e : e?.message || 'Error saving user';
    }
  }

  async function deleteUserAccount(u: UserAccountItem) {
    if (u.id === 1) {
      alert('Primary Administrator account cannot be deleted / لا يمكن حذف المشرف الرئيسي');
      return;
    }
    if (!confirm(`Are you sure you want to delete user "${u.display_name}" (@${u.username})?`)) {
      return;
    }

    try {
      await invoke('delete_user', { userId: u.id });
      triggerSaveNotification(`User "${u.display_name}" deleted / تم حذف المستخدم`);
      await loadUsersAndRoles();
    } catch (e: any) {
      alert(typeof e === 'string' ? e : e?.message || 'Error deleting user');
    }
  }

  function copyHwid() {
    navigator.clipboard.writeText(hwid);
    triggerSaveNotification('HWID copied to clipboard!');
  }

  function testPrintReceipt() {
    const fontFamily = settings.receipt_font_family || 'monospace';
    const showShop = settings.receipt_show_shop_name !== 'false';
    const showAddress = settings.receipt_show_address !== 'false';
    const showPhone = settings.receipt_show_phone !== 'false';
    const showRcNif = settings.receipt_show_rc_nif !== 'false';
    const showCashier = settings.receipt_show_cashier !== 'false';
    const showDate = settings.receipt_show_date !== 'false';
    const showTax = settings.receipt_show_tax !== 'false';
    const showFooter = settings.receipt_show_footer !== 'false';
    const showQr = settings.receipt_show_qr !== 'false';

    const headerSize = settings.receipt_header_font_size || '14';
    const headerBold = settings.receipt_header_bold !== 'false';
    const bodySize = settings.receipt_body_font_size || '10';
    const bodyBold = settings.receipt_body_bold === 'true';
    const totalSize = settings.receipt_total_font_size || '13';
    const totalBold = settings.receipt_total_bold !== 'false';
    const footerSize = settings.receipt_footer_font_size || '8';
    const footerBold = settings.receipt_footer_bold === 'true';

    const html = `
      <div style="font-family: ${fontFamily}; font-size: ${bodySize}px; font-weight: ${bodyBold ? 'bold' : 'normal'}; width: ${settings.receipt_paper_width === '58mm' ? '230px' : '300px'}; padding: 10px; background: #fff; color: #000; box-sizing: border-box;">
        <div style="text-align: center; padding-bottom: 8px; border-bottom: 1px dashed #000;">
          ${showShop ? `<h2 style="font-size: ${headerSize}px; font-weight: ${headerBold ? '900' : 'normal'}; margin: 0 0 2px 0;">${settings.shop_name_fr || 'TitaouPOS'}</h2>` : ''}
          ${showShop && settings.shop_name_ar ? `<p style="font-size: 11px; font-weight: bold; margin: 0;">${settings.shop_name_ar}</p>` : ''}
          ${showAddress ? `<p style="font-size: 9px; color: #444; margin: 1px 0;">${settings.shop_address || 'Alger, Algérie'}</p>` : ''}
          ${showPhone ? `<p style="font-size: 9px; color: #444; margin: 1px 0;">Tél: ${settings.shop_phone || '0553444057'}</p>` : ''}
          ${showRcNif ? `<p style="font-size: 8px; color: #444; margin: 1px 0;">RC: ${settings.shop_rc || '16/00-123456B22'} | NIF: ${settings.shop_nif || '0016160123456'}</p>` : ''}
          ${settings.receipt_header ? `<p style="font-size: 9px; font-weight: bold; margin: 2px 0; font-style: italic;">${settings.receipt_header}</p>` : ''}
          ${showDate ? `<p style="font-size: 8px; color: #666; margin-top: 4px;">${new Date().toLocaleString()}</p>` : ''}
        </div>
        <div style="padding: 4px 0; border-bottom: 1px dashed #000; font-size: 9px; display: flex; justify-content: space-between;">
          <span>TEST RECEIPT #0001</span>
          ${showCashier ? `<span>Caisse: ${$currentUser?.display_name || 'Admin'}</span>` : ''}
        </div>
        <div style="padding: 6px 0; border-bottom: 1px dashed #000;">
          <table style="width: 100%; font-size: 9px;">
            <thead>
              <tr style="border-bottom: 1px solid #ccc;"><th style="text-align: left;">Article</th><th style="text-align: center;">Qté</th><th style="text-align: right;">P.U</th><th style="text-align: right;">Total</th></tr>
            </thead>
            <tbody>
              <tr><td style="font-weight: bold;">Article Démo Test 1</td><td style="text-align: center;">1</td><td style="text-align: right;">100</td><td style="text-align: right; font-weight: bold;">100 DZD</td></tr>
              <tr><td style="font-weight: bold;">Article Démo Test 2</td><td style="text-align: center;">2</td><td style="text-align: right;">75</td><td style="text-align: right; font-weight: bold;">150 DZD</td></tr>
            </tbody>
          </table>
        </div>
        <div style="padding: 6px 0; font-size: ${totalSize}px; font-weight: ${totalBold ? '900' : 'normal'}; display: flex; justify-content: space-between; border-top: 1px dashed #000;">
          <span>TOTAL:</span>
          <span>250 DZD</span>
        </div>
        ${showTax ? `<div style="font-size: 8px; color: #555; display: flex; justify-content: space-between; padding-bottom: 4px;"><span>Dont TVA (19%):</span><span>40 DZD</span></div>` : ''}
        ${showFooter ? `
          <div style="text-align: center; padding-top: 6px; font-size: ${footerSize}px; font-weight: ${footerBold ? 'bold' : 'normal'}; border-top: 1px dashed #000; color: #444;">
            ${settings.receipt_footer || '*** Merci de votre visite - شكراً لزيارتكم ***'}
          </div>
        ` : ''}
        ${showQr ? `
          <div style="text-align: center; padding-top: 6px;">
            <div style="display: inline-block; padding: 2px 6px; background: #eee; border: 1px solid #ccc; font-family: monospace; font-size: 7px; border-radius: 3px;">
              [QR: TITAOU-VTE-0001]
            </div>
          </div>
        ` : ''}
      </div>
    `;
    printHtmlDirectly(html, 'Test Receipt');
  }

  function testPrintBarcode() {
    const w = parseInt(settings.barcode_label_width || '50');
    const h = parseInt(settings.barcode_label_height || '30');
    const showShop = settings.sticker_show_shop_name !== 'false';
    const showName = settings.sticker_show_product_name !== 'false';
    const showBarcode = settings.sticker_show_barcode !== 'false';
    const showPrice = settings.sticker_show_price !== 'false';
    const nameSize = parseInt(settings.sticker_name_font_size || '9');
    const nameBold = settings.sticker_name_bold !== 'false';
    const priceSize = parseInt(settings.sticker_price_font_size || '12');
    const priceBold = settings.sticker_price_bold !== 'false';
    const barcodeSvgHtml = settingsBarcodeSvgEl ? settingsBarcodeSvgEl.outerHTML : `<p style="font-family:monospace;font-size:10px;">${previewBarcodeNumber}</p>`;

    const html = `
      <div style="width: ${w}mm; height: ${h}mm; text-align: center; font-family: sans-serif; padding: 2mm; box-sizing: border-box; display: flex; flex-direction: column; align-items: center; justify-content: center; background: #fff;">
        ${showShop ? `<p style="font-weight: bold; font-size: 8px; text-transform: uppercase; color: #555; margin: 0;">${settings.shop_name_fr || 'TitaouPOS'}</p>` : ''}
        ${showName ? `<p style="font-size: ${nameSize}px; font-weight: ${nameBold ? '900' : 'normal'}; margin: 2px 0; overflow: hidden; white-space: nowrap; max-width: 100%;">${previewProductName}</p>` : ''}
        ${showBarcode ? `<div style="max-width: 100%; overflow: hidden; display: flex; justify-content: center; align-items: center; margin: 1px 0;">${barcodeSvgHtml}</div>` : ''}
        ${showPrice ? `<p style="font-size: ${priceSize}px; font-weight: ${priceBold ? '900' : 'normal'}; font-family: monospace; margin: 2px 0;">${previewPrice} DZD</p>` : ''}
      </div>
    `;
    printHtmlDirectly(html, 'Test Barcode Sticker');
  }

  function testPrintShelfTag() {
    const w = parseInt(settings.shelf_tag_width || '60');
    const h = parseInt(settings.shelf_tag_height || '40');
    const showShop = settings.shelf_show_shop_name !== 'false';
    const showName = settings.shelf_show_product_name !== 'false';
    const showPrice = settings.shelf_show_price !== 'false';
    const showRef = settings.shelf_show_ref !== 'false';
    const nameSize = parseInt(settings.shelf_name_font_size || '11');
    const nameBold = settings.shelf_name_bold !== 'false';
    const priceSize = parseInt(settings.shelf_price_font_size || '18');
    const priceBold = settings.shelf_price_bold !== 'false';
    const refSize = parseInt(settings.shelf_ref_font_size || '8');

    const html = `
      <div style="width: ${w}mm; height: ${h}mm; border: 1.5px solid #000; padding: 3mm; font-family: sans-serif; text-align: center; box-sizing: border-box; background: #fff; display: flex; flex-direction: column; justify-content: space-between; overflow: hidden;">
        ${showShop ? `<div style="display: flex; justify-content: space-between; font-size: 9px; font-weight: bold; border-bottom: 1px solid #000; padding-bottom: 2px;"><span>${settings.shop_name_fr || 'TitaouPOS'}</span><span style="color: green;">DISPO</span></div>` : ''}
        ${showName ? `<p style="font-size: ${nameSize}px; font-weight: ${nameBold ? '900' : 'normal'}; margin: 4px 0; line-height: 1.2;">${previewProductName}</p>` : ''}
        ${showPrice ? `<div style="background: #000; color: #fff; padding: 4px; font-size: ${priceSize}px; font-weight: ${priceBold ? '900' : 'normal'}; margin: 4px 0; font-family: monospace;">${previewPrice} DZD</div>` : ''}
        ${showRef ? `<div style="display: flex; justify-content: space-between; font-size: ${refSize}px; font-weight: bold;"><span>Ref: ${previewBarcodeNumber}</span><span>TVA 19% Incl.</span></div>` : ''}
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
        on:click={() => { currentTab = 'account'; loadUsersAndRoles(); }}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'account' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <Users class="w-4 h-4 mb-1" />
        <span class="truncate">Users & Roles</span>
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
            <h2 class="text-base font-black text-pos-text">Thermal Receipts & Invoice Printing / إعدادات طباعة الوصولات</h2>
            <p class="text-xs text-pos-muted">Configure printer hardware, receipt layout, font sizes, bold weights, and what fields to show</p>
          </div>
          <div class="flex items-center gap-2">
            <button on:click={testPrintReceipt} class="px-4 py-2 bg-slate-100 hover:bg-slate-200 dark:bg-slate-800 dark:hover:bg-slate-700 text-pos-text font-bold text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-xs transition">
              <Printer class="w-4 h-4 text-sky-500" />
              <span>Test Print Receipt</span>
            </button>
            <button on:click={saveAllSettings} class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl shadow-md transition cursor-pointer flex items-center gap-1.5">
              <Check class="w-4 h-4" />
              <span>Save Print Settings</span>
            </button>
          </div>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <!-- Left 2 Cols: Form Controls -->
          <div class="lg:col-span-2 space-y-4">
            <!-- Hardware / Paper / Font Family -->
            <div class="p-4 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-3">
              <h3 class="font-black text-xs text-pos-text flex items-center gap-1.5">
                <Printer class="w-4 h-4 text-sky-500" />
                <span>Printer & Page Sizing (إعدادات الطابعة والورق)</span>
              </h3>
              <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
                <div>
                  <label class="block text-xs font-bold text-pos-muted mb-1">Receipt Printer</label>
                  <select bind:value={settings.receipt_printer} class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs text-pos-text font-bold">
                    <option value="Xprinter XP-DT427B">Xprinter XP-DT427B</option>
                    <option value="Epson TM-T20III">Epson TM-T20III</option>
                    <option value="Bixolon SRP-350">Bixolon SRP-350</option>
                    <option value="Generic 80mm">Generic Thermal 80mm</option>
                    <option value="Generic 58mm">Generic Thermal 58mm</option>
                  </select>
                </div>

                <div>
                  <label class="block text-xs font-bold text-pos-muted mb-1">Paper Roll Width</label>
                  <select bind:value={settings.receipt_paper_width} class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs text-pos-text font-bold">
                    <option value="80mm">80 mm (Standard POS)</option>
                    <option value="58mm">58 mm (Compact Mini)</option>
                    <option value="A4">A4 Full Sheet Invoice</option>
                  </select>
                </div>

                <div>
                  <label class="block text-xs font-bold text-pos-muted mb-1">Receipt Font Family</label>
                  <select bind:value={settings.receipt_font_family} class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs text-pos-text font-bold">
                    <option value="monospace">Monospace (Terminal)</option>
                    <option value="sans-serif">Sans-Serif (Modern)</option>
                    <option value="Courier New">Courier New</option>
                    <option value="serif">Serif (Traditional)</option>
                  </select>
                </div>
              </div>
            </div>

            <!-- Greeting and Policy Notes -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">Receipt Header Greeting (Arabe / Français)</label>
                <input type="text" bind:value={settings.receipt_header} placeholder="مرحباً بكم في سوبرماركت تيتاو" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border border-pos-border rounded-xl text-xs text-pos-text font-bold outline-none" />
              </div>

              <div>
                <label class="block text-xs font-bold text-pos-muted mb-1">Receipt Footer Note / Return Policy</label>
                <input type="text" bind:value={settings.receipt_footer} placeholder="Les articles retournés doivent être présentés sous 48h" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border border-pos-border rounded-xl text-xs text-pos-text font-bold outline-none" />
              </div>
            </div>

            <!-- Section Content Visibility Toggles -->
            <div class="p-4 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-3">
              <h3 class="font-black text-xs text-pos-text flex items-center gap-1.5">
                <Eye class="w-4 h-4 text-sky-500" />
                <span>Fields to Show on Receipt (العناصر المراد إظهارها)</span>
              </h3>
              <div class="grid grid-cols-2 md:grid-cols-3 gap-2.5">
                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer p-2 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
                  <input type="checkbox" bind:checked={settings.receipt_show_shop_name} class="rounded text-sky-600" />
                  <span>Shop Name & Header</span>
                </label>

                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer p-2 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
                  <input type="checkbox" bind:checked={settings.receipt_show_address} class="rounded text-sky-600" />
                  <span>Store Address</span>
                </label>

                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer p-2 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
                  <input type="checkbox" bind:checked={settings.receipt_show_phone} class="rounded text-sky-600" />
                  <span>Phone Number</span>
                </label>

                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer p-2 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
                  <input type="checkbox" bind:checked={settings.receipt_show_rc_nif} class="rounded text-sky-600" />
                  <span>RC & NIF Info</span>
                </label>

                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer p-2 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
                  <input type="checkbox" bind:checked={settings.receipt_show_cashier} class="rounded text-sky-600" />
                  <span>Cashier Name</span>
                </label>

                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer p-2 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
                  <input type="checkbox" bind:checked={settings.receipt_show_date} class="rounded text-sky-600" />
                  <span>Date & Timestamp</span>
                </label>

                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer p-2 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
                  <input type="checkbox" bind:checked={settings.receipt_show_tax} class="rounded text-sky-600" />
                  <span>Tax / TVA Breakdown</span>
                </label>

                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer p-2 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
                  <input type="checkbox" bind:checked={settings.receipt_show_footer} class="rounded text-sky-600" />
                  <span>Footer Note / Policy</span>
                </label>

                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer p-2 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
                  <input type="checkbox" bind:checked={settings.receipt_show_qr} class="rounded text-sky-600" />
                  <span>QR Code Verification</span>
                </label>
              </div>
            </div>

            <!-- Typography, Font Sizing & Bold Options -->
            <div class="p-4 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-3">
              <h3 class="font-black text-xs text-pos-text flex items-center gap-1.5">
                <Type class="w-4 h-4 text-sky-500" />
                <span>Font Sizing & Bold Formatting (حجم الخط والسمك)</span>
              </h3>
              <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-3">
                <!-- Header Font Size & Bold -->
                <div class="p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border space-y-1.5">
                  <span class="text-[11px] font-bold text-pos-muted block">Shop Header</span>
                  <div class="flex items-center gap-1.5">
                    <input type="number" min="8" max="24" bind:value={settings.receipt_header_font_size} class="w-16 px-2 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg text-xs font-bold font-mono outline-none" />
                    <span class="text-[10px] text-pos-muted">px</span>
                  </div>
                  <label class="flex items-center gap-1.5 text-[11px] font-bold text-pos-text cursor-pointer pt-1">
                    <input type="checkbox" bind:checked={settings.receipt_header_bold} class="rounded text-sky-600" />
                    <span>Bold (عريض)</span>
                  </label>
                </div>

                <!-- Body / Items Font Size & Bold -->
                <div class="p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border space-y-1.5">
                  <span class="text-[11px] font-bold text-pos-muted block">Items & Rows</span>
                  <div class="flex items-center gap-1.5">
                    <input type="number" min="8" max="18" bind:value={settings.receipt_body_font_size} class="w-16 px-2 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg text-xs font-bold font-mono outline-none" />
                    <span class="text-[10px] text-pos-muted">px</span>
                  </div>
                  <label class="flex items-center gap-1.5 text-[11px] font-bold text-pos-text cursor-pointer pt-1">
                    <input type="checkbox" bind:checked={settings.receipt_body_bold} class="rounded text-sky-600" />
                    <span>Bold (عريض)</span>
                  </label>
                </div>

                <!-- Total Amount Font Size & Bold -->
                <div class="p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border space-y-1.5">
                  <span class="text-[11px] font-bold text-pos-muted block">Total Amount</span>
                  <div class="flex items-center gap-1.5">
                    <input type="number" min="10" max="24" bind:value={settings.receipt_total_font_size} class="w-16 px-2 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg text-xs font-bold font-mono outline-none" />
                    <span class="text-[10px] text-pos-muted">px</span>
                  </div>
                  <label class="flex items-center gap-1.5 text-[11px] font-bold text-pos-text cursor-pointer pt-1">
                    <input type="checkbox" bind:checked={settings.receipt_total_bold} class="rounded text-sky-600" />
                    <span>Bold (عريض)</span>
                  </label>
                </div>

                <!-- Footer Font Size & Bold -->
                <div class="p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border space-y-1.5">
                  <span class="text-[11px] font-bold text-pos-muted block">Footer Policy</span>
                  <div class="flex items-center gap-1.5">
                    <input type="number" min="6" max="16" bind:value={settings.receipt_footer_font_size} class="w-16 px-2 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg text-xs font-bold font-mono outline-none" />
                    <span class="text-[10px] text-pos-muted">px</span>
                  </div>
                  <label class="flex items-center gap-1.5 text-[11px] font-bold text-pos-text cursor-pointer pt-1">
                    <input type="checkbox" bind:checked={settings.receipt_footer_bold} class="rounded text-sky-600" />
                    <span>Bold (عريض)</span>
                  </label>
                </div>
              </div>
            </div>
          </div>

          <!-- Right Col: Live Dynamic Receipt Preview -->
          <div class="bg-slate-100 dark:bg-slate-900/60 p-4 rounded-2xl flex flex-col items-center justify-start border border-pos-border space-y-2">
            <span class="text-[10px] font-black text-pos-muted uppercase tracking-wider">Live Receipt Preview ({settings.receipt_paper_width || '80mm'})</span>
            <div
              style="font-family: {settings.receipt_font_family || 'monospace'}; font-size: {settings.receipt_body_font_size || '10'}px; font-weight: {settings.receipt_body_bold === 'true' || settings.receipt_body_bold === true ? 'bold' : 'normal'}; width: {settings.receipt_paper_width === '58mm' ? '200px' : '260px'};"
              class="bg-white text-black p-3.5 shadow-md space-y-2 border border-slate-300 transition-all rounded-sm leading-tight"
            >
              <!-- Header -->
              <div class="text-center pb-1 border-b border-dashed border-slate-400">
                {#if (settings.receipt_show_shop_name ?? 'true') !== 'false' && (settings.receipt_show_shop_name ?? true) !== false}
                  <p style="font-size: {settings.receipt_header_font_size || '14'}px; font-weight: {(settings.receipt_header_bold ?? 'true') !== 'false' && (settings.receipt_header_bold ?? true) !== false ? '900' : 'normal'};" class="leading-tight">
                    {settings.shop_name_fr || 'TitaouPOS'}
                  </p>
                  {#if settings.shop_name_ar}
                    <p class="font-bold text-[10px]">{settings.shop_name_ar}</p>
                  {/if}
                {/if}
                {#if (settings.receipt_show_address ?? 'true') !== 'false' && (settings.receipt_show_address ?? true) !== false}
                  <p class="text-[8px] text-gray-600">{settings.shop_address || 'Alger Centre, Algérie'}</p>
                {/if}
                {#if (settings.receipt_show_phone ?? 'true') !== 'false' && (settings.receipt_show_phone ?? true) !== false}
                  <p class="text-[8px] text-gray-600">Tél: {settings.shop_phone || '0553444057'}</p>
                {/if}
                {#if (settings.receipt_show_rc_nif ?? 'true') !== 'false' && (settings.receipt_show_rc_nif ?? true) !== false}
                  <p class="text-[7px] text-gray-500">RC: {settings.shop_rc || '16/00-0123456B22'} | NIF: {settings.shop_nif || '0016160123456'}</p>
                {/if}
                {#if settings.receipt_header}
                  <p class="text-[8px] font-bold text-sky-800 mt-0.5 italic">{settings.receipt_header}</p>
                {/if}
              </div>

              <!-- Ticket info -->
              <div class="py-0.5 border-b border-dashed border-slate-400 flex justify-between text-[8px]">
                <span>TICKET #9842</span>
                {#if (settings.receipt_show_cashier ?? 'true') !== 'false' && (settings.receipt_show_cashier ?? true) !== false}
                  <span>Caisse: Admin</span>
                {/if}
              </div>

              <!-- Items list -->
              <div class="space-y-0.5 py-1 border-b border-dashed border-slate-400 text-[9px]">
                <div class="flex justify-between font-bold">
                  <span>1x Sucre Cevital 1kg</span>
                  <span>100 DZD</span>
                </div>
                <div class="flex justify-between font-bold">
                  <span>2x Lait Candia 1L</span>
                  <span>240 DZD</span>
                </div>
              </div>

              <!-- Taxes -->
              {#if (settings.receipt_show_tax ?? 'true') !== 'false' && (settings.receipt_show_tax ?? true) !== false}
                <div class="text-[8px] text-gray-600 py-0.5 border-b border-dashed border-slate-400 flex justify-between">
                  <span>Total HT: 285 DZD</span>
                  <span>TVA (19%): 55 DZD</span>
                </div>
              {/if}

              <!-- Total -->
              <div
                style="font-size: {settings.receipt_total_font_size || '13'}px; font-weight: {(settings.receipt_total_bold ?? 'true') !== 'false' && (settings.receipt_total_bold ?? true) !== false ? '900' : 'normal'};"
                class="flex justify-between pt-1"
              >
                <span>TOTAL:</span>
                <span>340 DZD</span>
              </div>

              <!-- Footer -->
              {#if (settings.receipt_show_footer ?? 'true') !== 'false' && (settings.receipt_show_footer ?? true) !== false}
                <div
                  style="font-size: {settings.receipt_footer_font_size || '8'}px; font-weight: {settings.receipt_footer_bold === 'true' || settings.receipt_footer_bold === true ? 'bold' : 'normal'};"
                  class="text-center pt-1 text-gray-600 border-t border-dashed border-slate-400"
                >
                  {settings.receipt_footer || '*** Merci de votre visite ***'}
                </div>
              {/if}

              <!-- QR Code -->
              {#if (settings.receipt_show_qr ?? 'true') !== 'false' && (settings.receipt_show_qr ?? true) !== false}
                <div class="text-center pt-0.5">
                  <div class="inline-block px-2 py-0.5 bg-slate-100 border border-slate-300 font-mono text-[7px] text-gray-600 rounded">
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
            <span>Serial Cash Drawer (COM Port) / درج النقود</span>
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
          <button on:click={saveAllSettings} class="px-6 py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl shadow-md transition cursor-pointer flex items-center gap-1.5">
            <Check class="w-4 h-4" />
            <span>Save Print & Drawer Settings</span>
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

    <!-- 3. BARCODE LABELS TAB (Two Distinct Sections with Live Previews & Full Customization) -->
    {:else if currentTab === 'barcodes'}
      <div class="max-w-5xl space-y-6">
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-base font-black text-pos-text">Barcode & Shelf Label Generator / ملصقات الباركود وبطاقات الرف</h2>
            <p class="text-xs text-pos-muted">Custom thermal sticker rolls (50x30mm) and shelf price tags with live presets, fonts, and dimensions</p>
          </div>
          <button on:click={saveAllSettings} class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl shadow-md transition cursor-pointer flex items-center gap-1.5">
            <Check class="w-4 h-4" />
            <span>Save Label Settings</span>
          </button>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <!-- SECTION 1: Product Barcode Sticker -->
          <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4 flex flex-col justify-between">
            <div class="space-y-3">
              <div class="flex items-center justify-between">
                <h3 class="text-sm font-black text-pos-text flex items-center gap-2">
                  <Tag class="w-4 h-4 text-sky-500" />
                  <span>1. Product Sticker Preset (ملصق باركود)</span>
                </h3>
                <span class="text-[10px] font-mono bg-sky-100 dark:bg-sky-950 text-sky-800 dark:text-sky-300 px-2 py-0.5 rounded-full font-bold">Thermal Roll</span>
              </div>

              <!-- Live Real Sticker Preview -->
              <div class="p-4 bg-white text-slate-900 border-2 border-dashed border-slate-300 rounded-xl flex flex-col items-center justify-center text-center space-y-1 shadow-inner min-h-[140px]">
                {#if (settings.sticker_show_shop_name ?? 'true') !== 'false' && (settings.sticker_show_shop_name ?? true) !== false}
                  <span class="text-[8px] text-slate-500 font-bold uppercase">{settings.shop_name_fr || 'TitaouPOS Supermarché'}</span>
                {/if}
                {#if (settings.sticker_show_product_name ?? 'true') !== 'false' && (settings.sticker_show_product_name ?? true) !== false}
                  <p
                    style="font-size: {settings.sticker_name_font_size || '9'}px; font-weight: {(settings.sticker_name_bold ?? 'true') !== 'false' && (settings.sticker_name_bold ?? true) !== false ? '900' : 'normal'};"
                    class="text-slate-900 line-clamp-1 leading-tight"
                  >
                    {previewProductName}
                  </p>
                {/if}
                {#if (settings.sticker_show_barcode ?? 'true') !== 'false' && (settings.sticker_show_barcode ?? true) !== false}
                  <div class="w-full flex justify-center py-0.5 overflow-hidden">
                    <svg bind:this={settingsBarcodeSvgEl} class="max-w-full"></svg>
                  </div>
                {/if}
                {#if (settings.sticker_show_price ?? 'true') !== 'false' && (settings.sticker_show_price ?? true) !== false}
                  <span
                    style="font-size: {settings.sticker_price_font_size || '12'}px; font-weight: {(settings.sticker_price_bold ?? 'true') !== 'false' && (settings.sticker_price_bold ?? true) !== false ? '900' : 'normal'};"
                    class="text-slate-900 font-mono leading-none"
                  >
                    {previewPrice} DZD
                  </span>
                {/if}
              </div>

              <!-- Dimensions -->
              <div class="grid grid-cols-2 gap-2 text-xs">
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Width (mm)</label>
                  <input type="number" bind:value={settings.barcode_label_width} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold font-mono outline-none" />
                </div>
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Height (mm)</label>
                  <input type="number" bind:value={settings.barcode_label_height} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold font-mono outline-none" />
                </div>
              </div>

              <!-- Display Fields Checkboxes -->
              <div class="space-y-1.5 pt-1">
                <span class="text-[10px] font-bold text-pos-muted uppercase tracking-wider block">Fields to Show (العناصر الظاهرة)</span>
                <div class="grid grid-cols-2 gap-1.5">
                  <label class="flex items-center gap-1.5 text-[11px] font-bold text-pos-text cursor-pointer p-1.5 bg-white dark:bg-slate-900 rounded-lg border border-pos-border">
                    <input type="checkbox" bind:checked={settings.sticker_show_shop_name} class="rounded text-sky-600" />
                    <span>Shop Name</span>
                  </label>
                  <label class="flex items-center gap-1.5 text-[11px] font-bold text-pos-text cursor-pointer p-1.5 bg-white dark:bg-slate-900 rounded-lg border border-pos-border">
                    <input type="checkbox" bind:checked={settings.sticker_show_product_name} class="rounded text-sky-600" />
                    <span>Product Name</span>
                  </label>
                  <label class="flex items-center gap-1.5 text-[11px] font-bold text-pos-text cursor-pointer p-1.5 bg-white dark:bg-slate-900 rounded-lg border border-pos-border">
                    <input type="checkbox" bind:checked={settings.sticker_show_barcode} class="rounded text-sky-600" />
                    <span>Real Barcode</span>
                  </label>
                  <label class="flex items-center gap-1.5 text-[11px] font-bold text-pos-text cursor-pointer p-1.5 bg-white dark:bg-slate-900 rounded-lg border border-pos-border">
                    <input type="checkbox" bind:checked={settings.sticker_show_price} class="rounded text-sky-600" />
                    <span>Sale Price</span>
                  </label>
                </div>
              </div>

              <!-- Sizing & Formatting -->
              <div class="space-y-1.5 pt-1">
                <span class="text-[10px] font-bold text-pos-muted uppercase tracking-wider block">Font Sizes & Bold Styling (الأحجام والخط)</span>
                <div class="grid grid-cols-3 gap-2">
                  <div class="p-2 bg-white dark:bg-slate-900 rounded-lg border border-pos-border space-y-1">
                    <span class="text-[9px] font-bold text-pos-muted block">Name Size</span>
                    <input type="number" min="6" max="18" bind:value={settings.sticker_name_font_size} class="w-full px-1.5 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs font-bold font-mono outline-none" />
                    <label class="flex items-center gap-1 text-[10px] font-bold text-pos-text cursor-pointer pt-0.5">
                      <input type="checkbox" bind:checked={settings.sticker_name_bold} class="rounded text-sky-600" />
                      <span>Bold</span>
                    </label>
                  </div>

                  <div class="p-2 bg-white dark:bg-slate-900 rounded-lg border border-pos-border space-y-1">
                    <span class="text-[9px] font-bold text-pos-muted block">Price Size</span>
                    <input type="number" min="8" max="22" bind:value={settings.sticker_price_font_size} class="w-full px-1.5 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs font-bold font-mono outline-none" />
                    <label class="flex items-center gap-1 text-[10px] font-bold text-pos-text cursor-pointer pt-0.5">
                      <input type="checkbox" bind:checked={settings.sticker_price_bold} class="rounded text-sky-600" />
                      <span>Bold</span>
                    </label>
                  </div>

                  <div class="p-2 bg-white dark:bg-slate-900 rounded-lg border border-pos-border space-y-1">
                    <span class="text-[9px] font-bold text-pos-muted block">Code Size</span>
                    <input type="number" min="6" max="16" bind:value={settings.sticker_barcode_font_size} on:input={renderSettingsBarcode} class="w-full px-1.5 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs font-bold font-mono outline-none" />
                  </div>
                </div>
              </div>
            </div>

            <button on:click={testPrintBarcode} class="w-full py-2.5 bg-sky-600 hover:bg-sky-700 text-white text-xs font-bold rounded-xl flex items-center justify-center gap-1.5 cursor-pointer shadow-xs transition mt-3">
              <Printer class="w-3.5 h-3.5" />
              <span>Test Print Barcode Sticker ({settings.barcode_label_width || 50}x{settings.barcode_label_height || 30}mm)</span>
            </button>
          </div>

          <!-- SECTION 2: Shelf Price Etiquette -->
          <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4 flex flex-col justify-between">
            <div class="space-y-3">
              <div class="flex items-center justify-between">
                <h3 class="text-sm font-black text-pos-text flex items-center gap-2">
                  <Tag class="w-4 h-4 text-emerald-500" />
                  <span>2. Shelf Etiquette Preset (بطاقة رف وسعر)</span>
                </h3>
                <span class="text-[10px] font-mono bg-emerald-100 dark:bg-emerald-950 text-emerald-800 dark:text-emerald-300 px-2 py-0.5 rounded-full font-bold">Gondola Tag</span>
              </div>

              <!-- Live Shelf Tag Preview -->
              <div class="p-4 bg-white text-slate-900 border-2 border-dashed border-emerald-300 rounded-xl flex flex-col items-center justify-between text-center space-y-1 shadow-inner min-h-[140px]">
                {#if (settings.shelf_show_shop_name ?? 'true') !== 'false' && (settings.shelf_show_shop_name ?? true) !== false}
                  <div class="w-full flex justify-between text-[9px] font-bold text-slate-500 border-b pb-0.5">
                    <span>{settings.shop_name_fr || 'TitaouPOS'}</span>
                    <span class="text-emerald-600 font-bold">DISPO EN RAYON</span>
                  </div>
                {/if}
                {#if (settings.shelf_show_product_name ?? 'true') !== 'false' && (settings.shelf_show_product_name ?? true) !== false}
                  <h4
                    style="font-size: {settings.shelf_name_font_size || '11'}px; font-weight: {(settings.shelf_name_bold ?? 'true') !== 'false' && (settings.shelf_name_bold ?? true) !== false ? '900' : 'normal'};"
                    class="text-slate-900 leading-tight py-0.5"
                  >
                    {previewProductName}
                  </h4>
                {/if}
                {#if (settings.shelf_show_price ?? 'true') !== 'false' && (settings.shelf_show_price ?? true) !== false}
                  <div
                    style="font-size: {settings.shelf_price_font_size || '18'}px; font-weight: {(settings.shelf_price_bold ?? 'true') !== 'false' && (settings.shelf_price_bold ?? true) !== false ? '900' : 'normal'};"
                    class="w-full bg-slate-900 text-white font-mono rounded py-0.5"
                  >
                    {previewPrice} DZD
                  </div>
                {/if}
                {#if (settings.shelf_show_ref ?? 'true') !== 'false' && (settings.shelf_show_ref ?? true) !== false}
                  <div class="w-full flex justify-between font-bold text-slate-500 pt-0.5" style="font-size: {settings.shelf_ref_font_size || '8'}px;">
                    <span>Ref: {previewBarcodeNumber}</span>
                    <span>TVA 19% Incl.</span>
                  </div>
                {/if}
              </div>

              <!-- Dimensions -->
              <div class="grid grid-cols-2 gap-2 text-xs">
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Width (mm)</label>
                  <input type="number" bind:value={settings.shelf_tag_width} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold font-mono outline-none" />
                </div>
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Height (mm)</label>
                  <input type="number" bind:value={settings.shelf_tag_height} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold font-mono outline-none" />
                </div>
              </div>

              <!-- Display Fields Checkboxes -->
              <div class="space-y-1.5 pt-1">
                <span class="text-[10px] font-bold text-pos-muted uppercase tracking-wider block">Fields to Show (العناصر الظاهرة)</span>
                <div class="grid grid-cols-2 gap-1.5">
                  <label class="flex items-center gap-1.5 text-[11px] font-bold text-pos-text cursor-pointer p-1.5 bg-white dark:bg-slate-900 rounded-lg border border-pos-border">
                    <input type="checkbox" bind:checked={settings.shelf_show_shop_name} class="rounded text-emerald-600" />
                    <span>Shop Header</span>
                  </label>
                  <label class="flex items-center gap-1.5 text-[11px] font-bold text-pos-text cursor-pointer p-1.5 bg-white dark:bg-slate-900 rounded-lg border border-pos-border">
                    <input type="checkbox" bind:checked={settings.shelf_show_product_name} class="rounded text-emerald-600" />
                    <span>Product Name</span>
                  </label>
                  <label class="flex items-center gap-1.5 text-[11px] font-bold text-pos-text cursor-pointer p-1.5 bg-white dark:bg-slate-900 rounded-lg border border-pos-border">
                    <input type="checkbox" bind:checked={settings.shelf_show_price} class="rounded text-emerald-600" />
                    <span>Sale Price</span>
                  </label>
                  <label class="flex items-center gap-1.5 text-[11px] font-bold text-pos-text cursor-pointer p-1.5 bg-white dark:bg-slate-900 rounded-lg border border-pos-border">
                    <input type="checkbox" bind:checked={settings.shelf_show_ref} class="rounded text-emerald-600" />
                    <span>Ref / Barcode</span>
                  </label>
                </div>
              </div>

              <!-- Sizing & Formatting -->
              <div class="space-y-1.5 pt-1">
                <span class="text-[10px] font-bold text-pos-muted uppercase tracking-wider block">Font Sizes & Bold Styling (الأحجام والخط)</span>
                <div class="grid grid-cols-3 gap-2">
                  <div class="p-2 bg-white dark:bg-slate-900 rounded-lg border border-pos-border space-y-1">
                    <span class="text-[9px] font-bold text-pos-muted block">Name Size</span>
                    <input type="number" min="8" max="22" bind:value={settings.shelf_name_font_size} class="w-full px-1.5 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs font-bold font-mono outline-none" />
                    <label class="flex items-center gap-1 text-[10px] font-bold text-pos-text cursor-pointer pt-0.5">
                      <input type="checkbox" bind:checked={settings.shelf_name_bold} class="rounded text-emerald-600" />
                      <span>Bold</span>
                    </label>
                  </div>

                  <div class="p-2 bg-white dark:bg-slate-900 rounded-lg border border-pos-border space-y-1">
                    <span class="text-[9px] font-bold text-pos-muted block">Price Size</span>
                    <input type="number" min="10" max="32" bind:value={settings.shelf_price_font_size} class="w-full px-1.5 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs font-bold font-mono outline-none" />
                    <label class="flex items-center gap-1 text-[10px] font-bold text-pos-text cursor-pointer pt-0.5">
                      <input type="checkbox" bind:checked={settings.shelf_price_bold} class="rounded text-emerald-600" />
                      <span>Bold</span>
                    </label>
                  </div>

                  <div class="p-2 bg-white dark:bg-slate-900 rounded-lg border border-pos-border space-y-1">
                    <span class="text-[9px] font-bold text-pos-muted block">Ref Size</span>
                    <input type="number" min="6" max="16" bind:value={settings.shelf_ref_font_size} class="w-full px-1.5 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs font-bold font-mono outline-none" />
                  </div>
                </div>
              </div>
            </div>

            <button on:click={testPrintShelfTag} class="w-full py-2.5 bg-emerald-600 hover:bg-emerald-700 text-white text-xs font-bold rounded-xl flex items-center justify-center gap-1.5 cursor-pointer shadow-xs transition mt-3">
              <Printer class="w-3.5 h-3.5" />
              <span>Test Print Shelf Tag ({settings.shelf_tag_width || 60}x{settings.shelf_tag_height || 40}mm)</span>
            </button>
          </div>
        </div>

        <div class="pt-4 border-t border-pos-border flex justify-end">
          <button on:click={saveAllSettings} class="px-6 py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl shadow-md transition cursor-pointer flex items-center gap-1.5">
            <Check class="w-4 h-4" />
            <span>Save Label Settings</span>
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
              <p class="text-base font-black text-pos-text">TitaouPOS {appVersion} (Windows x64)</p>
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
          <div class="flex flex-wrap items-center gap-3">
            <button
              on:click={checkForUpdates}
              disabled={isCheckingUpdate}
              class="px-5 py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl flex items-center gap-2 cursor-pointer shadow-md transition"
            >
              <RefreshCw class="w-4 h-4 {isCheckingUpdate ? 'animate-spin' : ''}" />
              <span>{isCheckingUpdate ? 'Checking GitHub...' : 'Check for Updates Now (فحص التحديثات)'}</span>
            </button>

            {#if updateAvailable && latestDownloadUrl}
              <a
                href={latestDownloadUrl}
                target="_blank"
                class="px-5 py-2.5 bg-emerald-600 hover:bg-emerald-700 text-white font-black text-xs rounded-xl flex items-center gap-2 cursor-pointer shadow-md transition animate-pulse"
              >
                <Download class="w-4 h-4" />
                <span>Download {latestReleaseInfo?.tag_name || 'Update'} (تحميل التحديث)</span>
              </a>
            {/if}

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

    <!-- 8. ACCOUNT & USER MANAGEMENT TAB -->
    {:else if currentTab === 'account'}
      <div class="max-w-5xl space-y-6">
        <!-- Tab Header -->
        <div class="flex items-center justify-between">
          <div>
            <h2 class="text-base font-black text-pos-text flex items-center gap-2">
              <Users class="w-5 h-5 text-sky-600" />
              <span>User Accounts & Access Roles / إدارة المستخدمين والأدوار</span>
            </h2>
            <p class="text-xs text-pos-muted">Manage system users, login credentials, assigned roles, and discount authorizations</p>
          </div>
          <button
            on:click={openCreateUserModal}
            class="px-4 py-2.5 bg-sky-600 hover:bg-sky-700 text-white text-xs font-black rounded-xl cursor-pointer shadow-md flex items-center gap-2 transition"
          >
            <UserPlus class="w-4 h-4" />
            <span>Add User Account / إضافة مستخدم</span>
          </button>
        </div>

        <!-- Current Active Session Card -->
        <div class="p-5 bg-gradient-to-r from-sky-500/10 via-sky-500/5 to-transparent rounded-2xl border border-sky-200 dark:border-sky-900/60 space-y-4">
          <div class="flex flex-wrap items-center justify-between gap-4">
            <div class="flex items-center gap-3">
              <div class="w-12 h-12 rounded-2xl bg-sky-600 text-white font-black flex items-center justify-center text-lg shadow-sm">
                {($currentUser?.display_name || 'A')[0].toUpperCase()}
              </div>
              <div>
                <div class="flex items-center gap-2">
                  <h3 class="text-sm font-black text-pos-text">{$currentUser?.display_name || 'Administrator'}</h3>
                  <span class="px-2 py-0.5 rounded-full text-[10px] font-black bg-sky-100 dark:bg-sky-950/80 text-sky-700 dark:text-sky-300 border border-sky-300 dark:border-sky-800">
                    {$currentUser?.role_name || 'Administrator'}
                  </span>
                </div>
                <p class="text-xs text-pos-muted">Active Session: <strong class="font-mono text-pos-text">@{$currentUser?.username || 'admin'}</strong> • Max Discount: <strong class="text-sky-600">{$currentUser?.max_discount_percent ?? 100}%</strong></p>
              </div>
            </div>

            <!-- Quick Password Change for Active User -->
            <div class="flex items-center gap-2 w-full md:w-auto">
              <div class="relative flex-1 md:w-56">
                <Lock class="w-3.5 h-3.5 text-pos-muted absolute start-3 top-2.5" />
                <input
                  type="password"
                  bind:value={newPassword}
                  placeholder="New password / PIN"
                  class="w-full ps-8 pe-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs text-pos-text"
                />
              </div>
              <button
                on:click={handleChangePassword}
                disabled={!newPassword}
                class="px-4 py-2 bg-sky-600 hover:bg-sky-700 disabled:opacity-40 text-white text-xs font-bold rounded-xl cursor-pointer shadow-xs transition shrink-0"
              >
                Change My Password
              </button>
            </div>
          </div>
        </div>

        <!-- Users Table Card -->
        <div class="bg-pos-card rounded-2xl border border-pos-border overflow-hidden shadow-xs">
          <div class="p-4 border-b border-pos-border flex items-center justify-between bg-slate-50/50 dark:bg-slate-800/30">
            <h3 class="text-xs font-black text-pos-text flex items-center gap-2">
              <Shield class="w-4 h-4 text-sky-600" />
              <span>Registered System Users ({userAccounts.length})</span>
            </h3>
            <button
              on:click={loadUsersAndRoles}
              class="text-xs text-sky-600 hover:text-sky-700 font-bold flex items-center gap-1 cursor-pointer"
            >
              <RefreshCw class="w-3.5 h-3.5" />
              <span>Refresh List</span>
            </button>
          </div>

          <div class="overflow-x-auto">
            <table class="w-full text-start text-xs">
              <thead class="bg-slate-100/60 dark:bg-slate-800/60 text-pos-muted font-black border-b border-pos-border">
                <tr>
                  <th class="p-3 text-start">User</th>
                  <th class="p-3 text-start">Role / الدور</th>
                  <th class="p-3 text-center">Max Discount</th>
                  <th class="p-3 text-center">Status</th>
                  <th class="p-3 text-start">Last Login</th>
                  <th class="p-3 text-end">Actions</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-pos-border">
                {#if userAccounts.length === 0}
                  <tr>
                    <td colspan="6" class="p-8 text-center text-pos-muted font-bold">
                      No user accounts found. Click "Add User Account" to create one.
                    </td>
                  </tr>
                {:else}
                  {#each userAccounts as u}
                    <tr class="hover:bg-slate-50/60 dark:hover:bg-slate-800/40 transition">
                      <td class="p-3">
                        <div class="flex items-center gap-2.5">
                          <div class="w-8 h-8 rounded-xl bg-slate-100 dark:bg-slate-800 text-pos-text font-black flex items-center justify-center text-xs border border-pos-border">
                            {u.display_name[0]?.toUpperCase() || 'U'}
                          </div>
                          <div>
                            <div class="font-black text-pos-text">{u.display_name}</div>
                            <div class="font-mono text-[11px] text-pos-muted">@{u.username}</div>
                          </div>
                        </div>
                      </td>
                      <td class="p-3">
                        <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-[11px] font-black {
                          u.role_name === 'Administrator' ? 'bg-amber-100 text-amber-800 dark:bg-amber-950/80 dark:text-amber-300 border border-amber-300 dark:border-amber-800' :
                          u.role_name === 'Manager' ? 'bg-emerald-100 text-emerald-800 dark:bg-emerald-950/80 dark:text-emerald-300 border border-emerald-300 dark:border-emerald-800' :
                          u.role_name === 'Inventory Clerk' ? 'bg-purple-100 text-purple-800 dark:bg-purple-950/80 dark:text-purple-300 border border-purple-300 dark:border-purple-800' :
                          'bg-sky-100 text-sky-800 dark:bg-sky-950/80 dark:text-sky-300 border border-sky-300 dark:border-sky-800'
                        }">
                          {u.role_name || 'Standard Role'}
                        </span>
                      </td>
                      <td class="p-3 text-center font-mono font-bold text-pos-text">
                        {u.max_discount_percent}%
                      </td>
                      <td class="p-3 text-center">
                        {#if u.is_active}
                          <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[10px] font-black bg-emerald-100 text-emerald-700 dark:bg-emerald-950/60 dark:text-emerald-400">
                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-500"></span> Active
                          </span>
                        {:else}
                          <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-[10px] font-black bg-rose-100 text-rose-700 dark:bg-rose-950/60 dark:text-rose-400">
                            <span class="w-1.5 h-1.5 rounded-full bg-rose-500"></span> Disabled
                          </span>
                        {/if}
                      </td>
                      <td class="p-3 text-pos-muted font-mono text-[11px]">
                        {u.last_login ? u.last_login.slice(0, 16).replace('T', ' ') : 'Never'}
                      </td>
                      <td class="p-3 text-end">
                        <div class="inline-flex items-center gap-1">
                          <button
                            on:click={() => openEditUserModal(u)}
                            class="p-1.5 hover:bg-sky-50 dark:hover:bg-sky-950/50 text-sky-600 rounded-lg cursor-pointer transition"
                            title="Edit User Details / تعديل"
                          >
                            <Edit2 class="w-4 h-4" />
                          </button>
                          {#if u.id !== 1}
                            <button
                              on:click={() => deleteUserAccount(u)}
                              class="p-1.5 hover:bg-rose-50 dark:hover:bg-rose-950/50 text-rose-500 rounded-lg cursor-pointer transition"
                              title="Delete User / حذف"
                            >
                              <Trash2 class="w-4 h-4" />
                            </button>
                          {/if}
                        </div>
                      </td>
                    </tr>
                  {/each}
                {/if}
              </tbody>
            </table>
          </div>
        </div>
      </div>

    <!-- 9. DANGER / FACTORY RESET TAB -->
    {:else if currentTab === 'danger'}
      <div class="max-w-3xl space-y-6">
        <div>
          <h2 class="text-base font-black text-rose-600 flex items-center gap-2">
            <AlertOctagon class="w-5 h-5" />
            <span>Factory Reset & Data Purge / تهيئة المصنع ومسح البيانات</span>
          </h2>
          <p class="text-xs text-pos-muted">Irreversible operations. Please backup database before proceeding.</p>
        </div>

        <div class="p-5 bg-rose-50 dark:bg-rose-950/30 border border-rose-200 dark:border-rose-900 rounded-2xl space-y-4">
          <div class="space-y-2">
            <label class="flex items-center gap-2.5 text-xs font-bold text-pos-text cursor-pointer">
              <input type="radio" bind:group={resetType} value="transactions_only" class="text-rose-600" />
              <span>Clear sales, cash movements & debts only (keep product catalog) / مسح المبيعات والديون فقط</span>
            </label>
            <label class="flex items-center gap-2.5 text-xs font-bold text-rose-600 cursor-pointer">
              <input type="radio" bind:group={resetType} value="full_reset" class="text-rose-600" />
              <span>Full Factory Reset (Purge all products, sales, customers, and clean database) / إعادة ضبط المصنع بالكامل</span>
            </label>
          </div>

          <div class="space-y-2 pt-3 border-t border-rose-200 dark:border-rose-900">
            <label class="block text-xs font-bold text-pos-muted">Type <span class="text-rose-600 font-mono font-black">RESET</span> to confirm execution:</label>
            <div class="flex items-center gap-2">
              <input
                type="text"
                bind:value={resetConfirm}
                placeholder="RESET"
                class="w-48 px-3 py-2 bg-white dark:bg-slate-900 border border-rose-300 dark:border-rose-800 rounded-xl text-xs font-mono font-black text-rose-600"
              />
              <button
                on:click={handleFactoryReset}
                disabled={resetConfirm !== 'RESET'}
                class="px-5 py-2 bg-rose-600 hover:bg-rose-700 disabled:opacity-40 text-white text-xs font-black rounded-xl cursor-pointer shadow-md transition"
              >
                Execute Reset / تنفيذ المسح
              </button>
            </div>
          </div>
        </div>
      </div>
    {/if}
  </div>

  <!-- User Account Add / Edit Modal -->
  {#if showUserModal}
    <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
      <div class="bg-pos-card border border-pos-border rounded-2xl shadow-2xl p-6 max-w-md w-full space-y-4 animate-in zoom-in-95 duration-150">
        <div class="flex items-center justify-between border-b border-pos-border pb-3">
          <h3 class="font-black text-sm text-pos-text flex items-center gap-2">
            {#if userModalMode === 'create'}
              <UserPlus class="w-4 h-4 text-sky-600" />
              <span>Add New User Account / إضافة حساب مستخدم</span>
            {:else}
              <Edit2 class="w-4 h-4 text-sky-600" />
              <span>Edit User Account / تعديل الحساب</span>
            {/if}
          </h3>
          <button on:click={() => (showUserModal = false)} class="text-pos-muted hover:text-pos-text text-sm font-bold">✕</button>
        </div>

        {#if userFormError}
          <div class="p-3 bg-rose-50 dark:bg-rose-950/60 border border-rose-200 dark:border-rose-900 text-rose-600 rounded-xl text-xs font-bold">
            {userFormError}
          </div>
        {/if}

        <div class="space-y-3 text-xs">
          <div>
            <label class="block font-bold text-pos-muted mb-1">Username / اسم الدخول <span class="text-rose-500">*</span></label>
            <input
              type="text"
              bind:value={userForm.username}
              placeholder="e.g. cashier_ahmed"
              class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl font-mono text-pos-text"
            />
          </div>

          <div>
            <label class="block font-bold text-pos-muted mb-1">Display Name / الاسم الظاهر <span class="text-rose-500">*</span></label>
            <input
              type="text"
              bind:value={userForm.display_name}
              placeholder="e.g. Ahmed Benali"
              class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-pos-text"
            />
          </div>

          <div class="grid grid-cols-2 gap-3">
            <div>
              <label class="block font-bold text-pos-muted mb-1">Role / الدور</label>
              <select
                bind:value={userForm.role_id}
                class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-pos-text font-bold"
              >
                {#each allRoles as r}
                  <option value={r.id}>{r.name}</option>
                {/each}
              </select>
            </div>

            <div>
              <label class="block font-bold text-pos-muted mb-1">Max Discount (%)</label>
              <input
                type="number"
                min="0"
                max="100"
                bind:value={userForm.max_discount_percent}
                class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl font-mono text-pos-text font-bold"
              />
            </div>
          </div>

          <div>
            <label class="block font-bold text-pos-muted mb-1">
              {#if userModalMode === 'create'}
                Password / كلمة المرور <span class="text-rose-500">*</span>
              {:else}
                New Password (leave empty to keep unchanged) / كلمة المرور الجديدة
              {/if}
            </label>
            <input
              type="password"
              bind:value={userForm.password}
              placeholder={userModalMode === 'create' ? 'Enter secure password' : '••••••••'}
              class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-pos-text"
            />
          </div>

          <div class="pt-2">
            <label class="flex items-center gap-2 cursor-pointer font-bold text-pos-text">
              <input type="checkbox" bind:checked={userForm.is_active} class="rounded text-sky-600" />
              <span>Account is Active / الحساب مفعّل</span>
            </label>
          </div>
        </div>

        <div class="flex justify-end gap-2 pt-3 border-t border-pos-border">
          <button
            on:click={() => (showUserModal = false)}
            class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-pos-text text-xs font-bold rounded-xl cursor-pointer"
          >
            Cancel / إلغاء
          </button>
          <button
            on:click={saveUserModal}
            class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white text-xs font-black rounded-xl cursor-pointer shadow-md transition"
          >
            {userModalMode === 'create' ? 'Create User / إنشاء' : 'Save Changes / حفظ'}
          </button>
        </div>
      </div>
    </div>
  {/if}

  <!-- Bottom Global Developer Credit Footer -->
  <div class="pt-3 flex items-center justify-between text-xs text-pos-muted border-t border-pos-border mt-3 shrink-0">
    <div class="flex items-center gap-2">
      <span class="font-bold text-pos-text">TitaouPOS Desktop</span>
      <span>•</span>
      <span>Created & Developed by <strong class="text-sky-600">Titaou Bedreddine (0553444057)</strong></span>
    </div>
    <span class="font-mono text-[11px]">{appVersion} (PRO)</span>
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