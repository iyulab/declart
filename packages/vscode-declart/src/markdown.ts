import * as path from 'path';

type RenderFn = (input: string, theme: string, width: number | undefined) => string;

let renderFn: RenderFn | undefined;
let loadError: string | undefined;

function getRender(): RenderFn | undefined {
  if (renderFn) { return renderFn; }
  if (loadError) { return undefined; }
  try {
    // Node.js WASM target: loaded synchronously via require().
    // In the VSIX, this file is at media/node-wasm/declart_wasm.js.
    // __dirname is dist/ after esbuild bundling, so go one level up.
    const wasmPath = path.join(__dirname, '..', 'media', 'node-wasm', 'declart_wasm.js');
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const wasm = require(wasmPath) as { render: RenderFn };
    renderFn = wasm.render;
    return renderFn;
  } catch (e) {
    loadError = String(e);
    return undefined;
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function extendMarkdownIt(md: any): any {
  const originalFence =
    md.renderer.rules.fence ??
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    ((tokens: any[], idx: number, options: any, _env: unknown, self: any) =>
      self.renderToken(tokens, idx, options));

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  md.renderer.rules.fence = (tokens: any[], idx: number, options: any, env: unknown, self: any): string => {
    const token = tokens[idx] as { info: string; content: string };
    if (token.info.trim() !== 'declart') {
      return originalFence(tokens, idx, options, env, self) as string;
    }

    const render = getRender();
    if (!render) {
      return `<div class="declart-error">Declart WASM not available${loadError ? ': ' + escapeHtml(loadError) : ''}</div>\n`;
    }

    try {
      const svg = render(token.content, 'default', undefined);
      return `<figure class="declart-diagram">${svg}</figure>\n`;
    } catch (e) {
      return `<div class="declart-error">Declart error: ${escapeHtml(String(e))}</div>\n`;
    }
  };
  return md;
}

function escapeHtml(str: string): string {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;');
}
