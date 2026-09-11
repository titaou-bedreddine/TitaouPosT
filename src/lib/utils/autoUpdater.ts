/**
 * Shared silent in-app updater. Downloads the signed update through the
 * Tauri updater plugin (latest.json + .sig verified against the embedded
 * pubkey), installs it, and relaunches — on progress, the OS browser is
 * NEVER opened. Used by BOTH the top banner (App.svelte) and the
 * Settings → Updates tab, so every entry point behaves identically.
 */
export interface SilentUpdateResult {
  ok: boolean;
  error?: string;
}

export async function runSilentUpdate(
  onProgress?: (downloaded: number, total: number, pct: number) => void,
): Promise<SilentUpdateResult> {
  try {
    const { check } = await import('@tauri-apps/plugin-updater');
    const { relaunch } = await import('@tauri-apps/plugin-process');
    const update = await check();
    if (!update) return { ok: true }; // nothing to do

    let downloaded = 0;
    let total = 0;
    await update.downloadAndInstall((event) => {
      if (event.event === 'Started') {
        total = event.data.contentLength ?? 0;
      } else if (event.event === 'Progress') {
        downloaded += event.data.chunkLength;
        const pct = total > 0 ? Math.min(100, Math.round((downloaded / total) * 100)) : 0;
        onProgress?.(downloaded, total, pct);
      } else if (event.event === 'Finished') {
        onProgress?.(total, total, 100);
      }
    });
    await relaunch();
    return { ok: true };
  } catch (e: any) {
    return { ok: false, error: typeof e === 'string' ? e : e?.message || String(e) };
  }
}
