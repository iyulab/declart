// Declart Preview — webview non-module script
// Coordinates between the inline module script (WASM init) and VS Code host

const vscode = acquireVsCodeApi();
const previewEl = document.getElementById('preview');
const errorEl = document.getElementById('error');
const themeSelect = document.getElementById('theme-select');

let pendingContent = null;

function onWasmReady() {
  const initial = pendingContent ?? window.__INITIAL_CONTENT__;
  pendingContent = null;
  if (initial != null) {
    renderContent(initial);
  }
}

function onWasmError(event) {
  showError('WASM load failed: ' + event.detail);
}

// Sync toolbar selection with initial theme
if (themeSelect && window.__DECLART_THEME__) {
  themeSelect.value = window.__DECLART_THEME__;
}

// Theme change via toolbar: re-render with new theme
if (themeSelect) {
  themeSelect.addEventListener('change', () => {
    window.__DECLART_THEME__ = themeSelect.value;
    if (window.__declartRender && window.__LAST_CONTENT__) {
      renderContent(window.__LAST_CONTENT__);
    }
    // Notify extension host so the setting can be persisted if desired
    vscode.postMessage({ type: 'themeChanged', theme: themeSelect.value });
  });
}

// Race condition: WASM may already be ready before this script runs
if (window.__declartRender) {
  onWasmReady();
} else {
  document.addEventListener('wasm-ready', onWasmReady, { once: true });
  document.addEventListener('wasm-error', onWasmError, { once: true });
}

window.addEventListener('message', (event) => {
  const { type, content, theme } = event.data;
  if (type === 'update') {
    if (theme) { window.__DECLART_THEME__ = theme; }
    if (window.__declartRender) {
      renderContent(content);
    } else {
      pendingContent = content;
    }
  } else if (type === 'setTheme') {
    window.__DECLART_THEME__ = theme;
    if (themeSelect) { themeSelect.value = theme; }
    if (window.__declartRender && window.__LAST_CONTENT__) {
      renderContent(window.__LAST_CONTENT__);
    }
  }
});

function renderContent(content) {
  window.__LAST_CONTENT__ = content;
  try {
    const theme = getTheme();
    const svgString = window.__declartRender(content, theme, undefined);

    // Use DOMParser (not innerHTML) — keeps CSP clean and avoids XSS via SVG scripts
    const parser = new DOMParser();
    const parsed = parser.parseFromString(svgString, 'image/svg+xml');
    const svgEl = parsed.documentElement;

    previewEl.replaceChildren(svgEl);
    errorEl.style.display = 'none';
    previewEl.style.display = 'flex';

    vscode.postMessage({ type: 'renderOk' });
  } catch (err) {
    const msg = err.message ?? String(err);
    showError(msg);
    vscode.postMessage({ type: 'renderError', message: msg });
  }
}

function getTheme() {
  // Theme can be injected via window.__DECLART_THEME__ (set by extension in cycle-111)
  return window.__DECLART_THEME__ ?? 'default';
}

function showError(msg) {
  errorEl.textContent = msg;
  errorEl.style.display = 'block';
  previewEl.style.display = 'none';
}
