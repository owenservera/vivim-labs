# DesktopCommanderMCP-VIVIM — ARBOR harness intake, first source assessment

**Source**: https://github.com/owenservera/DesktopCommanderMCP-VIVIM  
**Pinned source**: `main@9a101b17e44577f0baefa4c7120f087881c5cc56`  
**Origin**: fork of `wonderwhy-er/DesktopCommanderMCP`; public, MIT; package version `0.3.4`.  
**Assessment date**: 2026-11-22. **Status**: preliminary source-tree and manifest inspection; no builds or runtime tests executed.

## What was actually inspected

- Root source files and packages; `package.json` scripts/dependency names.
- `src/tools/`, `src/handlers/`, `src/remote-device/`, `src/ui/` directories.
- Canonical ARBOR directories `harness/`, `apps/`, `native/`, `studio/`, `wiki/`, `contracts/` observed; *source-level compatibility remains unverified*.

### Extraction candidates and provisional dispositions

| Component | Source paths | Fit / potential benefit | Research disposition |
|---|---|---|---|
| Terminal/process lifecycle | `src/terminal-manager.ts`, `src/command-manager.ts`, `src/tools/process.ts`, `src/handlers/terminal-handlers.ts`, `process-handlers.ts` | bounded local runtime diagnostics, output streaming, process cancellation, verified exit evidence | **ADAPT / investigate**; preserve Host effect authority |
| Filesystem + search | `src/tools/filesystem.ts`, `src/search-manager.ts`, `src/handlers/filesystem-handlers.ts`, `search-handlers.ts` | allowlisted local context discovery and SITREP evidence, not universal disk access | **ADAPT / investigate**; permission gating required |
| Diff editing and fuzzy search | `src/tools/edit.ts`, `src/tools/fuzzySearch*.ts`, `src/handlers/edit-search-handlers.ts` | diagnostics and constrained repair tooling | **REFERENCE** first; must not bypass cloud-first GitHub source workflow |
| Typed MCP transport/contracts | `src/server.ts`, `src/tools/schemas.ts`, `src/types.ts` | interoperable tool registration, schemas and results | **REFERENCE**; avoid duplicate router/service |
| Configuration and guardrails | `src/config-manager.ts`, `src/config-recovery.ts`, `src/tools/config.ts` | allowlist, runtime policy, recovery inspiration | **ADAPT / investigate** within existing host contracts |
| Remote device and network | `src/remote-device/` includes MQTT, authentication, remote channels, telemetry | may inform post-push local device verification; raises remote-execution risk | **DEFER** pending threat model and existing transport comparison |
| UI runtime | `src/ui/` includes config-editor/file-preview | not a GPUI renderer or TierFlow migration target | **REFERENCE only**, do not build separate PC UI |

**Not yet proven:** individual APIs, security behavior, input validation, runtime performance, OS compatibility, dependency necessity, actual GPUI integration, or mobile parity.

## Architectural rule

**ARBOR owns the harness and effects.** Desktop Commander is an optional, bounded *local adapter* for explicit inspections and on-device verifications, not another first-class PC interface or primary code-writing agent. No automatic shell execution from untrusted text, broad filesystem write capabilities, remote-device tunnel, browser session capture or local source development authority.

User-visible acceptance must tie any integrated capability to a real command/event on the GPUI command surface, truthful AutoWiki feedback/receipts, and corresponding mobile semantic state. An adapter with no visible PWA/GPUI behavior is at most an enabling dependency, not an independently approved roadmap item.

## Next engineering investigation

1. Read exact source implementation and tests for process execution, search, patching, config/permissions, and remote device; identify real APIs and test coverage.
2. Compare against current ARBOR harness and native Host files at pinned ARBOR commit; list duplicates and safe extension seams with concrete paths.
3. Design **read-only SITREP inspector** as first candidate: allowlisted filesystem and process status → typed Host receipt → wiki projection → GPUI/mobile parity. Require explicit user approval for any action that changes local OS state.
4. Define isolation for binaries/subprocesses, path traversal prevention, output redaction, command injection prevention, and cancellation.
5. Record exact license/attributions and compare TypeScript/Node requirements to Rust-first policy; prefer native Rust port only when it is simpler than minimal interop.
6. Run build/test, then Windows on-device checks; mark every unperformed check UNRUN.

### Evidence boundaries
- Source SHA above is observed from GitHub's main branch at intake; do not infer it is a VIVIM-specific change.
- `package.json` references `npm`, `tsc`, and a UI runtime builder. That is upstream implementation data, *not* an ARBOR decision to adopt npm or JavaScript.
- No changes made to source repository or ARBOR; this file is public-safe Labs research.
