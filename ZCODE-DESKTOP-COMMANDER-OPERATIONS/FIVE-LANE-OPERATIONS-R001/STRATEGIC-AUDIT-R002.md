# Strategic audit R002 — five mission lanes over the existing ZCode/ARBOR stack

**Audit:** 2026-11-23, Europe/Madrid. **Verdict:** retain five *product-delivery missions*, correct task coverage, constrain ZCode workflow admission. This is a source-grounded design/disposition and documentation update; **not** a new product executor, scheduler, model router, agent fleet, work tracker or release.

## 1. Verified sources and moving-head truth

Inspected the actual local `C:\0-BlackBoxProject-0\ARBOR\.zcode\workflows` definitions, including A–E, `arbor-integrate`, `arbor-life`, `arbor-product`, `arbor-chatgpt-bridge`, and the three narrow verifier definitions; read Labs [WORKFLOWS-AGENTS](../WORKFLOWS-AGENTS.md), [COMMS-AND-THREADS](../COMMS-AND-THREADS.md), [MODEL routing](../MODELS-AND-PROVIDERS.md), [shared claims](../TASK-CLAIMS.md), [Oracle role](../ORACLE/AGENTS.md), [Dev role](../CHATGPT-DEV/AGENTS.md), [R002/R003 loop designs](../OWNER-WORK/ONE-UI-TRACKER-LOOPS-R003-EXECUTION-SPEC.md), ARBOR [canonical UI closure](https://github.com/owenservera/ARBOR/blob/main/docs/product/CANONICAL-UI-SURFACE-CLOSURE.md), [#33](https://github.com/owenservera/ARBOR/issues/33), [#32](https://github.com/owenservera/ARBOR/issues/32), [#31](https://github.com/owenservera/ARBOR/issues/31), and current Dev source evidence.

- The latest canonical UI policy (Nov 22/23) is **native Windows GPUI + the sole established Android PWA**, sharing existing Host/VEC semantics. Historical Nov 21 Windows Edge/PWA-only decisions and the ten-loop First Contact plan are **design inputs**, not overrides of the newer native-GPUI directive.
- Actual ZCode CLI on the PC: **0.18.0**, 18 current `.dwf.ts` definitions. App/runtime processes do not establish model-workflow execution. The read-only `dwf_run` auditor completed at **2026-11-23T20:45:53Z**, exit 0: **361** history rows (102 completed, 26 failed, 231 cancelled, 2 labelled running), **0 recent live owner-process candidates**; both running labels orphaned. No five-model or six-model fleet proved alive or free.
- An existing independent **Windows native VLANG deterministic observer** is scheduled. The last complete matching report previously read was **BLOCKED** at `a34a7f5a` with VLANG/safety FAIL/TIMEOUT and separately AutoWiki 12/12, visual 21/21 PASS; a newer source/reporter cannot be labelled full PASS without a new receipt.
- Current ARBOR moves while agents work. A recent Dev SITREP reported a Studio API-route recovery and current source `f147ddf7`; live Studio captures idle and signed-in provider browser *not running*, with no genuine new ChatGPT send/reply or Android acceptance. A later observed local source `f37987d71a59` adds Studio send-control regression tests. **Re-read GitHub main when a lane starts.**
- Labs `TASK-CLAIMS.md` is a file/file-scope ownership overlay, not a backlog. Local Labs may have unrelated uncommitted changes; this R002 update uses GitHub files in this package only and does not reset or overwrite the local checkout.

## 2. Why five missions are correct

The preexisting `arbor-area-a`…`e` workflows are **implementation component lanes**: (A) Rust/HQ, (B) Host/Harness, (C) Studio, (D) PWA/TierFlow, (E) packaging/foundations. The new L01–L05 are **end-to-end product and evidence missions** across these components. This is deliberately not one-to-one, and it is the right separation: a ChatGPT reply becomes useful only through Studio → Host → GPUI/PWA, not by completing Studio source alone.

The former ten-worker plan is *one fungible bounded improvement job* repeated over existing tracker tasks: observe real UI → reproduce one problem → primary coder fixes → independent replay → original tracker check-in. It can run **inside** the five missions once current model/browser/isolation grants work. It is not a sixth team, a second tracker, or a license to run `arbor-life` unchanged.

**Single integrator/control plane, not a sixth workstream:** ChatGPT owner-facing orchestrator + existing #33, sole Dev ARBOR source writer, Labs claims for file collisions, Oracle for provenance/outcome integrity, Planning for bundles, ZCode for bounded independent tests and Desktop Commander for local evidence. The historic `arbor-integrate.dwf.ts` **is not** a safe substitute: its present definition performs local commits and merges under older assumptions.

## 3. Exact primary task ownership — all 64 open IDs once, zero duplication

These are *delivery bookkeeping owners*, not permissions for an unclaimed agent to edit someone else's product source.

| Primary lane | Existing open task IDs | Acceptance gate and cross-lane dependency |
|---|---|---|
| **L01 Provider conversations and capture** | **A01–A16, C01–C03** (19) | Signed-in Studio/browser identity, composer + send + reply + Host; L02 consumes the verified result for native/phone. C01–C03 cover authenticated capture, streaming, dialogs, attachments and interaction proofs. |
| **L02 Native GPUI, PWA, native capture parity and release** | **S02–S14, U01–U04, C04–C05** (19) | Shared GPUI/PWA/Host source revision and physical device proof; C04–C05 port capture/interaction to approved Rust/native surfaces. L04 produces U10 semantics. L02 owns controlled production promotion; L05 independently audits it. |
| **L03 Authorized Work, Oracle and AutoWiki** | **U05, O01–O06** (7) | Existing PY-001 Dev claim is a cross-cutting dependency, **not another #33 task**. Real reviewed Work producer, owner-authorized initialization, persisted CAS, authorized knowledge + provenance + Wiki. U07 is *not* L03's source claim. |
| **L04 Deterministic VLANG, visual feedback and gestures** | **U06–U11, G02–G05** (10) | U07 = VLANG plan → Host approval/execution receipt; U10 = semantic VEC/TierFlow surface. L02 builds native/phone adapters; L03 consumes approved Work semantics. |
| **L05 Verification, ZCode, CI and evidence integrity** | **Z02–Z04, Z07, R01, R03–R04, R06–R07** (9) | Live workflow admission, CI runner, WIP preservation, cross-device evidence; no production deployment or source-write authority. Historical checked Z01/Z05/Z06/R02/R05 are not reopened. |

**Crosswalk check:** 19 + 19 + 7 + 10 + 9 = **64**. Historical already-checked S01 and G01 are not requeued. All 71 canonical IDs remain defined solely in ARBOR #33; this is a no-duplicate lane mapping.

## 4. Workflow admission matrix (inspect before invocation)

| Existing .zcode/workflows asset | Strategic role | Launch disposition |
|---|---|---|
| `arbor-product-verification-parallel.dwf.ts` | Three independent source-SHA-pinned Node/PWA/Host/Studio deterministic suites; `integration/scripts/pwa-run-now.mjs` uses Node syntax/tests, not a background Host replacement | **CONDITIONAL TEST-ONLY**: fresh clean isolated source SHA, review exact invoked test effects/temporary data, no provider/account or production processes, collect real run ID/exit |
| `arbor-oracle-corpus-verify.dwf.ts` | Current Host/Studio onboarding tests, no account extraction | **CONDITIONAL TEST-ONLY**: clean exact-SHA source + permitted disposable test fixture; no real corpus read |
| `arbor-vlang-iterative-observers.dwf.ts` | Four independent read-only scouts + adversarial verifier after Node baseline | **CONDITIONAL MODEL-ASSISTED**: exact matching source, deterministic baseline green, actual model cost/health/grant admission, bounded workers and run IDs; cannot assume prompt-level read-only means zero capability |
| `arbor-area-a.dwf.ts` … `arbor-area-e.dwf.ts` | Historical A–E component-specific implement-and-verify/ledger progression; older NIM pins/legacy source tracker | **REFERENCE ONLY** until rewritten/reviewed for claimed files, cloud-source ownership, no live writes and current route admission |
| `arbor-integrate.dwf.ts`, `arbor-integration.dwf.ts` | Historical local integration/merge process; includes source/ledger writes, `git commit` | **REFERENCE ONLY**; no direct automatic integration on live main |
| `arbor-chatgpt-bridge.dwf.ts` | One ChatGPT turn through Studio transport; current code contains an explicit `studio.mjs chat ask --site chatgpt` instruction | **NO AUTOMATIC INVOCATION** until uncertain prior request is reconciled, owner-approved fresh prompt, correct thread, browser owner and exact send receipt; never replay |
| `arbor-life.dwf.ts` | Historical full product evolution loop; includes explicit *kill/restart production Host 43187* assistant instructions | **DO NOT LAUNCH UNMODIFIED**, even `mode=audit`, until scope/permissions/scratch isolation verified and restart/source-writing paths removed or provably denied |
| `arbor-product.dwf.ts`, `arbor-build.dwf.ts`, `arbor-roadmap.dwf.ts`, `arbor-batch-engine.dwf.ts`, `arbor-demo.dwf.ts` | Older writer/demo/queue flows, some NIM and OpenRouter hard-coded providers, roadmap flips, Host effects/approvals | **REVIEW ONLY**; do not treat these as bounded source observers |
| `arbor-batch1.dwf.ts` | Historical settled batch 1 | **ARCHIVE / NEVER RELAUNCH** |

**Why this matters:** a `world.run`, `agent`, `model`, or workflow's self-description does not guarantee current permissions, no effects, free quotas, thread custody or receipt quality. Operator must inspect the *current* workflow body, referenced scripts and actual runtime grants; no safe-by-name assumption. No extra router/scheduler gets built.

## 5. Runtime and concurrency design

**Five work queues/workspaces != five running models.** All five lanes can be prepared/observed in parallel with direct ChatGPT analysis and deterministic tests. ZCode *model-driven* concurrency starts at **0 until admission is verified**, then at most two independent read-only workers using existing `workflow-mode` and protocol. Each running claim requires sessionId, parent/runId, current model/provider/cost ceiling, process liveness, two increasing event observations, expected source SHA, explicit scope and a terminal result.

The incumbent **hourly ChatGPT development automation** and **existing Windows deterministic observer** remain separate real schedulers. Do not register a duplicate observer or create five always-on ZCode schedules. Reuse ZCode's actual provider registry and fallback; configured, labelled free, past responsive and current admitted are four distinct states. Stop on 429 rather than silently spending on paid fallback.

**Runtime tests first, model fanout later:** read-only isolated Node/Rust suites may proceed without an LLM. Actual phone UI, screenshot pixels and provider sends require the appropriate authorized real device/provider path; source tests and HTTP 200 do not satisfy them.

## 6. First cycle and quality gates

- **L01** read-only current Studio/browser/thread/worker reconciliation and no-replay fence. Complete a new *user-approved* ChatGPT one-send/settled-reply only after owner-session evidence.
- **L04** isolate current exact-SHA VLANG/safety negative cases without launching an additional scheduled observer. One sharp repro → one Dev source fix → independent fresh-source retest.
- **L02** independently inspect single-source Host/GPUI/PWA package drift and rollback plan. Do not promote or change paired phone URL before the live drafts/permissions check.
- **L03** inspect active PY-001 claim and genuine Studio Work `initialized` state, then Host-reviewed Work producer + CAS seam. No auto-bootstrap of personal data.
- **L05** verify ZCode work-run reality, currently admitted models, CI `startup_failure`, source claims and evidence. Its role is a gatekeeper, not a competing source integrator.

**Gate sequencing:** L01 delivery and L02 physical UI proof meet at one signed-in AI conversation; L04 command planning and L03 authorized Work meet at Host approval, with L02 showing the result; L05 independently certifies source→test→package→served→real-device facts. Prioritize these user-visible loops over background branch/archive housekeeping.

## 7. Decisions

**KEEP:** five named product missions and their already-created folders, prompts and shared tracker links.  
**FIXED:** all open #33 tasks now have one primary lane, missing C01–C05 are owned, U07/U10 no longer double-primary, completed tasks are not presented as unclaimed active work.  
**HARDEN:** reference-only vs conditional vs unsafe ZCode workflow classes and explicit launch approval.  
**DEFER:** no new DWF, router, model registry, scheduler, source-writing agent, autonomous five-seat startup, Host restart or provider send from this audit.  
**NEXT PROOF:** a genuinely admitted deterministic verifier or bounded ZCode run with runId and receipts, then one end-to-end ChatGPT → Host → GPUI/phone milestone when owner-authorized.

**Acceptance meaning:** this document is **STRATEGIC DESIGN / SOURCE INSPECTION**, not claim that each agent has received a task or that 64 items became integrated. No master checklist closure is earned merely by remapping IDs.
