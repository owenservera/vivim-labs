// LSP 3.17 stdio, full document sync. Source-annotation diagnostics and hover only.
// All positions are LSP 0-based UTF-16 offsets; HQ annotation lines are 1-based.
import { createConnection, TextDocuments, TextDocumentSyncKind, DiagnosticSeverity } from 'vscode-languageserver/node.js';
import { TextDocument } from 'vscode-languageserver-textdocument';

function diagnostic(d) {
  const line = Math.max(0, (Number.isSafeInteger(d.line) ? d.line : 1) - 1);
  return {
    range: { start: { line, character: 0 }, end: { line, character: 1 } },
    severity: d.severity === 'error' ? DiagnosticSeverity.Error : DiagnosticSeverity.Warning,
    source: 'VLANG/HQ', code: String(d.code || 'HQ_ANNOTATION'),
    message: String(d.message || d.code || 'HQ annotation diagnostic'),
  };
}
export function createVlangLspServer(inspector, input = process.stdin, output = process.stdout) {
  const connection = createConnection(input, output);
  const documents = new TextDocuments(TextDocument);
  const inspected = new Map();
  connection.onInitialize(() => ({ serverInfo: { name: 'vivim-vlang', version: '0.0.1' },
    capabilities: { textDocumentSync: TextDocumentSyncKind.Full, hoverProvider: true } }));
  documents.onDidChangeContent(async ({ document }) => {
    const version = document.version;
    try {
      const result = await inspector.inspectUri({ uri: document.uri, source: document.getText() });
      if (documents.get(document.uri)?.version !== version) return;
      inspected.set(document.uri, result);
      connection.sendDiagnostics({ uri: document.uri, version, diagnostics: result.diagnostics.map(diagnostic) });
    } catch (e) {
      if (documents.get(document.uri)?.version !== version) return;
      inspected.delete(document.uri);
      connection.sendDiagnostics({ uri: document.uri, version, diagnostics: [{
        range: { start: { line: 0, character: 0 }, end: { line: 0, character: 1 } },
        severity: DiagnosticSeverity.Warning, source: 'VLANG/HQ',
        code: String(e?.code || 'INSPECT_UNAVAILABLE'), message: String(e?.message || e),
      }] });
    }
  });
  documents.onDidClose(({ document }) => {
    inspected.delete(document.uri);
    connection.sendDiagnostics({ uri: document.uri, diagnostics: [] });
  });
  connection.onHover(({ textDocument, position }) => {
    const doc = documents.get(textDocument.uri), result = inspected.get(textDocument.uri);
    if (!doc || !result) return null;
    const lines = doc.getText().split(/\r?\n/);
    const line = lines[position.line] || '';
    if (!line.includes('@hq:')) return null;
    const records = result.annotations.filter(r => r.line === position.line + 1);
    if (!records.length) return null;
    const descriptions = records.map(r => '- **' + r.kind + '**: ' + String(r.value));
    return { contents: { kind: 'markdown', value: [
      '**VLANG source declaration**', ...descriptions,
      '', 'Declared, not verified. No execution authorization. Scope: ' + result.scope,
    ].join('\n') } };
  });
  connection.onShutdown(() => null);
  documents.listen(connection);
  return { connection, documents, listen: () => connection.listen() };
}
