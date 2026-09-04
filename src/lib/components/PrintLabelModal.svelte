<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Product } from '../types';
  import { printHtmlDirectly, printLabelSilently, type LabelPrintOutcome } from '../utils/printer';
  import {
    LABEL_PRESETS,
    LABEL_PRESET_IDS,
    buildLabelPresetHtml,
    toLabelCurrency,
    type LabelPresetId,
  } from '../printing/labelPresets';
  import { Printer, QrCode, Tag, X, RefreshCw, ZoomIn, ZoomOut, Loader2, CheckCircle2, AlertTriangle } from 'lucide-svelte';
  import JsBarcode from 'jsbarcode';

  export let isOpen = false;
  export let product: Product | null = null;
  export let onClose: () => void;
  export let initialType: 'barcode' | 'etiquette' = 'barcode';
  export let initialQty: number = 1;

  let labelType: 'barcode' | 'etiquette' = 'barcode';
  let copies = 1;
  // Built-in mm-true thermal presets are the default; 'custom' selects the
  // legacy settings-driven sticker/shelf modes below.
  let presetId: 'custom' | LabelPresetId = 'vprice40x20';
  let zoom = 1;
  // Print job state (exact-media silent pipeline).
  let isPrinting = false;
  let printOutcome: LabelPrintOutcome | null = null;

  // Settings loaded from DB
  let settings: Record<string, string> = {};
  let settingsLoaded = false;

  // Derived label dimensions from settings
  $: widthMm = parseInt(labelType === 'barcode'
    ? (settings.barcode_label_width || '50')
    : (settings.shelf_tag_width || '60'));
  $: heightMm = parseInt(labelType === 'barcode'
    ? (settings.barcode_label_height || '30')
    : (settings.shelf_tag_height || '40'));

  // Sticker preset config
  $: stickerShowShop    = (settings.sticker_show_shop_name  || 'true')  === 'true';
  $: stickerShowName    = (settings.sticker_show_product_name || 'true') === 'true';
  $: stickerShowBarcode = (settings.sticker_show_barcode    || 'true')  === 'true';
  $: stickerShowPrice   = (settings.sticker_show_price      || 'true')  === 'true';
  $: stickerNameSize    = parseInt(settings.sticker_name_font_size  || '12');
  $: stickerPriceSize   = parseInt(settings.sticker_price_font_size || '16');
  $: stickerBarcodeSize = parseInt(settings.sticker_barcode_font_size || '12');
  $: stickerNameBold    = (settings.sticker_name_bold   || 'true') === 'true';
  $: stickerPriceBold   = (settings.sticker_price_bold  || 'true') === 'true';
  $: stickerAlign       = settings.sticker_text_align || 'center';
  $: stickerOrientation = settings.sticker_orientation || 'landscape';

  // Shelf preset config
  $: shelfShowShop   = (settings.shelf_show_shop_name    || 'true') === 'true';
  $: shelfShowName   = (settings.shelf_show_product_name || 'true') === 'true';
  $: shelfShowPrice  = (settings.shelf_show_price        || 'true') === 'true';
  $: shelfShowRef    = (settings.shelf_show_ref          || 'true') === 'true';
  $: shelfNameSize   = parseInt(settings.shelf_name_font_size  || '16');
  $: shelfPriceSize  = parseInt(settings.shelf_price_font_size || '28');
  $: shelfRefSize    = parseInt(settings.shelf_ref_font_size   || '10');
  $: shelfNameBold   = (settings.shelf_name_bold  || 'true') === 'true';
  $: shelfPriceBold  = (settings.shelf_price_bold || 'true') === 'true';
  $: shelfAlign      = settings.shelf_text_align || 'center';
  $: shelfOrientation = settings.shelf_orientation || 'landscape';

  $: shopName = settings.shop_name_fr || 'TitaouPOS';

  let barcodeSvgEl: SVGSVGElement;
  let barcodeError = false;

  $: barcode = product?.barcodes?.[0] || product?.sku || '';

  function renderBarcode() {
    if (!barcodeSvgEl || !barcode) return;
    try {
      barcodeError = false;
      JsBarcode(barcodeSvgEl, barcode, {
        format: barcode.length === 13 ? 'EAN13' :
                barcode.length === 8  ? 'EAN8'  :
                barcode.length === 12 ? 'UPC'   : 'CODE128',
        width: 1.8,
        height: 44,
        displayValue: true,
        fontSize: stickerBarcodeSize,
        margin: 0,
        background: '#ffffff',
        lineColor: '#000000',
      });
    } catch (e) {
      barcodeError = true;
      try {
        JsBarcode(barcodeSvgEl, barcode, {
          format: 'CODE128',
          width: 1.8, height: 44,
          displayValue: true, fontSize: stickerBarcodeSize, margin: 0,
          background: '#ffffff', lineColor: '#000000',
        });
        barcodeError = false;
      } catch {
        barcodeError = true;
      }
    }
  }

  // Re-render barcode whenever relevant reactive values change
  $: if (barcode && barcodeSvgEl && labelType === 'barcode' && isOpen) {
    tick().then(renderBarcode);
  }

  $: if (isOpen) {
    labelType = initialType;
    copies = initialQty;
    // Default to the built-in preset matching how the modal was opened:
    // barcode sticker → Vertical Price, shelf etiquette → Shelf Price.
    presetId = initialType === 'etiquette' ? 'shelf40x20' : 'vprice40x20';
    zoom = 1;
  }

  // ---- Built-in thermal preset (mm-true) ----
  $: presetDef = presetId !== 'custom' ? LABEL_PRESETS[presetId] : null;
  $: presetData = {
    shopName: settings.shop_name_fr || 'TITAOU POS',
    productName: product?.name_fr || product?.name_ar || '',
    barcode: product?.barcodes?.[0] || product?.sku || '',
    price: product?.sale_price ?? 0,
    currency: toLabelCurrency(settings.default_currency),
  };
  $: presetHtml = presetDef && product ? presetDef.build(presetData) : '';

  onMount(async () => {
    try {
      const fetched = await invoke<Record<string, string>>('get_all_settings');
      settings = { ...settings, ...fetched };
      settingsLoaded = true;
      await tick();
      if (labelType === 'barcode') renderBarcode();
    } catch (e) {
      settingsLoaded = true;
    }
  });

  // Re-fetch settings every time the modal opens so recent changes made in
  // Settings (font sizes, bold, alignment...) are reflected without remount.
  $: if (isOpen) {
    invoke<Record<string, string>>('get_all_settings')
      .then((fetched) => {
        settings = { ...settings, ...fetched };
        settingsLoaded = true;
        if (labelType === 'barcode') tick().then(renderBarcode);
      })
      .catch(() => {});
  }

  async function handleTabChange(type: 'barcode' | 'etiquette') {
    labelType = type;
    copies = type === 'barcode' ? initialQty : 1;
    if (type === 'barcode') {
      await tick();
      renderBarcode();
    }
  }

  function getFlexAlign(align: string): string {
    if (align === 'left') return 'flex-start';
    if (align === 'right') return 'flex-end';
    return 'center';
  }

  function getFlexJustify(position: string): string {
    if (position === 'top') return 'flex-start';
    if (position === 'bottom') return 'flex-end';
    return 'center';
  }

  function buildStickerHtml(): string {
    const svgContent = barcodeSvgEl ? barcodeSvgEl.outerHTML : '';
    const flexAlign = getFlexAlign(stickerAlign);
    const positionJustify = getFlexJustify(settings.sticker_content_position || 'middle');
    const finalWidth = stickerOrientation === 'portrait' ? heightMm : widthMm;
    const finalHeight = stickerOrientation === 'portrait' ? widthMm : heightMm;
    const nameStyle = `font-size:${stickerNameSize}px; font-weight:${stickerNameBold ? '900' : '500'}; text-align:${stickerAlign}; margin:2px 0;`;
    const priceStyle = `font-size:${stickerPriceSize}px; font-weight:${stickerPriceBold ? '900' : '700'}; font-family:monospace; text-align:${stickerAlign}; margin:2px 0;`;
    return `
      <div style="width:${finalWidth}mm; height:${finalHeight}mm; padding:2mm; text-align:${stickerAlign};
                  display:flex; flex-direction:column; align-items:${flexAlign}; justify-content:${positionJustify};
                  font-family:sans-serif; overflow:hidden; box-sizing:border-box; background:#fff;">
        ${stickerShowShop ? `<span style="font-size:10px; font-weight:bold; text-transform:uppercase; color:#444; display:block; width:100%; text-align:${stickerAlign};">${shopName}</span>` : ''}
        ${stickerShowName ? `<span style="${nameStyle} display:block; overflow:hidden; white-space:nowrap; max-width:100%; width:100%;">${product?.name_fr || product?.name_ar || ''}</span>` : ''}
        ${stickerShowBarcode && barcode ? `<div style="max-width:100%; overflow:hidden; display:flex; justify-content:${flexAlign}; width:100%; align-items:center;">${svgContent}</div>` : ''}
        ${stickerShowPrice ? `<span style="${priceStyle} display:block; width:100%;">${product?.sale_price?.toLocaleString() || '0'} DZD</span>` : ''}
      </div>
    `;
  }

  function buildShelfTagHtml(): string {
    const flexAlign = getFlexAlign(shelfAlign);
    const positionJustify = getFlexJustify(settings.shelf_content_position || settings.sticker_content_position || 'middle');
    const finalWidth = shelfOrientation === 'portrait' ? heightMm : widthMm;
    const finalHeight = shelfOrientation === 'portrait' ? widthMm : heightMm;
    const nameStyle = `font-size:${shelfNameSize}px; font-weight:${shelfNameBold ? '900' : '600'}; text-align:${shelfAlign};`;
    const priceStyle = `font-size:${shelfPriceSize}px; font-weight:${shelfPriceBold ? '900' : '700'}; text-align:${shelfAlign};`;
    return `
      <div style="width:${finalWidth}mm; height:${finalHeight}mm; border:2px solid #000; padding:3mm;
                  font-family:sans-serif; text-align:${shelfAlign}; box-sizing:border-box; background:#fff;
                  display:flex; flex-direction:column; justify-content:${positionJustify}; overflow:hidden;">
        ${shelfShowShop ? `<div style="display:flex;justify-content:space-between;font-size:11px;font-weight:bold;border-bottom:1.5px solid #000;padding-bottom:2px;"><span>${shopName}</span><span style="color:#059669; font-weight:900;">DISPO</span></div>` : ''}
        ${shelfShowName ? `<p style="${nameStyle} margin:4px 0; line-height:1.2; width:100%;">${product?.name_fr || product?.name_ar || ''}</p>` : ''}
        ${shelfShowPrice ? `<div style="background:#000;color:#fff;padding:6px;${priceStyle}margin:4px 0;font-family:monospace;border-radius:4px;width:100%;">${product?.sale_price?.toLocaleString() || '0'} DZD</div>` : ''}
        ${shelfShowRef ? `<div style="display:flex;justify-content:space-between;font-size:${shelfRefSize}px;font-weight:bold;width:100%;"><span>Ref: ${barcode}</span><span>TVA 19% Incl.</span></div>` : ''}
      </div>
    `;
  }

  /**
   * Batch page assembly for browser-printed labels.
   *
   * Chromium's print engine produces a PHANTOM BLANK PAGE per label when the
   * wrapper uses `display:inline-block` + `page-break-after:always` (the
   * inline-level box after the forced break becomes its own empty page), and
   * the trailing break-after on the LAST label appends one more blank. The
   * rule: exactly N page nodes, break-after only on the first N-1, block
   * display, zero box model so content can never overflow the media box.
   */
  function buildLabelBatchHtml(singleHtml: string, wMm: number, hMm: number, count: number): string {
    const pages: string[] = [];
    for (let i = 0; i < count; i++) {
      const brk = i < count - 1 ? 'page-break-after:always;break-after:page;' : '';
      pages.push(
        `<div class="label-page" style="display:block;${brk}` +
        `width:${wMm}mm;height:${hMm}mm;margin:0;padding:0;overflow:hidden;` +
        `box-sizing:border-box;background:#fff;">${singleHtml}</div>`
      );
    }
    return pages.join('');
  }

  /**
   * Runtime assertion: the batch must contain exactly `count` page nodes —
   * duplicates or missing pages mean the phantom-page class of bug is back.
   * Returns an error message to surface loudly, or null when healthy.
   */
  function assertBatchPages(batchHtml: string, count: number): string | null {
    const found = (batchHtml.match(/class="label-page"/g) || []).length;
    if (found !== count) {
      return `Batch page assertion failed: expected ${count} label-page node(s), found ${found} — aborting print.`;
    }
    return null;
  }

  async function triggerPrint() {
    // Built-in mm-true presets use the exact-media silent pipeline: the
    // driver gets a custom 40×20mm DEVMODE and one page per copy — no A4
    // stock, no blank gaps, no print dialog. Falls back to the browser
    // print only if the backend command is unavailable (dev mode).
    if (presetDef) {
      isPrinting = true;
      printOutcome = null;
      try {
        printOutcome = await printLabelSilently({
          html: presetHtml,
          label: presetDef.name,
          widthMm: presetDef.widthMm,
          heightMm: presetDef.heightMm,
          copies,
          printer: settings.label_printer || undefined,
          dpi: parseInt(settings.label_printer_dpi || '203', 10) || 203,
        });
      } catch (e: any) {
        // Backend missing (old binary / dev) → legacy iframe print, using
        // the same exact-N-pages assembly as the fixed batch path.
        const combined = buildLabelBatchHtml(presetHtml, presetDef.widthMm, presetDef.heightMm, copies);
        const bad = assertBatchPages(combined, copies);
        if (bad) {
          printOutcome = { ok: false, message: bad, diagnostics: null } as any;
          isPrinting = false;
          return;
        }
        printHtmlDirectly(
          combined,
          `Label ${presetDef.name}`,
          { widthMm: presetDef.widthMm, heightMm: presetDef.heightMm }
        );
        printOutcome = null;
      } finally {
        isPrinting = false;
      }
      return;
    }
    let singleHtml = labelType === 'barcode' ? buildStickerHtml() : buildShelfTagHtml();
    // Print on the exact configured label size so px font settings map 1:1
    // to paper instead of being rescaled by the default 80mm receipt page.
    const finalW = labelType === 'barcode'
      ? (stickerOrientation === 'portrait' ? heightMm : widthMm)
      : (shelfOrientation === 'portrait' ? heightMm : widthMm);
    const finalH = labelType === 'barcode'
      ? (stickerOrientation === 'portrait' ? widthMm : heightMm)
      : (shelfOrientation === 'portrait' ? widthMm : heightMm);
    const combinedHtml = buildLabelBatchHtml(singleHtml, finalW, finalH, copies);
    const badCustom = assertBatchPages(combinedHtml, copies);
    if (badCustom) {
      alert(badCustom);
      return;
    }
    printHtmlDirectly(
      combinedHtml,
      labelType === 'barcode' ? 'Product Sticker' : 'Shelf Etiquette',
      { widthMm: finalW, heightMm: finalH }
    );
  }
</script>

{#if isOpen && product}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-2xl shadow-2xl w-full max-w-lg overflow-hidden animate-in fade-in duration-150">

      <!-- Header -->
      <div class="flex items-center justify-between px-5 py-3.5 border-b border-pos-border bg-slate-50 dark:bg-slate-800/50">
        <h3 class="font-black text-sm text-pos-text flex items-center gap-2">
          <QrCode class="w-4 h-4 text-sky-500" />
          <span>Print Label / طباعة الملصق</span>
        </h3>
        <button on:click={onClose} class="text-pos-muted hover:text-pos-text p-1 rounded-lg cursor-pointer">
          <X class="w-4 h-4" />
        </button>
      </div>

      <div class="p-5 space-y-4">
        <!-- Print Preset Tabs (built-in mm-true thermal presets vs settings-driven custom) -->
        <div>
          <label class="block text-[11px] font-bold text-pos-muted mb-1.5">Preset / نوع الملصق</label>
          <div class="grid grid-cols-3 gap-2">
            <button
              type="button"
              on:click={() => { presetId = 'vprice40x20'; }}
              class="px-2.5 py-2 rounded-xl border text-xs font-bold transition-all cursor-pointer flex flex-col items-center justify-center text-center gap-0.5 {presetId === 'vprice40x20' ? 'border-sky-500 bg-sky-50 dark:bg-sky-950/60 text-sky-600 dark:text-sky-400 shadow-xs' : 'border-pos-border bg-slate-50 dark:bg-slate-800/40 text-pos-muted hover:text-pos-text'}"
            >
              <span class="leading-tight">Vertical Price</span>
              <span class="text-[10px] opacity-75 font-mono">40×20 mm</span>
            </button>
            <button
              type="button"
              on:click={() => { presetId = 'shelf40x20'; }}
              class="px-2.5 py-2 rounded-xl border text-xs font-bold transition-all cursor-pointer flex flex-col items-center justify-center text-center gap-0.5 {presetId === 'shelf40x20' ? 'border-emerald-500 bg-emerald-50 dark:bg-emerald-950/60 text-emerald-600 dark:text-emerald-400 shadow-xs' : 'border-pos-border bg-slate-50 dark:bg-slate-800/40 text-pos-muted hover:text-pos-text'}"
            >
              <span class="leading-tight">Shelf Price</span>
              <span class="text-[10px] opacity-75 font-mono">40×20 mm</span>
            </button>
            <button
              type="button"
              on:click={() => { presetId = 'custom'; }}
              class="px-2.5 py-2 rounded-xl border text-xs font-bold transition-all cursor-pointer flex flex-col items-center justify-center text-center gap-0.5 {presetId === 'custom' ? 'border-indigo-500 bg-indigo-50 dark:bg-indigo-950/60 text-indigo-600 dark:text-indigo-400 shadow-xs' : 'border-pos-border bg-slate-50 dark:bg-slate-800/40 text-pos-muted hover:text-pos-text'}"
            >
              <span class="leading-tight">Custom</span>
              <span class="text-[10px] opacity-75">Settings</span>
            </button>
          </div>
        </div>

        {#if presetId === 'custom'}
        <!-- Label Type Tabs -->
        <div class="grid grid-cols-2 gap-2">
          <button
            type="button"
            on:click={() => handleTabChange('barcode')}
            class="p-2.5 rounded-xl border font-bold text-xs transition cursor-pointer flex items-center justify-center gap-1.5 {labelType === 'barcode' ? 'border-sky-500 bg-sky-50 dark:bg-sky-950 text-sky-600' : 'border-pos-border text-pos-muted'}"
          >
            <QrCode class="w-3.5 h-3.5" />
            Product Sticker (ملصق باركود)
          </button>
          <button
            type="button"
            on:click={() => handleTabChange('etiquette')}
            class="p-2.5 rounded-xl border font-bold text-xs transition cursor-pointer flex items-center justify-center gap-1.5 {labelType === 'etiquette' ? 'border-emerald-500 bg-emerald-50 dark:bg-emerald-950 text-emerald-600' : 'border-pos-border text-pos-muted'}"
          >
            <Tag class="w-3.5 h-3.5" />
            Shelf Etiquette (بطاقة رف)
          </button>
        </div>

        <!-- Preview Card -->
        {#if labelType === 'barcode'}
          <div
            style="text-align: {stickerAlign};"
            class="p-4 bg-white text-slate-900 border-2 border-dashed border-slate-300 rounded-xl flex flex-col justify-center space-y-1.5 shadow-xs min-h-[130px] w-full"
          >
            {#if stickerShowShop}
              <span class="text-[10px] text-slate-500 font-bold uppercase tracking-wider block" style="text-align: {stickerAlign};">{shopName}</span>
            {/if}
            {#if stickerShowName}
              <p class="text-slate-900 leading-tight line-clamp-2 block" style="font-size:{stickerNameSize}px; font-weight:{stickerNameBold ? '900' : '500'}; text-align: {stickerAlign};">
                {product.name_fr || product.name_ar}
              </p>
            {/if}
            {#if stickerShowBarcode && barcode}
              <div class="w-full flex py-0.5 overflow-hidden" style="justify-content: {getFlexAlign(stickerAlign)};">
                {#if barcodeError}
                  <div class="font-mono text-sm tracking-[0.25em] font-black border-y border-slate-900 py-0.5 inline-block px-4">
                    ||| | |||| | |||<br/>
                    <span class="text-[10px]">{barcode}</span>
                  </div>
                {:else}
                  <svg bind:this={barcodeSvgEl} class="max-w-full"></svg>
                {/if}
              </div>
            {/if}
            {#if stickerShowPrice}
              <span class="font-mono block" style="font-size:{stickerPriceSize}px; font-weight:{stickerPriceBold ? '900' : '700'}; text-align: {stickerAlign};">
                {product.sale_price.toLocaleString()} DZD
              </span>
            {/if}
          </div>

        {:else}
          <!-- Shelf Etiquette Preview -->
          <div
            style="text-align: {shelfAlign};"
            class="p-4 bg-white text-slate-900 border-2 border-dashed border-emerald-300 rounded-xl flex flex-col justify-between space-y-2 shadow-xs min-h-[140px] w-full"
          >
            {#if shelfShowShop}
              <div class="w-full flex justify-between text-[10px] font-bold text-slate-500 border-b border-slate-300 pb-0.5">
                <span>{shopName}</span>
                <span class="text-emerald-600 font-black">DISPO EN RAYON</span>
              </div>
            {/if}
            {#if shelfShowName}
              <p class="text-slate-900 leading-tight block" style="font-size:{shelfNameSize}px; font-weight:{shelfNameBold ? '900' : '600'}; text-align: {shelfAlign};">
                {product.name_fr || product.name_ar}
              </p>
            {/if}
            {#if shelfShowPrice}
              <div class="w-full bg-slate-900 text-white font-mono font-black rounded py-1.5 px-3 block" style="font-size:{shelfPriceSize}px; font-weight:{shelfPriceBold ? '900' : '700'}; text-align: {shelfAlign};">
                {product.sale_price.toLocaleString()} DZD
              </div>
            {/if}
            {#if shelfShowRef}
              <div class="w-full flex justify-between font-bold text-slate-500" style="font-size:{shelfRefSize}px;">
                <span>Ref: {barcode}</span>
                <span>TVA 19% Incl.</span>
              </div>
            {/if}
          </div>
        {/if}
        {/if}

        {#if presetDef}
          <!-- Exact-media preview: every copy stacked consecutively, exactly
               as the print job emits them (N × 20mm, zero gaps). -->
          <div class="space-y-2">
            <div class="flex items-center justify-between">
              <span class="text-[11px] font-bold text-pos-muted">
                Print preview — {copies} × {presetDef.widthMm}×{presetDef.heightMm}mm ({copies * presetDef.heightMm}mm total)
              </span>
              <div class="flex items-center gap-1">
                <button
                  type="button"
                  on:click={() => (zoom = Math.max(1, zoom - 1))}
                  class="p-1.5 rounded-lg bg-slate-100 dark:bg-slate-800 text-pos-text cursor-pointer"
                  title="Zoom out"
                >
                  <ZoomOut class="w-3.5 h-3.5" />
                </button>
                <span class="text-[11px] font-black font-mono text-pos-text w-8 text-center">{zoom}×</span>
                <button
                  type="button"
                  on:click={() => (zoom = Math.min(8, zoom + 1))}
                  class="p-1.5 rounded-lg bg-slate-100 dark:bg-slate-800 text-pos-text cursor-pointer"
                  title="Zoom in"
                >
                  <ZoomIn class="w-3.5 h-3.5" />
                </button>
              </div>
            </div>
            <div class="bg-white border-2 border-dashed border-slate-300 rounded-xl p-3 overflow-auto flex flex-col items-center gap-[2px] max-h-64">
              {#each Array(Math.min(copies, 12)) as _, i}
                <div
                  class="label-strip-item"
                  style="width: calc({presetDef.widthMm}mm * {zoom}); height: calc({presetDef.heightMm}mm * {zoom}); flex: 0 0 auto;"
                >
                  <div class="label-strip-inner" style="width: {presetDef.widthMm}mm; height: {presetDef.heightMm}mm; transform: scale({zoom}); transform-origin: top left;">
                    {@html presetHtml}
                  </div>
                </div>
              {/each}
              {#if copies > 12}
                <span class="text-[10px] text-pos-muted font-bold py-1">+ {copies - 12} more…</span>
              {/if}
            </div>
            <p class="text-[10px] text-pos-muted text-center">
              Physical Size: <span class="font-bold">{presetDef.widthMm} × {presetDef.heightMm} mm</span> •
              Orientation: <span class="font-bold">Landscape</span> •
              Media locked per label — no gaps, {copies} page(s)
            </p>
            <style>
              .label-strip-item { position: relative; overflow: hidden; }
              .label-strip-inner { position: absolute; top: 0; left: 0; }
            </style>
          </div>

          <!-- Print job diagnostics (exact-media pipeline) -->
          {#if printOutcome}
            <div class="rounded-xl border p-2.5 text-[10px] font-mono {printOutcome.ok ? 'border-emerald-300 bg-emerald-50 dark:bg-emerald-950/40 text-emerald-800 dark:text-emerald-300' : 'border-amber-300 bg-amber-50 dark:bg-amber-950/40 text-amber-800 dark:text-amber-300'}">
              <div class="flex items-start gap-1.5">
                {#if printOutcome.ok}
                  <CheckCircle2 class="w-3.5 h-3.5 shrink-0 mt-0.5" />
                {:else}
                  <AlertTriangle class="w-3.5 h-3.5 shrink-0 mt-0.5" />
                {/if}
                <div class="space-y-0.5 leading-relaxed">
                  <div class="font-bold">{printOutcome.message}</div>
                  <div>Printer: {printOutcome.diagnostics.printer} • DPI: {printOutcome.diagnostics.dpi}</div>
                  <div>Media: {printOutcome.diagnostics.media_width_mm}×{printOutcome.diagnostics.media_height_mm}mm • Pages: {printOutcome.diagnostics.page_count} • Raster: {printOutcome.diagnostics.raster_width_px}×{printOutcome.diagnostics.raster_height_px}px</div>
                  <div>Generated print: {printOutcome.diagnostics.print_width_mm}mm × {printOutcome.diagnostics.print_height_mm}mm total</div>
                </div>
              </div>
            </div>
          {/if}
        {/if}

        <!-- Print Parameters: Copies + Dimensions (read-only from settings) -->
        <div class="grid grid-cols-3 gap-2">
          <div>
            <label class="block text-[11px] font-bold text-pos-muted mb-1">
              {presetDef ? 'Copies (Labels)' : labelType === 'barcode' ? 'Copies (Stickers)' : 'Copies (Tags)'}
            </label>
            <input
              type="number"
              bind:value={copies}
              min="1"
              class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-bold font-mono outline-none"
            />
          </div>
          <div>
            <label class="block text-[11px] font-bold text-pos-muted mb-1">Width (mm)</label>
            <div class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-bold font-mono text-center opacity-70">
              {presetDef ? presetDef.widthMm : widthMm}mm
            </div>
          </div>
          <div>
            <label class="block text-[11px] font-bold text-pos-muted mb-1">Height (mm)</label>
            <div class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-bold font-mono text-center opacity-70">
              {presetDef ? presetDef.heightMm : heightMm}mm
            </div>
          </div>
        </div>

        {#if !settingsLoaded}
          <p class="text-[11px] text-pos-muted flex items-center gap-1">
            <RefreshCw class="w-3 h-3 animate-spin" /> Loading label settings...
          </p>
        {:else if presetDef}
          <p class="text-[10px] text-pos-muted">
            Built-in preset with fixed mm geometry — real barcode, auto-fit text, dynamic product data.
          </p>
        {:else}
          <p class="text-[10px] text-pos-muted">
            Dimensions & content configured in
            <span class="font-bold text-sky-600">Settings → Barcode Labels</span>
          </p>
        {/if}
      </div>

      <!-- Action Footer -->
      <div class="px-5 py-3.5 border-t border-pos-border bg-slate-50 dark:bg-slate-800/50 flex justify-end gap-2">
        <button on:click={onClose} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-xl cursor-pointer">
          Cancel
        </button>
        <button
          type="button"
          on:click={triggerPrint}
          disabled={isPrinting}
          class="px-5 py-2 {presetDef || labelType === 'barcode' ? 'bg-sky-600 hover:bg-sky-700' : 'bg-emerald-600 hover:bg-emerald-700'} text-white font-black text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-md transition active:scale-95 disabled:opacity-50 disabled:cursor-wait"
        >
          {#if isPrinting}
            <Loader2 class="w-4 h-4 animate-spin" />
            <span>Printing…</span>
          {:else}
            <Printer class="w-4 h-4" />
            <span>Print {copies}× {presetDef ? 'Label(s)' : labelType === 'barcode' ? 'Sticker(s)' : 'Tag(s)'}</span>
          {/if}
        </button>
      </div>
    </div>
  </div>
{/if}