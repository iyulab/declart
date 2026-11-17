'use strict';

const wasm = require('./wasm/declart_wasm');

/**
 * Renders a TOML or JSON diagram declaration to SVG.
 * @param {string} input - TOML or JSON declaration string (auto-detected)
 * @param {string} [theme='default'] - Theme name: 'default', 'monochrome', 'accessible', 'warm'
 * @param {number} [width] - Optional canvas width in pixels
 * @param {{ transparent?: boolean }} [options] - `transparent: true` leaves the backdrop unpainted
 * @returns {string} SVG string
 */
function render(input, theme = 'default', width, options) {
    return wasm.render(input, theme, width ?? null, options?.transparent ?? null);
}

/**
 * Renders a TOML or JSON diagram declaration to SVG and reports what the engine changed to make the
 * labels fit. An empty `warnings` list means every label is in the picture as declared.
 * @param {string} input - TOML or JSON declaration string (auto-detected)
 * @param {string} [theme='default'] - Theme name
 * @param {number} [width] - Optional canvas width in pixels
 * @param {{ transparent?: boolean }} [options] - `transparent: true` leaves the backdrop unpainted
 * @returns {{ svg: string, warnings: Array<{ code: 'label_truncated' | 'label_shrunk', label: string, message: string, hint: string }> }}
 */
function renderDetailed(input, theme = 'default', width, options) {
    return JSON.parse(wasm.render_detailed(input, theme, width ?? null, options?.transparent ?? null));
}

/**
 * Renders a JSON diagram declaration to SVG.
 * @param {string} input - JSON declaration string
 * @param {string} [theme='default'] - Theme name
 * @param {number} [width] - Optional canvas width in pixels
 * @param {{ transparent?: boolean }} [options] - `transparent: true` leaves the backdrop unpainted
 * @returns {string} SVG string
 */
function renderJson(input, theme = 'default', width, options) {
    return wasm.render_json(input, theme, width ?? null, options?.transparent ?? null);
}

/**
 * Renders a TOML or JSON diagram declaration with a custom theme TOML.
 * @param {string} input - TOML or JSON declaration string
 * @param {string} themeToml - TOML theme definition string
 * @param {number} [width] - Optional canvas width in pixels
 * @param {{ transparent?: boolean }} [options] - `transparent: true` leaves the backdrop unpainted
 * @returns {string} SVG string
 */
function renderWithThemeToml(input, themeToml, width, options) {
    return wasm.render_with_theme_toml(input, themeToml, width ?? null, options?.transparent ?? null);
}

/**
 * Validates a TOML or JSON diagram declaration without rendering.
 * With `strict: true` the declaration must also fit: a label the engine would cut short with "…"
 * is an error (a label only set smaller is not).
 * @param {string} input - TOML or JSON declaration string
 * @param {{ strict?: boolean }} [options]
 * @throws {Error} If the declaration is invalid, or — in strict mode — a label would be cut short
 */
function validate(input, options) {
    wasm.validate(input);
    if (options?.strict) {
        const cut = renderDetailed(input).warnings.filter((w) => w.code === 'label_truncated');
        if (cut.length > 0) {
            throw new Error(cut.map((w) => `${w.message} — ${w.hint}`).join('; '));
        }
    }
}

/**
 * Returns a one-paragraph text equivalent of a TOML or JSON declaration — the text the
 * rendered SVG carries as its `<desc>`. Use it as `alt` when the SVG is shown through `<img>`.
 * @param {string} input - TOML or JSON declaration string
 * @returns {string} Text equivalent
 * @throws {Error} If the declaration is invalid
 */
function describe(input) {
    return wasm.describe(input);
}

/**
 * Returns the list of supported theme names.
 * @returns {string[]}
 */
function themes() {
    return wasm.themes().split(',');
}

/**
 * Returns the list of supported diagram kind names.
 * @returns {string[]}
 */
function kinds() {
    return wasm.kinds().split(',');
}

/**
 * Reports how long a label can be in each place it goes in a diagram kind, measured with the same
 * diagnostics {@link renderDetailed} returns — so a limit and a `label_truncated` warning always agree.
 * @param {string} kind - Diagram kind, one of {@link kinds}
 * @param {string} [view] - View of the kind (defaults to the kind's first view)
 * @returns {Array<{ path: string, reference: string, hangul: { unchanged: number|null, whole: number|null }, ascii: { unchanged: number|null, whole: number|null } }>}
 * @throws {Error} If the kind or the view is unknown
 */
function limits(kind, view) {
    return JSON.parse(wasm.limits(kind, view ?? null));
}

module.exports = { render, renderDetailed, renderJson, renderWithThemeToml, validate, describe, themes, kinds, limits };
