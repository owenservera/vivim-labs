# Proposal 1 — Source-to-Destination Migration Map and Transformation Register

Date 2026-11-12. **Plan, not performed migration.** Paths are inspected source locations or proposed target directories. "Source-present" does not mean runtime-wired. All copied code must pass security/license review, behavioral parity, integration tests and source SHA evidence first.

## Rules for the full-stack, one-time reconstruction

1. **Do not concatenate repositories.** Port capabilities to their one owner contract. Keep a machine-readable inventory with each old path's destination, transformation, verification and retirement decision.
2. **No code copy before source classification.** Each source unit becomes \`PROMOTE\`, \`ADAPT\`, \`REIMPLEMENT\`, \`RETAIN_EXTERNAL\`, \`ARCHIVE\`, or \`REVIEW\`, with a signed owner decision.
3. **One controlled cutover, multiple tested implementation commits.** New ARBOR can run isolated read-only comparison against incumbent and cut over once. No simultaneous effect-journal writers or uncontrolled dual data stores.
4. **Not everything becomes Rust.** Pure stateful semantics, durable admission and computational graph belong in Rust; UI rendering/IME stays TypeScript; Chrome signed-in browser integration remains a small isolated JS worker.
5. **Every migration maps both runtime and data.** Preserve old plan/digest/receipt semantics, personal World identities, source lineage and graph revisions; migrate only authorized local owner data, never Git-commit private profiles or credentials.
6. **Retire entrypoints with proof.** No duplicate phone app, runtime publisher, action catalog, local proxy or graph store survives under a new name.

## High-level placement crosswalk

| Source and evidence | Target ARBOR owner | Transformation / dedupe | Cutover acceptance |
| --- | --- | --- | --- |
| Harness \`src-tauri/\` Windows Tauri v2 shell | \`apps/desktop/src-tauri/\` | **REIMPLEMENT shell/bootstrap policy** with currently patched compatible Tauri, exact trusted origin, service/process identity; preserve packaging/install identity via planned upgrade path | Windows MSI, localhost impersonation and native IPC tests |
| Harness \`apps/vivim-host/server.mjs\` | \`crates/host/\` + native desktop ingress | Replace Node central product HTTP runtime with Rust service endpoints; retain only audited static read/compat compatibility where necessary | Same desktop/phone journey with Node Host no longer required for pure requests |
| Harness \`control/intent-planner.mjs\`, \`command-intent.mjs\`, \`command-program.mjs\` | \`crates/intelligence/language,interpret\` + \`crates/contract\` | **PARITY PORT** deterministic semantics into typed multi-facet IR, source spans, ambiguity and negative clause handling. Do not import a second command catalog | differential corpus same or intentional safer outcomes |
| Harness \`apps/mobile-relay/nlcl/\`, \`control-language.mjs\` | \`crates/intelligence/language\` + \`apps/web/src/composer\` | Preserve Omega/NLCL golden fixtures, retire mobile authoritative parser after Rust parity; UI may keep provisional local grammar as hints | exact desktop/PWA semantic agreement and offline status |
| Harness \`control/action-catalog.v1.json\` | \`schemas/protocol\` + Host capability registry read contract | **SINGLE CANONICAL CATALOG**: migrate registered action IDs/risks/readiness/source owner; add new capabilities only via owner acceptance; avoid dual JSON catalogs | crosswalk each of currently observed 19 Host actions v6 with implementation and proof |
| Harness \`control/dispatcher.mjs\`, \`journal.mjs\`, \`receipt-runner.mjs\` | \`crates/effects/\` | **PORT ONLY AFTER CRASH PARITY**: preserve exact admission, approval, exclusive-claim, uncertain recovery, receipt and idempotency rules. Original JS remains sole writer until cutover | independent fault injection; never two writers |
| Harness \`control/personal-rule-store.mjs\`, \`personal-semantics.mjs\` | \`crates/world/\` + \`crates/intelligence/prediction\` | Separate personal preferences as durable owner state from read-only scoring; preserve consent/correction provenance | owner-scoped rollback, correct aliases, no inference as authority |
| Harness \`core/programmability/src/lib.rs\`, \`wire.rs\` | \`crates/contract\` + \`crates/intelligence\` | Preserve pure validation and transformation safety types; review current wire schema, map to canonical typed contracts, remove redundant adapter | no regression in native validator and policy tests |
| R001 \`core/programmability/src/user_io.rs\` (draft PR #55) | \`crates/node0/\` | **REIMPLEMENT STRUCTURE, REUSE TEST INTENT**. Replace scoped OS threads per edit and Tauri mutex-held computation with bounded scheduler, actor-bound sessions and partial evidence projections | real Cargo tests, liveness/load/security and source grounding |
| Harness \`apps/vivim-host/direct-phone.mjs\`, device control/pairing | \`crates/devices/\` + \`crates/host/ingress\` | Port already established pairing trust, phone-grant/revoke semantics and direct-only security; eliminate redundant proxy/semantic dispatch, but preserve PWA transport protections | revoked device, expired grant, offline/no double effect |
| Harness \`apps/mobile-relay/\` UI and \`apps/vivim-host/public/\` | \`apps/web/\` | Select one canonical responsive app and one composer/controller, migrate good screens and phone controls, remove competing UI routing/stores | one app surface; desktop/PWA equivalence; accessibility |
| Harness \`studio/src\`, \`studio/bin\`, \`harness/\` signed-in webapp | \`apps/browser-worker/\` + \`packages/browser-bridge/\` | **ADAPT** to isolated, single approved browser worker. No provider APIs, no Playwright/Puppeteer; preserve exact-thread and uncertain-send guards, capture provenance and host-issued effect leases | real logged-in browser read/send smoke, crash/unknown tests |
| Harness \`studio\` capture / conversation proof / source inventory | \`apps/browser-worker/capture\`, \`crates/world/ingestion\` | Separate immutable raw evidence from candidate extraction; no automatic unreviewed permanent memory import | verified source refs and user consent, no hidden send |
| Harness \`apps/vivim-host/media/\` + mobile streaming/cache | \`crates/host\` authorized media service + \`apps/web\` player | Preserve secure path/range, HEAD, byte-range and resumable playback contract; port only if coverage/policy verified | HTTP 200/206/416, authorization, offline resume, device playback |
| Harness \`apps/vivim-host/onboarding/\`, \`bootstrap/\` | \`crates/host/bootstrap\`, \`crates/world/ingestion\`, \`apps/web/screens\` | One lifecycle, signed-in Chrome-first provider connection, progressive index and owner consent; remove competing bootstrap UIs | t0-first-value, privacy/revoke, source availability honesty |
| Harness Studio/Host mission + recovery, supervisors | \`crates/host/supervisor\`, \`crates/intelligence/missions\` or retained \`host\` module if no stable crate | Reuse durable mission identities and exact state transitions; one mission experience, separate projections from effects | real worker crash recovery and receipt-backed mission status |
| HQ \`research/rust-knowledge-graph/src\` | \`crates/graph/\` | **PROMOTE** one shared versioned graph engine, Registry/DAG/scheduler/changes/invalidation; parameterize source domains; choose canonical release ownership | product/hq consumed same engine version, parity and privacy boundary |
| HQ \`hq/lib/indexer.mjs\`, \`search.mjs\`, \`wiki.mjs\` | **KEEP in repos-hq**, product reads only permitted evidence contract | Do not bundle full development indexer or engineering wiki into end-user installer. For personal wiki, build *distinct owner-data views* using shared graph computation | HQ unavailable does not break user product |
| HQ \`hq/lib/semantic-registry.mjs\`, action census | \`repos-hq\` source intelligence + \`crates/contract\` generated product IDs | Stable identity crosswalk at build/test time, don't let development registry mutate product permissions | schema diff parity, source line provenance |
| HQ R007 interpretation producer census | \`tests/contract\`, \`tools/integration\`, \`crates/node0/trace\` | Move verified producer manifest generation into product build/evidence; census remains a research source, not a live coverage claim | runtime actually reports ran/denied/timeout/unavailable |
| TierFlow \`src/visual-runtime/\`, \`src/ui/\` | \`apps/web/src/tierflow\` + pinned \`packages/ui\` dependency | **ADAPT** existing visual renderer and guarded revision publishers; don't clone TierFlow's standalone server into the product | one UI renderer; revision-safe cross-surface view |
| TierFlow \`core/\` optional Rust sidecar | \`crates/graph/\` or retained independent TierFlow upstream | REVIEW algorithm and ABI overlap, move only unique validated hotpath functionality; no competing Rust event bus | benchmark plus retirement crosswalk |
| TierFlow \`host/\` optional .NET/WebView2 shell | **DO NOT SHIP** | Product uses one Tauri/Windows shell; retain independent TierFlow upstream if useful | only one installed product shell |
| Labs \`user-io-rust-core/\` and Node 0 Prime | \`crates/node0/\`, \`tests/node0\`, \`docs/architecture\` | Adopt validated invariants/acceptance corpus, not copy unverified R001 implementation wholesale | actual native tests and actor security |
| Labs \`nlp-command-ui-lab/\` R004–R007 | \`apps/web/src/composer\` + \`crates/node0/trace\` | Separate presentation grammar and VISUAL lens from authoritative semantic producer; port only accepted browser behavior | quiet/assisted/VIEW ALL same source truth |
| Labs \`personal-knowledge-lifecycle/\` | \`crates/world/ingestion\`, \`crates/intelligence/context\` | Port consent/source atlas/contracts, then inspect prototype runtime; no auto-inject raw history or network provider APIs | consent, provenance, partial onboarding, revocation |
| Labs \`vivim/voice-to-tasks/\` | \`crates/node0/input\`, \`crates/intelligence\`, \`apps/web\` | Convert voice capture/transcript into same typed interaction event; preserve Day-0 voice-to-task ownership and audit | no unapproved durable task from ASR partial |
| Labs \`chronos-toolkit/\`, automation research | \`crates/host\` or \`crates/intelligence\` pure time logic | Import schedule/timing rules only if native source and tests inspected; use existing Host effect authority for reminders and background actions | time zones, restart scheduling, no duplicate clocks |
| Labs \`capability-observatory/\` | \`tools/integration\`, \`tests/migration\` | Development certification/manifest source only; never an end-user runtime or second product registry | source vs production gate status evidenced |
| Labs \`vivim/\` extension system proposals | \`crates/contract\` + \`crates/host\` opt-in extension ports (LATER) | Preserve source evidence, privacy and permission rules; do **not** build a new plugin loader merely because 153 candidate surfaces exist | actual owner hooks, sandbox/permission tests |
| Labs \`intelligence/\` algorithm tracks | \`crates/intelligence\` selectively | Benchmark each algorithm against existing Rust native path; promote only validated gains, no duplicate services | measured baseline and source correctness |
| Legacy private \`owenservera/vivim\` (Cosmic Desktop) | \`apps/web/\` SELECTIVELY | Its SolidJS kernel uses a client-side central singleton. Keep useful presentation/document components **only after UI compatibility review**; do not port its cloud-provider API model or mock IndexedDB as product authority | no old provider API or fake datastore semantics |
| Legacy private \`owenservera/vivim-platform\` | selective audit only | Older multi-server topology, API-first and Playwright dependencies conflict with currently selected direction. Classify code/algorithms, not mass-import services or credentials | no old provider APIs or cloud authority introduced |

## Structural transformations (the actual refactor, not feature upgrades)

### T01: Five entrypoints → one product runtime
Use **one resident Rust Host** as session/World authority. Desktop Tauri shell links it; phone PWA connects through authenticated transport; CLI invokes same protocol. The browser worker is not a second Host. Remove Node Host from Node 0 hot path after differential parity.

### T02: Multiple command interpretations → one source-anchored semantic IR
Retain all necessary *independent meaning facets*, rather than picking one winner, while only one canonical typed command/effect representation exists. Generated types ensure no JS/Rust risk disagreement. Preserve personal language corrections and quoted/negated clauses.

### T03: Research Rust graph + separate Rust validator + TierFlow sidecar → shared computation
One versioned graph/typed DAG/scheduler building block; integrate validator as pure safety logic and adapter traits; retire redundant hotpaths only after benchmark/parity. No new master graph database.

### T04: Heterogeneous status booleans → evidence-backed typed lifecycle
Convert \`success\`, \`ready\`, \`claimed\`, \`uncertain\`, \`verified\`, \`needs_approval\` etc. to explicit typed transitions with source-owner proof. No display-status inflation.

### T05: Localhost trust + scattered session secrets → authenticated process/surface boundary
Replace HTTP-health-as-service-identity before token transfer, wildcard native IPC origins, cross-surface caller-supplied identity. Native server mints trusted sessions and credential scopes; PWA uses revocable device grants.

### T06: Multiple data stores → domain-owned single write paths
Personal World, external provider receipt journal and owner asset store stay distinct domains under one Host writer; HQ development KB stays external. Derived indexes/caches are rebuildable. Data migration preserves IDs/revisions/lineage, and no original is erased before validation.

### T07: Multiple UI bundles → one canonical web build
Desktop and phone use same product design contracts with responsive shells and platform-specific transport. TierFlow renders without becoming another full application runtime. Visual lab expression becomes presentation over typed native trace.

### T08: Independent release machinery → one product release manifest
The Rust binary, Tauri shell, browser worker, PWA and protocol all share exact compatible version metadata. CI owns package, SBOM, signature, migration and recovery proof.

## Source revision/collision policy

All mappings are **preliminary** until inspected as exact source path, SHA, branch, test owner and live consumer. Migration inventory should be generated from checked out source trees and enriched with runtime evidence:
\`sourceRepo, sourceSha, sourcePath, symbols, runtimeConsumer, authOwner, dataOwner, targetPath, decision, requiredTransform, tests, proof, retirementCondition\`.

Do not infer a component is "ratified" because it appears in Labs or is described as architectural. "Ratified" means an explicit owner-accepted contract and release integration gate, with a source reference. If ratification cannot be verified, mark \`CANDIDATE_REVIEW\` and do not silently promote it.
