'use strict';

const wasm = require('./wasm/declart_wasm');

/**
 * Renders a TOML or JSON diagram declaration to SVG.
 * @param {string} input - TOML or JSON declaration string (auto-detected)
 * @param {string} [theme='default'] - Theme name: 'default', 'monochrome', 'accessible', 'warm'
 * @param {number} [width] - Optional canvas width in pixels
 * @returns {string} SVG string
 */
function render(input, theme = 'default', width) {
    return wasm.render(input, theme, width ?? null);
}

/**
 * Renders a JSON diagram declaration to SVG.
 * @param {string} input - JSON declaration string
 * @param {string} [theme='default'] - Theme name
 * @param {number} [width] - Optional canvas width in pixels
 * @returns {string} SVG string
 */
function renderJson(input, theme = 'default', width) {
    return wasm.render_json(input, theme, width ?? null);
}

/**
 * Renders a TOML or JSON diagram declaration with a custom theme TOML.
 * @param {string} input - TOML or JSON declaration string
 * @param {string} themeToml - TOML theme definition string
 * @param {number} [width] - Optional canvas width in pixels
 * @returns {string} SVG string
 */
function renderWithThemeToml(input, themeToml, width) {
    return wasm.render_with_theme_toml(input, themeToml, width ?? null);
}

/**
 * Validates a TOML or JSON diagram declaration without rendering.
 * @param {string} input - TOML or JSON declaration string
 * @throws {Error} If the declaration is invalid
 */
function validate(input) {
    return wasm.validate(input);
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

module.exports = { render, renderJson, renderWithThemeToml, validate, themes, kinds };
