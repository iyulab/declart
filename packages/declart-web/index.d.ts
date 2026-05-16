/**
 * Initializes the WASM module. Call once before using other functions
 * if your bundler does not handle WASM imports automatically.
 * With vite-plugin-wasm, this is not required.
 */
export function init(module_or_path?: string | URL | RequestInfo | BufferSource | WebAssembly.Module): Promise<void>;

/**
 * Renders a TOML or JSON diagram declaration to SVG.
 * @param input - TOML or JSON declaration string (format auto-detected)
 * @param theme - Theme name: 'default', 'monochrome', 'accessible', 'warm' (defaults to 'default')
 * @param width - Optional canvas width in pixels (height scales proportionally)
 * @returns SVG string
 * @throws If the declaration is invalid
 */
export function render(input: string, theme?: string, width?: number): string;

/**
 * Renders a JSON diagram declaration to SVG.
 * @param input - JSON declaration string
 * @param theme - Theme name (defaults to 'default')
 * @param width - Optional canvas width in pixels
 * @returns SVG string
 * @throws If the JSON declaration is invalid
 */
export function renderJson(input: string, theme?: string, width?: number): string;

/**
 * Renders a TOML or JSON diagram declaration with a custom theme TOML.
 * @param input - TOML or JSON declaration string
 * @param themeToml - TOML theme definition string
 * @param width - Optional canvas width in pixels
 * @returns SVG string
 * @throws If the declaration or theme is invalid
 */
export function renderWithThemeToml(input: string, themeToml: string, width?: number): string;

/**
 * Validates a TOML or JSON diagram declaration without rendering.
 * @param input - TOML or JSON declaration string
 * @throws If the declaration is invalid
 */
export function validate(input: string): void;

/**
 * Returns the list of supported theme names.
 */
export function themes(): string[];

/**
 * Returns the list of supported diagram kind names.
 */
export function kinds(): string[];
