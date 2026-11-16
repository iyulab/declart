import type { Engine } from './engine.ts';

/** Resolves the configured theme: "auto" follows the app's light/dark mode. */
export function resolveTheme(setting: string, isDark: boolean): string {
  return setting === 'auto' ? (isDark ? 'dark' : 'default') : setting;
}

/** Returns a usable canvas width, or undefined when the setting is not a positive number. */
export function sanitizeWidth(raw: unknown): number | undefined {
  if (typeof raw !== 'number' || !Number.isFinite(raw)) return undefined;
  const width = Math.floor(raw);
  return width > 0 ? width : undefined;
}

export type BlockResult =
  | { ok: true; svg: string; alt: string }
  | { ok: false; message: string };

/** Renders one code block's source; never throws. The backdrop is always left unpainted. */
export function renderBlock(
  engine: Engine,
  source: string,
  opts: { theme: string; width?: number },
): BlockResult {
  if (source.trim() === '') return { ok: false, message: 'Empty declart block' };
  try {
    const svg = engine.render(source, opts.theme, opts.width, true);
    const alt = engine.describe(source);
    return { ok: true, svg, alt };
  } catch (err) {
    return { ok: false, message: err instanceof Error ? err.message : String(err) };
  }
}
