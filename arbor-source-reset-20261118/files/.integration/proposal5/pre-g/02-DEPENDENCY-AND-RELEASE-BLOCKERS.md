# Proposal 5 — Pre-G Integration Dependency and Blocker Board

The following requirements are source-observed and must be recorded before G merges. **A prepared patch is not an implemented fix.**

| ID | Producer | Consumer | Existing source | Required G action | Severity |
| --- | --- | --- | --- | --- | --- |
| D01 | C Studio config/workspace/CLI | B Host server/bootstrap | B reports imports into studio/src/config.mjs, workspace-manifest.mjs, studio/bin/studio.mjs, bootstrap-owner-adapter and bridge-commands | Verify actual C source paths after merge; no donor processes | G1 linkage |
| D02 | E mobile relay/NLCL/UI | B Host phone, bootstrap and command | B cites mobile-relay/lib/relay-core.mjs, NLCL/control-language, generative-ui catalog/compiler, F01/F02, theme-plan | Validate actual direct-phone route and host imports, no external cloud fallback | G1 linkage |
| D03 | A Tauri spawn/proof | B Host native admission | A main.rs fixed localhost/status and B owner-auth UI sessions | Agree single A/B IPC and authenticated launch-code contract; implement safely in W2 | P0 S1 |
| D04 | E PWA key proof | B Host enrollment | NEW browser-device-key.mjs P-256/IndexedDB WebCrypto signer committed on E; original Android VivimDeviceKey remains | B/E/F must confirm exact Host verifier and actual browser/device security, test gate still UNRUN | P0 S2 |
| D05 | D HQ source/read API | B Host / A native graph | HQ arbor-service.mjs internal adapter | Bind Host-approved development roots to internal HQ; distinct Personal World | W2 core |
| D06 | A root Rust crates | G Cargo workspace | native/hqkg, core/programmability, src-tauri | One root Cargo.toml/Cargo.lock, corrected Tauri family, profile.dev and target cwd | G1/W2 |
| D07 | E TierFlow src/package | B Host view and G build | Host CSS generator; inert TierFlow source | One UI publisher and schema revision crosswalk; no TierFlow standalone server | W2 |
| D08 | A/E packaging and Host resources | G Windows installer | Excluded vivim.apk; tauri.conf.json still references generated APK | PWA-first release resource decision, no fake generated artifact; package Node24/HQ | G1/W2 |
| D09 | D HQ Node runtime | G product release | node:sqlite, FTS5, internal source-index storage | Bundle/verify compatible Node24 runtime, storage rights, Git user-selected sources | G1/W2 |
| D10 | B Host test utility | G integration tests | tools/kpi/utc-maturity.mjs referenced | Import selected source/test helper or classify test exception, never stub silently | G1 |
| D11 | F common interop and action IDs | A/B/D/E consumers | Host v6 test baseline (19 actions); Rust/TS/JSON validators | Keep owner catalog and effect statuses, resolve numeric revision limits, no second registry | G1/W2 |
| D12 | C provider source | B Host single sender | existing signed-in Chrome/CDP; exact target and uncertain-send | Preserve one authoritative Host claim, no provider APIs, evidence and recovery | G1/W3 |

## Blocker states (before G)

- S1 native bootstrap and origin: DESIGN PREPARED, CODE NOT FIXED, Windows adversarial test UNRUN.
- S2 browser-only PWA pairing: E source now implements WebCrypto P-256/IndexedDB signer and synthetic enrollment tests; compatibility with B Host and physical device remains UNVERIFIED/UNRUN. E worker changed its branch during this audit.
- G0 sequencing: integration branch absent; all workers branched from old architecture commit dd245d....
- G1 integrated Cargo/Node/Bun tests: UNRUN.
- C exact migration report: NOT FOUND in the current branch at pre-G inspection.
- HQ service: SOURCE IMPORTED, HOST MOUNT UNRUN.
- No-donor build / Windows packaging: UNRUN.
- Labs Phase 2: PREPARED_NOT_PROMOTED. No automatic promotion.

## What must be done before G vs after G

**Before G:** worker freeze; exact report provenance; one A/B security contract; one B/E/F browser-PWA admission decision; C canonical inventory; G linker gap list; source/privacy checks. No full native security implementation if it depends on combined Host/Tauri source tree.

**During G:** serial source merges, root workspace/manifest, internal import closure checks, test topology, precise W2 issue ledger. Do not make security or device claims from static source.

**After G:** W2 A/B implement safe launcher and patched Tauri; W2 E/B/F address browser PWA identity; W2 D/B mount HQ; W2 A/F interoperable native/context paths. W3/4 perform device/provider/recovery acceptance.

The aim is a **stable merge**, not impossible pre-integration runnability.
