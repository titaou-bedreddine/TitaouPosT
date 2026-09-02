<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { Employee, Payroll } from '../../lib/types';
  import { t } from '../../lib/i18n';
  import {
    Plus, Users, Award, DollarSign, Calendar, AlertTriangle,
    Check, X, Printer, UserCheck, CreditCard, Clock
  } from 'lucide-svelte';
  import { printHtmlDirectly } from '../../lib/utils/printer';
  import { Pencil, Trash2 } from 'lucide-svelte';
  import { activeSession } from '../../lib/stores/session';
  import { currentUser } from '../../lib/stores/auth';

  let employees: Employee[] = [];
  let isAddOpen = false;
  // Editing reuses the add modal; delete asks for confirmation.
  let editingEmployeeId: number | null = null;
  let employeeToDelete: Employee | null = null;

  let code = '';
  let name = '';
  let jobTitle = '';
  let baseSalary = 0;
  let phone = '';
  let rfidCode = '';

  // Advance modal state
  let selectedEmpForAdvance: Employee | null = null;
  let advanceAmount = 0;
  let advanceReason = 'Avance sur salaire';
  let advanceDate = new Date().toISOString().split('T')[0];

  // Absence modal state
  let selectedEmpForAbsence: Employee | null = null;
  let absenceDays = 1;
  let absenceReason = 'Absence non justifiée';
  let absenceDate = new Date().toISOString().split('T')[0];

  // Active advance/absence logs in memory for session display
  let advancesMap: Record<number, number> = {};
  // Full advance log per employee for the history section.
  let advanceLog: any[] = [];
  let historyEmployee: Employee | null = null;

  async function openAdvanceHistory(emp: Employee) {
    historyEmployee = emp;
    try {
      advanceLog = await invoke<any[]>('list_employee_advances', {
        employeeId: emp.id,
        month: null,
      });
    } catch {
      advanceLog = [];
    }
  }
  let absencesMap: Record<number, number> = {};

  onMount(async () => {
    await loadEmployees();
    await loadAdvances();
  });

  // Advances are persisted in the backend (booked as expenses too) — this
  // month's records rebuild the session map after a restart.
  async function loadAdvances() {
    try {
      const month = new Date().toISOString().slice(0, 7);
      const list = await invoke<any[]>('list_employee_advances', {
        employeeId: null,
        month,
      });
      advancesMap = {};
      for (const adv of list) {
        advancesMap[adv.employee_id] = (advancesMap[adv.employee_id] || 0) + adv.amount;
      }
    } catch (e) {
      console.warn('Could not load advances:', e);
    }
  }

  async function loadEmployees() {
    try {
      employees = await invoke<Employee[]>('list_employees');
    } catch (e) {
      console.error(e);
    }
  }

  // Generate the next free employee code (EMP-01, EMP-02...).
  function generateEmployeeCode() {
    let maxNum = 0;
    for (const e of employees) {
      const m = /^EMP-(\d+)$/.exec((e.employee_code || '').trim());
      if (m) {
        maxNum = Math.max(maxNum, parseInt(m[1], 10));
      }
    }
    code = 'EMP-' + String(maxNum + 1).padStart(2, '0');
  }

  function openAddEmployee() {
    editingEmployeeId = null;
    code = '';
    name = '';
    jobTitle = '';
    baseSalary = 0;
    phone = '';
    rfidCode = '';
    isAddOpen = true;
  }

  function openEditEmployee(emp: Employee) {
    editingEmployeeId = emp.id;
    code = emp.employee_code || '';
    name = emp.full_name || '';
    jobTitle = emp.job_title || '';
    baseSalary = emp.base_salary || 0;
    phone = (emp as any).phone || '';
    rfidCode = ((emp as any).rfid_code as string) || '';
    isAddOpen = true;
  }

  async function handleDeleteEmployee() {
    if (!employeeToDelete) return;
    try {
      await invoke('delete_employee', { employeeId: employeeToDelete.id });
      employeeToDelete = null;
      await loadEmployees();
    } catch (e: any) {
      alert('Failed to delete employee: ' + (e.message || e));
    }
  }

  async function handleSaveEmployee() {
    if (!name.trim() || !code.trim()) {
      alert('Name and code are required / الاسم والرمز إجباريان');
      return;
    }
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
        rfidCode: rfidCode || null,
        employeeId: editingEmployeeId,
      });
      isAddOpen = false;
      editingEmployeeId = null;
      code = '';
      name = '';
      rfidCode = '';
      jobTitle = '';
      baseSalary = 0;
      phone = '';
      await loadEmployees();
    } catch (e: any) {
      // Editing reuses the same employee_code — show why it failed.
      alert('Save failed: ' + (typeof e === 'string' ? e : e.message || e));
    }
  }

  async function recordAdvance() {
    if (!selectedEmpForAdvance || advanceAmount <= 0) return;
    const emp = selectedEmpForAdvance;
    try {
      // Persisted + booked as an "Avances Salaires" expense; cash advances
      // leave the drawer (register + statistics reflect them).
      await invoke('record_employee_advance', {
        input: {
          employee_id: emp.id,
          amount: advanceAmount,
          reason: advanceReason || 'Avance sur salaire',
          date: advanceDate,
          session_id: $activeSession?.id ?? null,
          user_id: $currentUser?.id || 1,
        },
      });
      advancesMap[emp.id] = (advancesMap[emp.id] || 0) + advanceAmount;
    } catch (e: any) {
      alert('Failed to record advance: ' + (typeof e === 'string' ? e : e.message || e));
    }
    selectedEmpForAdvance = null;
    advanceAmount = 0;
  }

  function recordAbsence() {
    if (!selectedEmpForAbsence || absenceDays <= 0) return;
    const empId = selectedEmpForAbsence.id;
    absencesMap[empId] = (absencesMap[empId] || 0) + absenceDays;
    selectedEmpForAbsence = null;
    absenceDays = 1;
  }

  function printPayrollSlip(emp: Employee) {
    const advances = advancesMap[emp.id] || 0;
    const daysAbsent = absencesMap[emp.id] || 0;
    const dailyRate = Math.round(emp.base_salary / 30);
    const absenceDeduction = daysAbsent * dailyRate;
    const netSalary = Math.max(0, emp.base_salary - advances - absenceDeduction);

    const slipHtml = `
      <div style="width: 72mm; font-family: monospace; font-size: 10px; margin: 0 auto; padding: 2mm; text-align: center;">
        <h3 style="font-size: 13px; font-weight: 900; margin: 0; text-transform: uppercase;">BULLETIN DE PAIE / كشف راتب</h3>
        <p style="font-size: 8px; margin: 2px 0;">TitaouPOS Retail System</p>
        <hr style="border-top: 1px dashed #000; margin: 4px 0;" />
        <div style="text-align: left; font-size: 9px; line-height: 1.4;">
          <div style="display: flex; justify-content: space-between;"><span>Employé:</span><strong>${emp.full_name}</strong></div>
          <div style="display: flex; justify-content: space-between;"><span>Matricule:</span><span>#${emp.employee_code}</span></div>
          <div style="display: flex; justify-content: space-between;"><span>Poste:</span><span>${emp.job_title}</span></div>
          <div style="display: flex; justify-content: space-between;"><span>Période:</span><span>${new Date().toLocaleDateString('fr-FR', { month: 'long', year: 'numeric' })}</span></div>
        </div>
        <hr style="border-top: 1px dashed #000; margin: 4px 0;" />
        <div style="text-align: left; font-size: 9px; line-height: 1.4;">
          <div style="display: flex; justify-content: space-between;"><span>Salaire de Base:</span><span>${emp.base_salary.toLocaleString()} DZD</span></div>
          ${advances > 0 ? `<div style="display: flex; justify-content: space-between; color: #b91c1c;"><span>- Avances accordées:</span><span>-${advances.toLocaleString()} DZD</span></div>` : ''}
          ${daysAbsent > 0 ? `<div style="display: flex; justify-content: space-between; color: #b91c1c;"><span>- Absences (${daysAbsent} j):</span><span>-${absenceDeduction.toLocaleString()} DZD</span></div>` : ''}
        </div>
        <hr style="border-top: 1px dashed #000; margin: 4px 0;" />
        <div style="display: flex; justify-content: space-between; font-size: 11px; font-weight: 900;">
          <span>NET À PAYER (الصافي للدفع):</span>
          <span>${netSalary.toLocaleString()} DZD</span>
        </div>
        <hr style="border-top: 1px dashed #000; margin: 4px 0;" />
        <div style="margin-top: 10px; display: flex; justify-content: space-between; font-size: 7px;">
          <span>Signature Employeur</span>
          <span>Émargement Salarié</span>
        </div>
        <p style="font-size: 7px; color: #666; margin-top: 15px;">TitaouPOS • Dev: Titaou Bedreddine (0553444057)</p>
      </div>
    `;
    printHtmlDirectly(slipHtml, `Payroll-${emp.employee_code}`);
  }
</script>

<div class="p-6 space-y-6 overflow-y-auto h-full bg-pos-bg">
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-xl font-black text-pos-text flex items-center gap-2">
        <UserCheck class="w-6 h-6 text-sky-600" />
        <span>{t('pay_title')}</span>
      </h1>
      <p class="text-xs text-pos-muted mt-0.5">{t('pay_subtitle')}</p>
    </div>
    <button
      on:click={openAddEmployee}
      class="px-4 py-2.5 bg-sky-600 hover:bg-sky-700 text-white font-black text-xs rounded-xl transition flex items-center gap-1.5 shadow-md cursor-pointer active:scale-95"
    >
      <Plus class="w-4 h-4" />
      <span>New Employee (موظف جديد)</span>
    </button>
  </div>

  {#if isAddOpen}
    <div class="bg-pos-card border border-pos-border rounded-2xl p-5 shadow-md space-y-4 animate-in zoom-in-95 duration-150">
      <div class="flex items-center justify-between border-b border-pos-border pb-3">
        <h3 class="font-black text-sm text-pos-text">{t('pay_modal_new')}</h3>
        <button on:click={() => (isAddOpen = false)} class="text-pos-muted hover:text-pos-text"><X class="w-4 h-4" /></button>
      </div>
      <div class="grid grid-cols-1 md:grid-cols-5 gap-4 text-xs">
        <div>
          <label class="block font-bold text-pos-muted mb-1">{t('pay_field_code')} *</label>
          <div class="flex items-center gap-1.5">
            <input type="text" bind:value={code} placeholder="EMP-01" class="flex-1 px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-pos-text font-mono font-bold outline-none" />
            <button
              type="button"
              on:click={generateEmployeeCode}
              class="px-2.5 py-2 bg-sky-600 hover:bg-sky-700 text-white text-[10px] font-black rounded-xl cursor-pointer shrink-0"
              title="Generate the next free code"
            >
              GEN
            </button>
          </div>
        </div>
        <div>
          <label class="block font-bold text-pos-muted mb-1">{t('pay_field_name')} *</label>
          <input type="text" bind:value={name} placeholder="Ahmed Benali" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-pos-text font-bold outline-none" />
        </div>
        <div>
          <label class="block font-bold text-pos-muted mb-1">{t('pay_field_job')} *</label>
          <input type="text" bind:value={jobTitle} placeholder="Head Cashier" class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-pos-text outline-none" />
        </div>
        <div>
          <label class="block font-bold text-pos-muted mb-1">{t('pay_field_salary')} *</label>
          <input type="number" bind:value={baseSalary} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl font-mono font-black text-pos-text outline-none" />
        </div>
        <div>
          <label class="block font-bold text-pos-muted mb-1">{t('emp_rfid')}</label>
          <input
            type="text"
            bind:value={rfidCode}
            placeholder="RFID-XXXX"
            class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 border-0 rounded-xl text-pos-text font-mono text-[11px] font-bold outline-none"
            title="Scan the employee RFID tag here — searchable from the POS search bar"
          />
        </div>
      </div>
      <div class="flex justify-end gap-2 pt-2 border-t border-pos-border">
        <button on:click={() => (isAddOpen = false)} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-xl cursor-pointer">Cancel</button>
        <button on:click={handleSaveEmployee} class="px-5 py-2 bg-sky-600 hover:bg-sky-700 text-white text-xs font-black rounded-xl shadow-xs cursor-pointer">{t('pay_save_employee')}</button>
      </div>
    </div>
  {/if}

  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
    {#each employees as emp}
      {@const advances = advancesMap[emp.id] || 0}
      {@const daysAbsent = absencesMap[emp.id] || 0}
      {@const dailyRate = Math.round(emp.base_salary / 30)}
      {@const absenceDeduction = daysAbsent * dailyRate}
      {@const netSalary = Math.max(0, emp.base_salary - advances - absenceDeduction)}
      <div class="relative bg-pos-card border border-pos-border rounded-3xl p-5 shadow-xs flex flex-col justify-between space-y-4 hover:shadow-md transition group">
        <div>
          <div class="flex items-center justify-between mb-2">
            <span class="font-mono text-xs font-black text-sky-600 bg-sky-50 dark:bg-sky-950 px-2 py-0.5 rounded-lg border border-sky-200 dark:border-sky-800">
              #{emp.employee_code}
            </span>
            <span class="text-[10px] px-2.5 py-0.5 rounded-full font-black uppercase bg-emerald-100 text-emerald-800 dark:bg-emerald-950 dark:text-emerald-300">
              ACTIVE
            </span>
          </div>
          <div class="absolute top-3 end-3 flex items-center gap-1 opacity-0 group-hover:opacity-100 transition">
            <button
              type="button"
              on:click={() => openEditEmployee(emp)}
              class="p-1.5 bg-white/95 dark:bg-slate-800/95 text-pos-muted hover:text-sky-600 rounded-lg shadow-xs cursor-pointer"
              title="Edit employee (تعديل)"
            >
              <Pencil class="w-3.5 h-3.5" />
            </button>
            <button
              type="button"
              on:click={() => (employeeToDelete = emp)}
              class="p-1.5 bg-white/95 dark:bg-slate-800/95 text-pos-muted hover:text-rose-600 rounded-lg shadow-xs cursor-pointer"
              title="Delete employee (حذف)"
            >
              <Trash2 class="w-3.5 h-3.5" />
            </button>
            <button
              type="button"
              on:click={() => openAdvanceHistory(emp)}
              class="px-2 py-1 bg-white/95 dark:bg-slate-800/95 text-pos-muted hover:text-sky-600 rounded-lg shadow-xs cursor-pointer text-[10px] font-black"
              title="Avances & payments history (السجل)"
            >
              History
            </button>
          </div>

          <h3 class="font-black text-base text-pos-text leading-tight">{emp.full_name}</h3>
          <p class="text-xs text-pos-muted mt-0.5">{emp.job_title}</p>
        </div>

        <div class="space-y-1.5 p-3 bg-slate-50 dark:bg-slate-800/40 rounded-2xl border border-pos-border text-xs">
          <div class="flex items-center justify-between">
            <span class="text-pos-muted font-bold">{t('pay_base_monthly')}:</span>
            <span class="font-bold font-mono text-pos-text">{emp.base_salary.toLocaleString()} DZD</span>
          </div>
          {#if advances > 0}
            <div class="flex items-center justify-between text-rose-600">
              <span>{t('pay_advances')}:</span>
              <span class="font-mono font-bold">-{advances.toLocaleString()} DZD</span>
            </div>
          {/if}
          {#if daysAbsent > 0}
            <div class="flex items-center justify-between text-rose-600">
              <span>Absences ({daysAbsent} days):</span>
              <span class="font-mono font-bold">-{absenceDeduction.toLocaleString()} DZD</span>
            </div>
          {/if}
          <div class="pt-1.5 border-t border-pos-border flex items-center justify-between">
            <span class="font-black text-pos-text">Estimated Net:</span>
            <span class="font-black font-mono text-sm text-emerald-600">{netSalary.toLocaleString()} DZD</span>
          </div>
        </div>

        <div class="grid grid-cols-3 gap-2 pt-1 text-xs">
          <button
            type="button"
            on:click={() => (selectedEmpForAdvance = emp)}
            class="py-2 bg-amber-50 dark:bg-amber-950/40 hover:bg-amber-100 text-amber-800 dark:text-amber-300 font-bold rounded-xl border border-amber-200 dark:border-amber-800 text-center cursor-pointer transition shadow-2xs"
            title="Record Salary Advance"
          >
            + Avance
          </button>

          <button
            type="button"
            on:click={() => (selectedEmpForAbsence = emp)}
            class="py-2 bg-rose-50 dark:bg-rose-950/40 hover:bg-rose-100 text-rose-800 dark:text-rose-300 font-bold rounded-xl border border-rose-200 dark:border-rose-800 text-center cursor-pointer transition shadow-2xs"
            title="Record Absence / Leave"
          >
            + Absence
          </button>

          <button
            type="button"
            on:click={() => printPayrollSlip(emp)}
            class="py-2 bg-sky-600 hover:bg-sky-700 text-white font-bold rounded-xl flex items-center justify-center gap-1 cursor-pointer transition shadow-xs"
            title="Print Monthly Slip"
          >
            <Printer class="w-3.5 h-3.5" />
            <span>Fiche</span>
          </button>
        </div>
      </div>
    {/each}
  </div>
</div>

<!-- Modal: Advance on Salary -->
{#if selectedEmpForAdvance}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl p-6 shadow-2xl max-w-sm w-full space-y-4 animate-in zoom-in-95 duration-150">
      <div class="flex items-center justify-between border-b border-pos-border pb-3">
        <h3 class="font-black text-sm text-pos-text">Record Advance (تسبيق راتب)</h3>
        <button on:click={() => (selectedEmpForAdvance = null)} class="text-pos-muted"><X class="w-4 h-4" /></button>
      </div>
      <p class="text-xs text-pos-muted font-bold">Employee: {selectedEmpForAdvance.full_name}</p>
      <div>
        <label class="block text-xs font-bold text-pos-muted mb-1">Advance Amount (DZD)</label>
        <input type="number" min="500" step="500" bind:value={advanceAmount} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 rounded-xl text-xs font-mono font-black text-pos-text" />
      </div>
      <div class="flex justify-end gap-2 pt-2">
        <button on:click={() => (selectedEmpForAdvance = null)} class="px-3 py-1.5 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-xl">Cancel</button>
        <button on:click={recordAdvance} class="px-4 py-1.5 bg-amber-600 hover:bg-amber-700 text-white text-xs font-black rounded-xl">Confirm Advance</button>
      </div>
    </div>
  </div>
{/if}

<!-- Modal: Absence Recording -->
{#if selectedEmpForAbsence}
  <div class="fixed inset-0 z-50 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-3xl p-6 shadow-2xl max-w-sm w-full space-y-4 animate-in zoom-in-95 duration-150">
      <div class="flex items-center justify-between border-b border-pos-border pb-3">
        <h3 class="font-black text-sm text-pos-text">Record Absence (تسجيل غياب)</h3>
        <button on:click={() => (selectedEmpForAbsence = null)} class="text-pos-muted"><X class="w-4 h-4" /></button>
      </div>
      <p class="text-xs text-pos-muted font-bold">Employee: {selectedEmpForAbsence.full_name}</p>
      <div>
        <label class="block text-xs font-bold text-pos-muted mb-1">Number of Days</label>
        <input type="number" min="1" max="30" bind:value={absenceDays} class="w-full px-3 py-2 bg-slate-100 dark:bg-slate-800 rounded-xl text-xs font-mono font-black text-pos-text" />
      </div>
      <div class="flex justify-end gap-2 pt-2">
        <button on:click={() => (selectedEmpForAbsence = null)} class="px-3 py-1.5 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-xl">Cancel</button>
        <button on:click={recordAbsence} class="px-4 py-1.5 bg-rose-600 hover:bg-rose-700 text-white text-xs font-black rounded-xl">Confirm Deduction</button>
      </div>
    </div>
  </div>
{/if}
<!-- Employee Advances History -->
{#if historyEmployee}
  <div class="fixed inset-0 z-[60] bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-2xl shadow-2xl w-full max-w-md p-6 space-y-4">
      <div class="flex items-center justify-between">
        <h3 class="font-black text-sm text-pos-text">{t('pay_advances_history')} — {historyEmployee.full_name}</h3>
        <button on:click={() => (historyEmployee = null)} class="text-pos-muted hover:text-pos-text p-1 rounded cursor-pointer">
          <X class="w-5 h-5" />
        </button>
      </div>
      <div class="max-h-72 overflow-y-auto space-y-1.5">
        {#each advanceLog as adv}
          <div class="flex items-center justify-between p-2 bg-slate-50 dark:bg-slate-800/40 rounded-lg text-xs">
            <span class="font-mono text-pos-muted">{adv.date}</span>
            <span class="text-pos-text truncate max-w-[160px]">{adv.reason || 'Avance sur salaire'}</span>
            <span class="font-mono font-black text-rose-600">{adv.amount.toLocaleString()} DZD</span>
          </div>
        {/each}
        {#if advanceLog.length === 0}
          <p class="text-xs text-pos-muted text-center py-4">{t('pay_no_advances')}</p>
        {/if}
      </div>
    </div>
  </div>
{/if}

<!-- Delete Employee Confirmation -->
{#if employeeToDelete}
  <div class="fixed inset-0 z-[60] bg-black/60 backdrop-blur-xs flex items-center justify-center p-4">
    <div class="bg-pos-card border border-pos-border rounded-2xl shadow-2xl p-6 max-w-sm w-full space-y-4">
      <h3 class="font-black text-sm text-pos-text flex items-center gap-2 text-rose-600">
        <Trash2 class="w-5 h-5" />
        <span>{t('pay_delete_employee')}</span>
      </h3>
      <p class="text-xs text-pos-muted">
        Delete <strong class="text-pos-text">{employeeToDelete.full_name}</strong>?
        Their advances and payroll history stay in the records.
      </p>
      <div class="flex justify-end gap-2 pt-2 border-t border-pos-border">
        <button on:click={() => (employeeToDelete = null)} class="px-4 py-2 bg-slate-200 dark:bg-slate-700 text-xs font-bold rounded-xl cursor-pointer">Cancel</button>
        <button on:click={handleDeleteEmployee} class="px-4 py-2 bg-rose-600 text-white text-xs font-black rounded-xl cursor-pointer shadow-md">Confirm Delete</button>
      </div>
    </div>
  </div>
{/if}
