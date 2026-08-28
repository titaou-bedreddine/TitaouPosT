<script lang="ts">
  import { onMount, tick } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Product } from '../types';
  import { printHtmlDirectly } from '../utils/printer';
  import { Printer, QrCode, Tag, X, RefreshCw } from 'lucide-svelte';
  import JsBarcode from 'jsbarcode';

  export let isOpen = false;
  export let product: Product | null = null;
  export let onClose: () => void;
  export let initialType: 'barcode' | 'etiquette' = 'barcode';
  export let initialQty: number = 1;

  let labelType: 'barcode' | 'etiquette' = 'barcode';
  let copies = 1;

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
  $: stickerNameSize    = parseInt(settings.sticker_name_font_size  || '9');
  $: stickerPriceSize   = parseInt(settings.sticker_price_font_size || '12');
  $: stickerBarcodeSize = parseInt(settings.sticker_barcode_font_size || '10');
  $: stickerNameBold    = (settings.sticker_name_bold   || 'true') === 'true';
  $: stickerPriceBold   = (settings.sticker_price_bold  || 'true') === 'true';

  // Shelf preset config
  $: shelfShowShop   = (settings.shelf_show_shop_name    || 'true') === 'true';
  $: shelfShowName   = (settings.shelf_show_product_name || 'true') === 'true';
  $: shelfShowPrice  = (settings.shelf_show_price        || 'true') === 'true';
  $: shelfShowRef    = (settings.shelf_show_ref          || 'true') === 'true';
  $: shelfNameSize   = parseInt(settings.shelf_name_font_size  || '11');
  $: shelfPriceSize  = parseInt(settings.shelf_price_font_size || '18');
  $: shelfRefSize    = parseInt(settings.shelf_ref_font_size   || '8');
  $: shelfNameBold   = (settings.shelf_name_bold  || 'true') === 'true';
  $: shelfPriceBold  = (settings.shelf_price_bold || 'true') === 'true';

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
        width: 1.5,
        height: 40,
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
          width: 1.5, height: 40,
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
  }

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

  async function handleTabChange(type: 'barcode' | 'etiquette') {
    labelType = type;
    copies = type === 'barcode' ? initialQty : 1;
    if (type === 'barcode') {
      await tick();
      renderBarcode();
    }
  }

  function buildStickerHtml(): string {
    const svgContent = barcodeSvgEl ? barcodeSvgEl.outerHTML : '';
    const nameStyle = `font-size:${stickerNameSize}px; font-weight:${stickerNameBold ? '900' : '400'}; margin:2px 0;`;
    const priceStyle = `font-size:${stickerPriceSize}px; font-weight:${stickerPriceBold ? '900' : '400'}; font-family:monospace; margin:2px 0;`;
    return `
      <div style="width:${widthMm}mm; height:${heightMm}mm; padding:2mm; text-align:center;
                  display:flex; flex-direction:column; align-items:center; justify-content:center;
                  font-family:sans-serif; overflow:hidden; box-sizing:border-box; background:#fff;">
        ${stickerShowShop ? `<span style="font-size:8px; font-weight:bold; text-transform:uppercase; color:#555; display:block;">${shopName}</span>` : ''}
        ${stickerShowName ? `<span style="${nameStyle} display:block; overflow:hidden; white-space:nowrap; max-width:100%;">${product?.name_fr || product?.name_ar || ''}</span>` : ''}
        ${stickerShowBarcode && barcode ? `<div style="max-width:100%; overflow:hidden; display:flex; justify-content:center; align-items:center;">${svgContent}</div>` : ''}
        ${stickerShowPrice ? `<span style="${priceStyle} display:block;">${product?.sale_price?.toLocaleString() || '0'} DZD</span>` : ''}
      </div>
    `;
  }

  function buildShelfTagHtml(): string {
    const nameStyle = `font-size:${shelfNameSize}px; font-weight:${shelfNameBold ? '900' : '400'};`;
    const priceStyle = `font-size:${shelfPriceSize}px; font-weight:${shelfPriceBold ? '900' : '400'};`;
    return `
      <div style="width:${widthMm}mm; height:${heightMm}mm; border:1.5px solid #000; padding:3mm;
                  font-family:sans-serif; text-align:center; box-sizing:border-box; background:#fff;
                  display:flex; flex-direction:column; justify-content:space-between; overflow:hidden;">
        ${shelfShowShop ? `<div style="display:flex;justify-content:space-between;font-size:9px;font-weight:bold;border-bottom:1px solid #000;padding-bottom:2px;"><span>${shopName}</span><span style="color:green;">DISPO</span></div>` : ''}
        ${shelfShowName ? `<p style="${nameStyle} margin:4px 0; line-height:1.2;">${product?.name_fr || product?.name_ar || ''}</p>` : ''}
        ${shelfShowPrice ? `<div style="background:#000;color:#fff;padding:4px;${priceStyle}margin:4px 0;font-family:monospace;">${product?.sale_price?.toLocaleString() || '0'} DZD</div>` : ''}
        ${shelfShowRef ? `<div style="display:flex;justify-content:space-between;font-size:${shelfRefSize}px;font-weight:bold;"><span>Ref: ${barcode}</span><span>TVA 19% Incl.</span></div>` : ''}
      </div>
    `;
  }

  function triggerPrint() {
    let singleHtml = labelType === 'barcode' ? buildStickerHtml() : buildShelfTagHtml();
    let combinedHtml = '';
    for (let i = 0; i < copies; i++) {
      combinedHtml += `<div style="page-break-after:always; display:inline-block;">${singleHtml}</div>`;
    }
    printHtmlDirectly(combinedHtml, labelType === 'barcode' ? 'Product Sticker' : 'Shelf Etiquette');
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
          <div class="p-4 bg-white text-slate-900 border-2 border-dashed border-slate-300 rounded-xl flex flex-col items-center justify-center text-center space-y-1 shadow-xs min-h-[120px]">
            {#if stickerShowShop}
              <span class="text-[9px] text-slate-500 font-bold uppercase tracking-wider">{shopName}</span>
            {/if}
            {#if stickerShowName}
              <p class="font-black text-slate-900 leading-tight line-clamp-2" style="font-size:{stickerNameSize}px; font-weight:{stickerNameBold ? '900' : '400'};">
                {product.name_fr || product.name_ar}
              </p>
            {/if}
            {#if stickerShowBarcode && barcode}
              <div class="w-full flex justify-center py-0.5 overflow-hidden">
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
              <span class="font-black text-slate-900 font-mono" style="font-size:{stickerPriceSize}px; font-weight:{stickerPriceBold ? '900' : '400'};">
                {product.sale_price.toLocaleString()} DZD
              </span>
            {/if}
          </div>

        {:else}
          <!-- Shelf Etiquette Preview -->
          <div class="p-4 bg-white text-slate-900 border-2 border-dashed border-emerald-300 rounded-xl flex flex-col justify-between text-center space-y-1.5 shadow-xs min-h-[120px]">
            {#if shelfShowShop}
              <div class="w-full flex justify-between text-[9px] font-bold text-slate-500 border-b border-slate-300 pb-0.5">
                <span>{shopName}</span>
                <span class="text-emerald-600">DISPO EN RAYON</span>
              </div>
            {/if}
            {#if shelfShowName}
              <p class="font-black text-slate-900 leading-tight" style="font-size:{shelfNameSize}px; font-weight:{shelfNameBold ? '900' : '400'};">
                {product.name_fr || product.name_ar}
              </p>
            {/if}
            {#if shelfShowPrice}
              <div class="w-full bg-slate-900 text-white font-mono font-black rounded py-1" style="font-size:{shelfPriceSize}px; font-weight:{shelfPriceBold ? '900' : '400'};">
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

        <!-- Print Parameters: Copies + Dimensions (read-only from settings) -->
        <div class="grid grid-cols-3 gap-2">
          <div>
            <label class="block text-[11px] font-bold text-pos-muted mb-1">
              {labelType === 'barcode' ? 'Copies (Stickers)' : 'Copies (Tags)'}
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
              {widthMm}mm
            </div>
          </div>
          <div>
            <label class="block text-[11px] font-bold text-pos-muted mb-1">Height (mm)</label>
            <div class="w-full px-2 py-1.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs text-pos-text font-bold font-mono text-center opacity-70">
              {heightMm}mm
            </div>
          </div>
        </div>

        {#if !settingsLoaded}
          <p class="text-[11px] text-pos-muted flex items-center gap-1">
            <RefreshCw class="w-3 h-3 animate-spin" /> Loading label settings...
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
          class="px-5 py-2 {labelType === 'barcode' ? 'bg-sky-600 hover:bg-sky-700' : 'bg-emerald-600 hover:bg-emerald-700'} text-white font-black text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-md transition active:scale-95"
        >
          <Printer class="w-4 h-4" />
          <span>Print {copies}× {labelType === 'barcode' ? 'Sticker(s)' : 'Tag(s)'}</span>
        </button>
      </div>
    </div>
  </div>
{/if}