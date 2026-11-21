# VLANG agentic interfaces — MCP + LSP (R0)

**Status**: adapter and transport source scaffold committed to Labs; syntax and isolated adapter tests pass. External MCP/LSP package installation and actual client sessions UNRUN. **Do not describe this as production-integrated.**

## Shape

Both transports call `createArborInspector` in `adapters/arbor-hq.mjs` which loads the **current local ARBOR** `hq/lib/annotation-v2.mjs` via an explicit filesystem path. The SDK does not copy, fork or reinterpret HQ's parser, registry, graph or journal. No HTTP server, remote provider, filesystem writer, model API or secret access.

### Install and start (Bun preferred)

```sh
cd VLANG-CANONICAL-PROTOCOL-R000/sdk
bun install
export VLANG_ARBOR_ROOT=/absolute/path/to/ARBOR
export VLANG_WORKSPACE_ROOT=/absolute/path/to/ARBOR
bun run mcp
# In another process: bun run lsp
```

On Windows set `VLANG_ARBOR_ROOT` and `VLANG_WORKSPACE_ROOT` in the process environment instead of using `export`. `VLANG_PROJECT_ID` is optional, default `arbor`.

### MCP agent configuration

```json
{
  "mcpServers": {
    "vlang": {
      "command": "node",
      "args": ["/absolute/path/to/VLANG-CANONICAL-PROTOCOL-R000/sdk/bin/vlang-mcp.mjs"],
      "env": {
        "VLANG_ARBOR_ROOT": "/absolute/path/to/ARBOR",
        "VLANG_WORKSPACE_ROOT": "/absolute/path/to/ARBOR"
      }
    }
  }
}
```

Advertised read-only tools: `vlang_inspect_source({path})` and `vlang_compare_source_pins({previous,current})`. `path` must be workspace-relative; read rejects traversal, sensitive paths and escaped symlinks. Results distinguish declaration from verification. These are source-inspection tools, not typed symbol navigation or authorization.

### LSP editor configuration

Configure your editor's language-server client to spawn `node /absolute/path/to/sdk/bin/vlang-lsp.mjs` over stdio and pass the two environment roots. The server implements LSP 3.17 initialize, full-document sync, close, push diagnostics, and annotation-line hover. No completion, rename, go-to-definition, workspace symbols, formatting or code action is advertised in R0. The LSP is **supplementary** to rust-analyzer and TypeScript language services, not a replacement.

### Context and authority boundaries

`getVerifiedContext()` still requires a trusted production adapter over HQ's existing `composeContext(db,...)` and `verifyContext(bundle)`. Since no bound, read-only service adapter was available in this research package, neither transport advertises a context tool. Inert source comments never confer Host grants, effect execution, passed-test receipts or proof of symbol binding.

### Verification ledger

- `node --check` on five new modules: PASS locally.
- `node --test test/*.test.mjs`: **11 passed / 0 failed**, isolated SDK + filesystem adapter test environment.
- Official package installation, MCP tools/list and tools/call handshake, LSP framed initialization/hover, real ARBOR parser integration: **UNRUN** (external package installation blocked in local sandbox).

Protocol sources: https://modelcontextprotocol.io/specification/2026-07-28 ; https://github.com/modelcontextprotocol/typescript-sdk ; https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/.

## Next high-value steps

1. Run `bun install` and real MCP + LSP smoke tests in the ARBOR checkout on current `main`.
2. Bind verified context to existing HQ service with explicit source revision and no implicit index mutations; add `vlang_context` to MCP only after verification.
3. Add TSX/Rust syntax-aware symbol attachment via tree-sitter/rust-analyzer/TypeScript, never inferred line ownership.
4. Add cross-file semantic navigation, hover evidence, references, diagnostics with evidence provenance, minimal MCP resources and prompts.
5. Ensure versioned client support and admission tests for current 2026-07-28 MCP and legacy client compatibility via official SDK.
