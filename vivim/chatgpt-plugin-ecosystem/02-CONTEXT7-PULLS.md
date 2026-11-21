# Context7 extraction record — 2026-11-21

**Method:** three library-ID resolutions and three targeted `query_docs` calls, executed in the research session. This file preserves selected identifiers, key technical findings, source URLs and limits. It does not claim a full verbatim upstream docs mirror, independently audited latest package release, or complete history.

## Resolution inventory

| Requested library | Chosen exact Context7 library ID | Coverage |
|---|---|---|
| Desktop Commander | `/wonderwhy-er/desktopcommandermcp` | 1,254 code snippets; high reputation, benchmark 77.92 in returned index |
| Model Context Protocol | `/modelcontextprotocol/modelcontextprotocol/2026-07-28` | Versioned MCP spec, 2026-07-28; main entry 5,576 snippets, high reputation |
| OpenAI Apps SDK / Plugins documentation | `/websites/developers_openai` | Developer documentation; 31,608 snippets, high reputation, benchmark 62.3 |

The Context7 index also offered `/websites/deepwiki_wonderwhy-er_desktopcommandermcp` and `/openai/openai-apps-sdk-examples`, but authoritative upstream and current developer documentation were prioritized. Context7 library match availability is not a guarantee of package currency.

## Pull A — Desktop Commander local server tool and security contracts

**Exact question:** "How does Desktop Commander MCP implement local filesystem read write edit search, persistent shell processes, directories access restrictions, safety and settings? Include tool names and security defaults."

**Extracted upstream files:**
- https://github.com/wonderwhy-er/desktopcommandermcp/blob/main/README.md
- https://github.com/wonderwhy-er/desktopcommandermcp/blob/main/src/tools/schemas.ts
- https://github.com/wonderwhy-er/desktopcommandermcp/blob/main/plugins/claude/skills/desktop-commander-overview/SKILL.md

**Findings:**
1. Persistent processes preserve cwd, environment and REPL state **inside a session**. A fresh `start_process` call does not inherit the previous process environment.
2. `get_config` returns `blockedCommands`, `defaultShell`, `allowedDirectories`, `fileReadLineLimit`, `fileWriteLineLimit` and `telemetryEnabled`.
3. `allowedDirectories` guards file operations only; terminal commands can still reach paths outside these directories. Upstream warns about symlinks, command substitution and scripts bypassing superficial filters.
4. Tool argument schemas use typed Zod validators, e.g. `read_file` {path, offset, length, sheet, range}, `edit_block` {file_path, old_string, new_string, expected_replacements} or structured cell range, `start_process` {command, timeout_ms, shell}, `interact_with_process` {pid, input, timeout_ms}.
5. Search is stateful: `start_search` returns a session ID, followed by `get_more_search_results` / `stop_search`.
6. There is a tool-call history; remote cloud transport/security requires checking the separate Remote Desktop Commander documentation.

**ARBOR implication:** an MCP connector that permits terminal runs cannot rely on per-tool path filtering for least privilege. OS isolation and Host-side policy are mandatory for a production analog.

## Pull B — MCP versioned spec 2026-07-28

**Exact question:** "Remote MCP Streamable HTTP transport security, OAuth authorization, tools/list and tools/call, JSON-RPC messages, local stdio vs remote servers. Provide developer architecture details."

**Extracted upstream files:**
- https://github.com/modelcontextprotocol/modelcontextprotocol/blob/2026-07-28/docs/specification/2026-07-28/basic/transports/streamable-http.mdx
- https://github.com/modelcontextprotocol/modelcontextprotocol/blob/2026-07-28/docs/docs/2026-07-28/tutorials/security/authorization.mdx
- https://github.com/modelcontextprotocol/modelcontextprotocol/blob/2026-07-28/docs/seps/2243-http-standardization.mdx
- https://github.com/modelcontextprotocol/modelcontextprotocol/blob/2026-07-28/docs/extensions/auth/oauth-client-credentials.mdx

**Findings:**
1. MCP uses JSON-RPC request/response semantics with capability negotiation. Servers expose tools via `tools/list` and accept `tools/call` with a name and structured arguments.
2. Remote protocol uses Streamable HTTP and MCP protocol/version headers; local application integrations commonly launch stdio servers.
3. OAuth-protected resource metadata, access token verification, scopes, resource audience and auth server discovery are separable obligations.
4. Versioned spec examples include `MCP-Protocol-Version: 2026-07-28` and `Mcp-Method / Mcp-Name` headers. Do not hard-code them without checking compatibility with the deployed ChatGPT MCP client.
5. Client authorization and user-facing approval do not automatically authorize domain-specific effects; those require a separate application policy decision.

**ARBOR implication:** implement MCP transport in a thin Rust adapter, and invoke the existing Host capability/approval/persistence path. Treat transport authentication as *caller identity*, not as effect permission.

## Pull C — OpenAI plugin packaging and public submission

**Exact question:** "2026 ChatGPT Plugins package manifest remote MCP server tool annotations and publication directory process, validation requirements and submission."

**Extracted official pages:**
- https://developers.openai.com/plugins/build/plugins
- https://developers.openai.com/plugins/deploy/submission
- https://developers.openai.com/plugins/deploy/submission-errors
- https://developers.openai.com/api/docs/guides/agents-api/tools/plugins

**Findings:**
1. Portable package supports root `plugin.json`, `skills/`, root `mcp.json` and assets; root `mcp.json` uses `mcpServers` and `type: "streamable-http"` for HTTPS remote endpoint.
2. Optional OAuth config is under the server entry's `extensions.com.openai.auth`, and **secret is not embedded in the ZIP**.
3. Public submission requires verified identity, stable HTTPS server for MCP, domain control, tool scan, listing/review information and approval.
4. Tool annotations `readOnlyHint`, `openWorldHint`, `destructiveHint` and justifications are enforced for submitted MCP tools.
5. ZIP package and public listing validation are separate from a functioning private custom MCP connection.

## Evidence limits and next Context7 refresh

- Three pulls were run; **no live installed Desktop Commander tool schema** was available in this session. Query installed tool metadata after pairing and preserve a versioned, redacted list.
- Do not mistake local upstream Zod schemas for hosted proprietary service code.
- On refresh, re-resolve IDs if versioned docs change, diff MCP transport version, read OpenAI plugin submission changelog, record remote endpoint response handshake and exact user device CLI package version.
- This is research, not native build, integration test or public publication proof.
