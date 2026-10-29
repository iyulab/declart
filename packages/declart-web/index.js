import {
  render as wasmRender,
  render_json,
  render_with_theme_toml,
  validate,
  describe,
  themes as wasmThemes,
  kinds as wasmKinds,
} from './wasm/declart_wasm.js';

export { validate, describe };

export function render(input, theme = 'default', width, options) {
  return wasmRender(input, theme, width ?? undefined, options?.transparent ?? undefined);
}

export function renderJson(input, theme = 'default', width, options) {
  return render_json(input, theme, width ?? undefined, options?.transparent ?? undefined);
}

export function renderWithThemeToml(input, themeToml, width, options) {
  return render_with_theme_toml(input, themeToml, width ?? undefined, options?.transparent ?? undefined);
}

export function themes() {
  return wasmThemes().split(',');
}

export function kinds() {
  return wasmKinds().split(',');
}
