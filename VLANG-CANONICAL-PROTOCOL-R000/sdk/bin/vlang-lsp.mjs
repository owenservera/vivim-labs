#!/usr/bin/env node
import { createArborInspector } from '../adapters/arbor-hq.mjs';
import { createVlangLspServer } from '../servers/lsp.mjs';
try {
  const inspector = await createArborInspector({
    arborRoot: process.env.VLANG_ARBOR_ROOT,
    workspaceRoot: process.env.VLANG_WORKSPACE_ROOT,
    projectId: process.env.VLANG_PROJECT_ID || 'arbor',
  });
  createVlangLspServer(inspector).listen();
} catch (error) { process.stderr.write('VLANG LSP: ' + (error?.message || error) + '\n'); process.exitCode = 1; }
