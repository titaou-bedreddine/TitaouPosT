/**
 * Style & Theme system (v0.5.26).
 *
 * THEMES recolor the app accent (sky/indigo utilities remap to --th-* CSS
 * variables set on [data-app-theme]). The DEFAULT theme uses the exact
 * Tailwind sky palette — pixel-identical to the pre-theme look.
 * SKINS reshape the UI (corner radii via --r-* on [data-app-skin]).
 * Persisted as the app_theme / app_skin settings; applied at startup in
 * App.svelte and live from Settings → Style & Theme.
 */
export interface ThemeMeta {
  id: string;
  nameKey: string;
  /** Two swatches: primary (600) + light (200). */
  swatch: [string, string];
}

export const THEMES: ThemeMeta[] = [
  { id: 'default', nameKey: 'th_default', swatch: ['#0284c7', '#bae6fd'] },
  { id: 'indigo', nameKey: 'th_indigo', swatch: ['#4f46e5', '#c7d2fe'] },
  { id: 'emerald', nameKey: 'th_emerald', swatch: ['#059669', '#a7f3d0'] },
  { id: 'mustard', nameKey: 'th_mustard', swatch: ['#a16207', '#fef08a'] },
  { id: 'rose', nameKey: 'th_rose', swatch: ['#e11d48', '#fecdd3'] },
  { id: 'teal', nameKey: 'th_teal', swatch: ['#0d9488', '#99f6e4'] },
  { id: 'orange', nameKey: 'th_orange', swatch: ['#ea580c', '#fed7aa'] },
  { id: 'violet', nameKey: 'th_violet', swatch: ['#7c3aed', '#ddd6fe'] },
];

export interface SkinMeta {
  id: string;
  nameKey: string;
  /** Preview corner radius in px for the settings card. */
  previewRadius: number;
}

export const SKINS: SkinMeta[] = [
  { id: 'classic', nameKey: 'sk_classic', previewRadius: 8 },
  { id: 'sharp', nameKey: 'sk_sharp', previewRadius: 3 },
  { id: 'soft', nameKey: 'sk_soft', previewRadius: 14 },
  { id: 'bubble', nameKey: 'sk_bubble', previewRadius: 20 },
];

export interface PresetMeta {
  id: string;
  nameKey: string;
  subKey: string;
  /** Mini-preview: [canvas, surface, accent]. */
  preview: [string, string, string];
}

/** Full-look theme skins: colors + surfaces + shapes + effects in one pick.
 *  Highest precedence — overrides both the color themes and the shape skins. */
export const PRESETS: PresetMeta[] = [
  { id: 'neu', nameKey: 'preset_neu', subKey: 'preset_neu_sub', preview: ['#F5F7FA', '#FFFFFF', '#1E75FF'] },
  { id: 'glass', nameKey: 'preset_glass', subKey: 'preset_glass_sub', preview: ['#E8DED8', '#F5EDE6', '#F28C28'] },
  { id: 'bold', nameKey: 'preset_bold', subKey: 'preset_bold_sub', preview: ['#FFCC00', '#FFFFFF', '#000000'] },
  { id: 'coral', nameKey: 'preset_coral', subKey: 'preset_coral_sub', preview: ['#FFF3F2', '#FFEBE9', '#FF3B30'] },
];

export function applyPreset(id: string | null | undefined): void {
  if (id && PRESETS.some((x) => x.id === id)) {
    document.documentElement.setAttribute('data-app-preset', id);
  } else {
    document.documentElement.removeAttribute('data-app-preset');
  }
}

export function applyTheme(id: string | null | undefined): void {
  const t = THEMES.some((x) => x.id === id) ? id : 'default';
  document.documentElement.setAttribute('data-app-theme', t!);
}

export function applySkin(id: string | null | undefined): void {
  const s = SKINS.some((x) => x.id === id) ? id : 'classic';
  document.documentElement.setAttribute('data-app-skin', s!);
}

export function applyThemeSettings(theme?: string | null, skin?: string | null, preset?: string | null): void {
  applyTheme(theme);
  applySkin(skin);
  applyPreset(preset);
}
