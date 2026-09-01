<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { cartItems, cartGrandTotal, cartSubtotal, globalDiscountAmount, globalDiscountMode, globalDiscountValue } from '../stores/cart';
  import { currentUser } from '../stores/auth';
  import { printHtmlDirectly, entityQrDataUrl } from '../utils/printer';
  import { buildProfessionalReceiptHtml } from '../printing/professionalReceipt';
  import { getLanguage } from '../i18n';
  import { Printer, X, FileText } from 'lucide-svelte';

  export let isOpen = false;
  export let onClose: () => void;
  // Optional live-sale context (defaults keep the standalone preview working).
  export let saleNumber = '';
  export let paymentMethod = 'Espèces';
  export let customerName = '';

  let receiptContainer: HTMLDivElement;
  let settings: Record<string, string> = {};
  // 'standard' = classic monospace ticket; 'professional' = 80mm graphic preset.
  let receiptPreset: 'standard' | 'professional' = 'standard';
  let proQrDataUrl = '';

  $: fontFamily = settings.receipt_font_family || 'monospace';
  $: is58mm = settings.receipt_paper_width === '58mm';
  $: showShopName = (settings.receipt_show_shop_name ?? 'true') !== 'false';
  $: showAddress = (settings.receipt_show_address ?? 'true') !== 'false';
  $: showPhone = (settings.receipt_show_phone ?? 'true') !== 'false';
  $: showRcNif = (settings.receipt_show_rc_nif ?? 'true') !== 'false';
  $: showCashier = (settings.receipt_show_cashier ?? 'true') !== 'false';
  $: showDate = (settings.receipt_show_date ?? 'true') !== 'false';
  $: showTax = (settings.receipt_show_tax ?? 'true') !== 'false';
  $: showFooter = (settings.receipt_show_footer ?? 'true') !== 'false';
  $: showQr = (settings.receipt_show_qr ?? 'true') !== 'false';

  $: headerFontSize = settings.receipt_header_font_size || '14';
  $: headerBold = (settings.receipt_header_bold ?? 'true') !== 'false';
  $: bodyFontSize = settings.receipt_body_font_size || '10';
  $: bodyBold = (settings.receipt_body_bold ?? 'false') === 'true';
  $: totalFontSize = settings.receipt_total_font_size || '13';
  $: totalBold = (settings.receipt_total_bold ?? 'true') !== 'false';
  $: footerFontSize = settings.receipt_footer_font_size || '8';
  $: footerBold = (settings.receipt_footer_bold ?? 'false') === 'true';

  onMount(async () => {
    try {
      const fetched = await invoke<Record<string, string>>('get_all_settings');
      settings = { ...settings, ...fetched };
    } catch (e) {
      console.warn('Could not load settings in PrintReceiptModal', e);
    }
  });

  $: if (isOpen) {
    invoke<Record<string, string>>('get_all_settings').then(fetched => {
      settings = { ...settings, ...fetched };
      receiptPreset = fetched['receipt_preset'] === 'standard' ? 'standard' : 'professional';
    }).catch(() => {});
    // Offline QR for the professional preset (local data URL, no network).
    const num = effectiveInvoiceNumber;
    if (num) {
      entityQrDataUrl(`SALE:${num}`, 240).then((url) => (proQrDataUrl = url)).catch(() => {});
    }
  }

  $: now = new Date();
  $: effectiveInvoiceNumber = saleNumber || `TCK-${Math.floor(Date.now() / 1000).toString().slice(-6)}`;

  // ---- Professional 80mm receipt (same builder powers preview AND print) ----
  $: proHtml = receiptPreset === 'professional'
    ? buildProfessionalReceiptHtml({
        shopName: settings.shop_name_fr || 'TITAOU POS',
        shopTagline: settings.receipt_header || '',
        shopAddress: settings.shop_address || '',
        shopPhone: settings.shop_phone || '',
        shopWebsite: settings.shop_website || '',
        shopLogoDataUrl: settings.shop_logo_base64 || undefined,
        invoiceNumber: effectiveInvoiceNumber.replace(/^TCK-/, ''),
        dateStr: now.toLocaleDateString('fr-FR'),
        timeStr: now.toLocaleTimeString('fr-FR'),
        cashierName: $currentUser?.display_name || 'Admin',
        customerName: customerName || settings.default_customer_name || '',
        paymentMethod,
        items: $cartItems.map((i) => ({
          name: i.name_fr || i.name_ar,
          quantity: i.quantity,
          unitPrice: i.unit_price,
          totalPrice: i.total_price,
          discountPerUnit: i.discount_amount || 0,
          isRefund: i.is_refund || false,
        })),
        subtotal: $cartSubtotal,
        discount: $globalDiscountAmount,
        grandTotal: $cartGrandTotal,
        amountPaid: $cartGrandTotal,
        change: 0,
        currency: settings.default_currency || 'DA',
        qrDataUrl: proQrDataUrl || undefined,
        showQr: (settings.receipt_show_qr ?? 'true') !== 'false',
        showBarcode: (settings.receipt_show_barcode ?? 'true') !== 'false',
        thankYou: settings.receipt_thank_you || 'MERCI POUR VOTRE CONFIANCE !',
        returnPolicy: settings.receipt_footer || '',
        lang: getLanguage(),
        paperWidthMm: is58mm ? 58 : 80,
      })
    : '';

  function triggerPrint() {
    if (!receiptContainer) return;
    if (receiptPreset === 'professional') {
      printHtmlDirectly(
        proHtml,
        'Ticket Professionnel - TitaouPosT',
        { widthMm: is58mm ? 58 : 80 }
      );
      return;
    }
    printHtmlDirectly(receiptContainer.innerHTML, 'Ticket de Caisse - TitaouPosT');
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-2xl shadow-2xl w-full max-w-md overflow-hidden animate-in fade-in duration-150">
      <!-- Modal Header -->
      <div class="flex items-center justify-between px-5 py-3.5 border-b border-pos-border bg-slate-50 dark:bg-slate-800/50">
        <h3 class="font-black text-sm text-pos-text flex items-center gap-2">
          <Printer class="w-4 h-4 text-sky-500" />
          <span>Thermal Receipt Preview (معاينة الوصل)</span>
        </h3>
        <button on:click={onClose} class="text-pos-muted hover:text-pos-text p-1 rounded-lg cursor-pointer">
          <X class="w-4 h-4" />
        </button>
      </div>

      <!-- Receipt Preset Toggle -->
      <div class="px-5 pt-4">
        <div class="grid grid-cols-2 gap-2">
          <button
            type="button"
            on:click={() => (receiptPreset = 'standard')}
            class="p-2 rounded-xl border font-bold text-xs transition cursor-pointer flex items-center justify-center gap-1.5 {receiptPreset === 'standard' ? 'border-sky-500 bg-sky-50 dark:bg-sky-950 text-sky-600' : 'border-pos-border text-pos-muted'}"
          >
            <FileText class="w-3.5 h-3.5" />
            Standard Ticket
          </button>
          <button
            type="button"
            on:click={() => (receiptPreset = 'professional')}
            class="p-2 rounded-xl border font-bold text-xs transition cursor-pointer flex items-center justify-center gap-1.5 {receiptPreset === 'professional' ? 'border-sky-500 bg-sky-50 dark:bg-sky-950 text-sky-600' : 'border-pos-border text-pos-muted'}"
          >
            <Printer class="w-3.5 h-3.5" />
            80 mm Professional
          </button>
        </div>
      </div>

      <!-- Printable Thermal Receipt Area -->
      <div class="p-6 bg-slate-100 dark:bg-slate-900/60 max-h-[65vh] overflow-y-auto flex justify-center">
        {#if receiptPreset === 'professional'}
          <div
            bind:this={receiptContainer}
            class="bg-white shadow-md select-text"
          >
            {@html proHtml}
          </div>
        {:else}
        <div
          bind:this={receiptContainer}
          style="font-family: {fontFamily}; font-size: {bodyFontSize}px; font-weight: {bodyBold ? 'bold' : 'normal'}; width: {is58mm ? '230px' : '300px'};"
          class="bg-white text-black p-4 shadow-md leading-relaxed select-text border border-slate-200"
        >
          <!-- Store Header -->
          <div class="text-center pb-2 border-b-dashed">
            {#if showShopName}
              <h2 style="font-size: {headerFontSize}px; font-weight: {headerBold ? '900' : 'normal'};" class="tracking-tight leading-tight">
                {settings.shop_name_fr || 'TitaouPosT Supermarché'}
              </h2>
              {#if settings.shop_name_ar}
                <p class="font-bold text-xs">{settings.shop_name_ar}</p>
              {/if}
            {/if}
            {#if showAddress}
              <p class="text-[9px] text-gray-700">{settings.shop_address || 'Alger Centre, Algérie'}</p>
            {/if}
            {#if showPhone}
              <p class="text-[9px] text-gray-700">Tél: {settings.shop_phone || '0553444057 / 021654321'}</p>
            {/if}
            {#if showRcNif}
              <p class="text-[8px] text-gray-700">RC: {settings.shop_rc || '16/00-0123456B22'} | NIF: {settings.shop_nif || '001616012345678'}</p>
            {/if}
            {#if settings.receipt_header}
              <p class="text-[9px] font-bold text-gray-800 mt-0.5 italic">{settings.receipt_header}</p>
            {/if}
            {#if showDate}
              <p class="text-[8px] text-gray-600 mt-1">{new Date().toLocaleString()}</p>
            {/if}
          </div>

          <!-- Invoice / Cashier Info -->
          <div class="py-1 border-b-dashed text-[9px] flex justify-between">
            <span>Ticket #: {Math.floor(Date.now() / 1000).toString().slice(-6)}</span>
            {#if showCashier}
              <span>Caisse: {$currentUser?.display_name || 'Admin'}</span>
            {/if}
          </div>

          <!-- Items Table -->
          <div class="py-2 border-b-dashed">
            <table class="w-full text-[9px]">
              <thead>
                <tr class="border-b border-gray-300">
                  <th class="text-start pb-0.5">Article</th>
                  <th class="text-center pb-0.5">Qté</th>
                  <th class="text-end pb-0.5">P.U</th>
                  <th class="text-end pb-0.5">Total</th>
                </tr>
              </thead>
              <tbody>
                {#each $cartItems as item}
                  <tr>
                    <td class="text-start font-bold py-0.5">
                      {item.name_ar || item.name_fr}
                      {#if item.is_refund}
                        <span class="text-[8px] font-black text-red-600">[RETOUR]</span>
                      {/if}
                    </td>
                    <td class="text-center py-0.5">{item.quantity}</td>
                    <td class="text-end py-0.5">{item.unit_price}</td>
                    <td class="text-end font-bold py-0.5">
                      {item.is_refund ? '-' : ''}{item.total_price}
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>

          <!-- Totals -->
          <div class="py-2 space-y-1">
            {#if $globalDiscountMode !== 'none' && $globalDiscountValue > 0}
              <div class="flex justify-between text-[9px] text-gray-700">
                <span>Remise Globale:</span>
                <span>-{$globalDiscountMode === 'percent' ? `${$globalDiscountValue}%` : `${$globalDiscountValue} DZD`}</span>
              </div>
            {/if}

            <div
              style="font-size: {totalFontSize}px; font-weight: {totalBold ? '900' : 'normal'};"
              class="flex justify-between pt-1 border-t-dashed"
            >
              <span>NET À PAYER (TOTAL):</span>
              <span>{$cartGrandTotal.toLocaleString()} DZD</span>
            </div>

            {#if showTax}
              <div class="flex justify-between text-[8px] text-gray-600 pt-0.5">
                <span>Dont TVA ({settings.default_tax_rate || '19'}%):</span>
                <span>{Math.round(($cartGrandTotal * Number(settings.default_tax_rate || 19)) / (100 + Number(settings.default_tax_rate || 19))).toLocaleString()} DZD</span>
              </div>
            {/if}
          </div>

          <!-- Footer Greetings & QR Code -->
          {#if showFooter}
            <div
              style="font-size: {footerFontSize}px; font-weight: {footerBold ? 'bold' : 'normal'};"
              class="text-center pt-2 border-t-dashed text-gray-700 space-y-0.5"
            >
              {#if settings.receipt_footer}
                <p>{settings.receipt_footer}</p>
              {:else}
                <p class="font-bold">*** شكراً لزيارتكم - Merci de votre visite ***</p>
                <p>Les articles retournés doivent être présentés sous 48h</p>
              {/if}
            </div>
          {/if}

          {#if showQr}
            <div class="text-center pt-2">
              <div class="inline-block px-2 py-0.5 bg-slate-100 border border-slate-300 font-mono text-[7px] text-gray-600 rounded">
                [QR: TITAOU-{Math.floor(Date.now() / 1000).toString().slice(-6)}]
              </div>
            </div>
          {/if}
        </div>
        {/if}
      </div>

      <!-- Action Footer -->
      <div class="px-5 py-3.5 border-t border-pos-border bg-slate-50 dark:bg-slate-800/50 flex justify-end gap-2">
        <button on:click={onClose} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-xl cursor-pointer">
          Close
        </button>
        <button
          type="button"
          on:click={triggerPrint}
          class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-md transition active:scale-95"
        >
          <Printer class="w-4 h-4" />
          <span>Print Receipt (طباعة الوصل)</span>
        </button>
      </div>
    </div>
  </div>
{/if}