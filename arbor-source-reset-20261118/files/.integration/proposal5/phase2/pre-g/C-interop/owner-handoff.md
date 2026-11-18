# P2-C owner handoff — maximum five post-G tasks

**Disposition: PREPARED_NOT_PROMOTED.** This is a source-selection work order, not permission to cherry-pick into G, merge W1 worker branches, or change product runtime before source integration. G is **not waiting** for this lane. Always resolve fresh frozen A–F refs at promotion time and compare their actual merged files before applying any patch.

## 1. G + F: tests/types only, after six-way source integration

**Input:** `.integration/proposal5/phase2/pre-g/C-interop/{interop-read.mjs,interop-read.test.mjs,interop.types.ts,source-delta.json}`.

**Target authority:** ARBOR F `contracts/interop/v1/types.ts`, `types.rs`, `validate.mjs`, `tests/compatibility/wave1f.test.mjs`.

**Do:** Run the eight staged Node synthetic suites. Compare `SourceRef` (id/domain/ownerRef/revision), `RevisionEvent`, inert status and actor/seq observation against F. Port any missing NEG-C01–05 fixtures into F's *existing* validator tests; retain `interop-read.mjs` only as staging evidence if validation duplicates F. Pin explicit SHA/results. F owns all final contract changes; G controls placement/merge.

**Dependency:** G source merge, F contracts installed, no automatic copy. **Reject:** second validator authority, second protocol, release-ready claim based on V8 checks.

## 2. A + B: selective pure Chronos native compute, **not** R001 native thread fabric

**Input:** Labs main `chronos-toolkit/wave2/repos-hq/research/rust-knowledge-graph/src/chronos.rs`, `tests/chronos.rs`, `contracts/chronos/contract.v1.json`; Harness R001 branch `core/programmability/src/user_io.rs` as **negative design comparison only**.

**Target owner module:** ARBOR A `native/hqkg/src/engine.rs` (existing `ComputeEngine`/`EngineRegistry`), proposed `native/hqkg/src/chronos.rs` after A chooses exact export feature; existing F `contracts/chronos/staged-contract.v1.json`.

**Do:** Source-select pure `ClockSource`/`FixedClock`, `TimeWindow`, deadline/freshness, lease and boot-fence tests; no additional `Scheduler`. A signs compile/test from G's Cargo baseline; B alone controls effects, claim/retry receipts. Do **not** transplant R001 `thread::scope` per keystroke or whole-core mutex, duplicate graphs or a new clock process.

**Dependency:** G root Cargo closure; A+B's S1 verified native child/actor provenance before live inputs. **Exit:** Cargo and Rust conformance observed, or remain UNRUN/DEFERRED.

## 3. D + B + F: truthful HQ development source and Chronos supervisor read

**Input:** D `hq/lib/arbor-service.mjs` `createDevIntelligence`, `sourceEvidence`, `indexWatermarks`; Labs `chronos-toolkit/wave2/repos-hq/hq/lib/chronos/supervisor-timeline.mjs`.

**Target owner:** D `hq/lib/chronos/supervisor-timeline.mjs` only after replay-port mapping; B's existing Host grants/auth/journal; F's SourceRef and authorization negative tests.

**Do:** B supplies authenticated actorId/sessionId, exact approvedRoots/authorize callback and revocation. D maps Labs `../../supervisor/recovery.mjs` to an **actually present canonical replay module** or marks Chronos HQ projection blocked; GitHub read found `hq/supervisor/recovery.mjs` absent on frozen D branch. Keep `development` separate from Personal World; use source hash/index watermark, reject stale/dirty reads and avoid pretending HEAD-only freshness is verified.

**Dependency:** G import of D and B; source owner provides replay adapter; tests for revocation/foreign source. **Reject:** a new supervisor, scheduler, source policy store or personal data ingestion.

## 4. C + E + F: one visual read projection, no publisher or UI action catalog

**Input:** Labs `chronos-toolkit/wave2/vivim-web-harness/studio/src/studio/chronos-conversation-projection.mjs`, `chronos-toolkit/wave2/tierflow/src/visual-runtime/host/chronos-read-projection.ts` and corresponding tests; R005 `nlp-command-ui-lab/src/semantic-lens.mjs` and `tests/semantic-lens.test.mjs`.

**Target owner modules:** C `studio/src/studio/chronos-conversation-projection.mjs`, E `packages/tierflow/src/visual-runtime/host/chronos-read-projection.ts`, E existing `packages/tierflow/src/visual-runtime/final-wave/integration/world-visual-bridge.ts`; F keeps negative test fixtures.

**Do:** Bind actual conversation snapshot source and revision (C), translate pure Chronos read to E's existing revisioned inert view, preserve R005 quote/negation/lexical-hint fixtures, and reject stale/foreign owner/render cues. Keep action catalog at B Host; Studio send, UI World publish and TierFlow execution untouched. Avoid Labs `bridge.mjs` runtime multi-checkout imports; rewrite only as an ARBOR-local read port after owner approval.

**Dependency:** G source+F interop, C's real conversation snapshot and E's revision guards. **Exit:** actual Node/TS tests recorded; no device-acceptance claim without device.

## 5. B + A + E + F: integration security and truth audit, not a feature sprint

**Input:** `acceptance.md` NEG-C01–12, PREG-A/D/E/F frozen reports and G's combined build.

**Do:** Test unauthorized caller, device session replay, wrong native child/port identity, stale revision, bad source digest, forged due/receipt, lost browser key and uncertain Host claim; maintain Host one-writer and reconcile instead of retry on uncertain. Record actual command, exit code, Windows/Android surface and failure evidence. Only source-backed and verified capabilities can be advertised.

**Dependency:** B native/Host S1, B/E PWA S2, F combined source verification, G manifests. **Exit:** security blockers separately resolved and device cases observed; otherwise keep BLOCKED/UNRUN.

---

**Explicit excluded work:** R007 total interpretation, R006 prediction engine, R008 AutoWiki, R009 Rock Core implementation, R001 native event-fabric rewrite, third-party provider APIs, second scheduler, second graph, second UI publisher, second action catalog, and new executor. These remain separate post-beta decisions unless a source owner later proves a tiny independent, safe improvement.

**Source status vocabulary:** `ALREADY_IMPORTED` means on the named A/D/E/F branch, not G or main; `TEST_ONLY` is inert fixtures; `ADAPTER_REQUIRED` needs owner mapping; `BLOCKED` identifies a real dependency; `DESIGN_ONLY` and `DEFERRED` must not become runtime claims.
