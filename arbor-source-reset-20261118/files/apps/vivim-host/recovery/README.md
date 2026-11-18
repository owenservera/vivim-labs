# Lane D — Autonomous Diagnosis & Self-Healing (isolated Host integration)

**Source status, 2026-11-10:** Pure recovery decision engine is committed. The existing Host status snapshot now includes an R0-only recovery read-model. **No autonomous mutations, restarts, provider sends, World repairs, automatic approval or deployed/physical-device acceptance.**

## Source owner and boundaries

- `intelligence.mjs` / `intelligence.d.ts`: versioned failure signatures, deterministic digest-pinned plans, expiry, authorization-epoch checks, independent verification state machine, non-executing R2/R3 proposals and recovery-outcome ranking.
- `host-adapter.mjs`: bounded metadata-only mapping from existing `workerHealth` / `messageJourney` fields to source-linked R0 recovery diagnoses. Missing heartbeats remain **missing observability**, not process-death proof; future-dated evidence cannot establish a failure. No auto-retries.
- `../server.mjs:hostSnapshot`: adds `recovery` read-model to existing owner-local status response; **not** a second Host recovery engine.
- `../bootstrap/worker-supervisor.mjs:status`: worker status no longer deletes evidence leases while observing a dead process.
- `../server.mjs`: legacy worker HTTP `POST /api/host/worker/:role/recover` checks origin, CSRF marker and owner token, then rejects with HTTP 409 `CONTROL_ACTION_PLAN_REQUIRED` and **does not call `supervisor.recover`**. Worker GET status is owner-authenticated. Direct lease repair remains blocked pending a real Host action catalogue, claim/receipt fence and security proof.
- Host `control/journal.mjs` + `control/dispatcher.mjs` remain exclusive authorities for owner plans/effects/receipts; Studio journal reconcilers retain no-resend semantics; TierFlow `RecoveryView/1` remains a non-executing projection. HQ supervisor crash recovery is for development lanes, not a Host action executor.

Recovery levels R0–R4 are separate from Host action risks R0–R3. R1 has **no admitted mutating executor**, R2/R3 are manual inert proposals only, R4 requires reviewed source/deployment changes.

## Verification, recorded without overclaiming

- `node --test apps/vivim-host/test/recovery-intelligence.test.mjs apps/vivim-host/test/recovery-host-adapter.test.mjs`: **29 passed / 0 failed**, in isolated Node 22.16 source checkout (device-free).
- Added `../test/worker-route-security.test.mjs`: negative HTTP expectations for invalid role, missing owner token, cross-origin/CSRF requests and non-mutating 409 with authenticated owner. **Not run against full Harness checkout**.
- Extended `../test/worker-supervisor.test.mjs`: repeated dead-process observation must retain uncertain lease. **Not run against full Harness checkout**.
- Extended `../test/host.test.mjs`: recovery read-model must have zero automatic mutations/sends. **Not run against full Harness checkout**.
- Full Host suite, clean-clone integration, Windows Host, browser, APK and physical Android tests are **UNRUN**. No CI, device or production acceptance implied.

## Required next gates

1. Apply complete Host regression suite on current main-derived branch and verify no read/status consumer relies on the previously unauthenticated worker status or direct recovery route.
2. Lane C→Lane D: pin source finding schema, causal evidence ordering, World/phone revision and authorization epoch; fail closed on missing provenance.
3. Admit only a named existing Host R1/R2 operation after runtime authority review, user-granted scoped policy, durable claim/receipt and independent postcondition verification. Never infer permission from confidence.
4. Recovery Memory persistence stays with owner-local Host operational state; HQ may index redacted engineering evidence. No competing state store.
5. Lane E fault-injection: lost connection/ACK, process death, stale phone snapshot, schema skew, revocation during recovery, duplicate command and crash between claim and receipt. Verify zero duplicate provider sends and no unbounded loops before live enablement.
6. **Portable Rust-core convergence:** product recovery decision logic ultimately belongs in the shared Rust core consumed by Windows and Android, not an independent JS execution subsystem. The inspected Harness baseline contains a Tauri Windows wrapper but no demonstrated portable shared product Rust crate. Do not turn HQ's research Rust knowledge graph or the Windows Tauri wrapper into a second product authority; establish and verify the canonical Rust/FFI boundary before moving the pure engine. Until then this source slice is intentionally bounded and has no live mutation path.

Do not merge as evidence of working autonomous self-healing. Isolated review/verification only.
