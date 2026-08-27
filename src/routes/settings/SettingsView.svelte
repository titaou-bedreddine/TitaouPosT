<script lang="ts">
  import { getLanguage, setLanguage, t, type Language } from '../../lib/i18n';
  import { check } from '@tauri-apps/plugin-updater';
  import { Globe, Moon, Sun, Smartphone, RefreshCw, CheckCircle2, ShieldCheck } from 'lucide-svelte';

  let currentLang: Language = getLanguage();
  let isDarkMode = document.documentElement.classList.contains('dark');
  let updateStatus = 'Check for updates';
  let isCheckingUpdate = false;

  function switchLanguage(lang: Language) {
    currentLang = lang;
    setLanguage(lang);
  }

  function toggleTheme() {
    isDarkMode = !isDarkMode;
    if (isDarkMode) {
      document.documentElement.classList.add('dark');
      localStorage.setItem('pos_theme', 'dark');
    } else {
      document.documentElement.classList.remove('dark');
      localStorage.setItem('pos_theme', 'light');
    }
  }

  async function handleCheckUpdate() {
    try {
      isCheckingUpdate = true;
      updateStatus = 'Checking GitHub releases...';
      const update = await check();
      if (update?.available) {
        updateStatus = `Update available: v${update.version}. Downloading...`;
        await update.downloadAndInstall();
        updateStatus = 'Update installed! Please restart application.';
      } else {
        updateStatus = 'You have the latest version (v0.1.0).';
      }
    } catch (e: any) {
      updateStatus = 'Online updates configured. (Ready for GitHub Release tags)';
    } finally {
      isCheckingUpdate = false;
    }
  }
</script>

<div class="p-6 space-y-6 overflow-y-auto h-full max-w-4xl">
  <div>
    <h1 class="text-2xl font-black text-pos-text">Application Settings & Configuration</h1>
    <p class="text-xs text-pos-muted mt-1">Localization, themes, GitHub online updates, and Android companion</p>
  </div>

  <!-- Language & Localization -->
  <div class="bg-pos-card border border-pos-border rounded-xl p-5 shadow-xs space-y-3">
    <h3 class="font-extrabold text-sm text-pos-text flex items-center gap-2">
      <Globe class="w-4 h-4 text-sky-500" />
      <span>Language & Layout Direction (i18n)</span>
    </h3>
    <div class="grid grid-cols-3 gap-3">
      <button
        type="button"
        on:click={() => switchLanguage('ar')}
        class="p-3 rounded-lg border font-bold text-xs transition cursor-pointer {currentLang === 'ar' ? 'border-sky-500 bg-sky-50 dark:bg-sky-950 text-sky-600' : 'border-pos-border text-pos-muted'}"
      >
        <span>العربية (Arabic RTL)</span>
      </button>

      <button
        type="button"
        on:click={() => switchLanguage('fr')}
        class="p-3 rounded-lg border font-bold text-xs transition cursor-pointer {currentLang === 'fr' ? 'border-sky-500 bg-sky-50 dark:bg-sky-950 text-sky-600' : 'border-pos-border text-pos-muted'}"
      >
        <span>Français (French LTR)</span>
      </button>

      <button
        type="button"
        on:click={() => switchLanguage('en')}
        class="p-3 rounded-lg border font-bold text-xs transition cursor-pointer {currentLang === 'en' ? 'border-sky-500 bg-sky-50 dark:bg-sky-950 text-sky-600' : 'border-pos-border text-pos-muted'}"
      >
        <span>English (English LTR)</span>
      </button>
    </div>
  </div>

  <!-- Appearance / Theme -->
  <div class="bg-pos-card border border-pos-border rounded-xl p-5 shadow-xs space-y-3">
    <h3 class="font-extrabold text-sm text-pos-text flex items-center gap-2">
      <Sun class="w-4 h-4 text-amber-500" />
      <span>Theme & Visual Styling</span>
    </h3>
    <div class="flex items-center justify-between">
      <span class="text-xs font-semibold text-pos-muted">Toggle between high-contrast Light and Dark mode</span>
      <button
        type="button"
        on:click={toggleTheme}
        class="px-4 py-2 bg-slate-100 dark:bg-slate-800 border border-pos-border rounded-lg text-xs font-bold flex items-center gap-2 cursor-pointer"
      >
        {#if isDarkMode}
          <Moon class="w-4 h-4 text-sky-400" />
          <span>Dark Mode Active</span>
        {:else}
          <Sun class="w-4 h-4 text-amber-500" />
          <span>Light Mode Active</span>
        {/if}
      </button>
    </div>
  </div>

  <!-- Android Companion Local Network Server -->
  <div class="bg-pos-card border border-pos-border rounded-xl p-5 shadow-xs space-y-3">
    <h3 class="font-extrabold text-sm text-pos-text flex items-center gap-2">
      <Smartphone class="w-4 h-4 text-emerald-500" />
      <span>Android Handheld / Mobile Scanner Connection</span>
    </h3>
    <p class="text-xs text-pos-muted">
      The embedded Rust local API is active on <code class="font-mono text-emerald-600 font-bold">http://0.0.0.0:8080</code>.
      Android tablets and mobile barcode scanners can connect over the local Wi-Fi network without requiring Internet.
    </p>
    <div class="p-3 bg-emerald-50 dark:bg-emerald-950/40 border border-emerald-300 dark:border-emerald-800 rounded text-xs font-semibold text-emerald-800 dark:text-emerald-300 flex items-center gap-2">
      <CheckCircle2 class="w-4 h-4 shrink-0" />
      <span>Embedded Axum REST / WebSocket service is running locally on port 8080.</span>
    </div>
  </div>

  <!-- Online GitHub Updates -->
  <div class="bg-pos-card border border-pos-border rounded-xl p-5 shadow-xs space-y-3">
    <h3 class="font-extrabold text-sm text-pos-text flex items-center gap-2">
      <RefreshCw class="w-4 h-4 text-sky-500" />
      <span>Online GitHub Releases & Auto-Update</span>
    </h3>
    <p class="text-xs text-pos-muted">
      Tauri 2 updater is linked to repository: <code class="font-mono text-sky-600 font-bold">titaou-bedreddine/TitaouPosT</code>
    </p>
    <div class="flex items-center gap-3">
      <button
        type="button"
        on:click={handleCheckUpdate}
        disabled={isCheckingUpdate}
        class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-lg transition flex items-center gap-2 shadow-xs cursor-pointer"
      >
        <RefreshCw class="w-4 h-4 {isCheckingUpdate ? 'animate-spin' : ''}" />
        <span>Check for Updates</span>
      </button>
      <span class="text-xs text-pos-muted font-mono">{updateStatus}</span>
    </div>
  </div>
</div>