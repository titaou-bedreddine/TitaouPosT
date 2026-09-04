<script lang="ts">
  // Quick date-range presets shared by every history/statistics page.
  // Emits ISO dates (YYYY-MM-DD) via bound startDate/endDate props.
  export let startDate = '';
  export let endDate = '';
  export let onChange: () => void = () => {};

  function toLocalISODate(d: Date): string {
    const pad = (n: number) => n.toString().padStart(2, '0');
    return `${d.getFullYear()}-${pad(d.getMonth() + 1)}-${pad(d.getDate())}`;
  }

  function apply(start: Date, end: Date) {
    startDate = toLocalISODate(start);
    endDate = toLocalISODate(end);
    onChange();
  }

  function today() {
    const now = new Date();
    apply(new Date(now), new Date(now));
  }

  function yesterday() {
    const d = new Date();
    d.setDate(d.getDate() - 1);
    apply(new Date(d), new Date(d));
  }

  function thisWeek() {
    const now = new Date();
    // Monday-based week start.
    const day = (now.getDay() + 6) % 7;
    const start = new Date(now);
    start.setDate(now.getDate() - day);
    apply(start, new Date(now));
  }

  function lastWeek() {
    const now = new Date();
    const day = (now.getDay() + 6) % 7;
    const start = new Date(now);
    start.setDate(now.getDate() - day - 7);
    const end = new Date(start);
    end.setDate(start.getDate() + 6);
    apply(start, end);
  }

  function thisMonth() {
    const now = new Date();
    apply(new Date(now.getFullYear(), now.getMonth(), 1), new Date(now));
  }

  function lastMonth() {
    const now = new Date();
    apply(new Date(now.getFullYear(), now.getMonth() - 1, 1), new Date(now.getFullYear(), now.getMonth(), 0));
  }

  function thisYear() {
    const now = new Date();
    apply(new Date(now.getFullYear(), 0, 1), new Date(now));
  }

  function allTime() {
    startDate = '';
    endDate = '';
    onChange();
  }

  const presets: Array<{ label: string; fn: () => void }> = [
    { label: 'Today (اليوم)', fn: today },
    { label: 'Yesterday (أمس)', fn: yesterday },
    { label: 'This Week (هذا الأسبوع)', fn: thisWeek },
    { label: 'Last Week (الأسبوع الماضي)', fn: lastWeek },
    { label: 'This Month (هذا الشهر)', fn: thisMonth },
    { label: 'Last Month (الشهر الماضي)', fn: lastMonth },
    { label: 'This Year (هذه السنة)', fn: thisYear },
    { label: 'All (الكل)', fn: allTime },
  ];
</script>

<div class="flex items-center gap-1.5 flex-wrap">
  {#each presets as preset}
    <button
      type="button"
      on:click={preset.fn}
      class="px-2.5 py-1 rounded-lg text-[10px] font-black bg-slate-100 dark:bg-slate-800 text-pos-muted hover:text-sky-600 hover:bg-sky-50 dark:hover:bg-sky-950 border border-pos-border transition cursor-pointer"
    >
      {preset.label}
    </button>
  {/each}
</div>
