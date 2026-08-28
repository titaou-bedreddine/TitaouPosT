export function printHtmlDirectly(htmlContent: string, title = 'Thermal Receipt') {
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

  doc.open();
  doc.write(`
    <!DOCTYPE html>
    <html>
      <head>
        <title>${title}</title>
        <meta charset="utf-8" />
        <style>
          @page {
            size: 80mm auto;
            margin: 0mm;
          }
          * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
            font-family: 'Courier New', Courier, monospace, 'Segoe UI', Tahoma, sans-serif;
            color: #000;
          }
          body {
            width: 76mm;
            margin: 2mm auto;
            font-size: 11px;
            line-height: 1.3;
            background: #fff;
            padding: 2mm;
            position: relative;
          }
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
  items: Array<{ name: string; quantity: number; unitPrice: number; totalPrice: number }>;
  subtotal: number;
  discount: number;
  grandTotal: number;
  paymentMethod: string;
  isCredit?: boolean;
  copyLabel?: string;
}) {
  const qrData = encodeURIComponent(`SALE:${options.saleNumber}`);
  const qrUrl = `https://api.qrserver.com/v1/create-qr-code/?size=100x100&data=${qrData}`;

  return `
    ${options.isCredit ? '<div class="watermark">CREDIT / دين</div>' : ''}
    <div class="text-center pb-2 border-b-dashed">
      <h2 class="font-black text-sm uppercase">${options.shopName || 'TitaouPOS'}</h2>
      <p class="text-xxs">${options.shopAddress || 'Alger, Algérie'}</p>
      <p class="text-xxs">Tél: ${options.shopPhone || '0553444057'}</p>
      ${options.shopRc ? `<p class="text-xxs">RC: ${options.shopRc} ${options.shopNif ? '| NIF: ' + options.shopNif : ''}</p>` : ''}
      <p class="text-xxs mt-1">${options.saleDate}</p>
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
      <table>
        <thead>
          <tr>
            <th style="width: 50%;">Article</th>
            <th class="text-center" style="width: 15%;">Qté</th>
            <th class="text-end" style="width: 35%;">Total</th>
          </tr>
        </thead>
        <tbody>
          ${options.items
            .map(
              (i) => `
            <tr>
              <td class="font-bold">${i.name}</td>
              <td class="text-center font-mono">${i.quantity}</td>
              <td class="text-end font-mono font-bold">${i.totalPrice.toLocaleString()} DZD</td>
            </tr>
          `
            )
            .join('')}
        </tbody>
      </table>
    </div>

    <div class="py-1 border-b-dashed text-xs space-y-1">
      ${options.discount > 0 ? `
        <div class="flex justify-between text-xxs">
          <span>Sous-Total:</span>
          <span class="font-mono">${options.subtotal.toLocaleString()} DZD</span>
        </div>
        <div class="flex justify-between text-xxs">
          <span>Remise:</span>
          <span class="font-mono text-rose-600">-${options.discount.toLocaleString()} DZD</span>
        </div>
      ` : ''}
      <div class="flex justify-between font-black text-sm pt-0.5">
        <span>TOTAL A PAYER:</span>
        <span class="font-mono">${options.grandTotal.toLocaleString()} DZD</span>
      </div>
    </div>

    <div class="text-center pt-2 border-t-dashed">
      <img src="${qrUrl}" alt="QR" class="qr-box" />
      <p class="text-xxs text-gray-700">Scan QR to verify or lookup ticket</p>
      <p class="text-xxs mt-1 font-bold">*** Merci pour votre visite ***</p>
      <p class="text-[8px] text-gray-500 mt-1">TitaouPOS • Created by Titaou Bedreddine 0553444057</p>
    </div>
  `;
}