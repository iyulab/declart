import * as vscode from 'vscode';
import * as path from 'path';

type DiagnosticsHandler = (uri: vscode.Uri, diagnostics: vscode.Diagnostic[]) => void;

export class DeclartPreviewPanel {
  private static panel: vscode.WebviewPanel | undefined;
  private static currentDoc: vscode.TextDocument | undefined;
  private static onDiagnostics: DiagnosticsHandler | undefined;

  static setDiagnosticsHandler(handler: DiagnosticsHandler): void {
    DeclartPreviewPanel.onDiagnostics = handler;
  }

  static createOrShow(
    extensionUri: vscode.Uri,
    document: vscode.TextDocument,
    column: vscode.ViewColumn,
    theme = 'default'
  ): void {
    if (DeclartPreviewPanel.panel) {
      DeclartPreviewPanel.panel.reveal(column);
      DeclartPreviewPanel.currentDoc = document;
      DeclartPreviewPanel.panel.title = `Preview: ${path.basename(document.fileName)}`;
      DeclartPreviewPanel.panel.webview.html = DeclartPreviewPanel.buildHtml(
        DeclartPreviewPanel.panel.webview,
        extensionUri,
        document.getText(),
        theme
      );
      return;
    }

    DeclartPreviewPanel.panel = vscode.window.createWebviewPanel(
      'declartPreview',
      `Preview: ${path.basename(document.fileName)}`,
      column,
      {
        enableScripts: true,
        localResourceRoots: [vscode.Uri.joinPath(extensionUri, 'media')],
        retainContextWhenHidden: true,
      }
    );
    DeclartPreviewPanel.panel.onDidDispose(() => {
      DeclartPreviewPanel.panel = undefined;
      DeclartPreviewPanel.currentDoc = undefined;
    });
    DeclartPreviewPanel.panel.webview.onDidReceiveMessage((msg) => {
      DeclartPreviewPanel.handleWebviewMessage(msg);
    });

    DeclartPreviewPanel.currentDoc = document;
    DeclartPreviewPanel.panel.webview.html = DeclartPreviewPanel.buildHtml(
      DeclartPreviewPanel.panel.webview,
      extensionUri,
      document.getText(),
      theme
    );
  }

  private static handleWebviewMessage(msg: { type: string; message?: string }): void {
    const docUri = DeclartPreviewPanel.currentDoc?.uri;
    if (!docUri || !DeclartPreviewPanel.onDiagnostics) { return; }

    if (msg.type === 'renderOk') {
      DeclartPreviewPanel.onDiagnostics(docUri, []);
    } else if (msg.type === 'renderError' && msg.message) {
      DeclartPreviewPanel.onDiagnostics(docUri, [parseErrorToDiagnostic(msg.message)]);
    }
  }

  static update(document: vscode.TextDocument, theme = 'default'): void {
    if (
      !DeclartPreviewPanel.panel ||
      DeclartPreviewPanel.currentDoc?.uri.toString() !== document.uri.toString()
    ) {
      return;
    }
    DeclartPreviewPanel.panel.webview.postMessage({
      type: 'update',
      content: document.getText(),
      theme,
    });
  }

  // Switch the panel to show a different document (preview follows active editor).
  static followDocument(extensionUri: vscode.Uri, document: vscode.TextDocument, theme = 'default'): void {
    if (!DeclartPreviewPanel.panel) { return; }
    if (DeclartPreviewPanel.currentDoc?.uri.toString() === document.uri.toString()) { return; }
    DeclartPreviewPanel.currentDoc = document;
    DeclartPreviewPanel.panel.title = `Preview: ${path.basename(document.fileName)}`;
    DeclartPreviewPanel.panel.webview.html = DeclartPreviewPanel.buildHtml(
      DeclartPreviewPanel.panel.webview,
      extensionUri,
      document.getText(),
      theme
    );
  }

  static refreshTheme(theme: string): void {
    if (!DeclartPreviewPanel.panel || !DeclartPreviewPanel.currentDoc) { return; }
    DeclartPreviewPanel.panel.webview.postMessage({
      type: 'setTheme',
      theme,
    });
  }

  static dispose(): void {
    DeclartPreviewPanel.panel?.dispose();
    DeclartPreviewPanel.panel = undefined;
  }

  private static buildHtml(
    webview: vscode.Webview,
    extensionUri: vscode.Uri,
    initialContent: string,
    theme = 'default'
  ): string {
    const wasmJsUri = webview.asWebviewUri(
      vscode.Uri.joinPath(extensionUri, 'media', 'wasm', 'declart_wasm.js')
    );
    const previewJsUri = webview.asWebviewUri(
      vscode.Uri.joinPath(extensionUri, 'media', 'preview.js')
    );
    const nonce = getNonce();
    const escapedContent = JSON.stringify(initialContent);
    const escapedTheme = JSON.stringify(theme);

    // Inline module script initializes WASM and exposes render on window.
    // preview.js (non-module) waits for 'wasm-ready' event then renders.
    return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'nonce-${nonce}'; style-src 'unsafe-inline';">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Declart Preview</title>
  <style>
    html, body { margin: 0; padding: 0; height: 100%; display: flex; flex-direction: column; background: var(--vscode-editor-background); color: var(--vscode-editor-foreground); font-family: var(--vscode-font-family, sans-serif); }
    #toolbar { display: flex; align-items: center; gap: 8px; padding: 4px 8px; background: var(--vscode-editorGroupHeader-tabsBackground, #1e1e1e); border-bottom: 1px solid var(--vscode-editorGroup-border, #444); flex-shrink: 0; font-size: 0.8em; }
    #toolbar label { opacity: 0.7; }
    #toolbar select { background: var(--vscode-dropdown-background); color: var(--vscode-dropdown-foreground); border: 1px solid var(--vscode-dropdown-border); padding: 2px 4px; border-radius: 3px; font-size: inherit; }
    #preview { flex: 1; display: flex; align-items: center; justify-content: center; overflow: auto; padding: 16px; box-sizing: border-box; }
    #preview svg { max-width: 100%; height: auto; }
    #loading { padding: 24px; opacity: 0.6; }
    #error { padding: 16px; color: var(--vscode-errorForeground); font-family: monospace; font-size: 0.85em; white-space: pre-wrap; display: none; }
  </style>
</head>
<body>
  <div id="toolbar">
    <label for="theme-select">Theme</label>
    <select id="theme-select">
      <option value="default">Default</option>
      <option value="monochrome">Monochrome</option>
      <option value="accessible">Accessible</option>
      <option value="warm">Warm</option>
    </select>
  </div>
  <div id="preview"><span id="loading">Loading diagram engine…</span></div>
  <div id="error"></div>
  <script nonce="${nonce}" type="module">
    import init, { render } from '${wasmJsUri}';
    try {
      await init();
      window.__declartRender = render;
      document.dispatchEvent(new Event('wasm-ready'));
    } catch (e) {
      document.dispatchEvent(new CustomEvent('wasm-error', { detail: e.message }));
    }
  </script>
  <script nonce="${nonce}" src="${previewJsUri}"></script>
  <script nonce="${nonce}">window.__INITIAL_CONTENT__ = ${escapedContent}; window.__DECLART_THEME__ = ${escapedTheme};</script>
</body>
</html>`;
  }
}

function getNonce(): string {
  let text = '';
  const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  for (let i = 0; i < 32; i++) {
    text += chars.charAt(Math.floor(Math.random() * chars.length));
  }
  return text;
}

function parseErrorToDiagnostic(message: string): vscode.Diagnostic {
  // TOML: "TOML parse error at line N, column M\n  |"
  let line = 0;
  let col = 0;

  const tomlMatch = message.match(/at line (\d+),?\s*column (\d+)/i);
  if (tomlMatch) {
    line = Math.max(0, parseInt(tomlMatch[1], 10) - 1);
    col = Math.max(0, parseInt(tomlMatch[2], 10) - 1);
  } else {
    // JSON: "... at line N column M"
    const jsonMatch = message.match(/at line (\d+) column (\d+)/i);
    if (jsonMatch) {
      line = Math.max(0, parseInt(jsonMatch[1], 10) - 1);
      col = Math.max(0, parseInt(jsonMatch[2], 10) - 1);
    }
  }

  const range = new vscode.Range(line, col, line, col + 1);
  return new vscode.Diagnostic(range, message, vscode.DiagnosticSeverity.Error);
}
