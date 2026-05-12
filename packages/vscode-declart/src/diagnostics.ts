import * as vscode from 'vscode';

export class DeclartDiagnostics implements vscode.Disposable {
  private collection: vscode.DiagnosticCollection;

  constructor() {
    this.collection = vscode.languages.createDiagnosticCollection('declart');
  }

  // Called when the document changes to clear stale diagnostics eagerly.
  // Actual error diagnostics come from the webview via renderError postMessage.
  validate(doc: vscode.TextDocument): void {
    this.collection.delete(doc.uri);
  }

  setDiagnostics(uri: vscode.Uri, diagnostics: vscode.Diagnostic[]): void {
    this.collection.set(uri, diagnostics);
  }

  dispose(): void {
    this.collection.dispose();
  }
}
