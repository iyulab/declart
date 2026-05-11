'use strict';

const wasm = require('./wasm/declart_wasm');

/**
 * Renders a TOML diagram declaration to SVG.
 * @param {string} input - TOML declaration string
 * @param {string} [theme='default'] - Theme name: 'default' or 'monochrome'
 * @param {number} [width] - Optional canvas width in pixels
 * @returns {string} SVG string
 */
function render(input, theme = 'default', width) {
    return wasm.render(input, theme, width ?? null);
}

/**
 * Validates a TOML diagram declaration without rendering.
 * @param {string} input - TOML declaration string
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

module.exports = { render, validate, themes, kinds };
