# Native command implementation waves — integration-first

**Target repo:** `owenservera/ARBOR` on `main`, without PR/feature branches per owner preference; re-pin latest HEAD and preserve all other changes. **This is a Labs proposal, not authorization to create a second runtime or modify existing Host policy.**

## P0 — make an actual read-only command work end to end

**Best first vertical slice:** “Show the five largest files in Downloads.”

1. Verify latest source and current command catalog. Confirm authoritative Host request endpoint and legitimate device owner token.
2. Expose a Host-owned scoped `filesystem.list/stat` **read capability** through a Rust adapter invoked by the incumbent dispatcher. No arbitrary path supplied by model becomes a grant.
3. Add one action to the existing catalog *only after* schema/owner/revision/readiness/OS adapter is real; begin `adapter_pending` rather than pretending source wiring implies live verification.
4. In Rust pure command layer, map `show the largest files`, `what is taking up space` and literal `/files.find.largest_files` to one structured candidate with source spans and missing-slot reasons.
5. Resolve `Downloads` to a user-consented folder handle through Host; use limited recursive iteration with file size/metadata and bounded results.
6. Return actual file paths and sizes (with privacy-aware display), provenance, bounded scan stats and error if denied/inaccessible. R0 needs scoped read permission; no user approval may be required for an already granted local read, but no silent grants.
7. Show identical source-backed command/plan/result in PWA and GPUI; validate cross-device behavior.
8. Execute on the real Windows user device, with an observed command transcript, exact test counts and exit statuses before claiming success.

**Acceptance:** actual five files sorted by size, stable target mapping, no access outside user-approved scope, actual Host command receipt, no empty synthetic data, no regressions to current 38 actions.

## P0+ — prove the trust boundary before bulk expansion

Second slice: “Find files containing X inside my chosen folder.” Read-only content search; enforce file count, byte count, file kind, symlink and sensitive files, cancellation. Third slice: “Open the resolved file in the default application” with a preview + exact user action.

**Crucial trap:** existing `harness/src/dev/workspace.mjs` is a restrictive *developer-repo sandbox*, not the universal filesystem broker. Do not merely delete its folder restrictions or repurpose it as universal execution authority.

## P1 — useful local reversible effects

Implement first as shared host-owned adapter primitives, not tens of copy-pasted action functions:

- `file.create`, `file.rename`, `file.move`, `file.copy`, scoped text patch with expected digest. Include overwrite conflict and recycle/undo semantics.
- `app.open`, `window.focus`, `process.list`, `process.details`, approved `process.spawn` from executable/argv allowlist, session read/stop.
- `build.rust.check`, `test.rust.run` via approved cargo arg arrays, isolated working directory and capped logs. No raw shell by default.
- Multi-step plan compile: find → select → preview → effect, per-step receipts, explicit partial/uncertain states; no client-side automatic replays.
- PWA command history, response diagnostics, user-visible target chooser, action preview, receipts, undo proposal.

**Acceptance:** real user-selected folder rename/move with receipt, stale-source rejection, crash-once retry dedupe, deny symlink escape, user cancellation, actual cargo check/test job with process session cleanup.

## P2 — broad daily-computing domain expansion

- Documents/archives/media/data adapters: safe format parsing, file bombs/zip-slip defenses, memory limits, receipts and intermediate ownership.
- Studio browser: typed read/search/tab/fill/submit with single signed-in Chrome profile lease. No independent browser driver, Playwright, Puppeteer or provider APIs.
- Notes/tasks/voice, calendar/email/contacts: use authorized real data sources only, separate draft from send, preflight external target and exact-content approval.
- Paired devices: independent device permissions and grants, no assumption PWA can access remote Windows filesystem by itself.
- Automation: durable triggers, explicit owner activation, per-step no-duplicate claims, retries only of proven idempotent reads.

**Acceptance:** one real end-to-end journey per adapter, negative permission cases and repeated/refreshed PWA + GPUI projection; external effect and recovery verified.

## P3 — privileged capability decisions

High-integrity OS settings, arbitrary terminal, permanent deletion, installation/privileged permissions, purchases and irreversible account changes remain **nonexecutable**. Consider separately only after OS-level isolation, local UAC/privilege flow, security threat model and proven Host owner approval. P3 may remain outside product scope indefinitely.

## Source owners and handoffs

| Concern | Actual ARBOR source entrypoint | Requested work, not yet done |
|---|---|---|
| Semantic NLCL and typed text input | `apps/mobile-relay/control-language.mjs`, `apps/vivim-host/control/command-intent.mjs` | Shadow Rust parser, parity tests and fast context compilation |
| Rust inert plan/program | `core/programmability/src/command_intent_projection.rs`, `command_program.rs`, `product_kernel.rs` | Add structured typed IR only after migration compatibility proof |
| Command and readiness catalogue | `apps/vivim-host/control/action-catalog.v1.json`, `intent-planner.mjs` | Source-backed registrations with truthful readiness, no 279 blind additions |
| Effects/approval/journal | `apps/vivim-host/control/dispatcher.mjs` | Scoped OS adapters callable **only after claim**; preserve exact digest and receipt semantics |
| Native desktop | `native/gpui-desktop/`, native Host supervisor | Render existing Host plans, previews and receipts; no OS effect ownership |
| Mobile PWA | `apps/mobile-relay/` | Host-connected previews/status, same utterance meaning, device readiness |
| Browser | `studio/src/studio/` | Reuse single-browser-lease authority; no extra controller |
| Knowledge | Host Wiki + `native/hqkg/` | Command source and action receipts become provenance-linked graph/wiki views only |

## Ownership and sequencing rules

Do not start by porting every current JS file to Rust or copying Desktop Commander source. Make the **native OS boundary actually work** with one representative read action, then harden it before broadening the command vocabulary. Compiler success without a callable real Host adapter is `adapter_pending`, not capability success. Never weaken the existing R3/nonexecutable policy or approval path to make a demo green. Use latest main and working tree safeguards; do not reset, clean, rewrite or force-push.

## Result states (all branches tested)

`candidate`, `clarification_required`, `interpretation_required`, `adapter_pending`, `unsupported`, `approval_required`, `denied`, `running`, `completed`, `partial`, `uncertain`, `cancelled`, `refused`. These are proposed product-visible states, not one-to-one aliases for every legacy component state. Build a versioned translation table before wire changes.
