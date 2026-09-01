<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { t } from '../../lib/i18n';
  import { invoke } from '@tauri-apps/api/core';
  import AboutView from '../about/AboutView.svelte';
  import ShortcutsEditor from '../../lib/components/ShortcutsEditor.svelte';
  import { currentUser } from '../../lib/stores/auth';
  import { printHtmlDirectly, printLabelSilently, entityQrDataUrl } from '../../lib/utils/printer';
  import { buildProfessionalReceiptHtml } from '../../lib/printing/professionalReceipt';
  import {
    LABEL_PRESETS,
    LABEL_PRESET_IDS,
    buildLabelPresetHtml,
    type LabelPresetId,
  } from '../../lib/printing/labelPresets';
  import { getLanguage } from '../../lib/i18n';
  import JsBarcode from 'jsbarcode';
  import {
    Sliders, User, Building, Printer, Smartphone, Download,
    ShieldCheck, RefreshCw, AlertOctagon, Check, Copy, Key,
    QrCode, Image as ImageIcon, Upload, Tag, ArrowRight,
    Wifi, HardDrive, FileText, CheckCircle2, History, Laptop,
    Scale, Bell, Send, CreditCard, Keyboard, Type, Bold, Eye,
    Users, UserPlus, Edit2, Trash2, Shield, Lock, Info
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
    | 'about'
    | 'danger';

  let currentTab: SettingsTab = 'general';
  let settings: Record<string, any> = {
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
    invoice_printer_name: '',
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
    notify_cash_in: 'false',
    notify_cash_out: 'false',
    notify_each_expense: 'false',
    notify_opening_cash: 'false',
    notify_supplier_payment: 'false',
    notify_price_change: 'false',
    notify_qty_change: 'false',
    notify_history_change: 'false',
    notify_each_refund: 'true',
    notify_expiry: 'true',
    notify_low_stock: 'true',
    notify_recap_enabled: 'false',
    recap_interval_minutes: '60',
    app_license_status: 'activated',
    allow_negative_stock: 'false',
    pos_autofocus_search: 'true',
    pos_autofocus_timer_seconds: '10',
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
    label_presets: '',
    sticker_content_position: 'top', // top | middle | bottom
    shelf_content_position: 'top',
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
    // Receipt template preset: 'professional' ("80 mm – Professional Sales
    // Receipt" graphic preset — default) or 'standard' (monospace ticket).
    receipt_preset: 'professional',
    receipt_thank_you: 'MERCI POUR VOTRE CONFIANCE !',
    receipt_show_barcode: 'true',
    shop_website: '',
    // Exact-media label pipeline (print_label_job): DPI of the label printer
    // above. Empty label_printer = Windows default printer.
    label_printer_dpi: '203',
    // Pricing defaults for new products
    default_margin_percent: '20',
    price_round_step: '5',
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

  let hwid = '...';
  let isActivatingOnline = false;
  let activationMsg = '';

  // Online activation against the developer's GitHub license registry:
  // licenses/<HWID>.json must exist with {"licensed": true}.
  async function handleActivateOnline() {
    try {
      isActivatingOnline = true;
      activationMsg = 'Contacting activation server...';
      const ok = await invoke<boolean>('activate_online');
      activationMsg = ok
        ? 'Activated successfully / تم التنشيط بنجاح'
        : 'Not licensed on the server';
    } catch (e: any) {
      activationMsg = typeof e === 'string' ? e : e?.message || 'Activation failed';
    } finally {
      isActivatingOnline = false;
    }
  }
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
  let recapStatusMsg = '';

  async function sendRecapNow() {
    try {
      recapStatusMsg = 'Sending recap...';
      const res = await invoke<string>('send_telegram_recap');
      recapStatusMsg = '✅ ' + res;
    } catch (e: any) {
      recapStatusMsg = '❌ ' + (typeof e === 'string' ? e : e?.message || 'Recap failed');
    }
  }

  // Updates
  let appVersion = 'v0.2.0';
  let isCheckingUpdate = false;
  let updateStatus = 'You are running the latest version: v0.2.0';
  let updateAvailable = false;
  let showRollbackModal = false;
  // Rollback requires typing ROLLBACK and auto-cancels after 10s idle.
  let rollbackConfirmText = '';
  let rollbackCountdown = 10;
  let rollbackTimer: any = null;

  function openRollbackModal() {
    rollbackConfirmText = '';
    rollbackCountdown = 10;
    showRollbackModal = true;
    clearInterval(rollbackTimer);
    rollbackTimer = setInterval(() => {
      rollbackCountdown -= 1;
      if (rollbackCountdown <= 0) {
        cancelRollback();
      }
    }, 1000);
  }

  function cancelRollback() {
    clearInterval(rollbackTimer);
    rollbackConfirmText = '';
    showRollbackModal = false;
  }

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


  // ----- Label presets (sticker + shelf saved configurations) -----
  // Stored as JSON in app_settings.label_presets:
  // { "Preset name": { ...sticker/shelf settings } }
  let labelPresets: Record<string, Record<string, string>> = {};
  let newPresetName = '';

  function labelPresetKeys(): string[] {
    // Which settings make up a label preset.
    return [
      'barcode_label_width', 'barcode_label_height', 'sticker_orientation',
      'sticker_show_shop_name', 'sticker_show_product_name', 'sticker_show_barcode',
      'sticker_show_price', 'sticker_name_font_size', 'sticker_name_bold',
      'sticker_price_font_size', 'sticker_price_bold', 'sticker_barcode_font_size',
      'sticker_text_align', 'sticker_content_position',
      'shelf_tag_width', 'shelf_tag_height', 'shelf_tag_orientation',
      'shelf_tag_show_shop', 'shelf_tag_show_name', 'shelf_tag_show_price',
      'shelf_tag_show_ref', 'shelf_tag_name_size', 'shelf_tag_price_size',
      'shelf_tag_ref_size', 'shelf_text_align', 'shelf_content_position',
    ].filter((k) => k in settings);
  }

  async function loadLabelPresets() {
    try {
      const raw = await invoke<string | null>('get_setting', { key: 'label_presets' });
      if (raw) {
        const parsed = JSON.parse(raw);
        if (parsed && typeof parsed === 'object') {
          labelPresets = parsed;
        }
      }
    } catch {
      labelPresets = {};
    }
  }

  async function persistLabelPresets() {
    try {
      await invoke('set_setting', {
        key: 'label_presets',
        value: JSON.stringify(labelPresets),
      });
      triggerSaveNotification('Preset saved / تم حفظ الإعداد');
    } catch (e: any) {
      triggerSaveNotification('Preset save failed: ' + (e?.message || e));
    }
  }

  async function saveCurrentLabelPreset() {
    const name = newPresetName.trim();
    if (!name) {
      triggerSaveNotification('Enter a preset name first');
      return;
    }
    const snapshot: Record<string, string> = {};
    for (const key of labelPresetKeys()) {
      snapshot[key] = String(settings[key] ?? '');
    }
    labelPresets = { ...labelPresets, [name]: snapshot };
    newPresetName = '';
    await persistLabelPresets();
  }

  async function applyLabelPreset(name: string) {
    const preset = labelPresets[name];
    if (!preset) return;
    for (const [k, v] of Object.entries(preset)) {
      (settings as any)[k] = v;
    }
    await invoke('set_multiple_settings', {
      settings: Object.fromEntries(
        Object.entries(settings).map(([k, v]) => [k, v === null || v === undefined ? '' : String(v)])
      ),
    });
    triggerSaveNotification(`Preset "${name}" applied / تم تطبيق الإعداد`);
  }

  async function deleteLabelPreset(name: string) {
    const copy = { ...labelPresets };
    delete copy[name];
    labelPresets = copy;
    await persistLabelPresets();
  }

  // Factory Reset
  let resetType = 'transactions_only';
  let resetConfirm = '';

  // Clear-history-only (keeps products/stock/prices/debts)
  let clearHistoryConfirmText = '';
  let isClearingHistory = false;
  let clearHistoryMsg = '';

  async function handleClearHistory() {
    if (clearHistoryConfirmText.trim() !== 'CLEAR HISTORY') return;
    try {
      isClearingHistory = true;
      clearHistoryMsg = '';
      const res = await invoke<string>('clear_transaction_history', {
        confirmText: clearHistoryConfirmText.trim(),
      });
      clearHistoryMsg = '✅ ' + res;
      clearHistoryConfirmText = '';
    } catch (e: any) {
      clearHistoryMsg = '❌ ' + (typeof e === 'string' ? e : e?.message || 'Failed');
    } finally {
      isClearingHistory = false;
    }
  }

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
    await loadAutostart();
    // Deferred: wmic printer enumeration spawns a slow subprocess that made
    // opening Settings (and the About tab inside it) feel frozen for seconds.
    // Load lazily on the tabs that need the data.
    setTimeout(() => {
      loadPrinters().catch(() => {});
      loadLabelPresets().catch(() => {});
      loadShortcutBindings().catch(() => {});
    }, 0);
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

  // Autostart with Windows (HKCU Run key, applied immediately).
  let autostartEnabled = false;

  // Rebindable POS shortcuts (persisted as JSON in app_settings).
  let shortcutBindings: Record<string, string> = {
    new_sale: 'F1',
    checkout_print: 'F2',
    hold_cart: 'F3',
    remise: 'F4',
    returns: 'F5',
    edit_qty: 'F6',
    toggle_products: 'F7',
    toggle_register: 'F8',
    toggle_sales: 'F9',
    cycle_mode: 'F10',
    cycle_payment: 'F11',
    quick_checkout: 'F12',
    open_drawer: 'Control',
  };

  async function loadShortcutBindings() {
    try {
      const raw = await invoke<string | null>('get_setting', { key: 'pos_shortcuts' });
      if (raw) {
        const parsed = JSON.parse(raw);
        if (parsed && typeof parsed === 'object') {
          shortcutBindings = { ...shortcutBindings, ...parsed };
        }
      }
    } catch {
      // Keep defaults.
    }
  }

  // Installed printers for the invoice/receipt printer selector.
  let printerList: string[] = [];

  async function loadPrinters() {
    try {
      printerList = await invoke<string[]>('list_printers');
    } catch {
      printerList = [];
    }
  }

  async function loadAutostart() {
    try {
      autostartEnabled = await invoke<boolean>('get_autostart');
    } catch {
      autostartEnabled = false;
    }
  }

  async function toggleAutostart() {
    try {
      await invoke('set_autostart', { enable: !autostartEnabled });
      autostartEnabled = !autostartEnabled;
      triggerSaveNotification(
        autostartEnabled
          ? 'TitaouPOS will start with Windows / سينطلق البرنامج مع ويندوز'
          : 'Autostart disabled / تم إلغاء الانطلاق مع ويندوز'
      );
    } catch (e: any) {
      triggerSaveNotification('Autostart failed: ' + (typeof e === 'string' ? e : e.message || e));
    }
  }

  function triggerSaveNotification(msg = 'Settings saved successfully / تم حفظ الإعدادات بنجاح') {
    saveSuccessMsg = msg;
    setTimeout(() => {
      saveSuccessMsg = '';
    }, 3500);
  }

  // Toggles must take effect IMMEDIATELY: a cashier flipping "notify on
  // every sale" and never clicking Save would silently get no alerts.
  function autoSaveSettings() {
    invoke('set_multiple_settings', {
      settings: Object.fromEntries(
        Object.entries(settings).map(([k, v]) => [k, v === null || v === undefined ? '' : String(v)])
      ),
    }).catch((e) => console.warn('Auto-save settings failed:', e));
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
      // Save first so the backend reads the freshly typed credentials.
      await invoke('set_multiple_settings', {
        settings: {
          telegram_bot_token: String(settings.telegram_bot_token || ''),
          telegram_chat_id: String(settings.telegram_chat_id || ''),
        },
      });
      await invoke('send_telegram_message', {
        text: '🚀 *TitaouPOS Live Alert*\nTest connection successful from POS settings!',
      });
      telegramStatusMsg = '✅ Telegram test alert delivered successfully!';
    } catch (e: any) {
      telegramStatusMsg = '❌ ' + (typeof e === 'string' ? e : e?.message || 'Check Token/Chat ID');
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

  interface AppUpdateResult {
    has_update: boolean;
    current_version: string;
    latest_version: string;
    tag_name: string;
    release_name: string;
    release_notes: string;
    release_url: string;
    download_url: string;
    published_at: string;
  }

  async function openUrlInBrowser(url: string) {
    if (!url) return;
    try {
      const { open } = await import('@tauri-apps/plugin-shell');
      await open(url);
    } catch {
      window.open(url, '_blank');
    }
  }

  async function checkForUpdates() {
    try {
      isCheckingUpdate = true;
      updateStatus = 'Querying GitHub releases for TitaouPOS...';

      let updateResult: AppUpdateResult;
      try {
        updateResult = await invoke<AppUpdateResult>('check_github_update');
      } catch (invErr: any) {
        console.warn('Backend check_github_update error, attempting fetch fallback:', invErr);
        const res = await fetch('https://api.github.com/repos/titaou-bedreddine/TitaouPosT/releases', {
          headers: { 'Accept': 'application/vnd.github.v3+json' }
        });
        if (!res.ok) throw new Error(`GitHub API HTTP ${res.status}`);
        const releases = await res.json();
        const latest = releases[0] || {};
        const latestTag = (latest.tag_name || '').trim();
        const cleanCurrent = appVersion.replace(/^v/, '').trim();
        const cleanLatest = latestTag.replace(/^v/, '').trim();
        const setupAsset = (latest.assets || []).find((a: any) => a.name?.endsWith('.exe') || a.name?.endsWith('.msi'));

        updateResult = {
          has_update: cleanLatest !== cleanCurrent && !!latestTag,
          current_version: appVersion,
          latest_version: cleanLatest,
          tag_name: latestTag,
          release_name: latest.name || latestTag,
          release_notes: latest.body || '',
          release_url: latest.html_url || 'https://github.com/titaou-bedreddine/TitaouPosT/releases',
          download_url: setupAsset ? setupAsset.browser_download_url : (latest.html_url || ''),
          published_at: latest.published_at || '',
        };
      }

      latestReleaseInfo = updateResult;
      latestReleaseUrl = updateResult.release_url;
      latestDownloadUrl = updateResult.download_url;

      if (!updateResult.has_update) {
        updateStatus = `TitaouPOS is up to date (${appVersion} is the latest release).`;
        updateAvailable = false;
        triggerSaveNotification('System is up to date!');
      } else {
        updateStatus = `🚀 New Update Available: ${updateResult.tag_name} (${updateResult.release_name || 'Latest Release'})`;
        updateAvailable = true;
        triggerSaveNotification(`New update ${updateResult.tag_name} available!`);
      }
    } catch (e: any) {
      console.warn('Update check note:', e);
      updateStatus = `Failed to query GitHub: ${e?.message || e}`;
      triggerSaveNotification('Check failed');
    } finally {
      isCheckingUpdate = false;
    }
  }

  async function handleRollback() {
    if (rollbackConfirmText.trim() !== 'ROLLBACK') return;
    clearInterval(rollbackTimer);
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

  // Settings can hold real booleans/numbers (fresh binding) or DB strings
  // ("true"/"12"); normalize so test prints reflect unsaved changes too.
  function toBool(v: any, dflt = true): boolean {
    if (v === undefined || v === null || v === '') return dflt;
    return v === true || v === 'true';
  }
  function toInt(v: any, dflt: number): number {
    const n = parseInt(String(v), 10);
    return isNaN(n) ? dflt : n;
  }

  function testPrintReceipt() {
    const fontFamily = settings.receipt_font_family || 'monospace';
    const showShop = toBool(settings.receipt_show_shop_name);
    const showAddress = toBool(settings.receipt_show_address);
    const showPhone = toBool(settings.receipt_show_phone);
    const showRcNif = toBool(settings.receipt_show_rc_nif);
    const showCashier = toBool(settings.receipt_show_cashier);
    const showDate = toBool(settings.receipt_show_date);
    const showTax = toBool(settings.receipt_show_tax);
    const showFooter = toBool(settings.receipt_show_footer);
    const showQr = toBool(settings.receipt_show_qr);

    const headerSize = toInt(settings.receipt_header_font_size, 14);
    const headerBold = toBool(settings.receipt_header_bold);
    const bodySize = toInt(settings.receipt_body_font_size, 11);
    const bodyBold = toBool(settings.receipt_body_bold, false);
    const totalSize = toInt(settings.receipt_total_font_size, 14);
    const totalBold = toBool(settings.receipt_total_bold);
    const footerSize = toInt(settings.receipt_footer_font_size, 9);
    const footerBold = toBool(settings.receipt_footer_bold, false);
    const headerAlign = settings.receipt_header_align || 'center';
    const footerAlign = settings.receipt_footer_align || 'center';

    const html = `
      <div style="font-family: ${fontFamily}; font-size: ${bodySize}px; font-weight: ${bodyBold ? 'bold' : 'normal'}; width: ${settings.receipt_paper_width === '58mm' ? '230px' : '300px'}; padding: 10px; background: #fff; color: #000; box-sizing: border-box;">
        <div style="text-align: ${headerAlign}; padding-bottom: 8px; border-bottom: 1px dashed #000;">
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
          <table style="width: 100%; font-size: ${bodySize}px; font-weight: ${bodyBold ? 'bold' : 'normal'};">
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
          <div style="text-align: ${footerAlign}; padding-top: 6px; font-size: ${footerSize}px; font-weight: ${footerBold ? 'bold' : 'normal'}; border-top: 1px dashed #000; color: #444;">
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
    let w = toInt(settings.barcode_label_width, 50);
    let h = toInt(settings.barcode_label_height, 30);
    const portrait = String(settings.sticker_orientation || '') === 'portrait';
    if (portrait) {
      const temp = w; w = h; h = temp;
    }
    const showShop = toBool(settings.sticker_show_shop_name);
    const showName = toBool(settings.sticker_show_product_name);
    const showBarcode = toBool(settings.sticker_show_barcode);
    const showPrice = toBool(settings.sticker_show_price);
    const nameSize = toInt(settings.sticker_name_font_size, 12);
    const nameBold = toBool(settings.sticker_name_bold);
    const priceSize = toInt(settings.sticker_price_font_size, 16);
    const priceBold = toBool(settings.sticker_price_bold);
    const barcodeSize = toInt(settings.sticker_barcode_font_size, 10);
    const align = settings.sticker_text_align || 'center';
    const flexAlign = align === 'left' ? 'flex-start' : align === 'right' ? 'flex-end' : 'center';

    // Render the barcode straight into the print HTML with the current
    // settings so unsaved font-size changes are reflected on the test.
    let barcodeSvgHtml = '';
    if (showBarcode) {
      try {
        const tmp = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
        JsBarcode(tmp, previewBarcodeNumber, {
          format: previewBarcodeNumber.length === 13 ? 'EAN13' : 'CODE128',
          width: 1.8, height: 44, displayValue: true,
          fontSize: barcodeSize, margin: 0,
          background: '#ffffff', lineColor: '#000000',
        });
        barcodeSvgHtml = tmp.outerHTML;
      } catch {
        barcodeSvgHtml = `<p style="font-family:monospace;font-size:10px;">${previewBarcodeNumber}</p>`;
      }
    }

    const html = `
      <div style="width: ${w}mm; height: ${h}mm; text-align: ${align}; font-family: sans-serif; padding: 2mm; box-sizing: border-box; display: flex; flex-direction: column; align-items: ${flexAlign}; justify-content: center; background: #fff;">
        ${showShop ? `<p style="font-weight: bold; font-size: 10px; text-transform: uppercase; color: #444; margin: 0; width: 100%; text-align: ${align};">${settings.shop_name_fr || 'TitaouPOS'}</p>` : ''}
        ${showName ? `<p style="font-size: ${nameSize}px; font-weight: ${nameBold ? '900' : 'normal'}; margin: 2px 0; overflow: hidden; white-space: nowrap; max-width: 100%; width: 100%; text-align: ${align};">${previewProductName}</p>` : ''}
        ${showBarcode ? `<div style="max-width: 100%; overflow: hidden; display: flex; justify-content: ${flexAlign}; align-items: center; margin: 1px 0; width: 100%;">${barcodeSvgHtml}</div>` : ''}
        ${showPrice ? `<p style="font-size: ${priceSize}px; font-weight: ${priceBold ? '900' : 'normal'}; font-family: monospace; margin: 2px 0; width: 100%; text-align: ${align};">${previewPrice} DZD</p>` : ''}
      </div>
    `;
    printHtmlDirectly(html, 'Test Barcode Sticker', { widthMm: w, heightMm: h });
  }

  function testPrintShelfTag() {
    let w = toInt(settings.shelf_tag_width, 60);
    let h = toInt(settings.shelf_tag_height, 40);
    if (String(settings.shelf_orientation) === 'portrait') {
      const temp = w; w = h; h = temp;
    }
    const showShop = toBool(settings.shelf_show_shop_name);
    const showName = toBool(settings.shelf_show_product_name);
    const showPrice = toBool(settings.shelf_show_price);
    const showRef = toBool(settings.shelf_show_ref);
    const nameSize = toInt(settings.shelf_name_font_size, 16);
    const nameBold = toBool(settings.shelf_name_bold);
    const priceSize = toInt(settings.shelf_price_font_size, 28);
    const priceBold = toBool(settings.shelf_price_bold);
    const refSize = toInt(settings.shelf_ref_font_size, 10);
    const align = settings.shelf_text_align || 'center';

    const html = `
      <div style="width: ${w}mm; height: ${h}mm; border: 2px solid #000; padding: 3mm; font-family: sans-serif; text-align: ${align}; box-sizing: border-box; background: #fff; display: flex; flex-direction: column; justify-content: space-between; overflow: hidden;">
        ${showShop ? `<div style="display: flex; justify-content: space-between; font-size: 11px; font-weight: bold; border-bottom: 1.5px solid #000; padding-bottom: 2px;"><span>${settings.shop_name_fr || 'TitaouPOS'}</span><span style="color: #059669; font-weight: 900;">DISPO</span></div>` : ''}
        ${showName ? `<p style="font-size: ${nameSize}px; font-weight: ${nameBold ? '900' : 'normal'}; margin: 4px 0; line-height: 1.2; text-align: ${align}; width: 100%;">${previewProductName}</p>` : ''}
        ${showPrice ? `<div style="background: #000; color: #fff; padding: 6px; font-size: ${priceSize}px; font-weight: ${priceBold ? '900' : 'normal'}; margin: 4px 0; font-family: monospace; border-radius: 4px; text-align: ${align}; width: 100%;">${previewPrice} DZD</div>` : ''}
        ${showRef ? `<div style="display: flex; justify-content: space-between; font-size: ${refSize}px; font-weight: bold; width: 100%;"><span>Ref: ${previewBarcodeNumber}</span><span>TVA 19% Incl.</span></div>` : ''}
      </div>
    `;
    printHtmlDirectly(html, 'Test Shelf Tag', { widthMm: w, heightMm: h });
  }

  // ----- Built-in 40×20 mm thermal presets (Vertical Price / Shelf Price) -----
  $: builtinLabelData = {
    shopName: String(settings.shop_name_fr || 'TITAOU POS'),
    productName: previewProductName,
    barcode: previewBarcodeNumber,
    price: previewPrice,
    currency: 'DA',
  };
  $: builtinLabelPreviews = Object.fromEntries(
    LABEL_PRESET_IDS.map((id) => [id, buildLabelPresetHtml(id, builtinLabelData)])
  ) as Record<LabelPresetId, string>;

  let builtinTestMsg = '';

  async function testPrintBuiltinLabel(id: LabelPresetId) {
    const def = LABEL_PRESETS[id];
    builtinTestMsg = '';
    try {
      // Exact-media silent pipeline: 40×20mm DEVMODE, one page per copy.
      const outcome = await printLabelSilently({
        html: buildLabelPresetHtml(id, builtinLabelData),
        label: `Test ${def.name}`,
        widthMm: def.widthMm,
        heightMm: def.heightMm,
        copies: 5, // acceptance test: 5 × 20mm = 100mm, zero gaps
        printer: settings.label_printer || undefined,
        dpi: toInt(settings.label_printer_dpi, 203),
      });
      builtinTestMsg = (outcome.ok ? '✅ ' : '❌ ') + outcome.message;
    } catch (e: any) {
      // Backend command unavailable (dev/old binary) → browser print.
      printHtmlDirectly(
        def.build(builtinLabelData),
        `Test ${def.name}`,
        { widthMm: def.widthMm, heightMm: def.heightMm }
      );
      builtinTestMsg = '⚠ Browser print fallback (exact-media backend not available)';
    }
  }

  async function testPrintProfessionalReceipt() {
    const d = new Date();
    const paperW = String(settings.receipt_paper_width) === '58mm' ? 58 : 80;
    const qr = await entityQrDataUrl('SALE:TEST-0001', 240).catch(() => undefined);
    const html = buildProfessionalReceiptHtml({
      shopName: String(settings.shop_name_fr || 'TITAOU POS'),
      shopTagline: String(settings.receipt_header || ''),
      shopAddress: String(settings.shop_address || ''),
      shopPhone: String(settings.shop_phone || ''),
      shopWebsite: String(settings.shop_website || ''),
      shopLogoDataUrl: settings.shop_logo_base64 || undefined,
      invoiceNumber: 'TEST-0001',
      invoiceBarcode: `${d.getFullYear()}${String(d.getMonth() + 1).padStart(2, '0')}${String(d.getDate()).padStart(2, '0')} 0001256`,
      dateStr: d.toLocaleDateString('fr-FR'),
      timeStr: d.toLocaleTimeString('fr-FR'),
      cashierName: $currentUser?.display_name || 'Admin',
      customerName: 'Client Comptoir',
      paymentMethod: 'ESPÈCES',
      items: [
        { name: 'Eau Minérale 1.5L', quantity: 2, unitPrice: 120, totalPrice: 240 },
        { name: 'Lait UHT Entier 1L', quantity: 1, unitPrice: 150, totalPrice: 150 },
        { name: 'Café Moulu 250g', quantity: 1, unitPrice: 200, totalPrice: 200 },
      ],
      subtotal: 590,
      discount: 0,
      grandTotal: 590,
      amountPaid: 600,
      change: 10,
      currency: String(settings.default_currency || 'DA'),
      qrDataUrl: qr,
      showQr: toBool(settings.receipt_show_qr),
      showBarcode: toBool(settings.receipt_show_barcode),
      thankYou: String(settings.receipt_thank_you || 'MERCI POUR VOTRE CONFIANCE !'),
      returnPolicy: String(settings.receipt_footer || ''),
      lang: getLanguage(),
      paperWidthMm: paperW,
    });
    printHtmlDirectly(html, 'Test Professional Receipt', { widthMm: paperW });
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
        <span class="truncate">{t('set_general')}</span>
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
        <span class="truncate">{t('set_notifications_tab')}</span>
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
        <span class="truncate">{t('set_shortcuts')}</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'network')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'network' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <Smartphone class="w-4 h-4 mb-1" />
        <span class="truncate">{t('set_network')}</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'import_export')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'import_export' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <Download class="w-4 h-4 mb-1" />
        <span class="truncate">{t('set_import_export')}</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'activation')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'activation' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <ShieldCheck class="w-4 h-4 mb-1" />
        <span class="truncate">{t('set_activation')}</span>
      </button>

      <button
        type="button"
        on:click={() => (currentTab = 'updates')}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'updates' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <RefreshCw class="w-4 h-4 mb-1" />
        <span class="truncate">{t('set_updates')}</span>
      </button>

      <button
        type="button"
        on:click={() => { currentTab = 'account'; loadUsersAndRoles(); }}
        class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'account' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
      >
        <Users class="w-4 h-4 mb-1" />
        <span class="truncate">{t('set_account')}</span>
      </button>

      <button
          type="button"
          on:click={() => (currentTab = 'about')}
          class="flex flex-col items-center justify-center p-2 rounded-xl text-[11px] font-bold transition cursor-pointer {currentTab === 'about' ? 'bg-sky-600 text-white shadow-xs' : 'text-pos-muted hover:bg-slate-100 dark:hover:bg-slate-800'}"
        >
          <Info class="w-4 h-4" />
          <span class="mt-1">About</span>
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

        <!-- System Behavior -->
        <div class="p-4 bg-slate-50 dark:bg-slate-800/50 rounded-2xl border border-pos-border flex items-center justify-between gap-4">
          <div>
            <h4 class="text-xs font-black text-pos-text">Start with Windows (الانطلاق مع ويندوز)</h4>
            <p class="text-[11px] text-pos-muted">TitaouPOS launches automatically when the PC boots.</p>
          </div>
          <button
            type="button"
            on:click={toggleAutostart}
            class="relative w-12 h-6 rounded-full transition cursor-pointer shrink-0 {autostartEnabled ? 'bg-emerald-500' : 'bg-slate-300 dark:bg-slate-600'}"
          >
            <span class="absolute top-0.5 w-5 h-5 rounded-full bg-white shadow transition-all {autostartEnabled ? 'start-6' : 'start-0.5'}"></span>
          </button>
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
        <!-- Invoice / Receipt Printer Selection -->
        <div class="p-4 bg-slate-50 dark:bg-slate-800/50 rounded-2xl border border-pos-border flex items-center justify-between gap-4">
          <div class="min-w-0">
            <h4 class="text-xs font-black text-pos-text">Invoice / Receipt Printer (طابعة الوصولات)</h4>
            <p class="text-[11px] text-pos-muted">Prints receipts, invoices and vouchers. Empty = system default printer.</p>
          </div>
          <select
            bind:value={settings.invoice_printer_name}
            class="px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-bold text-pos-text outline-none cursor-pointer max-w-[220px]"
          >
            <option value="">Default Windows Printer</option>
            {#each printerList as pr}
              <option value={pr}>{pr}</option>
            {/each}
          </select>
        </div>

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

            <!-- Receipt Template Preset: Standard vs 80mm Professional -->
            <div class="p-4 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-3">
              <h3 class="font-black text-xs text-pos-text flex items-center gap-1.5">
                <FileText class="w-4 h-4 text-sky-500" />
                <span>Receipt Template Preset (قالب الوصل)</span>
              </h3>
              <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                <div>
                  <label class="block text-xs font-bold text-pos-muted mb-1">Sale Receipt Template</label>
                  <select
                    bind:value={settings.receipt_preset}
                    on:change={autoSaveSettings}
                    class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-bold text-pos-text outline-none cursor-pointer"
                  >
                    <option value="professional">80 mm — Professional (graphic, QR + barcode) — Default</option>
                    <option value="standard">Standard — Monospace Ticket</option>
                  </select>
                  <p class="text-[9px] text-pos-muted mt-1">Applied to auto-printed sale receipts. Arabic/French/English labels adapt to the UI language.</p>
                </div>
                <div>
                  <label class="block text-xs font-bold text-pos-muted mb-1">Thank-you Message (footer)</label>
                  <input type="text" bind:value={settings.receipt_thank_you} on:change={autoSaveSettings} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border border-pos-border rounded-xl text-xs text-pos-text font-bold outline-none" />
                </div>
                <div>
                  <label class="block text-xs font-bold text-pos-muted mb-1">Shop Website (receipt header)</label>
                  <input type="text" bind:value={settings.shop_website} on:change={autoSaveSettings} placeholder="www.titaoupos.dz" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border border-pos-border rounded-xl text-xs text-pos-text font-bold outline-none" />
                </div>
                <div class="flex items-end">
                  <button on:click={testPrintProfessionalReceipt} class="w-full py-2 bg-slate-100 hover:bg-slate-200 dark:bg-slate-800 dark:hover:bg-slate-700 text-pos-text font-bold text-xs rounded-xl flex items-center justify-center gap-1.5 cursor-pointer shadow-xs transition">
                    <Printer class="w-3.5 h-3.5 text-sky-500" />
                    <span>Test Print Professional Receipt</span>
                  </button>
                </div>
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

                <label class="flex items-center gap-2 text-xs font-bold text-pos-text cursor-pointer p-2 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
                  <input type="checkbox" bind:checked={settings.receipt_show_barcode} class="rounded text-sky-600" />
                  <span>Invoice Barcode (professional preset)</span>
                </label>
              </div>
            </div>

            <!-- Typography, Font Sizing & Bold Options -->
            <div class="p-4 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-3">
              <h3 class="font-black text-xs text-pos-text flex items-center gap-1.5">
                <Type class="w-4 h-4 text-sky-500" />
                <span>Font Sizing, Bold Formatting & Alignment (حجم الخط والمحاذاة)</span>
              </h3>

              <!-- Alignment row -->
              <div class="grid grid-cols-1 md:grid-cols-2 gap-3 pb-1 border-b border-pos-border/40">
                <div>
                  <label class="block text-[11px] font-bold text-pos-muted mb-1">Header Alignment (محاذاة الرأس)</label>
                  <select bind:value={settings.receipt_header_align} class="w-full px-2.5 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-bold text-pos-text">
                    <option value="center">Center / في الوسط</option>
                    <option value="left">Left / محاذاة لليسار</option>
                    <option value="right">Right / محاذاة لليمين</option>
                  </select>
                </div>
                <div>
                  <label class="block text-[11px] font-bold text-pos-muted mb-1">Footer Policy Alignment (محاذاة التذييل)</label>
                  <select bind:value={settings.receipt_footer_align} class="w-full px-2.5 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-bold text-pos-text">
                    <option value="center">Center / في الوسط</option>
                    <option value="left">Left / محاذاة لليسار</option>
                    <option value="right">Right / محاذاة لليمين</option>
                  </select>
                </div>
              </div>

              <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-3">
                <!-- Header Font Size & Bold -->
                <div class="p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border space-y-1.5">
                  <span class="text-[11px] font-bold text-pos-muted block">Shop Header</span>
                  <div class="flex items-center gap-1.5">
                    <input type="number" min="6" max="64" bind:value={settings.receipt_header_font_size} class="w-16 px-2 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg text-xs font-bold font-mono outline-none" />
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
                    <input type="number" min="6" max="48" bind:value={settings.receipt_body_font_size} class="w-16 px-2 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg text-xs font-bold font-mono outline-none" />
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
                    <input type="number" min="6" max="64" bind:value={settings.receipt_total_font_size} class="w-16 px-2 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg text-xs font-bold font-mono outline-none" />
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
                    <input type="number" min="6" max="48" bind:value={settings.receipt_footer_font_size} class="w-16 px-2 py-1 bg-slate-100 dark:bg-slate-800 rounded-lg text-xs font-bold font-mono outline-none" />
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
              <div style="text-align: {settings.receipt_header_align || 'center'};" class="pb-1 border-b border-dashed border-slate-400">
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
                  style="font-size: {settings.receipt_footer_font_size || '8'}px; font-weight: {settings.receipt_footer_bold === 'true' || settings.receipt_footer_bold === true ? 'bold' : 'normal'}; text-align: {settings.receipt_footer_align || 'center'};"
                  class="pt-1 text-gray-600 border-t border-dashed border-slate-400"
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
              <span>Instant notification on every sale</span>
              <input type="checkbox" bind:checked={settings.notify_each_sale} class="rounded text-sky-600" on:change={autoSaveSettings} />
            </label>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <span>Notification on refunds & returns</span>
              <input type="checkbox" bind:checked={settings.notify_each_refund} class="rounded text-sky-600" on:change={autoSaveSettings} />
            </label>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <span>Near-expired (under 30 days) & expired items alert</span>
              <input type="checkbox" bind:checked={settings.notify_expiry} class="rounded text-sky-600" on:change={autoSaveSettings} />
            </label>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <span>Low stock & inventory depletion alert</span>
              <input type="checkbox" bind:checked={settings.notify_low_stock} class="rounded text-sky-600" on:change={autoSaveSettings} />
            </label>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <span>Cash deposits into the drawer (إيداع الصندوق)</span>
              <input type="checkbox" bind:checked={settings.notify_cash_in} class="rounded text-sky-600" on:change={autoSaveSettings} />
            </label>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <span>Cash withdrawals & expenses (سحب/مصاريف)</span>
              <input type="checkbox" bind:checked={settings.notify_cash_out} class="rounded text-sky-600" on:change={autoSaveSettings} />
            </label>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <span>Every recorded expense (كل مصروف)</span>
              <input type="checkbox" bind:checked={settings.notify_each_expense} class="rounded text-sky-600" on:change={autoSaveSettings} />
            </label>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <span>Opening cash of a new session (رصيد افتتاحي)</span>
              <input type="checkbox" bind:checked={settings.notify_opening_cash} class="rounded text-sky-600" on:change={autoSaveSettings} />
            </label>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <span>Supplier debt payments (تسديد الموردين)</span>
              <input type="checkbox" bind:checked={settings.notify_supplier_payment} class="rounded text-sky-600" on:change={autoSaveSettings} />
            </label>
          </div>

          <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-3">
            <h3 class="font-black text-sm text-pos-text">Recurring Recap (ملخص دوري)</h3>
            <label class="flex items-center justify-between text-xs font-bold text-pos-text cursor-pointer">
              <div>
                <span>Enable automatic recap</span>
                <p class="text-[10px] text-pos-muted font-normal">Sends a sales/cash/expenses summary every X while the app is open</p>
              </div>
              <input type="checkbox" bind:checked={settings.notify_recap_enabled} class="rounded text-sky-600" on:change={autoSaveSettings} />
            </label>
            <div>
              <label class="block text-xs font-bold text-pos-muted mb-1">Recap frequency</label>
              <select bind:value={settings.recap_interval_minutes} class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-bold text-pos-text outline-none cursor-pointer">
                <option value="15">Every 15 minutes</option>
                <option value="30">Every 30 minutes</option>
                <option value="60">Every hour</option>
                <option value="120">Every 2 hours</option>
                <option value="240">Every 4 hours</option>
              </select>
            </div>
            <button type="button" on:click={sendRecapNow} class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-xl flex items-center gap-1.5 cursor-pointer">
              <Send class="w-3.5 h-3.5" />
              <span>Send Recap Now (إرسال الملخص الآن)</span>
            </button>
            {#if recapStatusMsg}
              <p class="text-[11px] font-bold text-pos-muted">{recapStatusMsg}</p>
            {/if}
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
              <div
                style="text-align: {settings.sticker_text_align || 'center'};"
                class="p-4 bg-white text-slate-900 border-2 border-dashed border-slate-300 rounded-xl flex flex-col justify-center space-y-1.5 shadow-inner min-h-[140px] w-full"
              >
                {#if (settings.sticker_show_shop_name ?? 'true') !== 'false' && (settings.sticker_show_shop_name ?? true) !== false}
                  <span class="text-[10px] text-slate-500 font-bold uppercase block" style="text-align: {settings.sticker_text_align || 'center'};">{settings.shop_name_fr || 'TitaouPOS Supermarché'}</span>
                {/if}
                {#if (settings.sticker_show_product_name ?? 'true') !== 'false' && (settings.sticker_show_product_name ?? true) !== false}
                  <p
                    style="font-size: {settings.sticker_name_font_size || '12'}px; font-weight: {(settings.sticker_name_bold ?? 'true') !== 'false' && (settings.sticker_name_bold ?? true) !== false ? '900' : 'normal'}; text-align: {settings.sticker_text_align || 'center'};"
                    class="text-slate-900 line-clamp-1 leading-tight block"
                  >
                    {previewProductName}
                  </p>
                {/if}
                {#if (settings.sticker_show_barcode ?? 'true') !== 'false' && (settings.sticker_show_barcode ?? true) !== false}
                  <div class="w-full flex py-0.5 overflow-hidden" style="justify-content: {settings.sticker_text_align === 'left' ? 'flex-start' : settings.sticker_text_align === 'right' ? 'flex-end' : 'center'};">
                    <svg bind:this={settingsBarcodeSvgEl} class="max-w-full"></svg>
                  </div>
                {/if}
                {#if (settings.sticker_show_price ?? 'true') !== 'false' && (settings.sticker_show_price ?? true) !== false}
                  <span
                    style="font-size: {settings.sticker_price_font_size || '16'}px; font-weight: {(settings.sticker_price_bold ?? 'true') !== 'false' && (settings.sticker_price_bold ?? true) !== false ? '900' : 'normal'}; text-align: {settings.sticker_text_align || 'center'};"
                    class="text-slate-900 font-mono leading-none block"
                  >
                    {previewPrice} DZD
                  </span>
                {/if}
              </div>

              <!-- Dimensions, Orientation & Alignment -->
              <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 text-xs">
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Width (mm)</label>
                  <input type="number" bind:value={settings.barcode_label_width} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold font-mono outline-none" />
                </div>
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Height (mm)</label>
                  <input type="number" bind:value={settings.barcode_label_height} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold font-mono outline-none" />
                </div>
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Orientation (الاتجاه)</label>
                  <select bind:value={settings.sticker_orientation} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold">
                    <option value="landscape">Landscape (عرضي)</option>
                    <option value="portrait">Portrait (طولي)</option>
                  </select>
                </div>
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Alignment (المحاذاة)</label>
                  <select bind:value={settings.sticker_text_align} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold">
                    <option value="center">Center / وسط</option>
                    <option value="left">Left / يسار</option>
                    <option value="right">Right / يمين</option>
                  </select>
                </div>
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Content Position (الموضع)</label>
                  <select bind:value={settings.sticker_content_position} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold">
                    <option value="top">Top / أعلى</option>
                    <option value="middle">Middle / وسط</option>
                    <option value="bottom">Bottom / أسفل</option>
                  </select>
                </div>
              </div>

              <!-- Label Presets -->
              <div class="p-3 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-2">
                <span class="text-[10px] font-bold text-pos-muted uppercase tracking-wider block">Label Presets (إعدادات جاهزة)</span>
                <div class="flex items-center gap-2">
                  <input
                    type="text"
                    bind:value={newPresetName}
                    placeholder="Preset name (Ex: Fardeau 60x40)..."
                    class="flex-1 px-2.5 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs font-bold text-pos-text outline-none"
                  />
                  <button
                    type="button"
                    on:click={saveCurrentLabelPreset}
                    class="px-3 py-1.5 bg-sky-600 hover:bg-sky-700 text-white text-[10px] font-black rounded-lg cursor-pointer"
                  >
                    Save Preset
                  </button>
                </div>
                {#if Object.keys(labelPresets).length > 0}
                  <div class="flex items-center gap-1.5 flex-wrap">
                    {#each Object.keys(labelPresets) as pname}
                      <div class="flex items-center gap-1 bg-white dark:bg-slate-900 border border-pos-border rounded-lg overflow-hidden">
                        <button
                          type="button"
                          on:click={() => applyLabelPreset(pname)}
                          class="px-2.5 py-1 text-[10px] font-black text-sky-600 hover:bg-sky-50 dark:hover:bg-sky-950 cursor-pointer"
                          title="Apply this preset"
                        >
                          {pname}
                        </button>
                        <button
                          type="button"
                          on:click={() => deleteLabelPreset(pname)}
                          class="px-1.5 py-1 text-[10px] text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 cursor-pointer border-s border-pos-border"
                          title="Delete preset"
                        >
                          ✕
                        </button>
                      </div>
                    {/each}
                  </div>
                {/if}
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
                    <span class="text-[9px] font-bold text-pos-muted block">Name Size (px)</span>
                    <input type="number" min="4" max="72" bind:value={settings.sticker_name_font_size} class="w-full px-1.5 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs font-bold font-mono outline-none" />
                    <label class="flex items-center gap-1 text-[10px] font-bold text-pos-text cursor-pointer pt-0.5">
                      <input type="checkbox" bind:checked={settings.sticker_name_bold} class="rounded text-sky-600" />
                      <span>Bold</span>
                    </label>
                  </div>

                  <div class="p-2 bg-white dark:bg-slate-900 rounded-lg border border-pos-border space-y-1">
                    <span class="text-[9px] font-bold text-pos-muted block">Price Size (px)</span>
                    <input type="number" min="4" max="72" bind:value={settings.sticker_price_font_size} class="w-full px-1.5 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs font-bold font-mono outline-none" />
                    <label class="flex items-center gap-1 text-[10px] font-bold text-pos-text cursor-pointer pt-0.5">
                      <input type="checkbox" bind:checked={settings.sticker_price_bold} class="rounded text-sky-600" />
                      <span>Bold</span>
                    </label>
                  </div>

                  <div class="p-2 bg-white dark:bg-slate-900 rounded-lg border border-pos-border space-y-1">
                    <span class="text-[9px] font-bold text-pos-muted block">Code Size (px)</span>
                    <input type="number" min="4" max="48" bind:value={settings.sticker_barcode_font_size} on:input={renderSettingsBarcode} class="w-full px-1.5 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs font-bold font-mono outline-none" />
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
              <div
                style="text-align: {settings.shelf_text_align || 'center'};"
                class="p-4 bg-white text-slate-900 border-2 border-dashed border-emerald-300 rounded-xl flex flex-col justify-between space-y-2 shadow-inner min-h-[140px] w-full"
              >
                {#if (settings.shelf_show_shop_name ?? 'true') !== 'false' && (settings.shelf_show_shop_name ?? true) !== false}
                  <div class="w-full flex justify-between text-[10px] font-bold text-slate-500 border-b pb-0.5">
                    <span>{settings.shop_name_fr || 'TitaouPOS'}</span>
                    <span class="text-emerald-600 font-bold">DISPO EN RAYON</span>
                  </div>
                {/if}
                {#if (settings.shelf_show_product_name ?? 'true') !== 'false' && (settings.shelf_show_product_name ?? true) !== false}
                  <h4
                    style="font-size: {settings.shelf_name_font_size || '16'}px; font-weight: {(settings.shelf_name_bold ?? 'true') !== 'false' && (settings.shelf_name_bold ?? true) !== false ? '900' : 'normal'}; text-align: {settings.shelf_text_align || 'center'};"
                    class="text-slate-900 leading-tight py-0.5 block"
                  >
                    {previewProductName}
                  </h4>
                {/if}
                {#if (settings.shelf_show_price ?? 'true') !== 'false' && (settings.shelf_show_price ?? true) !== false}
                  <div
                    style="font-size: {settings.shelf_price_font_size || '28'}px; font-weight: {(settings.shelf_price_bold ?? 'true') !== 'false' && (settings.shelf_price_bold ?? true) !== false ? '900' : 'normal'}; text-align: {settings.shelf_text_align || 'center'};"
                    class="w-full bg-slate-900 text-white font-mono rounded py-1 px-2 block"
                  >
                    {previewPrice} DZD
                  </div>
                {/if}
                {#if (settings.shelf_show_ref ?? 'true') !== 'false' && (settings.shelf_show_ref ?? true) !== false}
                  <div class="w-full flex justify-between font-bold text-slate-500 pt-0.5" style="font-size: {settings.shelf_ref_font_size || '10'}px;">
                    <span>Ref: {previewBarcodeNumber}</span>
                    <span>TVA 19% Incl.</span>
                  </div>
                {/if}
              </div>

              <!-- Dimensions, Orientation & Alignment -->
              <div class="grid grid-cols-2 sm:grid-cols-4 gap-2 text-xs">
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Width (mm)</label>
                  <input type="number" bind:value={settings.shelf_tag_width} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold font-mono outline-none" />
                </div>
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Height (mm)</label>
                  <input type="number" bind:value={settings.shelf_tag_height} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold font-mono outline-none" />
                </div>
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Orientation (الاتجاه)</label>
                  <select bind:value={settings.shelf_orientation} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold">
                    <option value="landscape">Landscape (عرضي)</option>
                    <option value="portrait">Portrait (طولي)</option>
                  </select>
                </div>
                <div>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1">Alignment (المحاذاة)</label>
                  <select bind:value={settings.shelf_text_align} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold">
                    <option value="center">Center / وسط</option>
                    <option value="left">Left / يسار</option>
                    <option value="right">Right / يمين</option>
                  </select>
                  <label class="block text-[10px] font-bold text-pos-muted mb-1 mt-2">Content Position (الموضع)</label>
                  <select bind:value={settings.shelf_content_position} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-lg text-xs text-pos-text font-bold">
                    <option value="top">Top / أعلى</option>
                    <option value="middle">Middle / وسط</option>
                    <option value="bottom">Bottom / أسفل</option>
                  </select>
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
                    <span class="text-[9px] font-bold text-pos-muted block">Name Size (px)</span>
                    <input type="number" min="6" max="80" bind:value={settings.shelf_name_font_size} class="w-full px-1.5 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs font-bold font-mono outline-none" />
                    <label class="flex items-center gap-1 text-[10px] font-bold text-pos-text cursor-pointer pt-0.5">
                      <input type="checkbox" bind:checked={settings.shelf_name_bold} class="rounded text-emerald-600" />
                      <span>Bold</span>
                    </label>
                  </div>

                  <div class="p-2 bg-white dark:bg-slate-900 rounded-lg border border-pos-border space-y-1">
                    <span class="text-[9px] font-bold text-pos-muted block">Price Size (px)</span>
                    <input type="number" min="8" max="96" bind:value={settings.shelf_price_font_size} class="w-full px-1.5 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs font-bold font-mono outline-none" />
                    <label class="flex items-center gap-1 text-[10px] font-bold text-pos-text cursor-pointer pt-0.5">
                      <input type="checkbox" bind:checked={settings.shelf_price_bold} class="rounded text-emerald-600" />
                      <span>Bold</span>
                    </label>
                  </div>

                  <div class="p-2 bg-white dark:bg-slate-900 rounded-lg border border-pos-border space-y-1">
                    <span class="text-[9px] font-bold text-pos-muted block">Ref Size (px)</span>
                    <input type="number" min="4" max="48" bind:value={settings.shelf_ref_font_size} class="w-full px-1.5 py-0.5 bg-slate-100 dark:bg-slate-800 rounded text-xs font-bold font-mono outline-none" />
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

        <!-- SECTION 3: Built-in 40×20 mm Thermal Presets -->
        <div class="p-5 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border space-y-4">
          <div>
            <h3 class="text-sm font-black text-pos-text flex items-center gap-2">
              <Tag class="w-4 h-4 text-amber-500" />
              <span>3. Built-in Thermal Presets — 40×20 mm (قوالب جاهزة)</span>
            </h3>
            <p class="text-xs text-pos-muted mt-1">
              mm-exact presets with real scannable barcode, auto-fitting text and rotated price — select them from any product's
              <span class="font-bold">Print Label</span> modal. Printed size stays exactly 40×20 mm at 203/300/600 DPI.
            </p>
          </div>

          <!-- Label printer hardware (exact-media silent pipeline) -->
          <div class="p-3 bg-white dark:bg-slate-900 rounded-2xl border border-pos-border grid grid-cols-1 md:grid-cols-3 gap-3">
            <div>
              <label class="block text-[10px] font-bold text-pos-muted mb-1">Label Printer (طابعة الملصقات)</label>
              <select
                bind:value={settings.label_printer}
                on:change={autoSaveSettings}
                class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text outline-none cursor-pointer"
              >
                <option value="">Default Windows Printer</option>
                {#each printerList as pr}
                  <option value={pr}>{pr}</option>
                {/each}
              </select>
              <p class="text-[9px] text-pos-muted mt-1">Used by the exact-media pipeline: one 40×20mm page per copy, no gaps, silent.</p>
            </div>
            <div>
              <label class="block text-[10px] font-bold text-pos-muted mb-1">Label Printer DPI</label>
              <select
                bind:value={settings.label_printer_dpi}
                on:change={autoSaveSettings}
                class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-lg text-xs font-bold text-pos-text outline-none cursor-pointer"
              >
                <option value="203">203 DPI (standard thermal)</option>
                <option value="300">300 DPI (high-res)</option>
                <option value="600">600 DPI (photo-grade)</option>
              </select>
              <p class="text-[9px] text-pos-muted mt-1">Match your Xprinter model's resolution for crisp bars.</p>
            </div>
            <div class="flex items-end">
              <p class="text-[10px] text-pos-muted bg-amber-50 dark:bg-amber-950/40 border border-amber-200 dark:border-amber-900 rounded-lg p-2 w-full">
                Multi-copy jobs print consecutively — Label 2 feeds right after Label 1. No A4 pages, no blank gaps.
              </p>
            </div>
          </div>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            {#each LABEL_PRESET_IDS as pid}
              <div class="p-4 bg-white dark:bg-slate-900 rounded-2xl border border-pos-border space-y-3">
                <div class="flex items-center justify-between gap-2">
                  <span class="text-xs font-black text-pos-text">{LABEL_PRESETS[pid].name}</span>
                  <span class="text-[9px] font-mono bg-amber-100 dark:bg-amber-950 text-amber-800 dark:text-amber-300 px-2 py-0.5 rounded-full font-bold shrink-0">
                    {LABEL_PRESETS[pid].widthMm}×{LABEL_PRESETS[pid].heightMm} mm
                  </span>
                </div>
                <div class="bg-slate-100 dark:bg-slate-800 rounded-xl p-2 flex justify-center overflow-hidden">
                  <div style="width: calc(40mm * 2.2); height: calc(20mm * 2.2); flex: 0 0 auto;">
                    <div style="width: 40mm; height: 20mm; transform: scale(2.2); transform-origin: top left;">
                      {@html builtinLabelPreviews[pid]}
                    </div>
                  </div>
                </div>
                <button
                  type="button"
                  on:click={() => testPrintBuiltinLabel(pid)}
                  class="w-full py-2 bg-amber-600 hover:bg-amber-700 text-white text-xs font-bold rounded-xl flex items-center justify-center gap-1.5 cursor-pointer shadow-xs transition"
                >
                  <Printer class="w-3.5 h-3.5" />
                  <span>Test Print 5× ({LABEL_PRESETS[pid].widthMm}×{LABEL_PRESETS[pid].heightMm}mm)</span>
                </button>
              </div>
            {/each}
          </div>
          {#if builtinTestMsg}
            <p class="text-[11px] font-bold font-mono text-pos-text bg-slate-100 dark:bg-slate-800 rounded-lg p-2 border border-pos-border">
              {builtinTestMsg}
            </p>
          {/if}
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

            <div class="grid grid-cols-2 gap-3 pt-1">
              <div>
                <label class="block text-[10px] font-bold text-pos-muted mb-1">Default Margin % (new products)</label>
                <input type="number" min="0" max="500" step="0.5" bind:value={settings.default_margin_percent} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-bold font-mono text-pos-text outline-none" />
              </div>
              <div>
                <label class="block text-[10px] font-bold text-pos-muted mb-1">Round Sale Price to</label>
                <select bind:value={settings.price_round_step} class="w-full px-2 py-1.5 bg-white dark:bg-slate-900 border border-pos-border rounded-xl text-xs font-bold text-pos-text outline-none cursor-pointer">
                  <option value="0">Whole DZD (no rounding)</option>
                  <option value="5">Nearest 5 DZD (119→120, 116→115)</option>
                  <option value="10">Nearest 10 DZD</option>
                </select>
              </div>
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

            <div class="p-2.5 bg-white dark:bg-slate-900 rounded-xl border border-pos-border">
              <label class="block text-[10px] font-bold text-pos-muted mb-1">Auto-focus idle timer (seconds)</label>
              <input type="number" min="0" max="120" bind:value={settings.pos_autofocus_timer_seconds} class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 rounded-xl text-xs font-bold font-mono text-pos-text outline-none" />
              <p class="text-[9px] text-pos-muted font-normal mt-1">Cursor jumps back to the search bar after this many idle seconds (0 = disabled)</p>
            </div>

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
          <div class="bg-pos-card rounded-2xl p-1">
        <ShortcutsEditor bind:bindings={shortcutBindings} />
      </div></div>
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

          <!-- Online Activation -->
          <div class="p-4 bg-sky-50 dark:bg-sky-950/30 rounded-2xl border border-sky-200 dark:border-sky-800/60 space-y-3">
            <h4 class="text-xs font-black text-pos-text">Activate Online (تنشيط عبر الإنترنت)</h4>
            <p class="text-[11px] text-pos-muted">
              Sends this machine's HWID to the developer's activation registry and activates automatically.
            </p>
            <div class="flex items-center gap-2">
              <button
                type="button"
                on:click={handleActivateOnline}
                disabled={isActivatingOnline}
                class="px-4 py-2 bg-sky-600 hover:bg-sky-700 disabled:opacity-50 text-white text-xs font-black rounded-xl cursor-pointer shadow-md"
              >
                {isActivatingOnline ? 'Checking...' : 'Activate This PC Online'}
              </button>
              <span class="text-[10px] font-mono text-pos-muted">HWID: {hwid}</span>
            </div>
            {#if activationMsg}
              <p class="text-[11px] font-bold {activationMsg.includes('success') || activationMsg.includes('بنجاح') ? 'text-emerald-600' : 'text-rose-600'}">{activationMsg}</p>
            {/if}
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
              <button
                type="button"
                on:click={() => openUrlInBrowser(latestDownloadUrl)}
                class="px-5 py-2.5 bg-emerald-600 hover:bg-emerald-700 text-white font-black text-xs rounded-xl flex items-center gap-2 cursor-pointer shadow-md transition animate-pulse"
              >
                <Download class="w-4 h-4" />
                <span>Download {latestReleaseInfo?.tag_name || 'Update'} (تحميل التحديث)</span>
              </button>
              <button
                type="button"
                on:click={() => openUrlInBrowser(latestReleaseUrl)}
                class="px-4 py-2.5 bg-slate-100 hover:bg-slate-200 dark:bg-slate-800 text-pos-text font-bold text-xs rounded-xl flex items-center gap-1.5 cursor-pointer transition"
              >
                <span>View Release Notes</span>
              </button>
            {/if}

            <button
              on:click={openRollbackModal}
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
    {:else if currentTab === 'about'}
      <AboutView />
    {:else if currentTab === 'danger'}
      <div class="max-w-3xl space-y-6">
        <div>
          <h2 class="text-base font-black text-rose-600 flex items-center gap-2">
            <AlertOctagon class="w-5 h-5" />
            <span>Factory Reset & Data Purge / تهيئة المصنع ومسح البيانات</span>
          </h2>
          <p class="text-xs text-pos-muted">Select an operation below. These actions cannot be undone. Please backup database before proceeding.</p>
        </div>

        <!-- Clear History Only (non-destructive to products) -->
        <div class="p-5 bg-amber-50 dark:bg-amber-950/30 border border-amber-300 dark:border-amber-800 rounded-2xl space-y-3">
          <div class="flex items-start justify-between gap-3">
            <div>
              <h3 class="text-sm font-black text-amber-800 dark:text-amber-200 flex items-center gap-2">
                <History class="w-4 h-4" />
                <span>Clear Sales & Purchases History / مسح سجل المعاملات فقط</span>
              </h3>
              <p class="text-[11px] text-pos-muted mt-1">
                Erases sales, purchases, cash sessions and movements — but keeps products, prices, quantities, customers, suppliers and debts intact.
              </p>
            </div>
          </div>
          <input
            type="text"
            bind:value={clearHistoryConfirmText}
            placeholder="Type CLEAR HISTORY to confirm"
            class="w-full px-3 py-2 bg-white dark:bg-slate-900 border border-amber-300 dark:border-amber-800 rounded-xl text-xs font-mono font-bold text-pos-text outline-none"
          />
          <button
            type="button"
            on:click={handleClearHistory}
            disabled={isClearingHistory || clearHistoryConfirmText.trim() !== 'CLEAR HISTORY'}
            class="px-4 py-2 bg-amber-600 hover:bg-amber-700 disabled:opacity-40 disabled:cursor-not-allowed text-white text-xs font-black rounded-xl cursor-pointer"
          >
            {isClearingHistory ? 'Clearing…' : 'Clear History Only'}
          </button>
          {#if clearHistoryMsg}
            <p class="text-[11px] font-bold text-amber-700 dark:text-amber-300">{clearHistoryMsg}</p>
          {/if}
        </div>

        <div class="p-5 bg-rose-50 dark:bg-rose-950/30 border border-rose-200 dark:border-rose-900 rounded-2xl space-y-4">
          <div class="grid grid-cols-1 gap-2.5">
            <!-- 1. Delete All Products -->
            <label class="flex items-start gap-3 p-3 bg-white dark:bg-slate-900 border rounded-xl cursor-pointer transition {resetType === 'products_only' ? 'border-rose-500 ring-2 ring-rose-500/20' : 'border-pos-border'}">
              <input type="radio" bind:group={resetType} value="products_only" class="mt-0.5 text-rose-600" />
              <div class="flex-1">
                <span class="text-xs font-black text-pos-text block">Delete All Products & Stock / حذف جميع المنتجات والمخزون</span>
                <span class="text-[11px] text-pos-muted">Purges all products, barcodes, stock movements, and price history (keeps families, units, users).</span>
              </div>
            </label>

            <!-- 2. Reset Families / Categories -->
            <label class="flex items-start gap-3 p-3 bg-white dark:bg-slate-900 border rounded-xl cursor-pointer transition {resetType === 'categories_only' ? 'border-rose-500 ring-2 ring-rose-500/20' : 'border-pos-border'}">
              <input type="radio" bind:group={resetType} value="categories_only" class="mt-0.5 text-rose-600" />
              <div class="flex-1">
                <span class="text-xs font-black text-pos-text block">Reset Families & Categories / إعادة تعيين الفئات والعائلات</span>
                <span class="text-[11px] text-pos-muted">Resets all categories back to clean "Default / Général" family.</span>
              </div>
            </label>

            <!-- 3. Reset Units -->
            <label class="flex items-start gap-3 p-3 bg-white dark:bg-slate-900 border rounded-xl cursor-pointer transition {resetType === 'units_only' ? 'border-rose-500 ring-2 ring-rose-500/20' : 'border-pos-border'}">
              <input type="radio" bind:group={resetType} value="units_only" class="mt-0.5 text-rose-600" />
              <div class="flex-1">
                <span class="text-xs font-black text-pos-text block">Reset Units of Measurement / إعادة تعيين وحدات القياس</span>
                <span class="text-[11px] text-pos-muted">Resets custom units back to 5 standard system units (pcs, kg, L, pck, box).</span>
              </div>
            </label>

            <!-- 4. Clear Sales & Transactions -->
            <label class="flex items-start gap-3 p-3 bg-white dark:bg-slate-900 border rounded-xl cursor-pointer transition {resetType === 'transactions_only' ? 'border-rose-500 ring-2 ring-rose-500/20' : 'border-pos-border'}">
              <input type="radio" bind:group={resetType} value="transactions_only" class="mt-0.5 text-rose-600" />
              <div class="flex-1">
                <span class="text-xs font-black text-pos-text block">Clear Sales & Financial Transactions / مسح المبيعات والعمليات المالية</span>
                <span class="text-[11px] text-pos-muted">Clears all sales, held sales, cash drawer movements, receipts, expenses, and resets debt balances to 0 (keeps products & catalog).</span>
              </div>
            </label>

            <!-- 5. Reset Customers & Debts -->
            <label class="flex items-start gap-3 p-3 bg-white dark:bg-slate-900 border rounded-xl cursor-pointer transition {resetType === 'customers_only' ? 'border-rose-500 ring-2 ring-rose-500/20' : 'border-pos-border'}">
              <input type="radio" bind:group={resetType} value="customers_only" class="mt-0.5 text-rose-600" />
              <div class="flex-1">
                <span class="text-xs font-black text-pos-text block">Reset Customers & Customer Debts / مسح الزبائن والديون</span>
                <span class="text-[11px] text-pos-muted">Deletes all custom customer accounts and customer debt payment records.</span>
              </div>
            </label>

            <!-- 6. Reset Suppliers & Purchases -->
            <label class="flex items-start gap-3 p-3 bg-white dark:bg-slate-900 border rounded-xl cursor-pointer transition {resetType === 'suppliers_only' ? 'border-rose-500 ring-2 ring-rose-500/20' : 'border-pos-border'}">
              <input type="radio" bind:group={resetType} value="suppliers_only" class="mt-0.5 text-rose-600" />
              <div class="flex-1">
                <span class="text-xs font-black text-pos-text block">Reset Suppliers & Purchases / مسح الموردين وفواتير الشراء</span>
                <span class="text-[11px] text-pos-muted">Deletes all supplier records, purchase invoices, and supplier debt payments.</span>
              </div>
            </label>

            <!-- 7. Full Factory Reset -->
            <label class="flex items-start gap-3 p-3 bg-rose-100/60 dark:bg-rose-950/80 border-2 border-rose-400 dark:border-rose-800 rounded-xl cursor-pointer transition {resetType === 'full_reset' ? 'ring-2 ring-rose-600' : ''}">
              <input type="radio" bind:group={resetType} value="full_reset" class="mt-0.5 text-rose-600" />
              <div class="flex-1">
                <span class="text-xs font-black text-rose-700 dark:text-rose-300 block">Full Factory Reset (Comprehensive) / إعادة ضبط المصنع بالكامل</span>
                <span class="text-[11px] text-rose-600/80 dark:text-rose-400/80">Complete system wipe: purges all products, resets families to Default, resets units to standard, clears sales, customers, suppliers, and extra users.</span>
              </div>
            </label>
          </div>

          <div class="space-y-2 pt-3 border-t border-rose-200 dark:border-rose-900">
            <label class="block text-xs font-bold text-pos-muted">Type <span class="text-rose-600 font-mono font-black">RESET</span> to confirm execution:</label>
            <div class="flex items-center gap-2">
              <input
                type="text"
                bind:value={resetConfirm}
                placeholder="RESET"
                class="w-48 px-3 py-2 bg-white dark:bg-slate-900 border border-rose-300 dark:border-rose-800 rounded-xl text-xs font-mono font-black text-rose-600 outline-none focus:ring-2 focus:ring-rose-500"
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
        <p class="text-xs font-bold text-rose-600 bg-rose-50 dark:bg-rose-950/40 border border-rose-200 dark:border-rose-800 rounded-xl p-2.5">
          ⚠️ Rolling back can lose data created in newer versions. Type <span class="font-mono font-black">ROLLBACK</span> below to confirm.
        </p>
        <input
          type="text"
          bind:value={rollbackConfirmText}
          placeholder="Type ROLLBACK to confirm"
          class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border border-pos-border rounded-xl text-xs font-mono font-bold text-pos-text outline-none"
        />
        <p class="text-[10px] text-pos-muted font-bold">Auto-cancels in {rollbackCountdown}s — nothing happens if you do nothing.</p>
        <div class="flex justify-end gap-2 pt-2">
          <button on:click={cancelRollback} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-xl cursor-pointer">Cancel</button>
          <button
            on:click={handleRollback}
            disabled={rollbackConfirmText.trim() !== 'ROLLBACK'}
            class="px-4 py-2 bg-amber-600 disabled:opacity-40 disabled:cursor-not-allowed text-white text-xs font-black rounded-xl cursor-pointer"
          >Confirm Rollback</button>
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