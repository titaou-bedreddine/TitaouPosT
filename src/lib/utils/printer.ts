// Try silent OS printing first (headless Edge → PDF → default printer).
// If the backend command isn't available (older binary / dev mode), fall
// back to the classic hidden-iframe print.
// `paper` pins the physical page: widthMm + optional heightMm (omit for
// dynamic-height receipts). Without it the legacy 80mm receipt page is used.
export interface PrintPaper {
  widthMm: number;
  heightMm?: number;
}

let silentPrintAvailable: boolean | null = null;

export async function printHtmlSilently(
  htmlContent: string,
  title = 'Thermal Receipt',
  paper?: PrintPaper
): Promise<void> {
  if (silentPrintAvailable !== false) {
    try {
      const { invoke } = await import('@tauri-apps/api/core');
      // The backend needs the full document (styles included); the caller
      // passes raw content, so reuse the same wrapper as the iframe path.
      await invoke('print_html_direct', {
        html: wrapFullDocument(htmlContent, paper),
        title,
      });
      silentPrintAvailable = true;
      return;
    } catch (e) {
      silentPrintAvailable = false;
      console.warn('Silent print unavailable, falling back to iframe print:', e);
    }
  }
  printHtmlDirectly(htmlContent, title, paper);
}

function wrapFullDocument(htmlContent: string, paper?: PrintPaper): string {
  const pageRule = paper
    ? `@page { size: ${paper.widthMm}mm ${paper.heightMm ? paper.heightMm + 'mm' : 'auto'}; margin: 0mm; }
       body { width: ${paper.widthMm}mm; margin: 0; padding: 0; background: #fff; }`
    : `@page { size: 80mm auto; margin: 0mm; }
       body { width: 76mm; margin: 2mm auto; font-size: 11px; line-height: 1.3; background: #fff; padding: 2mm; }`;
  return `<!DOCTYPE html><html><head><meta charset="utf-8" /><style>
    ${pageRule}
    * { box-sizing: border-box; margin: 0; padding: 0; font-family: 'Courier New', Courier, monospace; color: #000; }
    .text-center { text-align: center; } .text-end { text-align: right; } .text-start { text-align: left; }
    .font-bold { font-weight: bold; } .font-black { font-weight: 900; }
    table { width: 100%; border-collapse: collapse; margin: 4px 0; }
    th { text-align: left; border-bottom: 1px dashed #000; font-size: 10px; padding: 2px 0; }
    td { padding: 2px 0; font-size: 10px; }
    .border-b-dashed { border-bottom: 1px dashed #000; } .border-t-dashed { border-top: 1px dashed #000; }
    .flex { display: flex; } .justify-between { justify-content: space-between; } .items-center { align-items: center; }
  </style></head><body>${htmlContent}</body></html>`;
}

export function printHtmlDirectly(
  htmlContent: string,
  title = 'Thermal Receipt',
  paper?: PrintPaper
) {
  const iframe = document.createElement('iframe');
  iframe.style.position = 'fixed';
  iframe.style.right = '0';
  iframe.style.bottom = '0';
  iframe.style.width = '0';
  iframe.style.height = '0';
  iframe.style.border = '0';
  iframe.style.visibility = 'hidden';

  document.body.appendChild(iframe);

  const doc = iframe.contentWindow?.document;
  if (!doc) {
    console.error('Failed to get iframe document');
    return;
  }

  // Labels pass an exact page size so px font sizes print 1:1 instead of
  // being rescaled by an 80mm receipt page. heightMm omitted = dynamic height.
  const pageRule = paper
    ? `@page { size: ${paper.widthMm}mm ${paper.heightMm ? paper.heightMm + 'mm' : 'auto'}; margin: 0mm; }`
    : `@page { size: 80mm auto; margin: 0mm; }`;
  const bodyRule = paper
    ? `body { width: ${paper.widthMm}mm; margin: 0 auto; background: #fff; position: relative; }`
    : `body {
          width: 76mm;
          margin: 2mm auto;
          font-size: 11px;
          line-height: 1.3;
          background: #fff;
          padding: 2mm;
          position: relative;
        }`;

  doc.open();
  doc.write(`
    <!DOCTYPE html>
    <html>
      <head>
        <title>${title}</title>
        <meta charset="utf-8" />
        <style>
          ${pageRule}
          * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            font-family: 'Courier New', Courier, monospace, 'Segoe UI', Tahoma, sans-serif;
            color: #000;
          }
          ${bodyRule}
          .watermark {
            position: absolute;
            top: 40%;
            left: 50%;
            transform: translate(-50%, -50%) rotate(-30deg);
            font-size: 28px;
            font-weight: 900;
            color: rgba(0,0,0,0.18);
            border: 3px dashed rgba(0,0,0,0.25);
            padding: 6px 16px;
            pointer-events: none;
            text-transform: uppercase;
            letter-spacing: 2px;
          }
          .text-center { text-align: center; }
          .text-end { text-align: right; }
          .text-start { text-align: left; }
          .font-bold { font-weight: bold; }
          .font-black { font-weight: 900; }
          .text-sm { font-size: 13px; }
          .text-xs { font-size: 10px; }
          .text-xxs { font-size: 9px; }
          .border-b-dashed { border-bottom: 1px dashed #000; }
          .border-t-dashed { border-top: 1px dashed #000; }
          .py-1 { padding-top: 3px; padding-bottom: 3px; }
          .py-2 { padding-top: 6px; padding-bottom: 6px; }
          .my-1 { margin-top: 3px; margin-bottom: 3px; }
          .flex { display: flex; }
          .justify-between { justify-content: space-between; }
          .items-center { align-items: center; }
          .w-full { width: 100%; }
          table { width: 100%; border-collapse: collapse; margin: 4px 0; }
          th { text-align: left; border-bottom: 1px dashed #000; font-size: 10px; padding: 2px 0; }
          td { padding: 2px 0; font-size: 10px; }
          .qr-box { width: 64px; height: 64px; margin: 6px auto 2px; }
        </style>
      </head>
      <body>
        ${htmlContent}
      </body>
    </html>
  `);
  doc.close();

  iframe.contentWindow?.focus();
  setTimeout(() => {
    iframe.contentWindow?.print();
    setTimeout(() => {
      if (document.body.contains(iframe)) {
        document.body.removeChild(iframe);
      }
    }, 1000);
  }, 300);
}

export function buildReceiptHtml(options: {
  shopName: string;
  shopAddress: string;
  shopPhone: string;
  shopRc?: string;
  shopNif?: string;
  saleNumber: string;
  saleDate: string;
  cashierName: string;
  customerName?: string;
  items: Array<{ name: string; quantity: number; unitPrice: number; totalPrice: number; discountPerUnit?: number; isRefund?: boolean }>;
  subtotal: number;
  discount: number;
  grandTotal: number;
  paymentMethod: string;
  isCredit?: boolean;
  copyLabel?: string;
  // Versement (layaway): deposit paid now and what the customer still owes
  // before the goods leave the shop.
  versementPaid?: number;
  versementRemaining?: number;
  headerFontSize?: number;
  headerBold?: boolean;
  bodyFontSize?: number;
  bodyBold?: boolean;
  totalFontSize?: number;
  totalBold?: boolean;
  footerFontSize?: number;
  footerBold?: boolean;
  headerAlign?: 'left' | 'center' | 'right';
  footerAlign?: 'left' | 'center' | 'right';
  receiptHeaderGreeting?: string;
  // Locally-generated QR image (data URL) so receipts print offline.
  qrDataUrl?: string;
  receiptFooterNote?: string;
}) {
  // Offline-first: the QR is a pre-rendered local data URL passed by the
  // caller (entityQrDataUrl); fall back to the online generator only when
  // the caller could not produce one.
  const qrUrl = options.qrDataUrl
    || `https://api.qrserver.com/v1/create-qr-code/?size=100x100&data=${encodeURIComponent(`SALE:${options.saleNumber}`)}`;

  const headerSize = options.headerFontSize || 14;
  const headerBold = options.headerBold !== false;
  const bodySize = options.bodyFontSize || 11;
  const bodyBold = options.bodyBold === true;
  const totalSize = options.totalFontSize || 14;
  const totalBold = options.totalBold !== false;
  const footerSize = options.footerFontSize || 9;
  const footerBold = options.footerBold === true;
  const headerAlign = options.headerAlign || 'center';
  const footerAlign = options.footerAlign || 'center';

  return `
    ${options.isCredit ? '<div class="watermark">CREDIT / دين</div>' : ''}
    <div style="text-align: ${headerAlign};" class="pb-2 border-b-dashed">
      <h2 style="font-size: ${headerSize}px; font-weight: ${headerBold ? '900' : 'normal'};" class="uppercase">${options.shopName || 'TitaouPOS'}</h2>
      <p style="font-size: ${Math.max(8, bodySize - 2)}px;">${options.shopAddress || 'Alger, Algérie'}</p>
      <p style="font-size: ${Math.max(8, bodySize - 2)}px;">Tél: ${options.shopPhone || '0553444057'}</p>
      ${options.shopRc ? `<p style="font-size: ${Math.max(7, bodySize - 3)}px;">RC: ${options.shopRc} ${options.shopNif ? '| NIF: ' + options.shopNif : ''}</p>` : ''}
      ${options.receiptHeaderGreeting ? `<p style="font-size: ${Math.max(8, bodySize - 2)}px; font-style: italic; font-weight: bold;">${options.receiptHeaderGreeting}</p>` : ''}
      <p style="font-size: ${Math.max(8, bodySize - 2)}px; margin-top: 2px;">${options.saleDate}</p>
      ${options.copyLabel ? `<p class="font-black text-xxs mt-0.5 bg-black text-white px-1">[ ${options.copyLabel} ]</p>` : ''}
    </div>

    <div class="py-1 border-b-dashed text-xxs flex justify-between">
      <span>TICKET: <strong>#${options.saleNumber}</strong></span>
      <span>Caisse / User: <strong>${options.cashierName}</strong></span>
    </div>

    ${options.customerName ? `
      <div class="py-0.5 text-xxs border-b-dashed flex justify-between">
        <span>Client: <strong>${options.customerName}</strong></span>
        <span>Mode: <strong>${options.paymentMethod.toUpperCase()}</strong></span>
      </div>
    ` : ''}

    <div class="py-1 border-b-dashed">
      <table style="font-size: ${bodySize}px; font-weight: ${bodyBold ? 'bold' : 'normal'};">
        <thead>
          <tr>
            <th style="width: 50%; font-size: ${bodySize}px;">Article</th>
            <th class="text-center" style="width: 15%; font-size: ${bodySize}px;">Qté</th>
            <th class="text-end" style="width: 35%; font-size: ${bodySize}px;">Total</th>
          </tr>
        </thead>
        <tbody>
          ${options.items
            .map(
              (i) => `
            <tr>
              <td style="font-weight: ${bodyBold ? '900' : 'bold'};">${i.name}${i.isRefund ? ' <span style="font-size:8px;">[RETOUR]</span>' : ''}${i.discountPerUnit && i.discountPerUnit > 0 ? `<div style="font-size: ${Math.max(8, bodySize - 2)}px; font-weight: normal;">Remise -${i.discountPerUnit.toLocaleString()} DZD/u</div>` : ''}</td>
              <td class="text-center font-mono">${i.quantity}</td>
              <td class="text-end font-mono" style="font-weight: bold;">${i.totalPrice.toLocaleString()} DZD</td>
            </tr>
          `
            )
            .join('')}
        </tbody>
      </table>
    </div>

    <div class="py-1 border-b-dashed space-y-1">
      ${options.discount > 0 ? `
        <div class="flex justify-between" style="font-size: ${Math.max(9, bodySize - 1)}px;">
          <span>Sous-Total:</span>
          <span class="font-mono">${options.subtotal.toLocaleString()} DZD</span>
        </div>
        <div class="flex justify-between" style="font-size: ${Math.max(9, bodySize - 1)}px;">
          <span>Remise:</span>
          <span class="font-mono text-rose-600">-${options.discount.toLocaleString()} DZD</span>
        </div>
      ` : ''}
      <div style="font-size: ${totalSize}px; font-weight: ${totalBold ? '900' : 'bold'};" class="flex justify-between pt-0.5">
        <span>TOTAL A PAYER:</span>
        <span class="font-mono">${options.grandTotal.toLocaleString()} DZD</span>
      </div>
      ${options.versementPaid !== undefined ? `
        <div class="flex justify-between" style="font-size: ${Math.max(9, bodySize - 1)}px;">
          <span>Verse (تسبقة):</span>
          <span class="font-mono">${options.versementPaid.toLocaleString()} DZD</span>
        </div>
        <div class="flex justify-between" style="font-size: ${totalSize - 2}px; font-weight: 900;">
          <span>RESTE A VERSER:</span>
          <span class="font-mono">${(options.versementRemaining ?? Math.max(0, options.grandTotal - options.versementPaid)).toLocaleString()} DZD</span>
        </div>
        <div class="text-center" style="font-size: ${Math.max(8, bodySize - 2)}px; font-weight: bold; border: 1px dashed #000; padding: 3px; margin-top: 4px;">
          BIENS CONSERVES AU MAGASIN / البضاعة تبقى في المحل
        </div>
      ` : ''}
    </div>

    <div style="text-align: ${footerAlign};" class="pt-2 border-t-dashed">
      <img src="${qrUrl}" alt="QR" class="qr-box" />
      <p style="font-size: ${Math.max(7, footerSize - 1)}px; color: #555;">Scan QR to verify or lookup ticket</p>
      <p style="font-size: ${footerSize}px; font-weight: ${footerBold ? 'bold' : 'normal'}; margin-top: 3px;">
        ${options.receiptFooterNote || '*** Merci pour votre visite ***'}
      </p>
      <p class="text-[8px] text-gray-500 mt-1">TitaouPOS • Created by Titaou Bedreddine 0553444057</p>
    </div>
  `;
}
/**
 * Offline-friendly entity QR payload. Same scheme the receipt uses, so a
 * scanner app (or the POS omni-search) maps a QR back to its record:
 *   SALE:POS-20260831..., PUR:ACH-..., EXP:EXP-..., CUST:CUST-001, EMP:EMP-01
 */
export function entityQrPayload(type: 'SALE' | 'PUR' | 'EXP' | 'CUST' | 'SUP' | 'EMP', code: string): string {
  return `${type}:${code}`;
}

/** QR image URL for the payload (same generator as receipts). */
export function entityQrUrl(payload: string, size = 120): string {
  return `https://api.qrserver.com/v1/create-qr-code/?size=${size}x${size}&data=${encodeURIComponent(payload)}`;
}

// ---------------------------------------------------------------------------
// LOCAL QR GENERATION (offline — no api.qrserver.com round-trips).
// ---------------------------------------------------------------------------
import QRCode from 'qrcode';

let qrLibReady: typeof QRCode | null = null;

async function ensureQrLib(): Promise<typeof QRCode> {
  if (!qrLibReady) {
    qrLibReady = (await import('qrcode')).default as unknown as typeof QRCode;
  }
  return qrLibReady;
}

/**
 * Render a QR synchronously from cached SVG? qrcode is async — callers in
 * Svelte templates need a reactive wrapper instead. Use the dedicated
 * entityQrDataUrl store-friendly helper below.
 */
export async function entityQrDataUrl(payload: string, size = 120): Promise<string> {
  try {
    const lib = await ensureQrLib();
    return await lib.toDataURL(payload, {
      width: size,
      margin: 1,
      errorCorrectionLevel: 'M',
    });
  } catch {
    // Absolute fallback (online) — never render a broken image.
    return `https://api.qrserver.com/v1/create-qr-code/?size=${size}x${size}&data=${encodeURIComponent(payload)}`;
  }
}
