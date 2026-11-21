# ARBOR integration architecture: ChatGPT/MCP/skills without a second Host

**Status:** architecture proposal, not production code. Derives from 2026-11-21 official MCP / OpenAI developer docs and the existing Labs proposal `vivim/PLUGIN-SYSTEM-ARCHITECTURE.md`. ARBOR production HEAD was not fetched for this specific architecture note; verify actual current Rust modules before implementation.

## 1. Product thesis

Desktop Commander confirms the importance of **hosted conversational tool discovery → authenticated device agent → local execution**, but its general unrestricted terminal model is not the target VIVIM architecture.

ARBOR should expose a **small, typed MCP-facing facade** to its existing Rust Host. ChatGPT is a caller (tool proposer), not a new World/effects authority; Studio remains the browser interaction adapter, not a second autonomous effect owner. GPUI is an optional native renderer/companion; PWA remains primary. MCP should work with the existing semantic command/capability fabric.

~~~text
ChatGPT / Codex / compatible MCP client
       |
       | HTTPS Streamable HTTP + OAuth (user identity, scopes)
       v
VIVIM MCP gateway (Rust; PUBLIC INTERFACE, no effect rights)
       |
       | typed command/knowledge query + idempotency key + provenance
       v
ARBOR Host Rust core (one authority)
       +----> policy/grants/approvals ----> effect executor / Studio ----> receipt + journal
       +----> native HQKG/Wiki retrieval (scoped read, revision-aware)
       +----> VEC deterministic/pure visual projection
       +----> paired-device agent (explicit per-device grant)
       |
       +----> response: verified outcome OR proposed/denied/stale/unknown
~~~

No raw `execute(command)`, `read_any_path` or `browser_click` in the public tool catalog. These make review and capability security unnecessarily fragile.

## 2. Rust-first boundaries

- Transport: Rust HTTP (choose Axum or another maintained Rust stack only after dependency review), MCP JSON-RPC types with version compatibility and streaming, OAuth protected-resource metadata and validation. Rust is preferable here for shared domain contract and constrained memory.
- Policy: reuse Host's existing Rust approval, receipt and journal implementation. Each MCP request carries caller, owner, World, device, capability, correlation ID, tool schema revision and expected object revision.
- Knowledge read: existing HQKG and wiki projections, scoped to authenticated user and original source revision. Never fork a second knowledge database to serve a plugin.
- Actions: convert model request into typed **proposal**; authorize, present approval, execute once and journal through the canonical Host path. Retries use idempotency keys and return prior receipts.
- Visual: optional MCP UI can read a sanitized VEC output; don't create a second UI compiler. TypeScript 7+ only for browser widget glue where required; native logic remains Rust.
- Device: separate user/device pairing from OAuth to ChatGPT. A valid OAuth token cannot silently pair a new device or obtain unrestricted machine rights.
- Privacy: no model APIs required; MCP is the tool interface between ChatGPT and VIVIM, not a provider inference API. User-supplied signed-in browser provider sessions remain inside the authorized Studio/Host boundary.

## 3. Proposed minimal tool portfolio

| Public tool | Operation class | Who authorizes? | Data contract |
|---|---|---|---|
| `vivim_capabilities` | read-only | Authenticated owner scope | capabilities actually available for that owner/device, not wish list |
| `vivim_wiki_search` | read-only | Host policy + retrieval ACL | source/revision-qualified matches and freshness |
| `vivim_tasks_list` | read-only | Host scope | bounded paginated tasks |
| `vivim_task_propose` | creates proposal | Host grants, human approval before effect | typed plan, stable idempotency key, risk |
| `vivim_approval_status` | read-only | Host policy | approved/rejected/pending/expired |
| `vivim_receipt_lookup` | read-only | Host policy | effect status/reconcile evidence |

No `file_write` or `run_shell` until a separate, truly sandboxed workspace-specific design is proven. Tool metadata must accurately distinguish read-only, open-world and destructive actions.

## 4. Call lifecycle contract (candidate)

~~~text
1. HTTP OAuth token verified, resource audience and session scoped.
2. MCP schema/arguments parsed and size/depth bounded.
3. Identity mapped to owner + World; device separately selected/authorized.
4. Capability availability and revision checked against Host.
5. For reads: authorized immutable projection; redact secrets; return source + revision + stale marker.
6. For writes: create proposal; require approval; Host dispatches effect once; journal receipt.
7. On disconnect/timeouts: return unknown outcome and receipt lookup, never blindly replay.
8. On revoke: deny future calls, invalidate sessions and device effects, preserve audit retention policy.
~~~

Do not assume client-side UI confirmation is the same as Host's approval. Content returned by providers may contain prompt injection; it cannot modify call policy or capability descriptions.

## 5. Desktop Commander in the meantime

Use remote Desktop Commander **only on a deliberately restricted scratch or standard-user Windows environment** to prototype agent ergonomics and basic repo inspection, not as a covert Host execution bridge. Record manual grants, process lifecycle and audit logs. Since `allowedDirectories` does not sandbox terminal access, avoid connecting a privileged user session holding provider cookies, secrets or sensitive local archives. Its hosted proprietary relay adds a separate trust/data processor not present in a local-only design.

Desktop Commander and a VIVIM MCP plugin can coexist as different ChatGPT plugins: one general machine operator, one first-party domain-specific service. ARBOR should not depend on Desktop Commander being installed by customers.

## 6. Single source of truth and migration checklist

- Inspect exact ARBOR `main` first; pin current commit and current Rust Host/kernel/HQKG/VEC/device grant modules.
- Map every proposed MCP tool onto **an existing invoked Host capability**, or mark it NEW-CONTRACT, blocked pending authorization.
- Confirm build targets: Windows host, PWA thin client, optional GPUI; no requirement that ChatGPT desktop runs locally.
- Test scoped read/approval denial/revocation/stale revision/cancellation/duplicate request/device offline/journal replay.
- Only when source, tests, runtime and security evidence exist, package the external MCP adapter.
- Keep Labs as source-linked research, not an extra roadmap, registry, ledger, World writer or product codebase.

## 7. Open strategic decisions

1. External plugin entry point: VIVIM-hosted direct remote MCP vs relay-to-owner-host. Default recommendation: **gateway + paired device transport** only if hosting costs and security demands justify it; if product can answer from an authorized cloud service, skip device relay.
2. OAuth service/operator and user identity mapping: must be production-owned; no shared static API key.
3. How owner/offline device status is surfaced without leaking device inventory.
4. Threat model of user-local shell vs typed effects; arbitrary shell remains outside first public MVP.
5. Tool set boundaries to satisfy OpenAI's rules on unofficial third-party connectors.
6. Rust MCP SDK selection and protocol compatibility: confirm up-to-date implementation against MCP 2026-07-28; avoid copying TypeScript sample APIs blindly.

## Evidence

- https://github.com/desktop-commander/remote-desktop-commander
- https://github.com/wonderwhy-er/DesktopCommanderMCP
- https://modelcontextprotocol.io/specification/2026-07-28
- https://developers.openai.com/plugins/build/plugins
- https://developers.openai.com/plugins/deploy/submission
- https://developers.openai.com/plugins/plugin-guidelines
- https://github.com/owenservera/vivim-labs/blob/main/vivim/PLUGIN-SYSTEM-ARCHITECTURE.md
