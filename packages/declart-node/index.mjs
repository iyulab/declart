// ESM entry point — wraps the CJS module with named exports
import { createRequire } from 'module';

const require = createRequire(import.meta.url);
const declart = require('./index.js');

export const render = declart.render;
export const renderJson = declart.renderJson;
export const renderWithThemeToml = declart.renderWithThemeToml;
export const validate = declart.validate;
export const themes = declart.themes;
export const kinds = declart.kinds;
