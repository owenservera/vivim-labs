// MCP stdio via official Model Context Protocol TypeScript SDK v2.
// No write, effect, tool-install, or approval operations.
import { McpServer } from '@modelcontextprotocol/server';
import { StdioServerTransport } from '@modelcontextprotocol/server/stdio';
import * as z from 'zod/v4';

const response = result => ({ content: [{ type: 'text', text: JSON.stringify(result) }] });
const error = e => ({ isError: true, content: [{ type: 'text', text: String(e?.code || 'VLANG_ERROR') + ': ' + String(e?.message || e) }] });

export function createVlangMcpServer(inspector) {
  const server = new McpServer({ name: 'vivim-vlang', version: '0.0.1' });
  server.registerTool('vlang_inspect_source', {
    title: 'Inspect VLANG source',
    description: 'Read existing ARBOR HQ @hq:v2 annotations from a local workspace-relative source file. No authority, source edits, or proof of implementation.',
    inputSchema: z.object({ path: z.string().min(1).max(1024) }),
    annotations: { readOnlyHint: true, openWorldHint: false, destructiveHint: false },
  }, async ({ path }) => {
    try { return response(await inspector.inspect({ path })); } catch (e) { return error(e); }
  });
  const pin = z.object({ projectId: z.string(), path: z.string(), sourceSha256: z.string() });
  server.registerTool('vlang_compare_source_pins', {
    title: 'Compare source revisions',
    description: 'Conservatively flag source-hash changes; file moves require owner-reviewed lineage. Pure operation.',
    inputSchema: z.object({ previous: pin, current: pin }),
    annotations: { readOnlyHint: true, openWorldHint: false, destructiveHint: false },
  }, async ({ previous, current }) => {
    try { return response(inspector.compareSourcePins(previous, current)); } catch (e) { return error(e); }
  });
  return server;
}
export async function startVlangMcp(inspector) {
  const server = createVlangMcpServer(inspector);
  await server.connect(new StdioServerTransport());
  return server;
}
