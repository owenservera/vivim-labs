# VIVIM APK07 — Day-One complete mobile application research, PRD and build-ready handoff

**Date:** 2026-11-01  
**Origin:** ChatGPT remote GitHub source research, verified against `owenservera/vivim-web-harness/main` SHA `761266c1437fca9b93fcb0fd800d9a7cdd85c54b` (2026-10-31 22:22:38 UTC), owner-supplied APK QR research, relevant repository docs and official platform guidance.  
**Scope:** Full APK07 capability from the first release: custom/release-signed APKs, QR delivery, Tailscale Direct, per-device secure enrollment, Android-native interfaces, Personal World/Build, universal natural-language actions, offline ownership/export, update/recovery, physical Android acceptance.  
**Truthfulness:** Documentation and investigation package, **NOT** implemented APK07, NOT a tested physical Android release. Local checkout may be further advanced. No repository was modified by this package.

## Start here — local agent

**First file:** [`docs/agents/APK07-LOCAL-AGENT-INVESTIGATION-AND-EXECUTION.md`](docs/agents/APK07-LOCAL-AGENT-INVESTIGATION-AND-EXECUTION.md). This is a complete, direct instruction to the local agent; it does not rely on this ChatGPT conversation. It directs the agent to compare its current code against the source SHA above, independently investigate 12 known coverage weaknesses, reconcile existing roadmap state, execute the full integrated product and collect truth-graded real-device receipts.

**Primary working checkout:** `C:\0-BlackBoxProject-0\vivim-web-harness`. **GitHub:** https://github.com/owenservera/vivim-web-harness. If the local agent uses Linux, discover its actual location instead of assuming a Windows-mounted path.

Read/reconcile against local `AGENTS.md`, `context.md`, `docs/INDEX.md`, `docs/roadmap.json`, current source and `docs/library/DECISIONS-AND-CONFLICTS.md` before changing live runtime. The local agent **must not downgrade existing more advanced code to match these October 31 snapshots**.

## Documents

| File | Purpose |
|---|---|
| `dev/prd/12-android-apk07-day-one.md` | Full product contract, capabilities FR-A through FR-G, user journeys, evidence-based release definition |
| `docs/research/APK07-current-state-evidence-2026-11-01.md` | Source inspection/current state, evidence levels, known contradictions, official reference links |
| `docs/design/APK07-architecture-and-protocols.md` | Authority diagram, app/device/Host states, release/enrollment/action/sync protocol sketches |
| `docs/design/APK07-factory-signing-distribution-updates.md` | User-built signed APK factory, package/key continuity, QR download and verified update design |
| `docs/design/APK07-security-threat-model.md` | 24 attack scenarios with negative test expectations and trust boundaries |
| `docs/design/APK07-mobile-runtime-world-offline.md` | Android shell, native APIs, on-device data/World privacy, offline, cache and recovery |
| `docs/design/APK07-ux-and-real-device-acceptance.md` | Onboarding screens, error states, 30 physical Android acceptance tests, device matrix |
| `docs/agents/APK07-LOCAL-AGENT-INVESTIGATION-AND-EXECUTION.md` | Mandatory 12-part local investigation + code handoff + acceptance/receipt policy |
| `docs/roadmap/APK07-task-ledger.v1.json` | **59 non-authoritative** atomic execution candidates; source anchors, canonical ID candidates, dependencies and physical tests |
| `docs/roadmap/APK07-capability-matrix.csv` | Inspectable flat capability crosswalk for local reconciliation |
| `docs/roadmap/APK07-decisions.v1.json` | **16 open architecture decisions** to resolve against local source and owner policy |
| `docs/roadmap/APK07-evidence.schema.json` | Strict machine-readable receipt validation contract |
| `docs/roadmap/APK07-evidence-example.json` | Schema-valid illustrative **unverified** source review example |
| `INTEGRATION-MANIFEST.json` | Package metadata, file mapping, absolute local Windows target paths |

`APK07-COMPLETE-RESEARCH-PRD-DESIGN-AND-AGENT-HANDOFF.md` is a convenience single-file concatenation of the narrative docs. Use original individual files as the most inspectable source; don't treat the single-file compilation as a second canonical spec.

## Canonical local-repository integration — **do not duplicate authorities**

- Source PRD 12 is proposed as `dev/prd/12-android-apk07-day-one.md`; the local agent decides its exact filename after checking current numbering.
- Source-oriented design files go into `docs/design/`, research into `docs/research/`, agent task into `docs/agents/`. JSON/CSV candidates in `docs/roadmap/` are **not a second roadmap**.
- Reconcile all candidates **into `docs/roadmap.json`**, deduping existing `P01`, `DIR01`, `PACK01`, `GUI01–07`, `UTC01–07`, `BF00–03`, `W01–04`, `HOST/START`, and other canonical IDs.
- Cross-link relevant docs in `docs/INDEX.md` and repo PRD index. Regenerate actual docs/symbol catalogs with repo tooling rather than manually rewriting generated indexes.
- At each completion, attach timestamped/source-hash/physical-device evidence. A source file alone is not a passed real Android test.
- Preserve one existing Studio/CDP browser owner, send-once uncertain semantics, protected private data and no Vercel requirement.

## Execution order (internal dependency graph, **not** staged user releases)

1. Record local SHA/current services/toolchain/device and resolve policy/signing/Host identity. Map against source, not archived docs.
2. Build release APK factory with stable identity/signing/key recovery and on-device offline shell; create authenticated Direct versioned install/QR.
3. Add short-lived one-time device enrollment backed by Android Keystore proof + Host-scoped grants/revocation.
4. Wire canonical Personal World/GenUI, native Android integrations and UTC Host-typed actions on same app; preserve provider session journal.
5. Implement offline state/sync/export/import and reliable update/failure recovery; run threat/negative tests.
6. Run **all** T01–T30 on actual Android and evidence-grade the single **full APK07 first-release** gate. No claim of complete release without physical receipts.

**Reminder:** `GUI07` (already exists) is the real Android generative UI acceptance task. The broader **APK07** program must extend it; do not accidentally rename/recreate it as a parallel task.
