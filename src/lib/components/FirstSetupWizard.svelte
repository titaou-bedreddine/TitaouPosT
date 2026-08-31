<script lang="ts">
  // First-launch setup wizard: shop identity, contact and logo. Shows once
  // (until the user completes it), before any other UI.
  import { invoke } from '@tauri-apps/api/core';
  import { t } from '../i18n';
  import { Store, Phone, MapPin, Check } from 'lucide-svelte';

  export let onDone: () => void = () => {};

  let step = 1;
  let shopNameFr = '';
  let shopNameAr = '';
  let shopPhone = '';
  let shopAddress = '';
  let shopRc = '';
  let shopNif = '';
  let ownerName = '';
  let logoDataUrl: string | null = null;
  let isSaving = false;

  async function handleLogo(e: Event) {
    const file = (e.target as HTMLInputElement).files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = () => (logoDataUrl = String(reader.result));
    reader.readAsDataURL(file);
  }

  async function finish() {
    try {
      isSaving = true;
      const settings: Record<string, string> = {
        shop_name_fr: shopNameFr,
        shop_name_ar: shopNameAr,
        shop_phone: shopPhone,
        shop_address: shopAddress,
        shop_rc: shopRc,
        shop_nif: shopNif,
        shop_owner_name: ownerName,
        first_setup_completed: 'true',
      };
      if (logoDataUrl) {
        settings['shop_logo_data'] = logoDataUrl;
      }
      await invoke('set_multiple_settings', { settings });
      onDone();
    } catch (e: any) {
      alert('Setup failed: ' + (e?.message || e));
    } finally {
      isSaving = false;
    }
  }
</script>

<div class="fixed inset-0 z-[100] bg-slate-950/95 backdrop-blur-sm flex items-center justify-center p-6">
  <div class="bg-pos-card border border-pos-border rounded-3xl shadow-2xl w-full max-w-lg p-8 space-y-6">
    <div class="text-center">
      <div class="w-16 h-16 rounded-2xl bg-sky-600/10 text-sky-600 flex items-center justify-center mx-auto mb-3">
        <Store class="w-8 h-8" />
      </div>
      <h1 class="text-2xl font-black text-pos-text">Welcome to TitaouPOS</h1>
      <p class="text-xs text-pos-muted mt-1">
        First-time setup — configure your shop identity (مرحباً — إعداد بيانات محلّك)
      </p>
    </div>

    {#if step === 1}
      <div class="space-y-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Shop Name (Français)</label>
          <input type="text" bind:value={shopNameFr} placeholder="Superette Titaou" class="w-full px-3 py-2.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-sm font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">اسم المحل (بالعربية)</label>
          <input type="text" bind:value={shopNameAr} placeholder="سوبريت تيتاو" class="w-full px-3 py-2.5 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-sm font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500" dir="rtl" />
        </div>
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1 flex items-center gap-1"><Phone class="w-3 h-3" /> Phone</label>
            <input type="text" bind:value={shopPhone} placeholder="0550..." class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-sm text-pos-text font-mono outline-none" />
          </div>
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">Owner Name</label>
            <input type="text" bind:value={ownerName} placeholder="Owner / المسؤول" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-sm text-pos-text outline-none" />
          </div>
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1 flex items-center gap-1"><MapPin class="w-3 h-3" /> Address</label>
          <input type="text" bind:value={shopAddress} placeholder="Rue..., Alger" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-sm text-pos-text outline-none" />
        </div>
      </div>
    {:else}
      <div class="space-y-3">
        <div class="grid grid-cols-2 gap-3">
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">RC</label>
            <input type="text" bind:value={shopRc} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-sm text-pos-text font-mono outline-none" />
          </div>
          <div>
            <label class="block text-xs font-bold text-pos-muted mb-1">NIF</label>
            <input type="text" bind:value={shopNif} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-sm text-pos-text font-mono outline-none" />
          </div>
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-2">Shop Logo (optional)</label>
          <div class="flex items-center gap-4">
            <div class="w-20 h-20 rounded-2xl bg-white border-2 border-dashed border-pos-border flex items-center justify-center overflow-hidden shrink-0">
              {#if logoDataUrl}
                <img src={logoDataUrl} alt="Logo" class="w-full h-full object-contain" />
              {:else}
                <Store class="w-8 h-8 text-pos-muted/40" />
              {/if}
            </div>
            <label class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white text-xs font-bold rounded-xl cursor-pointer">
              Choose Logo
              <input type="file" accept="image/*" on:change={handleLogo} class="hidden" />
            </label>
          </div>
        </div>
        <p class="text-[11px] text-pos-muted bg-sky-50 dark:bg-sky-950/30 border border-sky-200 dark:border-sky-800 rounded-xl p-3">
          Licensing note: one license covers one PC (18000 DZD), each additional network PC +5000 DZD.
          Activation follows in Settings → Activation after this setup.
        </p>
      </div>
    {/if}

    <div class="flex items-center justify-between pt-4 border-t border-pos-border">
      <span class="text-[10px] font-black text-pos-muted">STEP {step} / 2</span>
      <div class="flex items-center gap-2">
        {#if step === 2}
          <button type="button" on:click={() => (step = 1)} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-pos-text text-xs font-bold rounded-xl cursor-pointer">Back</button>
        {/if}
        {#if step === 1}
          <button
            type="button"
            on:click={() => (shopNameFr.trim() || shopNameAr.trim() ? (step = 2) : undefined)}
            class="px-6 py-2.5 bg-sky-600 hover:bg-sky-700 text-white text-xs font-black rounded-xl shadow-md cursor-pointer"
          >
            Continue
          </button>
        {:else}
          <button
            type="button"
            on:click={finish}
            disabled={isSaving || !(shopNameFr.trim() || shopNameAr.trim())}
            class="px-6 py-2.5 bg-emerald-600 hover:bg-emerald-700 disabled:opacity-40 text-white text-xs font-black rounded-xl shadow-md cursor-pointer flex items-center gap-1.5"
          >
            <Check class="w-4 h-4" />
            {isSaving ? 'Saving...' : 'Finish Setup'}
          </button>
        {/if}
      </div>
    </div>
  </div>
</div>
