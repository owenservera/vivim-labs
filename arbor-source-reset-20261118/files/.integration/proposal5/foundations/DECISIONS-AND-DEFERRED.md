# FA-ADR-001 — Foundational Architecture Decisions, Authority, Deferred Scope

**Evidence status:** decision *proposals* and owner directives, not unreviewable new production ownership. ARBOR Proposal 5's current original G has NOT run. This ledger cannot substitute for its frozen integration contract.

| ADR | Architectural decision | Grounded incumbent | Risk / implementation gate |
|---|---|---|---|
| ADR-001 | One ARBOR self-contained runtime, no donor processes | P5 06/09 execution; A–F import branches | G sole source union, no-donor test |
| ADR-002 | Reuse native Rust EngineRegistry/typed DAG rather than new graph/daemon | A `engine.rs`, `typed_dag.rs`, `scoped_graph.rs` | Rust generation/cancel/rollback test |
| ADR-003 | Treat native programmability as pure compute; never an effect executor | A `core/programmability/src/lib.rs` | Negative test for unapproved write/shell/send |
| ADR-004 | Host remains sole effect catalog, approval and receipt writer | B `intent-planner.mjs`, `dispatcher.mjs`, `journal.mjs` | Original-text replan and actual journal test |
| ADR-005 | HQ semantic wiki is a source-linked read/compilation service, not another policy engine | D registry, annotation v2, wiki, comment-wiki bridge | No comment→execution, no new wiki writer |
| ADR-006 | Preserve three distinct code/command program dialects and map by typed compatibility | D wiki-program; B command-program; A pure Program | No grammar conflation or implicit handler |
| ADR-007 | Source attestation must come from trusted HQ walker/bundle, not user-supplied digest | D arbor-service; SWC pure research currently accepts hash input | Runtime source-hash/owner test |
| ADR-008 | UI uses one input-revisioned interpretation trace; Quiet/Assisted/VIEW ALL are masks | E typed visual compiler; V-visual prototype | No second interpreter or false effect |
| ADR-009 | Native input uses bounded fan-out of pure independent subscribers, not one "brain" router | A engines/typed DAG; Labs N0 fabric design | Real Rust actor-verified parallel event proof |
| ADR-010 | Personal learning is proposal-first, owner-approved and revocable | B personal-rule-store; C DCB definitions | No model-created memory grant |
| ADR-011 | AutoWiki is ephemeral page projection over existing permitted HQ/Host reads | D wiki/KB; W-autowiki prototype | No page-per-focus storage explosion |
| ADR-012 | Rock Core protects a per-owner minimum and caps rebuildable bytes; original records never auto-evicted | HQ Rock pure policy; D current count-based wiki | ENOSPC/rollback/source recovery test |
| ADR-013 | Mobile is **Direct PWA only** for first beta; native APK is future optional program | E webmanifest/SW/browser signer, B direct-phone | G remove mandatory missing APK resource; real HTTPS phone acceptance |
| ADR-014 | Browser key proof and per-request Host authorization are source-owner B/E/F responsibilities | E browser-device-key; B enrollment/direct-phone | Full replay/revocation and static allowlist tests |
| ADR-015 | Do not introduce a generic installed plugin platform until the *first pure extension* is signed, versioned, bounded and rollback proven | C fixed-kind plugin registry; Labs plugin concept | One safe pure hook with no new executor |
| ADR-016 | Studio provider uses existing signed-in Chrome/profile and exact-send owner; no provider APIs/Playwright/Puppeteer | C Studio/harness and recipe service | Real exact-thread send/read and uncertain receipt |
| ADR-017 | User voice enters as native typed draft/proposal; Host alone accepts durable tasks | Labs voice pure workbench | Real audio/UI + Host receipt required |
| ADR-018 | User automation timer is a pure due predicate; Host owns durable rule/effect/receipt | A native engine; D HQ watcher is *not* user scheduler | No Queen/second timer daemon |
| ADR-019 | App builder first proof is reversible TierFlow UI draft, not arbitrary full-stack app | E generative-ui compiler, TierFlow World | Preview/review/commit/rollback real UI |
| ADR-020 | Every feature status must be substantiated by exact source, test, runtime and device evidence | F interop/QA + Labs read-only observatory | No source-only/synthetic = product PASS |
| ADR-021 | Original serial G/W2–W6 beta runbook retains absolute integration control | `.integration/proposal5/prompts/W1/G.md` and `09-EXECUTION-PLAYBOOK.md` | No pre-G branch merges features into G |
| ADR-022 | PWA offline caches cannot store/replay authenticated mutations or secrets | E `sw.js` shell-only behavior | Actual service worker network and reconnect test |

## Open questions requiring real post-G source, not speculative decisions

1. **Which ARBOR source and exact build head actually lands?** No G has run, so a W2 feature patch cannot be confidently targeted yet.
2. **What Rust module is authenticated by the final Tauri/Host actor gateway?** Scoped source IDs are structurally checked in A, not authenticated.
3. **Which 19 Host catalog action IDs have real runtime handler/receipt parity?** Importing JSON is insufficient.
4. **How does accepted visual state transport from A/B to E without the V-visual prototype becoming a second command compiler?** Need a single source/version adapter and actual tests.
5. **Which wiki materialization parts are rebuildable versus uniquely durable?** Existing HQ wiki-program and kernel stores have different retention rules.
6. **Are personal DCB read definitions and source lifetime authoritative in B or currently C's Studio metadata store?** Owner source and effect boundary must be confirmed before any memory/relationship write.
7. **What actual Windows/PWA p95 latency budgets can hot pack and graph reads meet?** No device benchmark is established.
8. **What modules in media, Chrome agent skills, voice and app builder are genuinely wired and complete?** F must finish exhaustive source/dependency census before reclassifying stretch tasks.
9. **Which capabilities may become data-only user-configurable without code changes?** Start with tested phrase synonyms, bounded project contexts, visual themes and pure transformations, not arbitrary app executors.
10. **How will code-comment v3 metadata be reviewed?** An opt-in parser and registry semantics must be owner-released; unsupported syntax is not runtime accepted.
11. **How are product-owned user automations scheduled durably?** Do not assume HQ Git watcher or Rust pure `Scheduler` is durable user scheduling.

## Decisions deferred, not abandoned

- Full native Android APK, native Keystore-only client, signing and Store distribution: **DEFERRED TO A NEW MOBILE PROGRAM**; PWA is first.
- Universal 43-facet VIEW ALL, every screen AutoWiki, all 100 predictive UX ideas: **DEFERRED BEYOND FIRST SOURCE-BACKED SLICE**.
- Full plugin marketplace, third-party native code loading, permission sandbox ecosystem: **DEFERRED** until one pure engine/manifest/deny/rollback proof.
- Complete personal ChatGPT ZIP history import + live DCB graph + automatic memory: **DEFERRED** until explicit Host consent, idempotency/erasure and storage proof.
- Arbitrary full-stack app builder, broad autonomous agent/Queen, parallel scheduler, cloud provider APIs: **NOT AN AUTOMATIC BETA TARGET**.
- Major HQ JS→Rust rewrite and Host effect journal port: **DEFERRED** until measured performance and invariant-preserving shadow parity prove value.
- Mobile-side offline independent command executor: **NOT IN SCOPE**; PWA can only provide provisional pure hints offline.

**Review rule:** only owner-accepted source and test evidence may transition an ADR from `PROPOSED` to `ACCEPTED_INTEGRATION`. Future prompts must carry a decision delta instead of silently rewriting the architecture.
