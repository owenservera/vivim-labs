# Proposal 5 — PRE-G convergence and freeze
Date: 2026-11-12. Status: SOURCE LANDED / FOLLOW-UPS MIXED / G NOT STARTED.

## Objective
Lock six final worker snapshots and cross-owner decisions BEFORE the sole G integrator creates and merges the integrated product. Source-preserving migration, not a feature sprint. No product edits from this document branch.

## Source-grounded branch snapshot (refresh before use)

| Lane | HEAD observed | Evidence | Pre-G state |
|---|---|---|---|
| A Native | 1acce99e75d300208715f68b6db7ab11cc0cd93c | 99 files imported; native security follow-up is a design/patch packet, not runtime repair | LANDING DONE; SECURITY UNFIXED |
| B Host | c12f4a27a8c700c9df4cba17a9ff96efd5897748 | 135 source blobs; bootstrap/authority follow-up preparation, no new native launch implementation | LANDING DONE; A/B CONTRACT REQUIRED |
| C Studio | 4bca551d58a1972cc02a8b20b772e1e2417b874d | Studio and harness subtrees present, but canonical W01-C report not found | LANDING PRESENT; FINAL INVENTORY/REPORT REQUIRED |
| D HQ | 16c590dbd97dc617b783d949268ddc8fb77da2c5 | in-repo HQ JS and source tests; service adapter not mounted into Host | LANDING DONE; PRODUCT PORT UNWIRED |
| E PWA/TierFlow | d9e4d332ed7545d6038b620dd251e0b9aaeaccb7 (head changed during audit) | Original mobile/TierFlow source; new browser-device-key.mjs WebCrypto P-256/IndexedDB code and enrollment tests now committed | SOURCE PATCHED; HOST INTEROP/DEVICE UNRUN; FREEZE NOT CONFIRMED |
| F Contracts | 7538ebc706f98c15530fe39ba3a40b479448b37f | Interop/test/source evidence; reported pure V8 assertions, Node/Cargo unrun | LANDING DONE; COMBINED TESTS UNRUN |

Every lane branched from older architecture commit dd245d78ff3c110ba8630cad557ea0479c51a635, while the current Proposal 5 architecture branch is 6ab7e6976c02cac2ee71f030deda3f6541f8102f. No integrate/proposal5-beta-r001 branch existed when checked. Do not claim W0-G or W1-G GO. Later follow-up commits may still be arriving: these SHAs are observations, NOT final freeze receipts.

## Immediate pre-G protocol
1. Send each existing ChatGPT A–F lane its corresponding short PRE-G prompt. It MUST finish already-requested work, publish current HEAD and stop changing its own branch.
2. Each lane commits a final disjoint PRE-G report (integration/worker-reports/PREG-X.md, X=A..F) if it can safely do so. Report: exact starting base, donor pins, source import SHA, latest HEAD, follow-up commits, changed paths, complete source blob counts, tests actually executed/UNRUN, cross-owner requests, privacy/exclusions, operational state, and explicit FREEZE_READY or IN_PROGRESS.
3. For cross-owner issues, produce contract proposals only. Do not edit another lane's product code or root manifests.
4. A pre-G referee (distinct ChatGPT) audits all six FINAL heads and establishes a frozen input ledger. It MUST NOT merge to integration or main.
5. G starts only after each lane returns FREEZE_READY and a snapshot of the exact SHA. If any still active: hold G; do not invent a freeze or ask G to import while commits continue.

## Exclusive source ownership and G source linker
A owns native/hqkg/, core/programmability/, src-tauri/.
B owns apps/vivim-host/, sole World/effect journal/dispatcher.
C owns studio/ and harness/, signed-in Chrome browser and exact send-once.
D owns hq/, in-repo operational developer index/search/context/wiki.
E owns apps/mobile-relay/ and packages/tierflow/, one Host-served Direct PWA and inert renderer.
F owns contracts/, tests/, capability-spaces/, acceptance/test-only sources.
G owns root Cargo/Bun, lockfiles, release manifests, CI/runtime packaging, merged branch, cross-owner decisions.

Known Host cross-imports requiring C/E after G: studio/src/config.mjs, studio/src/workspace-manifest.mjs, studio/bin/studio.mjs, studio/src/studio/bridge-commands.mjs, bootstrap-owner-adapter, relay-journal-correlation, mobile-relay/lib/relay-core.mjs, control-language/NLCL, generative-ui catalog/compiler, F01/F02 bootstrap and theme-plan. Host test tools include tools/kpi/utc-maturity.mjs. G must validate dependency closure in the *combined* tree, not stub missing imports or call donor services.

### P0 S1 — Native launcher security
Current A Tauri main.rs trusts successful fixed loopback /api/host/status then sends a persistent owner bearer to localhost. The A security packet and B proposed arbor.host-launch/1 protocol are NOT implemented. Agree on one process-bound attestation/one-time launch-code contract, with Host retaining owner token. Candidate: protected inherited pipe/handle + per-launch secret, fresh challenge, HMAC/domain separation, authenticated child identity, replay/port-collision rejection and existing Host one-time browser session issuance. No durable bearer to unverified localhost. A owns native patch, B owns Host code; G must not invent a third authority. Tauri 2.0.0 remote wildcard/withGlobalTauri is security debt requiring actual W2 repair, compatible dependency resolution and tests. Pre-G contract agreement does NOT make S1 PASS.

### P0 S2 — Ordinary browser PWA pairing
Original mobile PWA required VivimDeviceKey Android bridge. DURING THIS PRE-G REVIEW E COMMITTED browser-device-key.mjs and modifications to device-enrollment.mjs with WebCrypto P-256 / IndexedDB nonextractable keys and synthetic enrollment tests. Its newest observed commit d9e4d332 adds refusal to resume a grant after losing the key. This is SOURCE IMPLEMENTED, but NOT an executed Node test, Host/B end-to-end verified flow or physical browser PWA acceptance. B/E/F must reconcile Host verification and replay/actor scopes; device gate remains UNRUN. Do not merge E until its ongoing worker explicitly freezes.

### Other G-critical decisions
- Root Cargo members native/hqkg, core/programmability, src-tauri, with workspace profiles and exact compatible patched Tauri family. G owns the root lockfile; A cannot change it alone.
- Tauri packaging references missing generated vivim.apk and old repo_root()/build working directory assumptions. PWA-first release should not bundle a fake APK. G resolves only after A/B/E imports.
- D HQ requires Node 24+, node:sqlite and FTS5, dev-source allowlist and per-user data dir, not external HQ process. Its imported arbor-service adapter is not yet Host mounted.
- E excluded donor-coupled TierFlow HQ sync-bus and executable job program examples, retained no second UI publisher. Direct PWA must not fall back to legacy cloud/Vercel runtime. Protect Host revision and mobile grant scopes.
- F Host catalog v6 19 source actions is a TEST BASELINE, not a second catalog; no Labs feature activation by inference.
- C must produce a canonical exact file/provenance report. GitHub compare files lists can cap at 300; never equate a truncated compare list with complete import.
- Source equality and V8 static assertions are not Node/Cargo/Windows/device runs. Keep UNRUN honest.

## Gate: PRE-G READY
All six worker frozen HEADs recorded, import provenance pinned, no secrets/private DB, C source completeness credible, cross-owner S1/S2 contract decision documented, internal-only paths selected, missing root/test tools recorded, no duplicate effect writer/graph engine/UI, tests accurately classified.

## Next: G, but only once
G creates integrate/proposal5-beta-r001 from latest approved architecture branch, then serially merges/imports six frozen worker changes (common ancestor dd245d...), resolving only necessary root/path conflicts, builds one workspace/lock, and emits W1-GATE. Never force-push main, overwrite workers, guess that >300-file compare responses are complete, or declare a beta from static source proof. Critical security and PWA enrollment tests remain hard W2/W3 blockers. Phase 2 Labs readiness remains PREPARED_NOT_PROMOTED until G explicitly schedules it after collision audit.
