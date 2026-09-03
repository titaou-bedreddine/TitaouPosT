/**
 * Three-state column sorting for list tables: click asc -> desc -> default.
 * `key` maps to a property name on the row objects; `defaultList` is the
 * unsorted source array the view already renders.
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
    const as = String(av).toLowerCase();
    const bs = String(bv).toLowerCase();
    return dir === 'asc' ? as.localeCompare(bs) : bs.localeCompare(as);
  });
  return copy;
}

/** Next state in the asc -> desc -> null cycle for a column. */
export function nextSortDir(current: SortDir): SortDir {
  if (current === null || current === undefined) return 'asc';
  if (current === 'asc') return 'desc';
  return null;
}

/** A Svelte-friendly store-free helper: returns [key, dir] after a click. */
export function clickSort(
  clickedKey: string,
  activeKey: string | null,
  activeDir: SortDir
): { key: string | null; dir: SortDir } {
  if (activeKey !== clickedKey) return { key: clickedKey, dir: 'asc' };
  return { key: clickedKey, dir: nextSortDir(activeDir) };
}
