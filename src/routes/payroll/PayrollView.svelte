<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Employee, Payroll } from '../../lib/types';
  import { Plus, Users, Award } from 'lucide-svelte';

  let employees: Employee[] = [];
  let isAddOpen = false;

  let code = '';
  let name = '';
  let jobTitle = '';
  let baseSalary = 0;
  let phone = '';

  onMount(async () => {
    await loadEmployees();
  });

  async function loadEmployees() {
    try {
      employees = await invoke<Employee[]>('list_employees');
    } catch (e) {
      console.error(e);
    }
  }

  async function handleSaveEmployee() {
    try {
      await invoke('save_employee', {
        code,
        name,
        phone: phone || null,
        email: null,
        nationalId: null,
        jobTitle,
        baseSalary,
        salaryType: 'monthly',
        hireDate: new Date().toISOString().split('T')[0],
        notes: null,
        employeeId: null,
      });
      isAddOpen = false;
      code = '';
      name = '';
      jobTitle = '';
      baseSalary = 0;
      await loadEmployees();
    } catch (e) {
      console.error(e);
    }
  }
</script>

<div class="p-6 space-y-4 overflow-y-auto h-full">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-black text-pos-text">Staff & Payroll Management (رواتب الموظفين)</h1>
      <p class="text-xs text-pos-muted mt-1">Manage employee contracts, base salaries, advances, and payroll slips</p>
    </div>
    <button
      on:click={() => isAddOpen = true}
      class="px-4 py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold text-xs rounded-lg transition flex items-center gap-1.5 shadow-xs cursor-pointer"
    >
      <Plus class="w-4 h-4" />
      <span>New Employee</span>
    </button>
  </div>

  {#if isAddOpen}
    <div class="bg-pos-card border border-pos-border rounded-xl p-4 shadow-sm space-y-3">
      <h3 class="font-bold text-sm text-pos-text">Add New Employee</h3>
      <div class="grid grid-cols-1 md:grid-cols-4 gap-3">
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Employee Code</label>
          <input type="text" bind:value={code} placeholder="EMP-01" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs text-pos-text font-mono" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Full Name</label>
          <input type="text" bind:value={name} placeholder="Ahmed Benali" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs text-pos-text font-bold" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Job Title</label>
          <input type="text" bind:value={jobTitle} placeholder="Head Cashier" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs text-pos-text" />
        </div>
        <div>
          <label class="block text-xs font-bold text-pos-muted mb-1">Base Monthly Salary (DZD)</label>
          <input type="number" bind:value={baseSalary} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded text-xs font-bold font-mono text-pos-text" />
        </div>
      </div>
      <div class="flex justify-end gap-2 pt-2">
        <button on:click={() => isAddOpen = false} class="px-3 py-1.5 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded">Cancel</button>
        <button on:click={handleSaveEmployee} class="px-4 py-1.5 bg-sky-600 text-white text-xs font-bold rounded">Save Employee</button>
      </div>
    </div>
  {/if}

  <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
    {#each employees as emp}
      <div class="bg-pos-card border border-pos-border rounded-xl p-4 shadow-xs space-y-2">
        <div class="flex items-center justify-between">
          <span class="font-mono text-xs text-pos-muted">{emp.employee_code}</span>
          <span class="text-[11px] px-2 py-0.5 rounded-full font-bold bg-emerald-100 text-emerald-800">Active</span>
        </div>
        <h3 class="font-bold text-base text-pos-text">{emp.full_name}</h3>
        <p class="text-xs text-pos-muted">{emp.job_title}</p>
        <div class="pt-2 border-t border-pos-border/60 flex items-center justify-between">
          <span class="text-xs font-bold text-pos-muted">Monthly Salary</span>
          <span class="font-bold font-mono text-sm text-sky-600">{emp.base_salary.toLocaleString()} DZD</span>
        </div>
      </div>
    {/each}
  </div>
</div>