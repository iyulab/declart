import wasmInit, {
  render as wasmRender,
  render_json,
  render_with_theme_toml,
  validate,
  themes as wasmThemes,
  kinds as wasmKinds,
} from './wasm/declart_wasm.js';

export { wasmInit as init, validate };

export function render(input, theme = 'default', width) {
  return wasmRender(input, theme, width ?? undefined);
}

export function renderJson(input, theme = 'default', width) {
  return render_json(input, theme, width ?? undefined);
}

export function renderWithThemeToml(input, themeToml, width) {
  return render_with_theme_toml(input, themeToml, width ?? undefined);
}

export function themes() {
  return wasmThemes().split(',');
}

export function kinds() {
  return wasmKinds().split(',');
}
