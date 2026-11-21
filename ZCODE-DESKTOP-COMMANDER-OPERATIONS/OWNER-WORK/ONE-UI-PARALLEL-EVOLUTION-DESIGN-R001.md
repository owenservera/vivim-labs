# VIVIM One-UI Parallel Evolution Engine — First-Pass Design R001
Date: 2026-11-21. Owner: ChatGPT orchestration + ZCode local workflow runtime. Status: DESIGN, NOT LAUNCHED. Product: the **single** First Contact text-box PWA. This is the first of several design passes, NOT an authorization to run 100 browser windows, modify production data, make billable model calls, or duplicate the customer UI.

## A. Source-grounded reconciliation (read on 2026-11-21)
- Working Windows First Contact PWA, source ARBOR main cf2e5c8da0d5f46f17a5de0cc24ab838778e60c0: Edge app, authenticated Tailnet HTTPS origin, real composer typing "Roadmap" -> 4 Wiki results, "Main" -> 1, both rev1. This proves a human Windows happy path, **not** Android, real agent-originated editing, concurrency, or arbitrary semantic correctness.
- ARBOR local ZCode saved definitions: 15 *.dwf.ts, including arbor-life, arbor-product, arbor-area-a..e, arbor-build, arbor-batch-engine, arbor-demo, arbor-roadmap, arbor-chatgpt-bridge, arbor-integrate, arbor-integration, and historical arbor-batch1. docs/WORKFLOWS.md describes role/queue behavior; saved definitions are locally ignored by Git and subject to uncommitted drift.
- arbor-life is the canonical USE -> OBSERVE -> DIAGNOSE -> FIX -> VERIFY -> REPLAY/EVOLVE loop. Earlier 3-page /chat /wiki /demo workflow assumptions are historical; the owner has overridden them: there is now only one customer-facing First Contact entry point. Audit and adapt the actual DWF, do not simply launch it unchanged. Its current source contains hard-coded port 43187 and legacy /api/state /api/mutate /demo route checks, potentially unsafe on the now-personal production Host.
- Local Continuous Improvement Fleet R004: 10 specialist lanes, 4 dependency waves, offline Rust graph validator, 15/15 focused Host tests. Its manifest explicitly states DESIGN_NOT_RUNNING, autoLaunch=false, model admission initially zero. Two previous read-only ZCode worker attempts received OpenRouter 429 and did not prove child fanout. A workflow file is not a running worker. The local 2026-11-21 SITREP confirms ZCode 0.18.0 + 15 saved workflow definitions, but not current active runs.
- Existing Rust kernel at ARBOR core/programmability/src/product_kernel.rs already dispatches first-contact, wiki-source-stamp/search/transition/delta, command-program, vec-admission, agent-composer, visual and semantic operations. Core Rust files first_contact.rs, wiki_read.rs, command_program.rs, vec.rs, native/hqkg exist. Inert compute != Host approval/permission.
- Existing Labs designs: VLANG-CANONICAL-PROTOCOL-R000 (research scaffold, no runtime proof), ARBOR-VLANG-UPGRADE U1..U7 (design/prep), OWNER-WORK/AGENT-NATIVE-CLOSED-LOOP-DESIGN.md (agent edit -> observation/ACK, not fully accepted), OWNER-WORK/BEHAVIOR-TO-DETERMINISM.md (episode -> candidate -> shadow -> canary -> promotion, not deployed), DESIGN/RUST-PARITY.md (active JS-to-Rust inventory and parity migration not executed), .private/VLANG-PWA-RUNTIME-INTEGRATION (source-aware Wiki enhancement proposal, not proof of production VLANG integration).
- Dedicated isolated Host helper scripts/life/scratch-host.mjs exists, but targets legacy /api/mutate and /api/state journeys. A hardening harness must additionally prove **the actual current Direct First Contact Wiki route with authenticated test identities**. Never claim legacy route tests establish single-UI parity.

## B. Product principle and hypothesis
ONE product UI, MANY ephemeral experiment states. "Ten/100 boxes" means logical independently revisioned composer+Wiki sessions, not 100 copies of the application UI or 100 always-on browser processes. Same exact Rust/WASM kernel, Host contract, and DOM adapter are exercised with varied state, text, device, grant, locale, concurrency, failure and expected behavior. All local trial scenarios are temporary; only verified failure evidence and reviewed corpus improvements persist.
Hypothesis: coverage-guided, deterministic seeded exploration + property-based mutation + independent user replay outperforms naive random clicking or a single AI-agent happy-path. Measure verified user-visible product improvement and false-action prevention, not generated tests, number of agents, or model tokens.

## C. Three execution planes (different concurrency counts)
1. PURE RUST TEST PLANE, 100-10,000 *logical* cases per suite, bounded memory/CPU: invoke canonical Rust product kernel natively, or existing WASM ABI for parity. Seeded scenarios and shrinking, no personal data, no network, no Host effects. VLANG/SCF contract checks, round-trip source identity, intent/proposal refusal, stale revision, VEC plan invariants. This is where high trial counts are cheap.
2. ISOLATED HOST/PWA PLANE, start 1-2 and ramp 4 -> 10 only after resource and isolation proof: actual browser control of the existing First Contact interface, supported native ZCode browser owner only. Each slot has isolated scratch data root/port/identity/grants, bounded lifecycle/retention. If the browser owner cannot provide distinct secure sessions, limit the plane to its supported concurrency; do not invent Playwright/Puppeteer or synthetic success. Never use production port 43187 or shared personal journal for test mutations.
3. REAL DEVICE PLANE, 1 Windows and 1 authorized Android canary when available: unchanged Tailnet URL and real UI, screenshots/DOM/visible Wiki revision and ACK. Explicitly distinguish non-mutating production smoke checks from owner-authorized effectful tests. Cannot claim 100 real UI sessions merely because 100 Rust cases pass.

## D. Existing DWF workflows and exact planned jobs
**Controller**: ChatGPT coordinates scopes, selects approved work, authors implementation in cloud and pushes ARBOR main as primary coder. ZCode orchestrates bounded local read-only analysis, scenario execution and local tests through existing DWF mechanisms when session/permissions/budget allow. Remote Desktop Commander handles inspection, launch, screenshots and receipts; not unapproved source editing. Existing roadmap keeper remains the only queue flip writer.
| Existing workflow | New role in this program | Inputs | Exact output |
|---|---|---|---|
| arbor-life | canonical outer improvement loop; **adapt legacy paths**, bounded 2 rounds | target PWA goal, source SHA, failures, budget | source-pinned USE/OBSERVE/FIX/VERIFY/replay outcome |
| arbor-product | existing product work coordinator; fixes restricted to cloud integration handoff | accepted candidate, file scope | scoped implementation work order (not an automatic local patch) |
| arbor-area-a | Rust/HQKG/VLANG semantics | source + evidence refs | semantic identity/provenance findings |
| arbor-area-b | Host grant/journal authority | test Host grant matrix | safety/refusal/receipt findings |
| arbor-area-c | Studio/agent interaction contract | actor/session/edit candidate | proposal/draft/ACK behavior report |
| arbor-area-d | First Contact PWA/WASM/visual parity | composer sequence + UI variant | actual UI render and input-event findings |
| arbor-area-e | build, packaging, CI and test runner | commit SHA + manifest | reproducible build/test result |
| arbor-build | consume eligible approved queue item (fallback to life; not co-own same item) | work item | scoped candidate/test receipts |
| arbor-batch-engine | independent verify and evidence aggregation | candidate + test evidence | verdict, no self-certification |
| arbor-integrate / arbor-integration | one writer convergence (disposition after collision audit) | accepted artifacts | serial integration/rollback evidence |
| arbor-roadmap | update only on owner-approved re-baseline, not test iteration | canonical queue | roadmap proposals, keeper owns flips |
| arbor-demo | historical developer/demo walkthrough: **not a second customer UI** | acceptance scenario | optional internal proof, no customer dashboard |
| arbor-chatgpt-bridge | strategic clarification/handoff only, not runtime trial transport | escalated bounded question | explicit correlation/receipt |
| arbor-batch1 | historical reference only, never relaunch | none | no active job |
**Specialist fleet lanes A-J** map onto these roles, not a second coordinator: A semantic-source, B rust-compiler, C command-language, D wiki-language, E visual-language, F knowledge-context, G experience-observer, H host-convergence one writer, I independent adversarial verifier, J verified-learning proposal recorder. One writer, separate independent verifier, no competing roadmap or graphs.

## E. Scenario engine — controlled stochastic exploration
Canonical proposed input (not implemented API):
\`\`\`json
{
  "schema":"vivim.experiment-scenario/1",
  "scenarioId":"wiki.rapid-edit.001",
  "seed":104729,
  "arborSha":"PIN_CURRENT_SHA",
  "kernelAbi":"PIN_CURRENT_ABI",
  "goal":"type-and-understand",
  "actor":"test-human",
  "session":"ephemeral-isolated",
  "sourceSnapshot":{"revision":1,"digest":"PIN_REAL_TEST_SOURCE_DIGEST"},
  "inputTimeline":[{"atMs":0,"operation":"insert","text":"Roadmap"},{"atMs":190,"operation":"replace","text":"Main"}],
  "perturbations":["response-reorder","revision-advance"],
  "expectedProperties":["latest-query-wins","only-current-source-visible","zero-host-effects"],
  "risk":"R0-read-only",
  "capture":"redacted-observation",
  "budgets":{"maxMs":10000,"maxMemoryMiB":64}
}
\`\`\`
Exact schema, budgets and clock mechanics need Rust implementation and source migration review; this sample is design only, not a real valid current API. Use source-grounded corpus seeds and explicit scenario labels. Families: text normalization, locale & IME, typos, ambiguity/negation, long/pasted input, rapid edit/delete, source revision race, stale/out-of-order fetch, revoked/missing grant, human+agent collision, offline/reconnect, Wiki provenance, accessibility, model fallback/cost. User data must not be copied into test corpora without scope/consent.
Run deterministic baseline cases first, then seeded mutation/property-based/metamorphic variation (same meaning phrased differently should produce equivalent inert intent; negation must not become permission; reordered response cannot overwrite newest result). Coverage-guided sampling increases weight of observed failures and understudied interactions. A separate curated/real-world holdout must remain unseen by proposal agents; no acceptance on synthetic-only success.

## F. One experiment's lifecycle
(1) Intake and pin live main SHA + Rust/WASM digest + Host/test catalog & permission policy revision.
(2) Generate test scenarios from consented reviewed corpus and seeded mutations; dedupe hashes; assign scenario IDs.
(3) Classify risk. Pure, no-effect Rust trials run in high volume; PWA/UI and any test effects require separately isolated real Host instances and explicit test grants.
(4) ZCode USE worker interacts with one real UI when available. Pure runner returns deterministic structured results. Neither may read implementation hints before blind user journey.
(5) OBSERVE collects inputs by digest/redacted ref, DOM/visible Wiki revision, causal ACK, Host receipt, error, resource/latency. Do not treat screenshot alone as source-backed evidence.
(6) DIAGNOSE collapses a failing scenario to minimal reproducer, distinguishing product defect, invalid test, auth failure, model outage, stale source and environment.
(7) Propose code/test change. ChatGPT cloud source integration owns product patch in agreed exact paths, on main; ZCode does not write parallel application source by default.
(8) VERIFY: Rust unit + WASM/native parity + Host isolated tests + real UI fresh-session replay + independent adversarial negatives. The original scenario and holdout remain independent of the candidate generator.
(9) H only promotes green evidence; existing keeper updates canonical queue. The previous working First Contact build remains rollback target until acceptance.
(10) J turns repeated verified failures into **proposed** VLANG vocabulary, parser/IR rules or precompiled Rust capability improvements; no self-grant, no automatic executable code from comments.

## G. Test invariants and scoring
HARD VETO (never average away): no mutation of production Host by experiments; no unauthorized action, stale approval, duplicate effect, leaked private source, fabricated provenance, invalid source identity, or older Wiki response overriding newer; all effectful tests need valid independent approved grants.
Truth ladder: SOURCE -> CONTRACT TEST -> RUST/WASM PARITY -> ISOLATED HOST -> VISIBLE UI -> INDEPENDENT FRESH USER -> REAL DEVICE. Each is individually PASS/FAIL/BLOCKED/UNRUN. Never equate an agent-written PASS string with receipt.
Optimization metrics measured by corpus class: supported intent precision/recall and false-action count, grounded Wiki match precision, no-result utility, p50/p95 time-to-visible Wiki, stale/revocation rejection, simulated user goal completion, rust-native/WASM parity, model calls per verified fix, CPU/RAM/disk, duplicate test rate, improvement throughput/hour. No promised numerical performance until baseline.
Multi-objective selection: safety gates first; then maximize new verified failures discovered, regressions prevented, real user task success and semantic coverage, subject to explicit cost/resource limits.

## H. Capacity and isolation roadmap (targets, not measured)
- R0: inventory/migrate critical active JS semantics to Rust incrementally; verify one real Windows First Contact baseline and existing 50 focused suite; do not break UI.
- R1: 100-1000 *pure seeded Rust cases*, 1 isolated Host, 1 actual UI user-agent journey, 1 fresh verifier. Crucial proof: exact isolated Direct phone auth behavior rather than old demo route.
- R2: 4 isolated UI logical sessions if supported, 2 independent ZCode agent children observed with distinct session IDs, no conflicts or real Host edits; provider budget/429 gate open only after health.
- R3: 10 ephemeral Host/PWA logical sessions with resource governor and deterministic fixtures, not necessarily 10 visual browser windows; measured CPU/RAM/disk/latency and teardown guarantee.
- R4: 100+ **logical Rust scenario sessions** per suite, millions only when measured inexpensive; real UI browser concurrency remains small and hardware-driven. Optional overnight campaigns need separately authorized scheduling and cost ceilings.
Do not scale based on agent count. First prove that 2 genuinely distinct sessions can run and clean up. An experiment batch is finite and terminates. Stop after 2 no-progress repair rounds or provider 429 and never silently move to paid models.

## I. Rust-first migration order
1. Map *active runtime call chain* PWA app.js -> packages/vec-ui-engines rust-wasm.mjs -> Rust kernel -> Host direct-phone.mjs -> product journal. A Rust file existing does not prove it owns the live behavior.
2. First migrate browser-side Wiki response state machine (debounce policy, query version, high-water mark, no-match/refusal classification) to canonical Rust/WASM pure reducer with minimal TS7+ DOM/fetch/IME adapter. Maintain first-contact UI parity and current source/grant checks.
3. Bind typed VLANG intent classification, ambiguity and source provenance into the **same** Rust program compiler/SCF, not a second language; keep authority separate.
4. Migrate eligible deterministic Host Node semantics to Rust behind identical Host policy and receipt rules, section by section, with differential test vectors.
5. Retire superseded JS logic only after source, native/WASM, Host and visible PWA acceptance; preserve rollback. No browser DOM rewritten in Rust if it adds needless glue.

## J. Multi-pass design sequence / next decisions
DESIGN PASS 1 (this document): reconcile workflows, execution planes, roles, safety boundary, trial model and proposed topology. **No launch, no product edit.**
DESIGN PASS 2: exact current DWF edit diff for arbor-life and area D, authoritative ZCode child-session invocation, scratch Direct Host authentication, Rust experiment scenario schema/API, capacity calibration, concrete scenario catalog with seeded oracles and shrinking. Do not edit live defs before reviewed diff.
DESIGN PASS 3: VLANG/SCF/SV/HQKG integration and Rust migration ledger; semantics/evidence/permissions crosswalk, 2-TAB and intent training/evaluation, accepted counterexample and learning-promotion thresholds.
DESIGN PASS 4: executable no-effect one/two session pilot, resource instrumentation, real UI screenshots, independent verifier and receipts; only then 4/10 logical slots.
DESIGN PASS 5: controlled 100-case campaign, regression mining, one user-visible Rust improvement, cloud source patch, immutable build/deploy and real Windows + Android acceptance. Always confirm current active agent claims and source SHA before changes.
Key unresolved questions: exact ZCode DWF Create/Amend saved-workflow API live proof, whether native control-browser can isolate 2+ sessions, scratch Direct phone owner test identity without bypassing auth, capacity/available disk, and migration overlap with existing first-contact/VLANG active claims. Unknown means blocked, not "probably working."

## K. Design verdict
Use **existing arbor-life as the single outer lifecycle**, and **existing 10 specialist lanes as roles**, not new standalone agent fleets or UI shells. Build only the missing Rust experiment reducer and isolated real-PWA adapter. Treat 100 as a number of **seeded scenarios**, never 100 privileged UI agents or 100 unisolated production Host sessions. The same First Contact UI and exact existing Rust kernel are always under test.
