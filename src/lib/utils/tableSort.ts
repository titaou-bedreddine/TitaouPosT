/**
 * Three-state column sorting for list tables, matching the Stock page's
 * behavior: click 1 → DESC (big→small / natural descending), click 2 → ASC
 * (small→big), click 3 → DEFAULT (the page's original order). The cycle
 * repeats. `key` maps to a property on the row objects; `defaultRows` is
 * the unsorted source array the view already renders.
 */
export type SortDir = 'asc' | 'desc' | null;

export function sortRows<T extends Record<string, any>>(
  rows: T[],
  key: string | null,
  dir: SortDir,
  defaultRows: T[]
): T[] {
  if (!key || !dir) return defaultRows;
  const copy = [...rows];
  copy.sort((a, b) => {
    const av = a[key];
    const bv = b[key];
    if (av == null && bv == null) return 0;
    if (av == null) return 1;
    if (bv == null) return -1;
    if (typeof av === 'number' && typeof bv === 'number') {
      return dir === 'asc' ? av - bv : bv - av;
    }
    // Dates-as-strings sort chronologically with a plain string compare
    // ("YYYY-MM-DD…" is lexicographically ordered); currency strings are
    // numeric in the data model so they took the numeric branch above.
    const as = String(av).toLowerCase();
    const bs = String(bv).toLowerCase();
    return dir === 'asc' ? as.localeCompare(bs) : bs.localeCompare(as);
  });
  return copy;
}

/** Next state in the DESC → ASC → default cycle for a column. */
export function nextSortDir(current: SortDir): SortDir {
  if (current === null || current === undefined) return 'desc';
  if (current === 'desc') return 'asc';
  return null;
}

/** A Svelte-friendly store-free helper: returns [key, dir] after a click. */
export function clickSort(
  clickedKey: string,
  activeKey: string | null,
  activeDir: SortDir
): { key: string | null; dir: SortDir } {
  if (activeKey !== clickedKey) return { key: clickedKey, dir: 'desc' };
  return { key: clickedKey, dir: nextSortDir(activeDir) };
}
