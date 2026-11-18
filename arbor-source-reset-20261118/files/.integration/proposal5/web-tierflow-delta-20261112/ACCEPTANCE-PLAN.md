# ARBOR Proposal 5 — Web/TierFlow Acceptance and Repair Handoff

**Status:** RECOMMENDED_NOT_EXECUTED. This document does not promote Phase 2/HVI workers, start a product server, authorize effects, or revise existing release gates. Use the current main tip and all owner leases before starting. **All commands below are planned, not executed by this audit**.

## A. Required handoff ordering and ownership

1. **G + A + F: reproducibility and installer preflight.** On an isolated Windows x64 working tree with enough disk, confirm repository commit, generate and review real root lockfiles, run dependency resolution, source closure and exact source safety checks. Respect one integration writer and never fabricate Node24 binary/lockfiles. Existing helpers: `integration/scripts/w01g-r001-windows.ps1` and `integration/scripts/w01g-r001-source-closure.mjs`; `tools/packaging/prepare-host-runtime.mjs`. Stop before owner data is exposed if native security remains unresolved.
2. **A + B + F: native child ↔ Host security.** Remove durable owner token transfer to unauthenticated fixed-loopback health responder. Bind one launch to an authenticated spawned child, one-time UI code, tested strict Tauri origins and patched compatible Tauri family. Test foreign prebound listener, raced port, orphan PID, replay, wrong origin and restart.
3. **B + E + F: PWA cryptographic request binding.** Preserve `browser-device-key.mjs`/pending approval. Require proof over method + canonical URL/path + body digest + actor/grant/session + server nonce/expiry; consume nonce atomically, deny replay across restart, revoke and cross-device mixup. Do not claim nonextractable IndexedDB key is hardware-backed.
4. **C + G + F: Studio test-dependency closure.** Compare exact 305 omitted `studio/test/` donor files in `SOURCE-BLOBS.v1.json`. Review fixtures for source safety and host-only assumptions, then import only relevant tests to the C-owned `studio/test/` destination. Keep original `studio/src` 269-file source pin stable unless a specific defect is proven. Reconcile root `test:studio` and Studio package scripts.
5. **B + E + F: authenticated TierFlow view port.** Read current Host source World snapshot under the existing actor read authorization. Bind `ownerRef, workspaceRef, worldId, sourceRevision, sourceHash, policyRevision, deviceSessionEpoch` to an immutable observation; reject stale/mismatched/revoked snapshots. Invoke the existing pure `world-visual-bridge.ts` and visual renderer. No TierFlow independent publisher/store/effect writer.
6. **E + G + F: cross-surface mount.** Mount the inert TierFlow projection into current Host and PWA views, not a new standalone UI. Verify same visual root, documented subset of components, safe unknown handling, focus/keyboard semantics and real receipts. Revisit historical root ID / renderer-wrapper mismatch by actual current-SHA test execution; never rely solely on the old blocker report.
7. **B + C + E + G + independent F: full acceptance.** Physical Windows + ordinary Android browser PWA, exact supported provider thread send/reply, force-stop/recover, revocation, stale projection, no-donor build, low-disk and rollback. Keep beta NO_GO until all required B01–B12 evidence exists.

## B. Reproducible command matrix — execute only in appropriate approved environment

| ID | Planned command / procedure | Environment | Required receipt | Status in this audit |
|---|---|---|---|---|
| T01 | `git rev-parse HEAD; git status --porcelain` | isolated ARBOR checkout | exact immutable SHA, clean/known tree | UNRUN |
| T02 | `cargo generate-lockfile; cargo metadata --locked --format-version 1` | pinned Rust; reviewed network deps | real Cargo.lock, full metadata, exits | UNRUN |
| T03 | `bun install --ignore-scripts; bun install --frozen-lockfile --ignore-scripts` | Bun 1.4.4, Node 24 | actual bun.lock, frozen install exits | UNRUN |
| T04 | `cargo test --workspace` | Windows x64 MSVC + correct Rust target | test names, failing cases, exit | UNRUN |
| T05 | `node --test apps/vivim-host/test/*.test.mjs` | Node 24 | Host auth/effect/recovery fixture receipt | UNRUN |
| T06 | `node --test apps/mobile-relay/test/*.test.mjs` | Node 24, cwd repo root | PWA signing, revision guard, service worker and static route tests | UNRUN |
| T07 | `bun test packages/tierflow/tests/visual-runtime` | Bun 1.4.4 | actual visual schema/render/revisions/integration-4 outcome | UNRUN |
| T08 | `bun run --cwd packages/tierflow typecheck` | installed TypeScript deps | real typecheck exit and errors | UNRUN |
| T09 | `node --test "studio/test/unit/**/*.test.mjs"` **after reviewed test restoration** | Node 24 | real Studio suite + exact source pin | BLOCKED: 305 donor tests absent |
| T10 | `node --test "harness/test/**/*.test.mjs"` | Node 24 | product bridge tests, exact exits | UNRUN |
| T11 | `pwsh -File integration/scripts/w01g-r001-windows.ps1 -Mode Verify` | Windows x64, MSVC/Bun/Node24 | full logs/artifacts, check all script side effects first | UNRUN |
| T12 | `node integration/scripts/w01g-r001-source-closure.mjs` | isolated complete checkout | full report and zero external donor runtime paths | UNRUN |
| T13 | Windows MSI install after approved security fixes | real target machine | signed/staged binaries, process identity, restart, rollback | UNRUN |
| T14 | Android Chrome HTTPS Host-served PWA install | physical Android, scoped user grant | pair→approve→read→revoke→re-pair, request proofs | UNRUN |
| T15 | Signed-in Chrome provider exact-thread send and reply | approved test account/browser | literal message hash, target thread, send-once claim and receipt | UNRUN |
| T16 | Isolated donor-removal test | move donor repos away; stop donor services | no checkout/network/API dependency, same UI and local indexing | UNRUN |
| T17 | Host/TierFlow visual parity and a11y | actual desktop + phone | same principal/World/source rev/IDs, DOM, keyboard, screen reader | UNRUN |
| T18 | Power loss / ENOSPC / recovery drill | disposable state, backups | protected originals, uncertain-not-resent, rollback and UI parity | UNRUN |

**Do not run tests with real private user data or signed-in live effects without an explicit approval and rollback fixture.** Exact test names can be refined by owners once dependencies and restored Studio tests are verified; a planned command may need correction, and its presence here is not evidence of passing.

## C. Mandatory negative acceptance scenarios

| ID | Attack / failure | Expected Host/TierFlow/PWA behavior |
|---|---|---|
| N01 | Malicious local service binds original Host health port | native refuses durable token, fails closed |
| N02 | Foreign Tauri content or unexpected origin calls privileged IPC | denied before owner data or command |
| N03 | QR/pair token reused | second enrollment rejected |
| N04 | Grant stolen without protected device private key | cannot prove enrollment or privileged request |
| N05 | Valid grant+session bearer replayed with new POST payload | rejected without fresh request proof |
| N06 | Previously accepted signed nonce replayed after restart | durable replay refusal |
| N07 | Revoked grant continues polling/editing | denied, stale views explicitly unavailable |
| N08 | Phone projection from different owner/world/revision | rejected, not silently merged |
| N09 | TierFlow World component unsupported, unsafe text, wrong source digest | fail closed; Host originals untouched |
| N10 | TierFlow visual command includes write/execute without approved Host plan | inert only; no effect, no optimistic receipt |
| N11 | Two UI publishers race on same revision | Host single-writer fence, explicit conflict and rollback |
| N12 | Network fails after provider send / missing receipt | mark uncertain, never blind resend |
| N13 | Offline cached PWA shell tries privileged mutation | no grant implied; deny until authenticated |
| N14 | Low disk during World/journal/index operation | protect originals, bounded cache eviction, recoverable error |
| N15 | Donor repo/services removed | installed ARBOR remains internally operational (external provider access excepted) |
| N16 | Physical phone and desktop disagree about receipt/status/root | fail cross-surface acceptance; do not silently GO |

## D. Exact evidence format for every accepted UI capability

Use the authoritative `integration` release ledger, not a competing truth store, and attach at least:

- `sourceSha`, `integrationSha`, `artifactSha`, `owner`, `scope` and `principal` (redacted);
- exact `command`, `environment` (OS/CPU/Node/Bun/Rust/device/browser), `exitCode` and bounded output excerpt;
- source vs runtime classification, `hostWorldRevision`, `visualRevision`, source digest and stale/revocation decisions;
- action catalog ID, plan digest and explicit Host receipt for effectful scenarios; no invented user action;
- screenshot/DOM/a11y evidence (redacted), signed-in-provider thread ID scoped/redacted, physical-device evidence where required;
- explicit `PASS`, `FAIL`, `BLOCKED` or `UNRUN` and rollback tested/unrun.

Do not mark `RUNTIME_WIRED` from a TS import, `DEVICE_ACCEPTED` from a synthetic fixture or `BETA_GO` from source-only GitHub merge.

## E. Gate mapping

- **B01/B02/B03:** packaged Windows Host + actual signed-in Studio/browser provider behavior.
- **B04:** ordinary browser PWA pair/approve/revoke/request proof.
- **B05:** bounded Node 0 composer; not proven by R005/R007 UI concepts.
- **B06:** Host one writer, plan, grant, send-once, crash receipt and security refusal.
- **B07/B08:** internal HQ Dev vs personal World boundaries, no external donor checkout.
- **B09:** common source IDs, TierFlow inert view, phone/desktop parity, one publisher.
- **B10:** data protection, offline/reconnect uncertainty and rollback.
- **B11:** truthfully generated capability statuses; optional HVI inspector only after gate.
- **B12:** protected storage and low-disk budget proof.

**Required release decision:** `NO_GO` until source closure, security and real-world behavior are independently accepted. G alone may update canonical release manifests/locks after executing and reviewing actual evidence.
