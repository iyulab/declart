/** Options for the render functions. */
export interface RenderOptions {
  /**
   * Leave the backdrop unpainted so the page shows through. Colors are unchanged —
   * on a dark page use the `dark` theme so text stays legible.
   */
  transparent?: boolean;
}

/**
 * Renders a TOML or JSON diagram declaration to SVG.
 * @param input - TOML or JSON declaration string (format auto-detected)
 * @param theme - Theme name: 'default', 'monochrome', 'accessible', 'warm', 'dark' (defaults to 'default')
 * @param width - Optional canvas width in pixels (height scales proportionally)
 * @param options - `transparent: true` leaves the backdrop unpainted
 * @returns SVG string
 * @throws If the declaration is invalid
 */
export function render(input: string, theme?: string, width?: number, options?: RenderOptions): string;

/**
 * Renders a JSON diagram declaration to SVG.
 * @param input - JSON declaration string
 * @param theme - Theme name (defaults to 'default')
 * @param width - Optional canvas width in pixels
 * @param options - `transparent: true` leaves the backdrop unpainted
 * @returns SVG string
 * @throws If the JSON declaration is invalid
 */
export function renderJson(input: string, theme?: string, width?: number, options?: RenderOptions): string;

/**
 * Renders a TOML or JSON diagram declaration with a custom theme TOML.
 * @param input - TOML or JSON declaration string
 * @param themeToml - TOML theme definition string
 * @param width - Optional canvas width in pixels
 * @param options - `transparent: true` leaves the backdrop unpainted
 * @returns SVG string
 * @throws If the declaration or theme is invalid
 */
export function renderWithThemeToml(input: string, themeToml: string, width?: number, options?: RenderOptions): string;

/**
 * Validates a TOML or JSON diagram declaration without rendering.
 * @param input - TOML or JSON declaration string
 * @throws If the declaration is invalid
 */
export function validate(input: string): void;

/**
 * Returns a one-paragraph text equivalent of a TOML or JSON declaration — the text the
 * rendered SVG carries as its `<desc>`. Use it as `alt` when the SVG is shown through `<img>`.
 * @param input - TOML or JSON declaration string
 * @returns Text equivalent, e.g. `"Process: Upload → Parse → Index."`
 * @throws If the declaration is invalid
 */
export function describe(input: string): string;

/**
 * Returns the list of supported theme names.
 */
export function themes(): string[];

/**
 * Returns the list of supported diagram kind names.
 */
export function kinds(): string[];
