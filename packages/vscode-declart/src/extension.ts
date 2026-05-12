import * as vscode from 'vscode';
import { DeclartPreviewPanel } from './preview';
import { DeclartDiagnostics } from './diagnostics';
export { extendMarkdownIt } from './markdown';

let diagnostics: DeclartDiagnostics;

export function activate(context: vscode.ExtensionContext): void {
  diagnostics = new DeclartDiagnostics();
  context.subscriptions.push(diagnostics);

  DeclartPreviewPanel.setDiagnosticsHandler((uri, diags) => {
    diagnostics.setDiagnostics(uri, diags);
  });

  context.subscriptions.push(
    vscode.commands.registerCommand('declart.previewSide', () => {
      const editor = vscode.window.activeTextEditor;
      if (editor) {
        DeclartPreviewPanel.createOrShow(context.extensionUri, editor.document, vscode.ViewColumn.Beside, getTheme());
      }
    }),
    vscode.commands.registerCommand('declart.previewCurrent', () => {
      const editor = vscode.window.activeTextEditor;
      if (editor) {
        DeclartPreviewPanel.createOrShow(context.extensionUri, editor.document, vscode.ViewColumn.Active, getTheme());
      }
    })
  );

  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor(async (editor) => {
      if (!editor) { return; }
      const isDeclart = isDeclartDocument(editor.document);
      await vscode.commands.executeCommand('setContext', 'declart.isDeclartFile', isDeclart);
      if (isDeclart) {
        diagnostics.validate(editor.document);
        // Preview follows the active Declart file
        DeclartPreviewPanel.followDocument(context.extensionUri, editor.document, getTheme());
      }
    }),
    vscode.workspace.onDidChangeTextDocument((event) => {
      const doc = event.document;
      if (isDeclartDocument(doc)) {
        diagnostics.validate(doc);
        DeclartPreviewPanel.update(doc, getTheme());
      }
    }),
    vscode.workspace.onDidChangeConfiguration((event) => {
      if (event.affectsConfiguration('declart.theme')) {
        DeclartPreviewPanel.refreshTheme(getTheme());
      }
    })
  );

  // Set initial context for the currently active editor
  const currentEditor = vscode.window.activeTextEditor;
  if (currentEditor) {
    const isDeclart = isDeclartDocument(currentEditor.document);
    vscode.commands.executeCommand('setContext', 'declart.isDeclartFile', isDeclart);
  }
}

export function deactivate(): void {
  DeclartPreviewPanel.dispose();
}

export function getTheme(): string {
  return vscode.workspace.getConfiguration('declart').get('theme', 'default');
}

export function isDeclartDocument(doc: vscode.TextDocument): boolean {
  const ext = doc.fileName.split('.').pop()?.toLowerCase();
  if (ext !== 'toml' && ext !== 'json') {
    return false;
  }
  const text = doc.getText();
  // TOML: kind = "pyramid"  |  JSON: "kind": "pyramid"
  return /^\s*kind\s*=\s*"[^"]+"/m.test(text) || /"kind"\s*:\s*"[^"]+"/m.test(text);
}
