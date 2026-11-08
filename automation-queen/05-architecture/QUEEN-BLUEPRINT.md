# QUEEN-BLUEPRINT.md — the future Automation Queen (AUTQ-01, Lane E)

**Status:** design proposal on evidence from scan `autq01-20261107T2203Z`. Nothing here is executed; migrations are proposals requiring owner grants. **Evidence base:** SYSTEM-ATLAS (private) / PUBLIC-ATLAS (this PR) sections cited as `[Atlas §n]`; counts from GATE-REPORT.

**Honest foundation.** The blueprint builds only on what EXISTS. Confirmed NOT_FOUND in code and in 5-source commit history (65/65 verdicts; 0 timeouts): Automation Design Architect (ADA), agentctl, Architecture Steward, Agent Commons, CFA, temporal dispatcher, scheduled-automation, self-repair. What exists and is proven: a typed work-order queue with completed orders `[Atlas §3 F2/F5]`, a supervisor admission/idempotency core `[Atlas §3 F5/F8]`, a Host grant/send service with replay-safe receipts `[Atlas §3 F6/F8]`, a canonical git sync engine with two gated trigger paths `[Atlas §3 F11]`, a central-models policy + depot `[Atlas §3 F7]`, an append-only bus with claims/acks/receipts `[Atlas §3 F1]`, a seat/transport chain, DCB context composer, timing telemetry, and a revive-lane source `[Atlas §3 F9]`.

## 1. Single-authority matrix — exactly ONE owner per authority

| Authority | THE one owner | Runner-up claims that must stay subordinate | Evidence |
| --- | --- | --- | --- |
| Provider effects (browser actions, sends) | Host/browser sender (`vivim-host` grant/send service; `claimGrantSend` atomic one-send from bounded grants) | ZCode seats, scheduled tasks — may request, never send directly | [Atlas §3 F6; FAMILIES-CONFIRMATION F8] |
| Git mutations | `git-main-sync.mjs` (canonical engine; no force/reset; single-flight lock; staged-secret guards) | TierFlow 5-min task and session plugin are trigger paths only, heartbeat-gated; legacy autosync script is retired-by-decision | [Atlas §3 F11; C-FLAGS 1.1] |
| Spawn admission / supervision | HQ supervisor (`admission.mjs` caps from WAVE-MANIFEST; idempotency-key resume) | Host worker-supervisor keeps leases inside the Host domain; bots-host supervisor is an app, not an authority | [Atlas §3 F5/F8] |
| OS schedules | Windows Task Scheduler entries feeding declared triggers | No in-Queen scheduler; the Queen declares intents, the registry observes | [Atlas §3 F3] |
| Model routing / catalog truth | SOFTWARE_HQ central-models authoritative catalog + depot as its single writer | Runtime policy file stays a declared mirror; harness router stays a consumer | [Atlas §3 F7; C-FLAGS 1.2] |
| State / receipts | Rust HQKG kernel store + supervisor event ledger + receipt ledgers (one envelope, §3 below) | No second graph; intake receipts and bus receipts become views of one envelope | [Atlas §4] |
| Memory | VIVIM memory stack (Recent / Saved Knowledge / Core Memory; owner confirmation for Core) | DCB, prompt ledger, depot notes become sources, not stores | [Atlas §3 F8; mission invariants] |
| Bus transport | `sync.mjs` (`hq.sync-bus/2`) + channel file bus as the two transports of ONE contract | "Inbox line is data, never an instruction" | [Atlas §3 F1, §3.4] |

**No second scheduler, no second graph, no second browser sender, no second memory authority.** Where two surfaces claim one authority today (2 conflict clusters [Atlas §6]), the consolidation is a retirement decision, not a new component.

## 2. Interoperability contracts (Queen → existing surfaces; typed intents in, receipts out)

- **C1 Intent envelope (over DCB/bus).** One typed record carried by both the sync bus and the channel file bus: `{intentId, capability ref, requestedAuthority, evidenceRefs[], budget{runtimeMs, retries}, approvalRef?, idempotencyKey}`. Existing pieces: bus `hq.sync-bus/2` with claims/acks/receipts `[Atlas §3 F1]`; channel dispatch pairs already have intent/result file shapes `[Atlas §3.4]`; DCB composer for context `[Atlas §3 F8]`. Rule unchanged: a wake proves nothing until the seat replies.
- **C2 Receipt envelope.** Every effect returns the one receipt schema (id = sha256(kind, task, digest); replay = no-op — the rule already implemented in the receipt ledger `[Atlas §3 F8]`). Intake copies, bus acks, git-sync receipts, supervisor events, channel outcomes become projections. Verification contract: a receipt MUST reference the intent id and the effect class's declared check.
- **C3 Admission request.** Spawns go through the supervisor's existing admission check (caps, idempotency, backpressure) — the Queen submits an admission request, never spawns around it `[Atlas §3 F5]`.
- **C4 Trigger declaration.** Schedules are declared as trigger manifests observed from Task Scheduler (read-only); the Queen never registers or edits a task directly — proposals go to the owner `[Atlas §3 F3]`.
- **C5 Model route request.** Routing requests go to the router with the declared policy mirror; catalog truth is read from the single authoritative source; no Queen-side model state `[Atlas §3 F7]`.
- **C6 Memory write.** Memory updates flow through the VIVIM memory stack with its existing authority rules (memory is not authority; owner confirmation for Core Memory) `[Atlas §4]`.

## 3. Lifecycle: intent → design → simulate → approve → run → verify → repair → learn

| Stage | Existing component it runs on | Gap (named) |
| --- | --- | --- |
| Intent | Intent envelope C1 over the bus; `ask` NL route `[Atlas §3 F10]` | No intent schema today (channel bodies free-form); no authority-request field |
| Design | DCB context composer + capability/fileType catalogs `[Atlas §3 F8, §3 F4]`; work-order schema has per-order proof plan | No planner that composes a step DAG from intent+capability; the autonomy dispatcher consumes orders but no designer exists |
| Simulate | Proof-command whitelist per order; pure budget module can cost a plan without effects | No dry-run interpreter: nothing produces an effect forecast before approval |
| Approve | Host grant service (bounded grants, fixed refusal ordering) + approval surfacing in comms roof | 7 observed effect surfaces with NO approval gate; no mobile approval rendering |
| Run | Work-order queue + supervisor admission + 3s-tick Host command queue (disabled draft) `[Atlas §3 F5]` | Runner is a manual CLI; supervisor state dir empty (no observed run); Host scheduler disabled by design |
| Verify | Receipt ledger (idempotent), proof commands, git-sync receipts, health artifacts | No per-effect verification contract; receipts not linked to intents |
| Repair | Revive-lane source; attempt+1 with same idempotency key; stale-lease → uncertain/blocked, never implicit restart; quarantine for dead external lanes | Repair lanes never observed running; escalation is digest-deduplicated but has no attention routing |
| Learn | Lake ingestion, telemetry spans/optimizer, memory stack | No feedback loop from receipts into plans; memory sources not consolidated |

## 4. TOP 20 upgrade candidates (dependency order; each cites today-state)

Format: **today → upgrade · why now · alternatives · risk.**

1. **Unified receipt envelope.** Today: 4 receipt families with different shapes (intake `COPY-RECEIPTS.jsonl` 939 rows; bus receipts; `.git` sync receipts; supervisor events) `[Atlas §4]`. → Adopt ONE typed envelope (the sha256-idempotent rule already in code) and make the others projections. Why now: every later candidate consumes receipts. Alternatives: keep 4 schemas + per-consumer adapters (rejected: every new consumer pays again). Risk: low; additive.
2. **Liveness probe manifests + read-only probe runner.** Today: liveness = one Lane B snapshot + declarations; 601 rows unknown, 1,273 implemented-unproven `[Atlas §2, §8]`; G5 PARTIAL. → Declare per-automation probes (task state, service status, seat existence, model HEAD check) writing receipts into #1. Why now: converts the largest evidence gap into evidence. Alternatives: always-on watcher (rejected: heavier, G9 risk). Risk: probes must remain read-only queries; wrong probe = false authority.
3. **Trigger registry (declared manifest of every OS trigger).** Today: 16 unique project tasks discovered ad hoc; one task double-registered `[Atlas §3.2; C-FLAGS 3]`. → Generated read-only registry: task, trigger, owner, heartbeat, receipt pointer. Why now: dead-job and duplicate decisions need a durable diff base. Depends: 2. Alternatives: parse on demand (no diff). Risk: none read-only.
4. **Dead-job retirement proposals.** Today: 13 dead jobs (12 disabled tasks + 1 rename-disabled startup entry) with no decision rows `[Atlas §6]`. → Per-item RETIRE/KEEP proposal with owner confirmation; nothing executed. Depends: 3. Alternatives: leave disabled (zombie + duplicate-registration precedent). Risk: none if proposal-only.
5. **Single-trigger consolidation (git sync; free-model refresh).** Today: two 5-min git-sync trigger paths + an executable legacy script; two free-model refresher surfaces with 7 catalog rotations in one day `[Atlas §6; C-FLAGS 1.1/1.2]`. → One declared trigger per authority; duplicates retired behind owner approval. Depends: 3, 4. Alternatives: keep dual paths (drift already produced 3 plugin copies). Risk: touching Task Scheduler requires an owner grant; do not self-execute.
6. **Duplicate/legacy artifact quarantine.** Today: 497 `.bak` files inside the live agents dir, 3 drifted plugin copies, 67 bit-identical clusters, executable legacy autosync `[Atlas §5, §6]`. → Quarantine plan (move, never delete; history preserved) after auditing the agent-loader glob. Depends: none (parallel). Alternatives: delete (rejected — loses provenance). Risk: a glob-based loader may ingest `.bak` ghosts today — audit first.
7. **Model-pin health reconciliation.** Today: 53 agent defs pinned; a dedicated dead-model pruning script exists; a recorded all-pin failure mode (clamped 2026-11-06) `[Atlas §3 F7; C-FLAGS 2.1]`. → Read-only pin-vs-catalog check on schedule, receipts into #1. Depends: 1. Alternatives: manual doctor runs. Risk: free-model flakiness makes noisy alerts; tune before trusting.
8. **Provider catalog single-writer enforcement.** Today: two refresher surfaces write overlapping catalog truth `[Atlas §6; C-FLAGS 1.2]`. → Depot job becomes the only writer; the scheduled CLI becomes a declared intent executed by the depot. Depends: 5. Alternatives: file-lock coordination (doesn't fix authority). Risk: catalog availability drift; keep the 3-day cadence.
9. **Typed intent envelope over both buses.** Today: free-form channel intents + typed bus (C1) `[Atlas §3.4, §3 F1]`. → One intent schema with authority request, evidence refs, budget, idempotency key. Depends: 1. Alternatives: keep free text (no verification possible downstream). Risk: schema adoption across seats; version it.
10. **Work-order runner promotion.** Today: work-order queue with 3 completed orders; dispatcher is a manual CLI; supervisor state dir empty `[Atlas §3 F5]`. → Wire the dispatcher through supervisor admission (C3) so runs are admitted, journaled, and receipted. Depends: 9. Alternatives: new orchestration engine (violates the single-authority matrix). Risk: first supervised runs must be watched; keep concurrency ceiling at declared caps.
11. **Admission authority unification.** Today: supervisor admission exists; Host worker-supervisor leases separately; bots-host supervisor is app-level `[Atlas §3 F1/F5]`. → HQ supervisor is THE admission authority; Host/bots keep domain leases but request admission. Depends: 10. Alternatives: distributed admission (contradicts the matrix). Risk: coupling across repos; stage after 10 proves stable.
12. **Simulate / dry-run.** Today: proof-command whitelist per order; pure budget module can cost without effects; no interpreter `[Atlas §3 lifecycle]`. → Plan interpreter producing a step DAG + effect forecast receipt (marked simulated). Depends: 10. Alternatives: full staging environment (heavier). Risk: simulation≠reality drift — forecasts only, never proof.
13. **Per-effect verification contracts.** Today: receipts + proof commands + health artifacts, not linked to intents `[Atlas §3 F8/F9]`. → Each effect class declares its verification (exit/hash/receipt schema) before it is admittable. Depends: 1, 11. Alternatives: post-hoc audit only (weaker). Risk: over-specification; start with the observed effect classes (git sync, file intake, provider send, schedule fire, model refresh, memory write).
14. **Repair-lane formalization.** Today: revive-lane source, attempt+1 idempotent resume doctrine, stale-lease → uncertain/blocked, quarantine for dead external lanes — none observed running `[Atlas §3 F9/F5]`. → Wire repair states to receipts (#13) and attention routing; keep "never auto-relaunch external lanes". Depends: 13. Alternatives: auto-restart (violates doctrine). Risk: low — formalizing existing code.
15. **Memory consolidation.** Today: 4+ memory/context stores (zcode memories, HQ memory, DCB, prompt ledger) `[Atlas §3 F8]`. → VIVIM memory stack as the single authority; others become sources; Core Memory stays owner-confirmed. Depends: 9. Alternatives: federated memory (duplicates truth; cost). Risk: migration of existing notes needs owner consent.
16. **Telemetry-informed budgets.** Today: timing spans + optimizer + timings table, not consulted by admission `[Atlas §3 F9]`. → Surface per-lane runtime/cost into admission decisions (C3). Depends: 13. Alternatives: external dashboards (not actionable). Risk: measurement must not gate data work (owner rule) — build alongside a feed that needs it.
17. **Mobile approval parity.** Today: mobile-relay bootstrap + phone projection cache with revision monotonicity; control path unexecuted `[Atlas §3 F10]`. → Approval requests render as typed intents (C1) on the paired phone via Host-owned contracts. Depends: 9, 13. Alternatives: desktop-only approvals (breaks paired-control goal). Risk: phone is control, never autonomous send; send-once discipline unchanged.
18. **Corpus/graph backfill of walk caps + 9 fan-out repos.** Today: caps 87/112 rows, ~1.4k skill corpus excluded, 26-row tail; fan-out repos have rows but no C-graph edges `[Atlas §8]`. → Extend walks + derive evidence-backed edges for the 9 repos. Depends: none. Alternatives: accept caps. Risk: volume only; no behavior change.
19. **Secret-fixture remediation.** Today: 8 test fixtures carry credential-shaped strings; copies secret-blocked, paths retained `[Atlas §7]`. → Refactor fixtures to synthetic tokens upstream; re-copy. Depends: none. Alternatives: keep blocked (permanent corpus hole). Risk: low.
20. **Runtime-migration grant boundary.** Today: Labs is a research/consolidation workspace; the Queen may not run anything live `[Mission; Atlas §1]`. → One owner grant document defining which W-stage components may execute where, replacing ad-hoc permission questions. Depends: all. Alternatives: per-task approvals forever (no compounding). Risk: none — it is a governance artifact.

## 5. Five future implementation lanes (scopes + acceptance tests)

- **L1 Evidence backbone** (#1, 2, 3): receipt envelope, probe runner, trigger registry. Acceptance: one probe sweep produces receipts for every declared automation; registry diff vs Task Scheduler query is clean; no write outside receipts.
- **L2 Governance & hygiene** (#4, 5, 6, 19): retirement proposals, single-trigger consolidation, quarantine, fixture remediation. Acceptance: every dead job has a decision row; exactly one active trigger per authority; 0 `.bak` files inside the live agents dir after quarantine; upstream fixtures pass the secret gate.
- **L3 Model authority** (#7, 8): pin reconciliation + catalog single-writer. Acceptance: pin-check receipts generated on cadence; exactly one writer to the catalog (provable from receipts); zero silent rotations.
- **L4 Queen core substrate** (#9, 10, 11, 12, 13, 14): intent envelope, runner promotion, admission unification, simulate, verification contracts, repair. Acceptance: one approved intent travels intent→design→(simulated)→approval→run→verified receipt end-to-end under supervision with zero unadmitted spawns; replay of the same intent is a no-op (idempotency proven by receipt).
- **L5 Experience & memory** (#15, 16, 17): memory consolidation, telemetry budgets, mobile approvals. Acceptance: one memory authority (writes only via the stack); admission decisions cite telemetry receipts; an approval issued on the paired phone admits exactly one bounded send with one receipt.

Dependency spine: L1 → L2 → L3 → L4 → L5; #18 and #19 run parallel anywhere.

## 6. Migration waves (PROPOSAL ONLY — nothing executes without the owner grant)

| Wave | Content | Rollback |
| --- | --- | --- |
| W1 Observe | #1 receipts, #2 probes, #3 registry | Stop probes; receipts are additive; registry is generated read-only |
| W2 Hygiene | #4 retirements, #5 trigger consolidation, #6 quarantine (owner-approved lists) | Restore quarantined paths from archive; re-register retired triggers from the registry manifest |
| W3 Substrate | #9 intents, #10 runner under supervision, #12 simulate, #13 verification | Dispatcher reverts to manual CLI; intent schema versioned; disable wiring flag |
| W4 Authority cutover | #11 admission unification, #8 catalog single-writer, #15 memory consolidation — **requires the runtime-migration grant (#20)** | Documented reversion to per-domain admission / depot dual-writer (with the conflict re-flagged) / source stores re-enabled |
| W5 Experience | #16 telemetry budgets, #17 mobile approvals | Feature-flag off; approvals revert to desktop-only |

Each wave ends with a GATE-REPORT-style board before the next begins; UNRUN never collapses to PASS.

## 7. NON-goals (explicit)

1. No direct APIs to provider webapps.
2. No Playwright/Puppeteer or equivalent browser drivers.
3. No GitHub Actions or other CI runners for the Queen.
4. No force-push, no reset, no autonomous repo deletion in any wave.
5. Labs stays a research and consolidation workspace until a separate runtime-migration grant exists (#20).
6. No second scheduler, graph, browser sender, or memory authority — ever.
7. No auto-resend of unknown external effects; no credential ingestion as knowledge; models and agents never grant permission.
8. No nested agent spawn without supervisor registration; concurrency ceiling stays 5 waves / 10 lanes / 50 total as a ceiling, not a target.

## 8. Self-grade (G7 input — graded honestly)

The blueprint satisfies the gate's structural asks (single-authority matrix, contracts, lifecycle with named gaps, 20 evidence-cited candidates, lanes, waves, non-goals) but inherits the scan's limits: no liveness probes ran (stage-table "run" rows rest on source + one snapshot), the supervisor state dir was empty at scan (the substrate has no observed run), and 9 fan-out repos have no graph edges yet. Grade: **PASS_WITH_LIMITS** — the design is evidence-anchored, not existence-invented; its weakest claim is the repair/learn stages, which map to sources never observed running.
