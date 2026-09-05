/**
 * UNIFIED RECEIPT PRESET SYSTEM (v0.5.16).
 *
 * One helper builds EVERY receipt the app prints — POS auto-print, reprint
 * from Sales History, reopen-last-receipt, credit/versement copies, refund
 * tickets — from the same app_settings keys, so a settings change applies
 * everywhere at once. Two templates exist ("professional" graphic 80mm and
 * "standard" monospace); the choice rides the `receipt_preset` setting.
 *
 * All inputs are plain data (sale + items + shop settings), so callers
 * never re-implement layout or option parsing.
 */
import { buildProfessionalReceiptHtml, type ProReceiptOptions } from './professionalReceipt';
import { buildReceiptHtml } from '../utils/printer';
import { getLanguage } from '../i18n';

export interface UnifiedReceiptItem {
  name: string;
  quantity: number;
  unitPrice: number;
  totalPrice: number;
  discountPerUnit?: number;
  isRefund?: boolean;
}

export interface UnifiedReceiptContext {
  saleNumber: string;
  saleDate: string;
  cashierName: string;
  customerName?: string;
  items: UnifiedReceiptItem[];
  subtotal: number;
  discount: number;
  grandTotal: number;
  amountPaid: number;
  change: number;
  paymentMethod: string;
  // shop + receipt settings (raw app_settings map; missing keys fall back
  // to the same defaults every other printer used before).
  settings: Record<string, string | undefined>;
  qrDataUrl?: string;
  copyLabel?: string;
  isCredit?: boolean;
  versementPaid?: number;
  versementRemaining?: number;
}

function bool(s: Record<string, string | undefined>, key: string, dflt = true): boolean {
  const v = s[key];
  if (v === undefined || v === null || v === '') return dflt;
  return v === 'true' || v === '1';
}

/** Shared settings-driven options for the professional template. */
function proOptionsFromContext(c: UnifiedReceiptContext): ProReceiptOptions {
  const s = c.settings;
  const d = new Date(c.saleDate?.replace(' ', 'T') || Date.now());
  const valid = !isNaN(d.getTime()) ? d : new Date();
  const paperWidthMm = s['receipt_paper_width'] === '58mm' ? 58 : 80;

  return {
    shopName: s['shop_name_fr'] || s['shop_name_ar'] || 'TITAOU POS',
    shopTagline: s['receipt_header'] || '',
    shopAddress: s['shop_address'] || '',
    shopPhone: s['shop_phone'] || '',
    shopWebsite: s['shop_website'] || '',
    shopLogoDataUrl: s['shop_logo_base64'] || undefined,
    invoiceNumber: c.saleNumber,
    invoiceBarcode: c.saleNumber,
    dateStr: valid.toLocaleDateString('fr-FR'),
    timeStr: valid.toLocaleTimeString('fr-FR'),
    cashierName: c.cashierName,
    customerName: c.customerName,
    paymentMethod: c.paymentMethod,
    items: c.items,
    subtotal: c.subtotal,
    discount: c.discount,
    grandTotal: c.grandTotal,
    amountPaid: c.amountPaid,
    change: c.change,
    currency: s['default_currency'] || 'DA',
    qrDataUrl: c.qrDataUrl,
    showQr: bool(s, 'receipt_show_qr'),
    showBarcode: bool(s, 'receipt_show_barcode'),
    thankYou: s['receipt_thank_you'] || 'MERCI POUR VOTRE CONFIANCE !',
    returnPolicy: s['receipt_footer'] || '',
    lang: getLanguage(),
    paperWidthMm,
    copyLabel: c.copyLabel,
    isCredit: c.isCredit,
    versementPaid: c.versementPaid,
    versementRemaining: c.versementRemaining,
  };
}

/** Shared options for the legacy monospace template. */
function standardOptionsFromContext(c: UnifiedReceiptContext) {
  const s = c.settings;
  return {
    shopName: s['shop_name_fr'] || s['shop_name_ar'] || 'TitaouPOS',
    shopAddress: s['shop_address'] || 'Alger, Algérie',
    shopPhone: s['shop_phone'] || '0553444057',
    shopRc: s['shop_rc'] || undefined,
    shopNif: s['shop_nif'] || undefined,
    saleNumber: c.saleNumber,
    saleDate: c.saleDate || new Date().toLocaleString(),
    cashierName: c.cashierName,
    customerName: c.customerName,
    items: c.items,
    subtotal: c.subtotal,
    discount: c.discount,
    grandTotal: c.grandTotal,
    paymentMethod: c.paymentMethod,
    isCredit: c.isCredit,
    copyLabel: c.copyLabel,
    versementPaid: c.versementPaid,
    versementRemaining: c.versementRemaining,
    headerFontSize: parseInt(s['receipt_header_font_size'] || '14', 10),
    headerBold: bool(s, 'receipt_header_bold'),
    bodyFontSize: parseInt(s['receipt_body_font_size'] || '11', 10),
    bodyBold: s['receipt_body_bold'] === 'true',
    totalFontSize: parseInt(s['receipt_total_font_size'] || '14', 10),
    totalBold: bool(s, 'receipt_total_bold'),
    footerFontSize: parseInt(s['receipt_footer_font_size'] || '9', 10),
    footerBold: s['receipt_footer_bold'] === 'true',
    headerAlign: (s['receipt_header_align'] as 'left' | 'center' | 'right') || 'center',
    footerAlign: (s['receipt_footer_align'] as 'left' | 'center' | 'right') || 'center',
    receiptHeaderGreeting: s['receipt_header'] || '',
    qrDataUrl: c.qrDataUrl,
    receiptFooterNote: s['receipt_footer'] || undefined,
  };
}

export interface BuiltReceipt {
  html: string;
  title: string;
  paperWidthMm: number;
}

/**
 * Build ONE receipt (any kind) from the unified context, honoring the
 * global `receipt_preset` setting. The returned paper width drives the
 * silent print job's DEVMODE.
 */
export function buildUnifiedReceipt(c: UnifiedReceiptContext): BuiltReceipt {
  const paperWidthMm = c.settings['receipt_paper_width'] === '58mm' ? 58 : 80;
  const preset = c.settings['receipt_preset'] || 'professional';
  const title = `Receipt #${c.saleNumber}${c.copyLabel ? ' — ' + c.copyLabel : ''}`;

  if (preset === 'professional') {
    return {
      html: buildProfessionalReceiptHtml(proOptionsFromContext(c)),
      title,
      paperWidthMm,
    };
  }
  return {
    html: buildReceiptHtml(standardOptionsFromContext(c)),
    title,
    paperWidthMm,
  };
}
