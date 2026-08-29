<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { currentUser } from '../../lib/stores/auth';
  import type { User } from '../../lib/types';
  import { Lock, ChevronDown, Eye, EyeOff } from 'lucide-svelte';

  let usersList: User[] = [];
  let selectedUsername = 'admin';
  let password = '';
  let showPassword = false;
  let errorMsg = '';
  let isLoading = false;

  onMount(async () => {
    try {
      usersList = await invoke<User[]>('get_active_users');
      if (usersList && usersList.length > 0) {
        selectedUsername = usersList[0].username;
      } else {
        selectedUsername = 'admin';
      }
    } catch (e) {
      console.error('Failed to load users:', e);
      usersList = [{ id: 1, username: 'admin', display_name: 'Administrator', role_id: 1, role_name: 'Administrator', max_discount_percent: 100, is_active: true, permissions: [] }];
      selectedUsername = 'admin';
    }
  });

  async function handleLogin() {
    const uname = (selectedUsername || 'admin').trim();
    const pwd = (password || '').trim();

    if (!uname || !pwd) {
      errorMsg = 'Please enter password / الرجاء إدخال كلمة المرور';
      return;
    }
    try {
      isLoading = true;
      errorMsg = '';
      const user = await invoke<User | null>('login', { username: uname, password: pwd });
      if (user) {
        currentUser.set(user);
        $currentUser = user;
      } else {
        errorMsg = 'Invalid password / كلمة المرور غير صحيحة';
      }
    } catch (err: any) {
      console.error('Login error:', err);
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
      <!-- User Selection Dropdown -->
      <div>
        <label class="block text-xs font-bold text-slate-400 mb-1">Select User / اختر المستخدم</label>
        <div class="relative">
          <select
            bind:value={selectedUsername}
            class="w-full px-4 py-3 bg-slate-800 border border-slate-700 rounded-xl text-sm font-bold text-white outline-none focus:border-sky-500 transition appearance-none cursor-pointer"
          >
            {#if usersList.length === 0}
              <option value="admin">Administrator (Administrator)</option>
            {:else}
              {#each usersList as u}
                <option value={u.username}>
                  {u.display_name} ({u.role_name || u.username})
                </option>
              {/each}
            {/if}
          </select>
          <div class="absolute inset-y-0 end-0 flex items-center px-3 pointer-events-none text-slate-400">
            <ChevronDown class="w-4 h-4" />
          </div>
        </div>
      </div>

      <div>
        <div class="flex items-center justify-between mb-1">
          <label class="block text-xs font-bold text-slate-400">Password / كلمة المرور</label>
          <button
            type="button"
            on:click={() => (showPassword = !showPassword)}
            class="text-[11px] text-slate-400 hover:text-sky-400 font-bold flex items-center gap-1 cursor-pointer"
          >
            {#if showPassword}
              <EyeOff class="w-3.5 h-3.5" />
              <span>Hide</span>
            {:else}
              <Eye class="w-3.5 h-3.5" />
              <span>Show</span>
            {/if}
          </button>
        </div>
        <input
          type={showPassword ? 'text' : 'password'}
          bind:value={password}
          placeholder="••••••••"
          autofocus
          on:keydown={handleKeyDown}
          class="w-full px-4 py-3 bg-slate-800 border border-slate-700 rounded-xl text-sm font-bold text-white outline-none focus:border-sky-500 transition font-mono"
        />
      </div>

      <button
        type="button"
        on:click={handleLogin}
        disabled={isLoading}
        class="w-full py-3.5 bg-sky-600 hover:bg-sky-700 disabled:opacity-50 text-white font-black text-sm rounded-xl transition shadow-lg shadow-sky-600/30 flex items-center justify-center gap-2 cursor-pointer"
      >
        <Lock class="w-4 h-4" />
        <span>{isLoading ? 'Signing In...' : 'Sign In to POS / تسجيل الدخول'}</span>
      </button>
    </div>
  </div>
</div>