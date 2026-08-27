<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import type { Category } from '../types';
  import { Layers, Plus, Trash2, Edit2, X, Check } from 'lucide-svelte';

  export let isOpen = false;
  export let onClose: () => void;
  export let onCategoryChanged: () => void;

  let categories: Category[] = [];
  let nameAr = '';
  let nameFr = '';
  let color = '#0284c7';
  let editingId: number | null = null;
  let errorMsg = '';

  $: if (isOpen) {
    loadCategories();
  }

  async function loadCategories() {
    try {
      categories = await invoke<Category[]>('get_categories');
    } catch (e) {
      console.error(e);
    }
  }

  async function handleSave() {
    if (!nameAr && !nameFr) {
      errorMsg = 'Please enter a category name.';
      return;
    }
    try {
      errorMsg = '';
      await invoke('save_category', {
        nameAr: nameAr || nameFr,
        nameFr: nameFr || nameAr,
        nameEn: nameFr || nameAr,
        color,
        categoryId: editingId,
      });
      nameAr = '';
      nameFr = '';
      editingId = null;
      await loadCategories();
      onCategoryChanged();
    } catch (err: any) {
      errorMsg = typeof err === 'string' ? err : err.message || 'Error saving category';
    }
  }

  function startEdit(cat: Category) {
    editingId = cat.id;
    nameAr = cat.name_ar;
    nameFr = cat.name_fr;
    color = cat.color || '#0284c7';
  }

  async function handleDelete(id: number) {
    try {
      await invoke('delete_category', { categoryId: id });
      await loadCategories();
      onCategoryChanged();
    } catch (e) {
      console.error(e);
    }
  }
</script>

{#if isOpen}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-xl shadow-2xl w-full max-w-lg overflow-hidden animate-in fade-in duration-150">
      <div class="flex items-center justify-between px-5 py-3.5 border-b border-pos-border bg-slate-50 dark:bg-slate-800/50">
        <h3 class="font-extrabold text-base text-pos-text flex items-center gap-2">
          <Layers class="w-5 h-5 text-sky-500" />
          <span>Manage Product Groups / Categories (الأقسام)</span>
        </h3>
        <button on:click={onClose} class="text-pos-muted hover:text-pos-text p-1 rounded cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>

      <div class="p-5 space-y-4">
        {#if errorMsg}
          <div class="p-2.5 bg-rose-100 text-rose-700 text-xs font-bold rounded">{errorMsg}</div>
        {/if}

        <!-- Add / Edit Input Bar -->
        <div class="grid grid-cols-1 md:grid-cols-4 gap-2 bg-slate-100 dark:bg-slate-800/60 p-3 rounded-xl border border-pos-border">
          <input
            type="text"
            bind:value={nameAr}
            placeholder="الاسم بالعربية"
            class="px-2.5 py-1.5 bg-pos-card border border-pos-border rounded text-xs text-pos-text font-bold outline-none"
          />
          <input
            type="text"
            bind:value={nameFr}
            placeholder="Nom en Français"
            class="px-2.5 py-1.5 bg-pos-card border border-pos-border rounded text-xs text-pos-text outline-none"
          />
          <input
            type="color"
            bind:value={color}
            class="h-8 w-full p-0.5 bg-pos-card border border-pos-border rounded cursor-pointer"
          />
          <button
            type="button"
            on:click={handleSave}
            class="px-3 py-1.5 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded transition flex items-center justify-center gap-1 cursor-pointer"
          >
            <Plus class="w-3.5 h-3.5" />
            <span>{editingId ? 'Update' : 'Add Group'}</span>
          </button>
        </div>

        <!-- Categories List -->
        <div class="max-h-60 overflow-y-auto space-y-1.5">
          {#each categories as cat}
            <div class="flex items-center justify-between p-2.5 bg-pos-card border border-pos-border rounded-lg hover:border-sky-400 transition">
              <div class="flex items-center gap-2.5">
                <span class="w-3.5 h-3.5 rounded-full shrink-0" style="background-color: {cat.color || '#0284c7'}"></span>
                <span class="font-bold text-xs text-pos-text">{cat.name_ar}</span>
                <span class="text-xs text-pos-muted">({cat.name_fr})</span>
              </div>
              <div class="flex items-center gap-1">
                <button
                  type="button"
                  on:click={() => startEdit(cat)}
                  class="p-1 text-sky-600 hover:bg-sky-50 dark:hover:bg-sky-950 rounded cursor-pointer"
                >
                  <Edit2 class="w-3.5 h-3.5" />
                </button>
                <button
                  type="button"
                  on:click={() => handleDelete(cat.id)}
                  class="p-1 text-rose-500 hover:bg-rose-50 dark:hover:bg-rose-950 rounded cursor-pointer"
                >
                  <Trash2 class="w-3.5 h-3.5" />
                </button>
              </div>
            </div>
          {/each}
        </div>
      </div>

      <div class="px-5 py-3 border-t border-pos-border bg-slate-50 dark:bg-slate-800/50 flex justify-end">
        <button on:click={onClose} class="px-4 py-1.5 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded">
          Close
        </button>
      </div>
    </div>
  </div>
{/if}