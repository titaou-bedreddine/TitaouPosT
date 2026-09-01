/**
 * Built-in mm-true thermal label presets.
 *
 * Every preset is a parameterized layout definition: fixed canvas (mm) +
 * absolutely-positioned elements whose geometry (x/y/w/h in mm), fonts
 * (size in mm, weight) and auto-fit rules live in one place. The builder
 * emits self-contained HTML (inline styles + one scoped <style>) that is
 * used for BOTH the on-screen preview and the actual print job, so what
 * you see is exactly what prints. CSS mm units are physical, which keeps
 * the printed size at exactly W×H mm regardless of DPI (203/300/600).
 *
 * Data is always dynamic ({{SHOP_NAME}}/{{PRODUCT_NAME}}/{{BARCODE}}/
 * {{PRICE}}/{{CURRENCY}}) — nothing from the reference artwork is baked in.
 */
import JsBarcode from 'jsbarcode';

export interface LabelData {
  shopName: string;
  productName: string;
  barcode: string;
  price: number;
  currency: string;
}

export type LabelPresetId = 'vprice40x20' | 'shelf40x20';

export interface LabelPresetDef {
  id: LabelPresetId;
  name: string;
  widthMm: number;
  heightMm: number;
  build: (data: LabelData) => string;
}

// DZD is the ISO code; labels show the local symbol "DA" like the reference.
export function toLabelCurrency(cur?: string | null): string {
  const c = (cur || 'DA').trim().toUpperCase();
  return c === 'DZD' || c === '' ? 'DA' : c;
}

// 1.20 DZD = "120.00" style grouping: thousands separated with plain spaces
// (plain space — narrow no-break space is missing from thermal font sets).
export function formatPriceAmount(v: number, decimals = 2): string {
  const fixed = (Number.isFinite(v) ? v : 0).toFixed(decimals);
  const [intPart, decPart] = fixed.split('.');
  const grouped = intPart.replace(/\B(?=(\d{3})+(?!\d))/g, ' ');
  return decPart ? `${grouped}.${decPart}` : grouped;
}

// ---------------------------------------------------------------------------
// Geometry helpers (CSS mm units are physical: 1mm = 96/25.4 CSS px)
// ---------------------------------------------------------------------------
const PX_PER_MM = 96 / 25.4;

// Windows condensed-heavy grotesques; Bahnschrift (Win10+) matches the
// reference artwork closest, Impact/Arial Black are the fallbacks.
const FONT_HEAVY = `'Bahnschrift Condensed','Bahnschrift','Arial Narrow',Impact,'Arial Black',Arial,sans-serif`;
const FONT_DIGITS = `'Bahnschrift','Arial Narrow','Segoe UI',Arial,sans-serif`;

function mm(v: number): string {
  return `${Math.round(v * 100) / 100}mm`;
}

function esc(s: string): string {
  return String(s ?? '')
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}

let measureCtx: CanvasRenderingContext2D | null = null;
function textWidthMm(text: string, sizeMm: number, weight: number, family: string): number {
  if (!measureCtx) measureCtx = document.createElement('canvas').getContext('2d');
  const ctx = measureCtx;
  if (!ctx) return text.length * sizeMm * 0.6;
  ctx.font = `${weight} ${sizeMm * PX_PER_MM}px ${family}`;
  return ctx.measureText(text).width / PX_PER_MM;
}

/**
 * Shrink the font size until the text fits maxWidMm on ONE line, then
 * hard-truncate with an ellipsis if even the minimum size overflows.
 */
function fitText(
  text: string,
  weight: number,
  family: string,
  maxSizeMm: number,
  minSizeMm: number,
  maxWidMm: number
): { size: number; text: string } {
  let size = maxSizeMm;
  let t = text;
  while (size > minSizeMm && textWidthMm(t, size, weight, family) > maxWidMm) {
    size = Math.max(minSizeMm, Math.round((size - 0.1) * 10) / 10);
  }
  if (textWidthMm(t, size, weight, family) > maxWidMm) {
    while (t.length > 1 && textWidthMm(`${t}…`, size, weight, family) > maxWidMm) {
      t = t.slice(0, -1);
    }
    t = `${t}…`;
  }
  return { size, text: t };
}

// Centered absolutely-positioned text box (all values mm).
function textBox(
  x: number,
  y: number,
  w: number,
  h: number,
  inner: string,
  extra = ''
): string {
  return `<div style="position:absolute;left:${mm(x)};top:${mm(y)};width:${mm(w)};height:${mm(h)};display:flex;align-items:center;justify-content:center;${extra}">${inner}</div>`;
}

function textSpan(size: number, weight: number, content: string, family = FONT_HEAVY, extra = ''): string {
  return `<span style="font-family:${family};font-weight:${weight};font-size:${mm(size)};line-height:1;white-space:nowrap;${extra}">${content}</span>`;
}

// Physical width of one barcode module (X-dimension) in mm. Retail scanners
// need >= 0.19mm; 0.26mm gives comfortable margin at 203 DPI (2 printer dots).
const BARCODE_MODULE_MM = 0.26;

/**
 * REAL machine-readable barcode rendered by JsBarcode. The SVG carries a
 * viewBox in JsBarcode "px" units and is stretched (via width/height
 * attributes in mm) to the exact physical box, so every module prints at a
 * constant physical width — no uniform "meet" scaling that would silently
 * narrow bars on small labels. EAN-13/UPC/EAN-8 by length, CODE128 fallback.
 * The mm box must be >= barcodeRequiredWidthMm(value, moduleMm) or the bars
 * compress; receipts with long alphanumeric values can pass a smaller
 * moduleMm (>= 0.19mm stays scanner-safe).
 */
export function barcodeSvgHtml(
  value: string,
  boxWMm: number,
  boxHMm: number,
  moduleMm: number = BARCODE_MODULE_MM
): string {
  const val = String(value || '').trim();
  if (!val) return '';
  const formats =
    val.length === 13 ? ['EAN13', 'CODE128'] :
    val.length === 8  ? ['EAN8', 'CODE128'] :
    val.length === 12 ? ['UPC', 'CODE128'] :
    ['CODE128'];
  for (const format of formats) {
    try {
      const el = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
      JsBarcode(el, val, {
        format,
        width: 1,               // 1 unit per module → viewBox maps 1:1 to modules
        height: 100,            // aspect only; height is set by the mm box
        displayValue: false,
        margin: 0,
        background: '#ffffff',
        lineColor: '#000000',
      });
      const vbW = parseFloat(el.getAttribute('width') || '');
      const vbH = parseFloat(el.getAttribute('height') || '');
      if (!vbW || !vbH) continue;
      // Effective module size: the requested physical width, compressed only
      // if the barcode's natural width would exceed the box (never enlarged
      // past it, so a short barcode keeps its scanner-safe module size).
      const effModule = Math.min(moduleMm, boxWMm / vbW);
      const physW = vbW * effModule;
      const physH = boxHMm;
      el.setAttribute('viewBox', `0 0 ${vbW} ${vbH}`);
      el.setAttribute('preserveAspectRatio', 'none');
      el.setAttribute('style', `width:${physW.toFixed(2)}mm;height:${physH}mm;display:block`);
      // Quiet zone: centered white padding inside the box around the bars.
      const pad = Math.max(0, (boxWMm - physW) / 2);
      return `<div style="width:${boxWMm}mm;height:${physH}mm;background:#fff;padding:0 ${pad.toFixed(2)}mm;box-sizing:border-box;overflow:hidden;">${el.outerHTML}</div>`;
    } catch {
      // invalid checksum etc → try the next format (CODE128 accepts anything)
    }
  }
  return '';
}

/** Physical width (mm) the barcode needs at the given module size. EAN-13 =
 * 113 modules incl. quiet zones, EAN-8 = 81, UPC = 111; CODE128 varies via
 * 11 modules/char + 35 overhead. */
export function barcodeRequiredWidthMm(value: string, moduleMm: number = BARCODE_MODULE_MM): number {
  const val = String(value || '').trim();
  const modules =
    val.length === 13 ? 113 :
    val.length === 8  ? 81 :
    val.length === 12 ? 111 :
    35 + val.length * 11;
  return modules * moduleMm;
}

// ---------------------------------------------------------------------------
// Preset 1 — "40×20 mm – Vertical Price"
// Barcode label: shop name / rule / product name / barcode + digits on the
// left, vertical rule, huge price rotated 90° (reads bottom→top) on the right.
// ---------------------------------------------------------------------------
export function buildVerticalPriceLabelHtml(data: LabelData): string {
  const W = 40;
  const H = 20;

  // Outer rounded border (kept inside the printable area)
  const bInset = 0.3;
  const bStroke = 0.45;
  const bRadius = 0.8;

  // Divider between main area and price column
  const divX = 28.85;
  const divY = 0.7;
  const divW = 0.35;
  const divH = 18.6;

  // Price column (rotated text length limit = column height - margins)
  const priceX = 29.3;
  const priceY = 0.7;
  const priceW = 9.7;
  const priceH = 18.6;
  const priceMaxLen = priceH - 1.2;

  const shop = fitText((data.shopName || 'TITAOU POS').toUpperCase(), 900, FONT_HEAVY, 4.2, 2.0, 25.5);
  const name = fitText((data.productName || '').toUpperCase(), 800, FONT_HEAVY, 2.9, 1.3, 25.5);

  const amount = formatPriceAmount(data.price ?? 0);
  const cur = toLabelCurrency(data.currency);
  const curSizeOf = (s: number) => Math.max(1.4, Math.round(s * 52) / 100);
  const priceLenAt = (s: number) =>
    textWidthMm(amount, s, 900, FONT_HEAVY) + 0.5 + textWidthMm(cur, curSizeOf(s), 800, FONT_HEAVY);
  let priceSize = 6.5;
  while (priceSize > 1.6 && priceLenAt(priceSize) > priceMaxLen) {
    priceSize = Math.round((priceSize - 0.1) * 10) / 10;
  }
  const curSize = curSizeOf(priceSize);

  // Barcode uses the full main-area width so modules print at the fixed
  // physical size (no shrink); quiet zones are the box's white padding.
  const bcBoxW = 26.6;
  const bcBoxH = 6.4;
  const svg = barcodeSvgHtml(data.barcode, bcBoxW, bcBoxH);
  const digits = String(data.barcode || '').trim();

  const style = `<style>.tpLblV,.tpLblV *{box-sizing:border-box;margin:0;padding:0;}</style>`;
  const parts: string[] = [];
  parts.push(
    `<div style="position:absolute;left:${mm(bInset)};top:${mm(bInset)};width:${mm(W - 2 * bInset)};height:${mm(H - 2 * bInset)};border:${mm(bStroke)} solid #000;border-radius:${mm(bRadius)};"></div>`
  );
  parts.push(
    `<div style="position:absolute;left:${mm(divX)};top:${mm(divY)};width:${mm(divW)};height:${mm(divH)};background:#000;"></div>`
  );
  parts.push(textBox(1.5, 1.0, 26, 3.8, textSpan(shop.size, 900, esc(shop.text))));
  parts.push(
    `<div style="position:absolute;left:${mm(1.7)};top:${mm(5.15)};width:${mm(25.6)};height:${mm(0.3)};background:#000;"></div>`
  );
  parts.push(textBox(1.5, 5.95, 26, 3.0, textSpan(name.size, 800, esc(name.text))));
  if (svg) {
    parts.push(
      `<div style="position:absolute;left:${mm(1.6)};top:${mm(9.1)};">${svg}</div>`
    );
  }
  if (digits) {
    parts.push(
      textBox(1.5, 16.1, 26, 2.0, textSpan(1.9, 500, esc(digits), FONT_DIGITS, 'letter-spacing:0.3mm;'))
    );
  }
  parts.push(
    `<div style="position:absolute;left:${mm(priceX)};top:${mm(priceY)};width:${mm(priceW)};height:${mm(priceH)};">` +
      `<div style="position:absolute;left:50%;top:50%;transform:translate(-50%,-50%) rotate(-90deg);white-space:nowrap;line-height:1;">` +
        textSpan(priceSize, 900, esc(amount)) +
        textSpan(curSize, 800, esc(cur), FONT_HEAVY, 'margin-left:0.5mm;') +
      `</div>` +
    `</div>`
  );

  return (
    `${style}` +
    `<div class="tpLblV" style="position:relative;width:${mm(W)};height:${mm(H)};background:#fff;overflow:hidden;">` +
    parts.join('') +
    `</div>`
  );
}

// ---------------------------------------------------------------------------
// Preset 2 — "40×20 mm – Shelf Price"
// Shelf tag: shop name / rule / product name, then the DOMINANT price with a
// smaller currency symbol beside it. No barcode, no divider.
// ---------------------------------------------------------------------------
export function buildShelfPriceLabelHtml(data: LabelData): string {
  const W = 40;
  const H = 20;

  const bInset = 0.25;
  const bStroke = 0.45;
  const bRadius = 0.8;

  const shop = fitText((data.shopName || 'TITAOU POS').toUpperCase(), 900, FONT_HEAVY, 4.2, 2.0, 34);
  const name = fitText((data.productName || '').toUpperCase(), 800, FONT_HEAVY, 2.9, 1.5, 34);

  // Price + currency render as ONE centered baseline group; the amount shrinks
  // until the whole group fits, staying as large as possible.
  const amount = formatPriceAmount(data.price ?? 0);
  const cur = toLabelCurrency(data.currency);
  const curSize = 2.6;
  const groupLenAt = (s: number) => textWidthMm(amount, s, 900, FONT_HEAVY) + 0.8 + textWidthMm(cur, curSize, 800, FONT_HEAVY);
  let priceSize = 8.0;
  while (priceSize > 2.2 && groupLenAt(priceSize) > 34) {
    priceSize = Math.round((priceSize - 0.1) * 10) / 10;
  }

  const style = `<style>.tpLblS,.tpLblS *{box-sizing:border-box;margin:0;padding:0;}</style>`;
  const parts: string[] = [];
  parts.push(
    `<div style="position:absolute;left:${mm(bInset)};top:${mm(bInset)};width:${mm(W - 2 * bInset)};height:${mm(H - 2 * bInset)};border:${mm(bStroke)} solid #000;border-radius:${mm(bRadius)};"></div>`
  );
  parts.push(textBox(2, 0.9, 36, 4.0, textSpan(shop.size, 900, esc(shop.text))));
  parts.push(
    `<div style="position:absolute;left:${mm(2)};top:${mm(5.45)};width:${mm(36)};height:${mm(0.35)};background:#000;"></div>`
  );
  parts.push(textBox(2, 6.05, 36, 3.3, textSpan(name.size, 800, esc(name.text))));
  parts.push(
    `<div style="position:absolute;left:${mm(2)};top:${mm(9.4)};width:${mm(36)};height:${mm(9.0)};display:flex;align-items:center;justify-content:center;">` +
      `<div style="display:flex;align-items:baseline;white-space:nowrap;line-height:1;">` +
        textSpan(priceSize, 900, esc(amount)) +
        textSpan(curSize, 800, esc(cur), FONT_HEAVY, 'margin-left:0.8mm;') +
      `</div>` +
    `</div>`
  );

  return (
    `${style}` +
    `<div class="tpLblS" style="position:relative;width:${mm(W)};height:${mm(H)};background:#fff;overflow:hidden;">` +
    parts.join('') +
    `</div>`
  );
}

// ---------------------------------------------------------------------------
// Registry — add future presets (80×40, price-only, shelf large…) here.
// ---------------------------------------------------------------------------
export const LABEL_PRESETS: Record<LabelPresetId, LabelPresetDef> = {
  vprice40x20: {
    id: 'vprice40x20',
    name: '40×20 mm – Vertical Price',
    widthMm: 40,
    heightMm: 20,
    build: buildVerticalPriceLabelHtml,
  },
  shelf40x20: {
    id: 'shelf40x20',
    name: '40×20 mm – Shelf Price',
    widthMm: 40,
    heightMm: 20,
    build: buildShelfPriceLabelHtml,
  },
};

export const LABEL_PRESET_IDS = Object.keys(LABEL_PRESETS) as LabelPresetId[];

export function buildLabelPresetHtml(id: LabelPresetId, data: LabelData): string {
  return LABEL_PRESETS[id].build(data);
}
