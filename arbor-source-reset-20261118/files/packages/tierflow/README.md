# TierFlow — independent visual and interaction runtime

TierFlow is a **domain-neutral three-tier agentic execution and visualization system**. Its Bun TypeScript server exposes its own HTTP/WS interfaces and static `web/` UI; the optional Rust sidecar handles selected hot-path event processing.

**Repository identity:** GitHub `owenservera/tierflow`; owner's local workspace folder `vivim-ui`. These are the same project, not duplicates.

## Quick start

```sh
bun install
bun run start          # default localhost:7717, override with TIERFLOW_PORT
bun run repl           # standalone Tier-1 CLI
bun run typecheck
bun test tests/
bun run core:test      # Rust sidecar tests
bun run verify         # integrated checks, environment permitting
```

Validate actual executed commands; no historical suite count is assumed current. Some E2E tests rewrite `runtime/surface.json` and require state protection.

## Source ownership

| Path | Responsibility |
|---|---|
| `src/server.ts`, `src/surface.ts` | Independent live TierFlow server and surface layout |
| `src/ui/` | World/source UI contracts, revision and storage modules (verify routes before claiming mounted) |
| `src/visual-runtime/` | Canonical visual documents, components, inert command IR, renderers, adapters and experimental bridges |
| `web/` | Independent browser UI, not VIVIM's product publisher |
| `runtime/` | Stateful surfaces/programs and local execution artifacts |
| `core/` | Optional Rust sidecar |
| `host/` | Optional .NET WebView2 shell, not VIVIM's Host |
| `.hq-tool/`, `wiki/` | Bundled/generated HQ artifacts; do not retire until regeneration, installer and consumer checks pass |

## VIVIM integration boundary

- [VIVIM HQ](https://github.com/owenservera/repos-hq) indexes source and generates shared development docs.
- [VIVIM Web Harness](https://github.com/owenservera/vivim-web-harness) currently owns product UI publishing, Windows Host, signed-in browser effects and the selected Android app. Its [roadmap](https://github.com/owenservera/vivim-web-harness/blob/main/docs/ROADMAP.md) is the current product priority source.
- TierFlow's World schema, visual document schema and Harness phone surface are distinct contracts. Convergence requires an explicit adapter and behavioral/rollback proof—not a schema rename or second effect writer.

See [AGENTS.md](AGENTS.md), [Mission D ownership audit](VIVIM-VISUAL-RUNTIME/MISSION-D-TIERFLOW-CONVERGENCE-AUDIT-2026-11-10.md) and the [Z.ai conceptual convergence campaign](https://github.com/owenservera/vivim-labs/tree/main/zai) for safe cleanup and best-of-breed comparison.

Historical documentation was previously relocated into [Labs](https://github.com/owenservera/vivim-labs/tree/main/documentation-consolidation/tierflow); source-owned contracts remain here until consumer-safe migration.

**Security:** this repo is public. Never commit private sessions, credentials, raw owner logs or sensitive Git bundles. A clean checkout does not sanitize historic Git objects.
