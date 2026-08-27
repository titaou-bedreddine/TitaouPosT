<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { currentUser } from '../../lib/stores/auth';
  import type { User } from '../../lib/types';
  import { Lock, UserCheck, QrCode } from 'lucide-svelte';

  let username = 'admin';
  let password = 'admin';
  let errorMsg = '';
  let isLoading = false;

  async function handleLogin() {
    if (!username || !password) return;
    try {
      isLoading = true;
      errorMsg = '';
      const user = await invoke<User | null>('login', { username, password });
      if (user) {
        $currentUser = user;
      } else {
        errorMsg = 'Invalid username or password';
      }
    } catch (err: any) {
      errorMsg = typeof err === 'string' ? err : err.message || 'Login failed';
    } finally {
      isLoading = false;
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
      handleLogin();
    }
  }
</script>

<div class="h-screen w-screen bg-slate-950 flex items-center justify-center p-4 select-none">
  <div class="bg-slate-900 border border-slate-800 rounded-3xl p-8 max-w-md w-full shadow-2xl space-y-6">
    <div class="text-center space-y-2">
      <div class="w-16 h-16 rounded-2xl bg-white flex items-center justify-center mx-auto shadow-lg shadow-sky-600/20 overflow-hidden p-1 border border-slate-700">
        <img src="/logo.png" alt="TitaouPOS" class="w-full h-full object-contain" />
      </div>
      <h1 class="text-2xl font-black text-white tracking-tight">TitaouPOS</h1>
      <p class="text-xs text-sky-400 font-bold">Created by Titaou Bedreddine 0553444057</p>
    </div>

    {#if errorMsg}
      <div class="p-3 bg-rose-950/60 border border-rose-800 text-rose-300 text-xs font-bold rounded-xl text-center">
        {errorMsg}
      </div>
    {/if}

    <div class="space-y-4">
      <div>
        <label class="block text-xs font-bold text-slate-400 mb-1">Username / اسم المستخدم</label>
        <input
          type="text"
          bind:value={username}
          on:keydown={handleKeyDown}
          class="w-full px-4 py-3 bg-slate-800 border border-slate-700 rounded-xl text-sm font-bold text-white outline-none focus:border-sky-500 transition"
        />
      </div>

      <div>
        <label class="block text-xs font-bold text-slate-400 mb-1">Password / كلمة المرور</label>
        <input
          type="password"
          bind:value={password}
          on:keydown={handleKeyDown}
          class="w-full px-4 py-3 bg-slate-800 border border-slate-700 rounded-xl text-sm font-bold text-white outline-none focus:border-sky-500 transition"
        />
      </div>

      <button
        type="button"
        on:click={handleLogin}
        disabled={isLoading}
        class="w-full py-3.5 bg-sky-600 hover:bg-sky-700 disabled:opacity-50 text-white font-black text-sm rounded-xl transition shadow-lg shadow-sky-600/30 flex items-center justify-center gap-2 cursor-pointer"
      >
        <Lock class="w-4 h-4" />
        <span>{isLoading ? 'Signing In...' : 'Sign In to POS'}</span>
      </button>
    </div>

    <div class="text-center pt-2">
      <p class="text-[11px] text-slate-500 font-mono">Default credentials: admin / admin</p>
    </div>
  </div>
</div>