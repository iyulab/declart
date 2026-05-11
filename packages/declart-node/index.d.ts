/**
 * Renders a TOML diagram declaration to SVG.
 * @param input - TOML declaration string
 * @param theme - Theme name: 'default' or 'monochrome' (defaults to 'default')
 * @param width - Optional canvas width in pixels (height scales proportionally)
 * @returns SVG string
 * @throws If the declaration is invalid
 */
export function render(input: string, theme?: string, width?: number): string;

/**
 * Validates a TOML diagram declaration without rendering.
 * @param input - TOML declaration string
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
