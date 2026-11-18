# Proposal 5 HVI — GOVERNING SCOPE CORRECTION: Install native tooling, postpone comparisons
**User decision, 2026-11-12:** Implement, integrate, document, expose, and test HQ Version Intelligence as a native, agent-discoverable capability, ready for use *when expressly asked*. **Do not perform a real multi-repository or HQ migration audit in this wave.**
This explicit intent supersedes older instructions in this folder to run the five-repo audit, classify 361 missing HQ paths, invoke a final real self-comparison, generate unported recommendations from actual corpus or prepare a scheduled scanning plan. Already committed historical S0/D source-count proofs remain untouched.

## Definition of done
A user can later issue an ordinary natural-language command such as:
- “Compare these repos against ARBOR and tell me what landed.”
- “Compare this repository at commit A, commit B and commit C.”
- “Show source and capability changes, how certain they are, and the tests supporting them.”
Without teaching an agent implementation details, launching specialized worker prompts, or orchestrating a migration plan. The **already-installed** native capability must be discoverable in the existing agent/Host command registry and invoke the same HQ code paths, not custom standalone orchestration.

## Current build vs deferred usage
**BUILD NOW**: A–F real source engine and schemas, deterministic pin acquisition adapters, reuse identity registry, N-way compare, parity classification, export, CLI/service interface, Host live authorization, natural-language capability/tool discovery, TierFlow reusable optional view, offline replay, safe denial, test suite, machine-readable agent guidance, integration and readiness evidence.
**DO NOT EXECUTE NOW**: real ARBOR vs donor or Labs comparison; 361 original-path semantic review beyond preexisting D source-only proofs; final migration atlas; actual missing capabilities determination; real source snapshots acquired solely for “dogfood”; automatic comparisons when idle; background watches; migration recommendations from real source; personal-corpus reads; direct browser/provider access.

## Test exception, deliberately narrow
Developers **MUST run** meaningful executable tests if runner available, but on **synthetic invented fixtures and injected Git-object adapters**. Synthetic same-repo SHA1/SHA2/SHA3 and distinct synthetic repositories must cover 2..N comparison, exact relocation, lineage, digest, incomplete scans, invalid source/secret denial, redaction, authorization/revocation and offline no-donor replay. Tests are not a real migration analysis. Distinguish test-executed and not-run; optional real developer integration of existing repo code (HQ/Host modules) is allowed for build/test imports but not content comparisons.

## Agent readiness contract
Native tool should be visible through existing command/agent capability discovery. H must inspect actual registration, NL dispatch and Host authority owners; use their canonical interfaces (including `apps/vivim-host/bootstrap/capability-activation.mjs`, `control/command-intent.mjs`, `bootstrap/command-worker.mjs`, `hq/lib/action-registry.mjs`, `hq/lib/action-router.mjs`, `hq/lib/capabilities.mjs` as potential boundary sources). Implement actual native registration, typed input/output, stable capability ID, capability description/triggers/examples, source-grant request flow, refusal/unknown/partial statuses, agent usage instructions and version-pin resolution. A documentation-only “skill” or unregistered endpoint is NOT sufficient.
The agent may parse user intent and ask for explicit sources/permission; no model-inferred privilege. A request may explicitly select 2..N repo/commit pins, including 3 versions of the same repo. Authorized source adapters are injected and bounded; no browser provider API, no code execution from donor repositories, no second native action executor, graph or visual publisher.

## Updated remaining workflow
- **Z1X** currently launched by user: continue unchanged. May inspect real frozen code/source to diagnose integration contracts, but no new migration execution.
- **G1** serial A–F integration, schema reconciliation, Node24 combined **synthetic** tests; `GO_FOR_W2_TOOLING`.
- **H + U** parallel: H agent/Host native capability registration and live permission read port; U reusable TierFlow inspector and empty-state (synthetic fixtures only).
- **X** deferred / no launch / not a gate.
- **Z2** independent H/U authority + agent discovery referee.
- **G2** serial H/U integration + agent-tool invocation tests with synthetic fixtures.
- **R** **readiness certification**, not real self-audit.

## Gate / honesty
`HVI_TOOLING_READY` means installed, wired, agent-discoverable, authorized and tested with synthetic inputs in a permitted runtime. It never means a real user corpus was compared or that beta/device release gates passed. If full Host live security could not run, status `HVI_TOOLING_INSTALLED_NOT_VERIFIED`, not READY. Tests lacking actual executable exit remain UNRUN. In-product remote access may remain disabled until security owner accepts it; report installed vs enabled separately. Never change older worker commits or S0 immutable pins.
