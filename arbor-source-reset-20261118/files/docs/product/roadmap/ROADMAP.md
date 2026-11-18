# ARBOR Product Roadmap

*Schema `arbor.product-roadmap/1` — synthesized inventory 2026-11-16; active acceleration queue R002, batch ACCEL-A. Machine truth is `roadmap.v1.json`. This markdown is a projection; the JSON file is authoritative.*

## Generated from
- proposal5Corpus (.integration/proposal5, .integration/proposal4-one, .integration/proposal1, .integration/proposal4, integration/)
- vivimLabs (vivim-labs/ tech stack, capability observatory, evidence factory, wave boosters, mobile extensions, personal-knowledge-lifecycle, zai, past-orphaned-work)
- hqIndex (hq/, repos_hq/ sync bus, migration evidence)
- gapSweep (30 gap-fill L2 items folded, cited by repo/path; PROPOSAL4-SIGIL and PROPOSAL4-KERNEL-SPEC already present and not re-added)

## Counts
- L0 pillars: 3 (L0-DEMO, L0-BUILD, L0-WININSTALL)
- L1 programs: 7 (L1-DEMO-LOOP, L1-P5-INTEGRATION, L1-P5-FOUNDATIONS, L1-VLANG-VEC, L1-VEC-ENGINES, L1-P2-LABS, L1-WININSTALL)
- L2 work packages: 97 rows shown in this view (statuses: implemented 1, in-progress 6, todo 36, candidate 46, deferred 1, gated 1, queued 6); gap-sweep items folded into synthesis inputs at synthesis, not all enumerated here
- Active build queue: A00-A08 (A00 gated, A01/A02 in-progress, A03 build-failed/gated on A00 until A00 is independently accepted; A04-A08 queued or gated per their dependencies; all verification UNRUN, owner acceptance PENDING)
- archivedBuildBatches: 1 (BATCH-1 executed, acceptance OPEN_FINDINGS, historical bookkeeping only)

## Workstream emphasis (owner, 2026-11-14)
Three workstreams: (1) ARBOR build — roadmap implementation track; (2) ARBOR demo — realtime fast-iteration design-and-acceptance gate over the build; (3) Windows install — ordinary workstream, NOT a gate, never a blocking criterion. Priority: demo >> build >> new queue batch >> demo > build (the self-driving acceleration loop). Labs gap-fill items are supporting context only and never enter the beta critical path.

## Product pillars (L0)

### L0-DEMO — ARBOR demo workstream — realtime fast-iteration design-and-acceptance gate

Status: **active**

Owner emphasis 2026-11-14 (supersedes 2026-11-13 banked): demo is the realtime fast-iteration design and acceptance gate over the build workstream; top priority goes to every component the capture and gap sweep found that accelerates the realtime automated loop — the self-driving cycle: demo surfaces design verdicts → build applies them → new queue batch is drawn → repeat (demo >> build >> new queue batch >> demo > build). GAP STATEMENT: no compiled work package from the four L2 compilers covers the demo workstream — no demo backlog, no automated acceptance-check harness, no queue-batch generator exists in the compiler outputs. Nearest loop accelerators found by the gap sweep (candidates, not compiled): harness/CONTRACT.md (approval-gated sendOnce, receipt-before-effect journal), vivim-ui PLAN.md three-tier agentic runtime, FINAL-WAVE Gate 5/6 matrices, DUAL-EXECUTION-PROTOCOL session lifecycle. Candidate program compiled at L1-DEMO-LOOP.

Sources: OWNER EMPHASIS (ask, 2026-11-14)

### L0-BUILD — ARBOR build workstream — roadmap implementation track

Status: **active**

The build workstream: proposal5 master integration beta program (G00-G13, all PROPOSED_UNRUN per .integration/proposal5/08-BETA-GATES.json:14-209), VEC/vlang NL-programming system convergence, and the vec-ui-engines 100-agent fleet as co-equal pillars. 128 L2 work packages below (98 compiled + 30 gap-fill folded from the gap sweep). Gate statuses honest per 08-BETA-GATES.json — UNRUN is not pass; source presence is not runtime admission; the build queue stays pending owner prioritization (priority null throughout). Conflicting root integration/beta-gates.json ACCEPTED claims are recorded in conflicts, not adopted.

Sources: OWNER EMPHASIS (ask, 2026-11-14) ; .integration/proposal5/08-BETA-GATES.json:14-209

### L0-WININSTALL — Windows install workstream — ordinary workstream, NOT a gate

Status: **active**

Owner emphasis 2026-11-14: the Windows+PWA install/release contract is an ordinary workstream, one of the three, never a G-gate or a blocking criterion. CONFLICT FLAGGED (read-only stage, gate doc untouched): G13 in 08-BETA-GATES.json:196-209 documents 'Windows package and PWA version compatible' as a BLOCKER gate — owner reclassification needs reconciliation at synthesis. Candidate program compiled at L1-WININSTALL.

Sources: OWNER EMPHASIS (ask, 2026-11-14) ; .integration/proposal5/08-BETA-GATES.json:196-209

## Programs (L1)

### L1-DEMO-LOOP — Demo↔build self-driving loop — candidate program, no compiled package

Status: **active**

Candidate L1 from owner emphasis + gap sweep (not from the four compilers). Loop-accelerating components found untracked by proposal5/HVI sources: harness/CONTRACT.md (byte-identical ARBOR/vivim-web-harness, sha1 b47fb0be4afc637242955373e409652dcb5776a5: lane ownership A/B/C :10-28, receipt-before-effect journal :52-54, approval-gated sendOnce :55-61, single Chrome profile loopback 9333, Lane B six provider profiles chatgpt/claude/deepseek/qwen/zai/gemini, Lane C wh CLI + wh serve + wh-mcp, seven safety acceptance tests :72-74 — no roadmap gate cites it, adjacent to G04/G07); vivim-ui PLAN.md (three-tier agentic runtime, Intent→Plan→JobSpec→RunMeta→BusEvent, lanes L0-L9, gates G0-G9, verified 57 pass/1 fail of 58, Rust bench 709,697 ev/s vs TS 246,336 ev/s = 2.9×, Gate 0 PASS 2026-11-02); VIVIM-VISUAL-RUNTIME/FINAL-WAVE/README.md (Gate 5 BLOCKED/Gate 6 BLOCKED, 23/23 commands exit 0, 1046 pass/0 fail, Wave 4B repair src/visual-runtime/device/wave4-phone-adapter.ts:227); VIVIM-VISUAL-RUNTIME/DUAL-EXECUTION-PROTOCOL.md (eight-state session lifecycle, fresh-head CAS publication); docs/SESSION-LEDGER.md (8-lane halt state, 7 carried defects incl P0 registry substring routing bug). Status candidate — queue pending owner prioritization.
R002: ACTIVE ACCELERATION FIRST. Work orders A00/A03/A05 connect demo feedback, scoped building, independent verification and requeue. Readiness depends on source/evidence, not candidate descriptions.

Sources: OWNER EMPHASIS (ask, 2026-11-14) ; harness/CONTRACT.md ; vivim-ui/VIVIM-VISUAL-RUNTIME/FINAL-WAVE/README.md ; vivim-ui/VIVIM-VISUAL-RUNTIME/DUAL-EXECUTION-PROTOCOL.md ; vivim-ui/docs/SESSION-LEDGER.md
Depends on: L0-DEMO

### L1-P5-INTEGRATION — Proposal5 master integration program — W0-W6, serial G, gates G00-G13, status ledger

Status: **todo**

One-time master integration program on integrate/proposal5-beta-r001; wave matrix at 09-EXECUTION-PLAYBOOK.md:31-37; playbook status NOT STARTED (:4). Gates G00-G13 all PROPOSED_UNRUN with observed_evidence null and owner_accepted false (08-BETA-GATES.json) — the conflicting root integration/beta-gates.json (UNBLOCKED, G01-G13 ACCEPTED, betaGo true, mergeCommit 7586f28) is treated as an unverified ledger claim contradicted by the same repo's release-manifest.json (BLOCKED_NOT_BUILT, betaGo false, mergeCommit dc2ae811, releaseAllowed false); reconciliation is an open owner decision (see conflicts). Compiled as P5-W0..W6, P5-SERIAL-G, P5-STATUS-LEDGER, G00..G11, G12-G13.

Sources: .integration/proposal5/09-EXECUTION-PLAYBOOK.md:4,31-37 ; .integration/proposal5/08-BETA-GATES.json:14-209
Depends on: L0-BUILD

### L1-P5-FOUNDATIONS — Proposal5 foundations pre-G program — PG waves, Rust proof tasks, pre-G exit, G sole-source union

Status: **todo**

Pre-G waves PG-00..PG-03 isolated, no product writes (IMPLEMENTATION-WAVES.md:54). Real Rust proof tasks FA-PG01-03 (generation swap, pinned lease, cancellation, typed DAG budget on synthetic isolated source) and FA-W2F1-02 (pure language/graph transform as new engine generation with CAS rollback) require cross-language build/cargo test actual exit codes per NATIVE-MACHINERY-AND-CONTRACTS.md:75-82 — V8 evaluation not sufficient. Runtime evidence exists for PG02 only: Node synthetic 20/20 PASS exit 0 (PG02-semantic-fabric/RUN-LEDGER.json:42-52); FA-PG01-03 cargo tests UNRUN (cargo not installed, PG01-native-abi/RUN-LEDGER.json:214-239); FA-W2F1-02 no Rust runtime evidence in corpus. Pre-G exit gate PREPARED_NOT_PROMOTED; G sole-source union NOT_EXECUTED with wave1G NOT_RUN_AS_CONFIRMED_BY_OWNER.

Sources: .integration/proposal5/foundations/IMPLEMENTATION-WAVES.md:54-130 ; .integration/proposal5/foundations/NATIVE-MACHINERY-AND-CONTRACTS.md:75-82
Depends on: L0-BUILD

### L1-VLANG-VEC — VLang/VEC system-convergence program — gates G-00..G-09, SCF step 1, VECX P0

Status: **todo**

NL-programming pillar. S0 source/authority lock OPEN: lockedImplementationBase=false (machine/source-pins.json:4), three same-day main/draft pins none frozen (:6-10), zero executable evidence (machine/program-status.json:22-24); no A-F work authorized from the SITREP alone (04-ROADMAP-AND-HANDOFF.md:4-6). Headline gate G-02 zero-LLM NL demo NOT_IMPLEMENTED. Sole in-progress item: G-04 three-schema UI fidelity PARTIAL (static world-visual-bridge.ts only; lane-b reports SCHEMA_BRIDGE_NOT_READY). All other gates NOT_IMPLEMENTED/UNRUN/OPEN. Compiled as G-00..G-09 plus merged SCF-ABF-VECX-P0 planning item.

Sources: .integration/proposal5/vlang/machine/program-status.json:22-24 ; .integration/proposal5/vlang/vec/system-convergence/machine/gates.json:11-78 ; .integration/proposal5/vlang/04-ROADMAP-AND-HANDOFF.md:4-6
Depends on: L0-BUILD

### L1-VEC-ENGINES — vec-ui-engines source foundation + 100-agent fleet + acceptance gates

Status: **todo**

packages/vec-ui-engines committed not promoted at 9fc1c08ea4804050a7dc7c1845842757835a07b6 on work/p5-vec-ui-engine-foundation-r001: 37 engine families, 461 typed data-only capabilities, all availability DESCRIBED_NOT_INSTALLED, effect none; product decision NO_GO (source foundation only). Fleet W0-W5 = 100 work orders (16 core A001-A016 + 37 engine-owner A017-A053 + 16 adapter A054-A069 + 16 intent A070-A085 + 12 QA A086-A097 + 3 lead A098-A100), contract SHA b9932fac199b59a97be228f9f584de4b1dd287b8, actualWorkersStarted 0, merge NOT_AUTHORIZED, testing UNRUN, no automatic merge/product GO from A098-A100. Acceptance gates G0-G9 all UNRUN_OR_PENDING, accepted false, gate decision NO_GO. Compiled as P5-VEC-* items.

Sources: .integration/proposal5/vec-ui-engines/machine/implementation-status.json:2-12,38-39 ; .integration/proposal5/vec-ui-engines/machine/100-agent-fleet.json ; .integration/proposal5/vec-ui-engines/machine/gates.json:4-63
Depends on: L0-BUILD

### L1-P2-LABS — Phase 2 Labs intake — E/C/H categories, decision matrix, promotion windows

Status: **todo**

Phase 2 Labs intake runs concurrently with W1 A-F on separate branches; promotion only via source-owned G: E easy-migration items after W1-G, C core items after W2-G on fresh branches from exact baseline, H high-value post-beta (low-cost source-backed exception if G records no critical-path extension). Default write scope .integration/proposal5/phase2/ plus tests/labs-intake/, documentation/evidence only. PREPARED_NOT_PROMOTED. Labs items never enter the beta critical path. labs-gap-fill candidates from the gap sweep are synthesis-stage input, not compiled here.

Sources: .integration/proposal5/10-PHASE2-LABS-INTAKE.md:12-48,62-79 ; .integration/proposal5/11-PHASE2-DECISION-MATRIX.md:3-55
Depends on: L0-BUILD, P5-W1

### L1-WININSTALL — Windows+PWA install/release contract — candidate, no compiled package

Status: **candidate**

Ordinary workstream per owner emphasis 2026-11-14 (never a G-gate or blocking criterion). No compiler output covers install packaging as a workstream — no compiled package exists; candidate list deferred to synthesis. Nearest compiled evidence: G13 documented BLOCKER gate (08-BETA-GATES.json:196-209, UNRUN — reclassification flagged in conflicts), root integration/release-manifest.json BLOCKED_NOT_BUILT with releaseAllowed false (:4,14-16,43,48,50), G06 native security gate (UNRUN, documented P0 cannot-defer), and web-harness ACTIVATION-GAPS.v1.json G01 MSI runtime unaccepted / G02 signed-update continuity / G03 seamless startup.

Sources: OWNER EMPHASIS (ask, 2026-11-14) ; integration/release-manifest.json:4,14-16,43,48,50 ; .integration/proposal5/08-BETA-GATES.json:196-209 ; vivim-web-harness/docs/ux/ACTIVATION-GAPS.v1.json
Depends on: L0-WININSTALL

## Work packages (L2)

| ID | Title | Status | Depends On |
|---|---|---|---|
| RDCB-BASELINE | RDCB — Reprogramming Dynamic Context Bundles (owner top priority) | implemented | WIKI-BASELINE |
| WIKI-BASELINE | Realtime wiki baseline — page 2, roadmap.v1.json + journal rendered live | in-progress | — |
| CHAT-BOX-BASELINE | Chat box baseline — page 1 pure chat, human+AI same endpoints | in-progress | — |
| FA-PG01-03 | Real Rust generation swap / pinned lease / cancellation proof | todo | — |
| FA-W2F1-02 | Pure language/graph transform as new engine generation with CAS rollback | todo | — |
| ADR-013-PWA | Mobile Direct PWA only first beta; APK deferred | deferred | — |
| PRD-U12 | Pair phone in Chrome as PWA: WebCrypto key, Host grant, revocable session | in-progress | — |
| PREG-G-INTEGRATION | Pre-G exit gate + G sole-source union GO/NO_GO (merged) | todo | FA-PG01-03, FA-W2F1-02 |
| P5-W0 | W0 Source Truth — native/Tauri, Host/World, Studio, HQ, mobile/TierFlow lanes; serial gate G0 source-lock | todo | — |
| P5-W1 | W1 Physical Landing — import all source trees into ARBOR; G1 closure and no-external-path proof | todo | P5-W0, G00 |
| P5-W2 | W2 Build and Internal Wiring — coherent internal vertical slice, patched native boundary (absorbs W2F0-EXIT) | todo | P5-W1, G01 |
| P5-W3 | W3 Beta Journeys — critical real source-backed flows, conditional feature classification | todo | P5-W2, G02 |
| P5-W4 | W4 Red-Team Verification — exact release blockers, NO false GO (absorbs W5F6-REDTEAM) | todo | P5-W3, G03 |
| P5-W5 | W5 Blocker Closure — domain regression fixes, accepted release candidate | todo | P5-W4, G04 |
| P5-W6 | W6 Beta Handoff — cutover with tested backups, rollback, single-writer handoff (absorbs W6F7-CUTOVER) | todo | P5-W5, G05 |
| P5-SERIAL-G | Serial gate G — sole owner of integration branch, GO/NO_GO/REPAIR_REQUIRED with exact SHAs | todo | P5-W0 |
| P5-STATUS-LEDGER | One-time integration status ledger — every source item in exactly one status | todo | P5-W0 |
| P2-LABS-INTAKE | Phase 2 Labs intake — E/C/H categories, 14-workstream decision matrix, promotion windows (merged) | todo | P5-W1 |
| G00 | G00 Freeze source import list — donor SHAs pinned, incumbent runtimes classified (BLOCKER) | todo | — |
| G01 | G01 Self-contained workspace — all source inside ARBOR, builds from ARBOR only (BLOCKER) | todo | P5-W1 |
| G02 | G02 Native HQKG — imported HQKG source and tests pass, one canonical graph crate (BLOCKER) | todo | P5-W2 |
| G03 | G03 HQ operations internal — in-repo index/search/context/wiki, no external HQ process (BLOCKER) | todo | — |
| G04 | G04 Host effects — one current Host catalog and authoritative journal (BLOCKER) | todo | — |
| G05 | G05 Node 0 safe path — local IME responsiveness, stale-revision refusal, Tab cannot cause effect (BLOCKER) | todo | P5-W5 |
| G06 | G06 Native security — patched supported Tauri v2 manifest/lock, exact IPC origin scopes (BLOCKER) | todo | P5-W6 |
| G07 | G07 Studio browser — real signed-in provider conversation through ARBOR (BLOCKER) | todo | P5-W6 |
| G08 | G08 Paired PWA — real physical phone pairs/revokes, Host authority on phone input (BLOCKER) | todo | P5-W6 |
| G09 | G09 TierFlow UI — selected TierFlow source builds internally, single source-backed publisher (BLOCKER) | todo | P5-W6 |
| G10 | G10 Capabilities and Labs — source SHA/status/owner/target per selected Labs component (BLOCKER) | todo | P5-W6 |
| G11 | G11 World and storage — export/backup/restore integrity, Rock Core protected (BLOCKER) | todo | P5-W6 |
| G12-G13 | G12+G13 — zero outside wiring; one beta release (BLOCKER, merged) | todo | P5-W6 |
| P5-VEC-FOUNDATION | vec-ui-engines source foundation — 37 engine families, 461 typed capabilities (absorbs P5-VEC-ENUM-CHOICES) | todo | — |
| P5-VEC-FLEET-W0W1 | Agent fleet waves W0-W1 — 16 core + 37 engine-owner work orders, contract SHA b9932fac | todo | P5-VEC-FOUNDATION |
| P5-VEC-FLEET-W2W5 | Agent fleet waves W2-W5 — adapters, NL/UI-editor/Wiki/command, QA/security, serial convergence | todo | P5-VEC-FLEET-W0W1 |
| P5-VEC-GATES | Acceptance gates G0-G9 — all UNRUN_OR_PENDING, product decision NO_GO | todo | P5-VEC-FLEET-W2W5 |
| G-00 | S0 source and authority lock (vlang) | todo | — |
| G-01 | Runtime SDK install descriptor | todo | G-00 |
| G-02 | VEC NL demo — 3 synthetic intents compile or refuse without LLM | todo | G-00 |
| G-03 | Native Rust ABI parity | todo | G-00 |
| G-04 | Three-schema UI fidelity (World/Visual/Phone) | in-progress | G-00 |
| G-05-G06 | Independent SV trust roots + capsule sandbox (merged) | todo | G-00 |
| G-07-G08-G09 | One Host publisher + revocation/device security + performance/low disk (merged) | todo | G-00 |
| SCF-ABF-VECX-P0 | VLang Step 1 SCF source foundation + VECX P0 opportunity shortlist (merged) | todo | G-00 |
| HARNESS-CONTRACT | harness/CONTRACT.md — web-harness effect boundary (lane ownership, receipt-before-effect, approval-gated sendOnce) | candidate | — |
| BETA-GATES-CONFLICT | Conflicting gate records: root integration/beta-gates.json vs .integration/proposal5/08-BETA-GATES.json | candidate | — |
| SOURCE-TRUTH-CAPABILITY | capability-spaces/source-truth.v1.json — G10 capability-truth inputs and G05 blocking evidence | candidate | — |
| HQ-MIGRATION-REPORT | hq/MIGRATION-REPORT.md + W01D-FOLLOWUP-REPORT.md — lane-level migration evidence | candidate | — |
| HQ-CAPABILITY-REGISTRY | hq/capabilities/registry/contracts.v1.json — proposed in-repo capability contracts | candidate | — |
| LABS-WAVE3-READINESS | vivim-labs/docs/WAVE-3-READINESS-HANDOFF.md — sync, readiness, reserved-path governance | candidate | — |
| LABS-ARCHITECTURE | vivim-labs/Architecture/README.md — One Graph, Many Engines architectural working record | candidate | — |
| LABS-PERSONALIZATION | vivim-labs/personalization/README.md — personalization research (Top 5 agentic algorithms) | candidate | — |
| LABS-RESEARCH-INTAKE | vivim-labs/research-intake/ — external research intake (Omar Sar0 X-post, freellmapi, zcode-free-model-toolkit) | candidate | — |
| LABS-KB-CONSOLIDATION | vivim-labs/KB-consolidation/ — KB/data-systems consolidation discovery scaffold | candidate | — |
| LABS-CONTEXT-ALGOS | vivim-labs/context/TOP-5-AGENTIC-CONTEXT-ALGORITHMS-2026.md — dynamic context bundle research | candidate | — |
| LABS-MEMORY-ALGOS | vivim-labs/memory/TOP-5-AGENTIC-MEMORY-ALGORITHMS-2026.md — memory research (Mem0, Graphiti, HippoRAG, A-MEM, Letta) | candidate | — |
| LABS-DOC-CONSOLIDATION | vivim-labs/documentation-consolidation/INVENTORY.json — 1234 doc consolidation snapshot | candidate | — |
| LABS-AGENT-WORKSPACES | vivim-labs/agents/workspaces/ — 11 named agent keeper workspaces | candidate | — |
| VIVIM-UI-PLAN | vivim-ui/PLAN.md — independent three-tier agentic runtime (Tier 1/2/3, lanes L0-L9, gates G0-G9) | candidate | — |
| VIVIM-UI-WAVES | vivim-ui/VIVIM-VISUAL-RUNTIME/WAVES.md + WAVE-3 + INTEGRATION-2-CLOSURE — TierFlow wave-convergence governance | candidate | — |
| VIVIM-UI-CAPABILITY-CONTRACT | vivim-ui/docs/CAPABILITY-CONTRACT.md — TierFlow security capability model (read|surface.write|program.write|exec, TIERFLOW_CAPABILITIES boot config) | candidate | — |
| VIVIM-UI-CORE-SPEC | vivim-ui/docs/UI-CORE-SPEC.md — UI core command/draft/publish/store contract (SQLite WAL, HTTP/WebSocket) | candidate | — |
| VIVIM-UI-CAPABILITY-GAP | vivim-ui/docs/UI-CORE-SPEC.md §3 vs src/security.ts:19 — ui.publish capability mismatch | candidate | — |
| VIVIM-UI-DUAL-EXECUTION | vivim-ui/VIVIM-VISUAL-RUNTIME/DUAL-EXECUTION-PROTOCOL.md — two-operator delivery protocol (8-state lifecycle, fresh-head CAS) | candidate | — |
| VIVIM-UI-SESSION-LEDGER | vivim-ui/docs/SESSION-LEDGER.md — halted specialist-lane ledger (8 lanes, 7 carried defects incl P0 registry substring routing bug) | candidate | — |
| VIVIM-UI-FINAL-WAVE | vivim-ui/VIVIM-VISUAL-RUNTIME/FINAL-WAVE/README.md — eight-lane final convergence wave (Gate 5/6 BLOCKED, 1046 pass, 6 gated patches) | candidate | — |
| VIVIM-LOCAL-PNG | vivim-local/ — two PNG images only, no text manifests | candidate | — |
| HQ-CATALOGUE-IDENTITY | hq/catalogue-identity/README.md — HVI B identity crosswalk (C4 port, arbor.hvi.identity/1 projection, local tests UNRUN) | candidate | — |
| HOST-RECOVERY | apps/vivim-host/recovery/README.md — Lane D Autonomous Diagnosis & Self-Healing (29 passing isolated Node tests) | candidate | — |
| CONTRACTS-HVI | contracts/hq-version-intelligence/README.md — HVI normative v1 JSON Schema contracts (schemas/v1/, versioned adapters) | candidate | — |
| STUDIO-UI-PREVIEW | studio/ui-preview/README.md — VIVIM UI preview pipeline (build.mjs/serve.mjs, DEMO DATA fixture, Vercel preview config) | candidate | — |
| MOBILE-RELAY-VEC | apps/mobile-relay/vec/README.md — VEC first pure compiler utility (compiler.mjs, ui-adapter.mjs, build-canvas fallback) | candidate | — |
| WEB-HARNESS-ROADMAP | vivim-web-harness/docs/ROADMAP.md + docs/roadmap.json — web-harness own roadmap authorities (gates A-D, W01-W09, 176-item task ledger) | candidate | — |
| WEB-HARNESS-ATOMIC-REGISTRY | vivim-web-harness/docs/capabilities/atomic-registry.json — 354 atomic capabilities with dependency map | candidate | — |
| WEB-HARNESS-SOURCE-DISPOSITIONS | vivim-web-harness/docs/capabilities/source-dispositions.v1.json — 5,659 tracked files across six repos | candidate | — |
| WEB-HARNESS-USER-JOURNEY | vivim-web-harness/docs/ux/USER-JOURNEY-ACTIVATION-GRAPH.v1.json — 120 steps J001.. across phases A-C | candidate | — |
| WEB-HARNESS-ACTIVATION-GAPS | vivim-web-harness/docs/ux/ACTIVATION-GAPS.v1.json — 39 gaps G01-G39 (17 P0, 20 P1, 2 P2) | candidate | — |
| WEB-HARNESS-POST-INTEGRATION-GAPS | vivim-web-harness/docs/ux/POST-INTEGRATION-FIVE-FOCUS-GAPS-2026-11-01.v1.json — 6 confirmed open, 5 acceptance-not-proven | candidate | — |
| WEB-HARNESS-CHAT-FIRST-MOBILE | vivim-web-harness/docs/ux/CHAT-FIRST-MOBILE-CONVERGENCE-2026-11-08.md — mobile UI convergence spec (10-item physical-phone protocol NOT RUN) | candidate | — |
| WEB-HARNESS-AGENTS-MD | vivim-web-harness/AGENTS.md — root agent instructions (9 effect boundaries, no provider APIs, send-once, TierFlow not second Host writer) | candidate | — |
| WEB-HARNESS-BASELINING-RUNBOOK | vivim-web-harness/docs/BASELINING-RUNBOOK.md — independent full-product baseline runbook (evidence labels SOURCE_VERIFIED/TEST_VERIFIED/LIVE_VERIFIED/BLOCKED/NOT_TESTED) | candidate | — |
| WEB-HARNESS-KNOWLEDGE | vivim-web-harness/docs/knowledge/ — KB design registries (44 types, 30 sources, 33 jobs) | candidate | — |
| WEB-HARNESS-DATA-INTELLIGENCE | vivim-web-harness/docs/data-intelligence/ — data-intelligence surface (ingress-registry, field-crosswalk, wiring-manifest) | candidate | — |
| WEB-HARNESS-STUDIO | vivim-web-harness/studio/README.md — Studio README product vision + capability status table (chatgpt UNVERIFIED) | candidate | — |
| PROPOSAL4-SIGIL | proposal4-one/02-SIGIL-TYPE-SYSTEM.md — SIGIL one-type law + CI codegen-drift gate (absent from proposal5) | candidate | — |
| PROPOSAL4-KERNEL-SPEC | proposal4-one/03-KERNEL-SPEC.md — kernel specification K1-K13 invariants + 8ms/50ms/800ms targets (absent from proposal5) | candidate | — |
| PROPOSAL1-COMPATIBILITY-NEGOTIATION | proposal1/04-RUNTIME-DATAFLOWS-AND-GATES.md §C — version/compatibility contract + negotiation + degraded mode (absent from proposal5) | candidate | — |
| PROPOSAL1-RELEASE-MANIFEST | proposal1/02-MIGRATION-MAP.md:75-76 — CI product release manifest with SBOM, signature, migration, recovery proof (absent from proposal5) | candidate | — |
| PROPOSAL1-MIGRATION-SCOPE | proposal1/05-REVIEW-QUESTIONS.md:45,59 — migration scope + owner consent for legacy personal data (absent from proposal5) | candidate | — |
| PROPOSAL4-ACCEPTANCE-EXAMPLES | proposal4/02-ONE-SYSTEM-TRUTH.md:184-195 — nine critical acceptance examples (absent from proposal5 B01-B12/G00-G13) | candidate | — |
| A00 | Truth and queue repair | gated | — |
| A01 | One-command live Host and acceptance runner | in-progress | — |
| A02 | Canonical stateful chat-to-Wiki projection | in-progress | — |
| A03 | Demo feedback-to-work-order compiler | build-failed | A00 |
| A04 | Narrow realtime NL to VLang/VEC visual compiler | queued | A02 |
| A05 | Authorized build, verify, demonstrate and requeue | queued | A01, A02, A03 |
| A06 | Incremental RDCB context with reuse receipts | queued | A01, A02 |
| A07 | Evidence-driven parallel worker execution | queued | A05, A06 |
| A08 | Physical Windows Host and paired PWA acceptance | queued | A02, A04 |
| HQKG-CONTROL-PLANE | hqkg integrated control plane + engine_network attachment — documented, referenced artifacts absent | candidate | — |
| STUDIO-DEVELOPER-PREVIEW | Studio developer-preview docs suite + aion-capture-studio CLI surface — referenced, mostly absent | candidate | — |
| HOST-DATA-INTELLIGENCE-ARBOR | ARBOR Host data-intelligence surface (catalog + journal-adapter) — implemented locally, crosswalk artifact absent | candidate | — |
| HQ-DEV-INTELLIGENCE | hq dev-intelligence package (arbor-hq CLI, SHA256 source-root contracts, FTS5 store) — staged, unexecuted | candidate | — |
| CORE-PROGRAMMABILITY-SPIKE | core/programmability bounded Rust native programmability spike — shipping gates undefined | candidate | — |
| CONTRACTS-INTEROP-V1 | contracts/interop/v1 Wave 1F READ/PROJECTION contract — no gate or cost covers it | candidate | — |
| NATIVE-SECURITY-W02-PREP | src-tauri NATIVE-SECURITY-W02-PREP.md — Lane A W02 security/build prep outside G06 scope | candidate | — |
| STUDIO-CAPTURE-FAMILIES | studio/families detection schemas + studio/profiles capture profiles — unverified, untracked | candidate | — |
| HVI-F-TEST-MATRIX | tests/hq-version-intelligence HVI F suite (T01-T24) — acceptance-test-matrix artifact absent | candidate | — |
| TIERFLOW-MISSION-D-AUDIT | packages/tierflow TierFlow/VIVIM Mission D convergence audit — referenced directory absent | candidate | — |
| LABS-TECH-STACK-OBSERVATORY | vivim-labs Tech Stack Observatory — pinned research overlay, BLOCKED_NOT_BUILT | candidate | — |
| LABS-CAPABILITY-OBSERVATORY | vivim-labs capability-observatory seed tracker — ledgers and blocked HQ export | candidate | — |
| LABS-EVIDENCE-FACTORY | vivim-labs ENGINEERING-EVIDENCE-FACTORY.md — 12 evidence factories, program intent only | candidate | — |
| LABS-WAVE-BOOSTERS | vivim-labs WAVE-BOOSTERS.md — per-wave boosters W1-W9 tied to product gates | candidate | — |
| LABS-MOBILE-EXTENSIONS | vivim-labs MOBILE-EXTENSIONS/Android browser extension research — hold gates, untracked | candidate | — |
| LABS-PERSONAL-KNOWLEDGE-LIFECYCLE | vivim-labs personal-knowledge-lifecycle — DCB/intake scaffold with Labs runtime | candidate | — |
| LABS-ZAI | vivim-labs zai four-repo convergence campaign — rounds, bundles, pinned SHAs | candidate | — |
| LABS-PAST-ORPHANED-WORK | vivim-labs past-orphaned-work recovery index — migration candidates need acceptance proof | candidate | — |
| VIVIM-UI-WORLD-MODEL | vivim-ui WORLD-MODEL-SPEC.md — world data model + command/draft/publish/store contract | candidate | — |
| REPOS-HQ-SYNC-BUS | repos_hq SYNC-BUS — agent coordination ledger, cursors, receipts | candidate | — |
| WEB-HARNESS-PRODUCT-BOUNDARY | vivim-web-harness README product scope + source-topology table — ownership untraced | candidate | — |
| WEB-HARNESS-HOST-M1 | vivim-web-harness Host README — M1 diagnostics, PDV01-PDV03 data-intelligence, UTC read-only slice | candidate | — |
| WEB-HARNESS-UTC-V1 | vivim-web-harness UNIVERSAL-TEXT-CONTROL-V1.md — text-driven control invariant, not live-accepted | candidate | — |
| WEB-HARNESS-STUDIO-ARCHITECTURE | vivim-web-harness studio ARCHITECTURE.md — Capture Studio governing vision + evidence boundary | candidate | — |
| WEB-HARNESS-AGENT-OPERATING-PROTOCOL | vivim-web-harness docs/agents OPERATING-PROTOCOL.md — agent claim/execute/communicate protocol | candidate | — |
| PROPOSAL4-ONE-DONOR-GRADUATION | proposal4-one donor-graduation protocol (behavior graduates, code does not) — absent from proposal5 | candidate | — |
| PROPOSAL1-LABS-GRADUATION | proposal1 Labs graduation design — invariants, dispositions, ledger statuses — absent from proposal5 | candidate | — |
| PROPOSAL4-INTEGRATION-WAVES | proposal4 integration phases P0-P6 with gates A-K and per-gate evidence — absent from proposal5 | candidate | — |
| PROPOSAL4-ONE-PRD-LEDGER | proposal4-one PRD ledger/evidence contract (Truth/Materialized/Proof, LED-01-LED-53) — absent from proposal5 | candidate | — |
| PROPOSAL4-ONE-ACCEPTANCE-EVIDENCE | proposal4-one acceptance/evidence contract O1-O10 + independent review — absent from proposal5 | candidate | — |

## Active build queue — Acceleration Batch A (R002)

**Owner-selected focus:** maximize future verified output, not standalone features. [Execution and acceptance contract](ACCELERATION-R002.md). Machine truth: `roadmap.v1.json`; this document is a projection. **A00–A02 are in-progress/gated with source landed but Node/Windows/browser tests UNRUN. A03 is build-failed and remains gated on independent acceptance of A00 before activation; A04–A08 remain queued or gated per their own dependencies. Independent acceptance PENDING for all nine.** Historical Batch-1 `executed` bookkeeping is not a release or end-to-end acceptance.

| Order | Item | Lane | Estimated acceleration | Hard dependencies | State |
|---:|---|:---:|---:|---|---|
| 1 | A00 — Truth and queue repair | B | 89/100 | — | gated / UNRUN |
| 2 | A01 — One-command live Host and acceptance runner | C | 95/100 | — | in-progress / UNRUN |
| 3 | A02 — Canonical stateful chat-to-Wiki projection | A | 96/100 | — | gated / built |
| 4 | A03 — Demo feedback-to-work-order compiler | B | 96/100 | A00 | gated / built |
| 5 | A04 — Narrow realtime NL to VLang/VEC visual compiler | A | 91/100 | A02 | gated / built |
| 6 | A05 — Authorized build, verify, demonstrate and requeue | B | 94/100 | A01, A02, A03 | gated / UNRUN |
| 7 | A06 — Incremental RDCB context with reuse receipts | C | 90/100 | A01, A02 | gated / built |
| 8 | A07 — Evidence-driven parallel worker execution | C | 81/100 | A05, A06 | failed / UNRUN |
| 9 | A08 — Physical Windows Host and paired PWA acceptance | A | 67/100 | A02, A04 | queued / UNRUN |

**Initial independently ready frontier:** A00 (truth/verification), A01 (one-command Host runner), A02 (stateful chat↔Wiki). All remaining work orders follow their hard dependencies. The roadmap's acceleration scoring formula is 30% downstream unlocks, 25% reuse, 20% feedback speed, 15% saved recurring reasoning and 10% simplicity; estimates do not substitute for receipts.

**Batch-1 history retained:** `archivedBuildBatches[0]` in the machine roadmap retains CHAT-BOX-BASELINE (11/11 narrow), WIKI-BASELINE (8/10, two failures), and RDCB-BASELINE (12/12 generator-only). Independent keeper acceptance remains UNRUN; the current Wiki and chat are not a proven stateful two-screen product.

**Release ledger conflict remains open:** `integration/beta-gates.json` reports UNBLOCKED; `.integration/proposal5/08-BETA-GATES.json` reports PROPOSED_UNRUN; `integration/release-manifest.json` is BLOCKED_NOT_BUILT. A00 will resolve claims against executed receipts, not rewrite their history. Windows installation remains an ordinary parallel workstream; APK deferred.

Read-only operator checks: `node scripts/roadmap/acceleration-queue.mjs --check` and `node scripts/roadmap/acceleration-queue.mjs --ready`. They do not run workers or perform effects.

## Appendix — Already delivered (do not rebuild)

Items with `implemented` status are excluded from `buildQueue` and from actionable sections; they appear only here. No rebuild is authorized without an owner decision.

- **RDCB-BASELINE** — RDCB — Reprogramming Dynamic Context Bundles (owner top priority). Evidence: docs/product/proposals/chat-wiki-baseline/. Verification field: GENERATOR_ONLY_12_OF_12; execution: REPORTED_EXECUTED.

Count: 1 implemented item (RDCB-BASELINE).

