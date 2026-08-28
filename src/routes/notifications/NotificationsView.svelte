<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import {
    Bell, Send, CheckCircle2, AlertTriangle, AlertOctagon,
    Shield, RefreshCw, ShoppingCart, Undo2, DollarSign, Settings
  } from 'lucide-svelte';

  let telegramToken = '';
  let telegramChatId = '';
  let notifyDailySummary = true;
  let notifyEachSale = false;
  let notifyEachRefund = true;
  let notifyExpiry = true;
  let notifyLowStock = true;

  let isSendingTest = false;
  let statusMsg = '';
  let isSaving = false;

  interface NotificationLog {
    id: number;
    type: 'sale' | 'refund' | 'expiry' | 'stock' | 'system';
    title: string;
    message: string;
    timestamp: string;
  }

  let logs: NotificationLog[] = [
    {
      id: 1,
      type: 'expiry',
      title: 'Near Expiry Alert',
      message: 'Product "Lait Candia 1L" expires in 12 days (Stock: 24 pcs)',
      timestamp: 'Today, 10:30 AM',
    },
    {
      id: 2,
      type: 'stock',
      title: 'Low Stock Alert',
      message: 'Product "Huile Elio 5L" is below min threshold (Stock: 2 pcs)',
      timestamp: 'Today, 09:15 AM',
    },
    {
      id: 3,
      type: 'sale',
      title: 'Daily Summary Ready',
      message: 'Yesterday total revenue: 184,500 DZD across 86 transactions',
      timestamp: 'Yesterday, 11:59 PM',
    },
  ];

  onMount(async () => {
    try {
      const settings = await invoke<Record<string, string>>('get_all_settings');
      if (settings['telegram_bot_token']) telegramToken = settings['telegram_bot_token'];
      if (settings['telegram_chat_id']) telegramChatId = settings['telegram_chat_id'];
      if (settings['notify_daily_summary']) notifyDailySummary = settings['notify_daily_summary'] === 'true';
      if (settings['notify_each_sale']) notifyEachSale = settings['notify_each_sale'] === 'true';
      if (settings['notify_each_refund']) notifyEachRefund = settings['notify_each_refund'] === 'true';
      if (settings['notify_expiry']) notifyExpiry = settings['notify_expiry'] === 'true';
      if (settings['notify_low_stock']) notifyLowStock = settings['notify_low_stock'] === 'true';
    } catch (e) {
      console.error(e);
    }
  });

  async function handleSaveSettings() {
    try {
      isSaving = true;
      statusMsg = '';
      await invoke('set_multiple_settings', {
        settings: {
          telegram_bot_token: telegramToken,
          telegram_chat_id: telegramChatId,
          notify_daily_summary: notifyDailySummary ? 'true' : 'false',
          notify_each_sale: notifyEachSale ? 'true' : 'false',
          notify_each_refund: notifyEachRefund ? 'true' : 'false',
          notify_expiry: notifyExpiry ? 'true' : 'false',
          notify_low_stock: notifyLowStock ? 'true' : 'false',
        },
      });
      statusMsg = 'Telegram settings saved successfully!';
      setTimeout(() => (statusMsg = ''), 3000);
    } catch (e: any) {
      statusMsg = 'Error: ' + (typeof e === 'string' ? e : e.message);
    } finally {
      isSaving = false;
    }
  }

  async function sendTelegramTest() {
    if (!telegramToken || !telegramChatId) {
      statusMsg = 'Please enter Telegram Bot Token and Chat ID first';
      return;
    }
    try {
      isSendingTest = true;
      statusMsg = 'Sending test message to Telegram...';
      const text = encodeURIComponent('🚀 *TitaouPOS Live Alert*\nTest connection successful from POS terminal!');
      const url = `https://api.telegram.org/bot${telegramToken}/sendMessage?chat_id=${telegramChatId}&text=${text}&parse_mode=Markdown`;
      
      const res = await fetch(url);
      const data = await res.json();
      if (data.ok) {
        statusMsg = '✅ Telegram test alert delivered successfully!';
      } else {
        statusMsg = '❌ Telegram error: ' + (data.description || 'Check Token/Chat ID');
      }
    } catch (e: any) {
      statusMsg = 'Network error sending test: ' + e.message;
    } finally {
      isSendingTest = false;
    }
  }
</script>

<div class="h-full flex flex-col bg-pos-bg p-4 overflow-y-auto select-none space-y-4">
  <!-- Header -->
  <div class="flex items-center justify-between pb-4 border-b border-pos-border shrink-0">
    <div class="flex items-center gap-3">
      <div class="w-10 h-10 rounded-2xl bg-sky-100 dark:bg-sky-950 text-sky-600 flex items-center justify-center font-bold">
        <Bell class="w-5 h-5" />
      </div>
      <div>
        <h1 class="text-xl font-black text-pos-text tracking-tight">Notifications & Telegram Alerts / الإشعارات وتنبيهات تيليغرام</h1>
        <p class="text-xs text-pos-muted">Configure real-time Telegram alerts for sales, refunds, expiring items, and low stock</p>
      </div>
    </div>
  </div>

  {#if statusMsg}
    <div class="p-3 bg-sky-100 dark:bg-sky-950/80 border border-sky-300 dark:border-sky-800 text-sky-800 dark:text-sky-200 text-xs font-bold rounded-xl animate-in fade-in">
      {statusMsg}
    </div>
  {/if}

  <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
    <!-- Telegram Configuration Card -->
    <div class="bg-pos-card border border-pos-border rounded-2xl p-5 shadow-xs space-y-4">
      <div class="flex items-center gap-2">
        <Send class="w-5 h-5 text-sky-500" />
        <h3 class="font-black text-sm text-pos-text">Telegram Bot Integration</h3>
      </div>

      <div class="space-y-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Telegram Bot Token</label>
          <input
            type="text"
            bind:value={telegramToken}
            placeholder="123456789:ABCdefGhIJKlmNoPQRsTUVwxyZ"
            class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500"
          />
        </div>

        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Telegram Chat ID (Channel or User ID)</label>
          <input
            type="text"
            bind:value={telegramChatId}
            placeholder="-100123456789 or 987654321"
            class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-xs font-mono font-bold text-pos-text outline-none focus:ring-2 focus:ring-sky-500"
          />
        </div>

        <div class="flex items-center gap-2 pt-2">
          <button
            type="button"
            on:click={sendTelegramTest}
            disabled={isSendingTest || !telegramToken}
            class="px-4 py-2 bg-slate-200 dark:bg-slate-700 hover:bg-slate-300 disabled:opacity-40 text-pos-text font-bold text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-xs"
          >
            <Send class="w-3.5 h-3.5" />
            <span>{isSendingTest ? 'Sending...' : 'Send Test Alert (إرسال تجربة)'}</span>
          </button>

          <button
            type="button"
            on:click={handleSaveSettings}
            disabled={isSaving}
            class="px-5 py-2 bg-sky-600 hover:bg-sky-700 disabled:opacity-50 text-white font-black text-xs rounded-xl flex items-center gap-1.5 cursor-pointer shadow-md"
          >
            <CheckCircle2 class="w-4 h-4" />
            <span>{isSaving ? 'Saving...' : 'Save Settings (حفظ)'}</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Notification Rules & Toggles -->
    <div class="bg-pos-card border border-pos-border rounded-2xl p-5 shadow-xs space-y-4">
      <div class="flex items-center gap-2">
        <Settings class="w-5 h-5 text-indigo-500" />
        <h3 class="font-black text-sm text-pos-text">Automatic Notification Triggers</h3>
      </div>

      <div class="space-y-3 divide-y divide-pos-border/40">
        <label class="flex items-center justify-between pt-2 cursor-pointer">
          <div>
            <p class="text-xs font-bold text-pos-text">Daily End-of-Day Summary</p>
            <p class="text-[10px] text-pos-muted">Sends total revenue, transactions, and net cash at end of day</p>
          </div>
          <input type="checkbox" bind:checked={notifyDailySummary} class="w-4 h-4 text-sky-600 rounded cursor-pointer" />
        </label>

        <label class="flex items-center justify-between pt-2 cursor-pointer">
          <div>
            <p class="text-xs font-bold text-pos-text">Alert on Every Sale</p>
            <p class="text-[10px] text-pos-muted">Sends instant notification for each completed checkout</p>
          </div>
          <input type="checkbox" bind:checked={notifyEachSale} class="w-4 h-4 text-sky-600 rounded cursor-pointer" />
        </label>

        <label class="flex items-center justify-between pt-2 cursor-pointer">
          <div>
            <p class="text-xs font-bold text-pos-text">Alert on Refunds & Cancellations</p>
            <p class="text-[10px] text-pos-muted">Notifies manager whenever a refund or return is processed</p>
          </div>
          <input type="checkbox" bind:checked={notifyEachRefund} class="w-4 h-4 text-sky-600 rounded cursor-pointer" />
        </label>

        <label class="flex items-center justify-between pt-2 cursor-pointer">
          <div>
            <p class="text-xs font-bold text-pos-text">Near Expired & Expired Products</p>
            <p class="text-[10px] text-pos-muted">Daily alert for products reaching expiry date within 30 days</p>
          </div>
          <input type="checkbox" bind:checked={notifyExpiry} class="w-4 h-4 text-sky-600 rounded cursor-pointer" />
        </label>

        <label class="flex items-center justify-between pt-2 cursor-pointer">
          <div>
            <p class="text-xs font-bold text-pos-text">Low Stock & Out of Stock Alerts</p>
            <p class="text-[10px] text-pos-muted">Notifies when items drop below minimum inventory alert threshold</p>
          </div>
          <input type="checkbox" bind:checked={notifyLowStock} class="w-4 h-4 text-sky-600 rounded cursor-pointer" />
        </label>
      </div>
    </div>
  </div>

  <!-- Recent Notifications Feed -->
  <div class="bg-pos-card border border-pos-border rounded-2xl p-5 shadow-xs space-y-3">
    <h3 class="font-black text-sm text-pos-text">Recent Store Activity & Notifications Feed</h3>
    <div class="space-y-2">
      {#each logs as log}
        <div class="p-3 bg-slate-50 dark:bg-slate-800/40 rounded-xl border border-pos-border flex items-start justify-between gap-3">
          <div class="flex items-start gap-2.5">
            {#if log.type === 'expiry'}
              <div class="p-2 bg-amber-500/10 text-amber-600 rounded-lg"><AlertTriangle class="w-4 h-4" /></div>
            {:else if log.type === 'stock'}
              <div class="p-2 bg-rose-500/10 text-rose-600 rounded-lg"><AlertOctagon class="w-4 h-4" /></div>
            {:else}
              <div class="p-2 bg-sky-500/10 text-sky-600 rounded-lg"><DollarSign class="w-4 h-4" /></div>
            {/if}
            <div>
              <p class="text-xs font-black text-pos-text">{log.title}</p>
              <p class="text-xs text-pos-muted">{log.message}</p>
            </div>
          </div>
          <span class="text-[10px] text-pos-muted font-mono whitespace-nowrap">{log.timestamp}</span>
        </div>
      {/each}
    </div>
  </div>
</div>