#!/usr/bin/env node
import { createArborInspector } from '../adapters/arbor-hq.mjs';
import { startVlangMcp } from '../servers/mcp.mjs';
try {
  const inspector = await createArborInspector({
    arborRoot: process.env.VLANG_ARBOR_ROOT,
    workspaceRoot: process.env.VLANG_WORKSPACE_ROOT,
    projectId: process.env.VLANG_PROJECT_ID || 'arbor',
  });
  await startVlangMcp(inspector);
} catch (error) { process.stderr.write('VLANG MCP: ' + (error?.message || error) + '\n'); process.exitCode = 1; }
