export function printHtmlDirectly(htmlContent: string, title = 'Thermal Receipt') {
  // Create an invisible iframe to isolate the print job completely
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
          .barcode-img { max-width: 100%; height: 35px; margin: 4px auto; display: block; }
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
      document.body.removeChild(iframe);
    }, 1000);
  }, 300);
}