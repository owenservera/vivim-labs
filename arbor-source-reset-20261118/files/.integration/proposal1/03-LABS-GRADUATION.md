# Proposal 1 — Labs Graduation and Decision Provenance

Date: 2026-11-12. Proposal only. This is NOT a declaration that all Labs work is ratified.

## Scope of "integrate the components in Labs we've ratified"

ARBOR should take **accepted product requirements and proved algorithms/fixtures** from VIVIM Labs and land them into the product owners they belong to. Labs stays a research/evidence repository; no independent Labs service, command router, document generator, knowledge store or extension framework becomes runtime authority just because a folder exists.

The word **ratified** must not be guessed from strong prose. In this review:
- **Owner-established direction:** the user has expressly selected this principle/goal during prior VIVIM design; this validates architectural intent, not build status.
- **Source-described proposal:** documentation/contract with status DESIGN/RESEARCH/PROPOSED, even if detailed.
- **Source-implemented experiment:** code/test exists in Labs or donor branch, not proof of product integration.
- **Promotable component:** exact current source SHA + owner-approved contract + security and differential tests + target crate/API.
- **Product accepted:** actual integrated/installed test evidence with source owner, platform/version, result and rollback.

No component in this document is asserted product accepted without that evidence.

## Labs intake crosswalk

| Labs path / research area | Current evidence posture | What ARBOR would absorb | What stays in Labs / does NOT get transplanted |
| --- | --- | --- | --- |
| \`user-io-rust-core/node-0-prime/\` | Architecture and risk assessment, R001 code spike in **Harness draft PR #55** uncompiled | Node 0 input/session primitives, strict revisioned event model, privacy-scoped fan-out, concurrency limits, epistemic effect transitions and red-team tests into \`crates/node0\`, \`contract\` and \`tests/node0/security\` | Prototype Mutex+per-keystroke thread implementation; the Labs directory itself; old branch status claims |
| \`nlp-command-ui-lab/docs/R004–R007\` (UI Labs branch) | R005 reports 28 Node tests + 100 synthetic probes; R006/R007 design, real device cutover unverified | Surface grammar, semantic source-span overlays, VIEW ALL 10-layer layout as \`apps/web/src/composer\`; trace schema into \`crates/node0/trace\`; confidence/provenance/producer coverage | Visual Lab as authoritative parser, fake project fixtures, independently written command catalog |
| \`personal-knowledge-lifecycle/\` | Explicit \`DESIGN / RESEARCH SCAFFOLD\`, plus \`runtime/\` needing source audit | ChatGPT-first t0 onboarding lifecycle, source atlas, capture→evidence→candidate→personal graph→DCB, consent/revoke/provenance contracts in \`world\`, \`intelligence\`, desktop/PWA onboarding | Claimed working provider adapter, direct provider APIs, broad historical scraping, unconsented private memory writes |
| \`vivim/voice-to-tasks/\` | Owner-established Day-0 objective; exact Labs files/tests require per-source review | Audio capture as Node 0 typed modality, final transcript/evidence, task proposal with explicit approval, cross-device receipt | A separate voice app, task-store or background executor |
| \`chronos-toolkit/wave2/\` | Toolkit staging; exact runtime wiring needs verification | Deterministic temporal/scheduling computations where needed by existing Host mission/automation contracts, scoped into \`intelligence\`/Host | Second clock service, competing scheduler, daemon or speculative schedule as effect |
| \`capability-observatory/\` | Explicitly **seed only**, production tracker not complete | Source/target inventory schema, capability IDs, release proof rubric and migration status into \`tools/integration\` and acceptance tests | Live end-user registry, second product capability authority, false \`production_active\` labels |
| \`intelligence/algorithms/\`, \`intelligence/engines/\` | Isolated experiments and future engine notes | Selected tested algorithms into \`crates/intelligence\` and shared graph compute; benchmark against current source | Wholesale algorithm copies, unbounded automatic model prompts, independent intelligence service |
| \`vivim/\` extension architecture, \`extension-points/\`, \`families/\` | Explicit exploratory plugin-surface proposals; 153 candidate surfaces ≠ 153 verified hooks | Typed extension *seams* where source-owned owners exist, explicit consent/risk types; candidate registration rules, long-term \`host\` contract | A prematurely built plugin marketplace/loader, new privilege owner, private legal practice data, all 153 unverified hooks |
| \`Architecture/\` graph convergence documents | Conceptual one-graph design and implementation philosophy; components vary from source-present to proposed | Core invariants: one shared Rust graph-node engine, separate Development/Personal authority, transformation lineage, two configs, no extra governance layer | Parallel graph DB, direct product access to HQ dev-index credentials, research-only engines shipped without tests |
| \`context/\`, \`memory/\`, \`personalization/\`, \`learning/\` | Mixed research; inspect each exact source and version | Dynamic Context Bundles, context ranking, learning signal provenance, user-owned memory lifecycle into \`intelligence\` + \`world\` | Assumed imported personal data, auto-learn from untrusted text, duplicate memory store |
| \`automation-queen/\` and related autonomous missions | Research/drafts; automation permissions and state must be checked | One existing Host mission/scheduling contract with bounded autonomous **proposals**; explicit owner admission for effects | Separate control agent with its own permissions, effects or scheduler |
| \`zai/\` R001/R002, archived rounds and contributor output | Work products, bundles and plans with integration status requiring reconciliation | Source-confirmed, tested, license-safe improvements through their rightful ARBOR targets | Treating assumed merged bundles as live without exact upstream SHA/test proof |
| \`performance/\` and \`tests/\` | Lab benchmarks and fixtures; runtime comparability not established | Golden corpora, adversarial input, source-freshness, Graph/IME/latency benchmarks into \`tests/\` | Synthetic benchmark labels as real device guarantees |
| \`KB-consolidation/\`, \`documentation-consolidation/\`, \`evidence/\`, \`corpora/\` | Audits, historical artifacts and research data | Only verified source fixtures / provenance links or metadata necessary for migration and regression | Bulk historical docs, personal conversations, duplicated wiki text or unreviewed corpora in product binaries |
| \`vivim/PLUGIN-SYSTEM-ARCHITECTURE.md\` | Provisional extension architecture | Permission-safe optional capability port design, after actual source contract audit | New global governance layer or unverified extension runtime |

## Ratified architectural invariants to carry in Proposal 1

These represent **user-established direction**, not automatically implemented code:

1. **Node 0 Prime first.** I/O is the primary product success/failure boundary; instant local editing, parallel native comprehension, explicit commit/outcome truth.
2. **No central semantic router.** Subscriptions, graph dependencies and typed event engineering connect independently useful compute systems. Mechanical ingress/reducer/scheduler are allowed; semantic master dispatcher is not.
3. **Shared Rust computational foundation.** Reuse existing graph-node/DAG/scheduler research; no new graph store/permission layer. Owner-private personal graph and HQ development graph have distinct state owners.
4. **Natural-language-first product.** All capabilities should ultimately be reachable via text and appropriate visual/touch affordances, with deterministic validation and clear approval/receipt/recovery.
5. **No provider APIs**, including direct OpenAI/Claude/Gemini model APIs for product execution; signed-in Chrome web apps via user-owned Host profile. No Playwright/Puppeteer product automation.
6. **Host effects are authoritative.** The implementation language may move to Rust, but the effect grant/claim/receipt security model cannot be recreated in another owner.
7. **One paired PWA-first mobile product.** APK may come later. Phone is a presentation/input peer, not a second independent Host.
8. **Dynamic Context Bundles** feed source-scoped context into providers as if their session memory were off. User-private data stays local unless intentionally disclosed.
9. **Zero-management mission UX.** One understandable outcome experience; product complexity stays behind the user-facing interface.
10. **VIEW ALL interpretation transparency** is a desired higher abstraction: source spans, multiple semantic facets, evidence and producer coverage from actual underlying trace. Detail may grow incrementally; no fake 100% certainty.

## Feature and component disposition rules

**Promotion takes place in destination modules, not as separate product apps.**
- A pure Rust algorithm from Labs becomes a well-tested function/engine under the appropriate \`crates/\` owner, with dependency/permission declarations.
- A UI visual prototype becomes a presentation module under \`apps/web\`, consuming typed semantic output with an owner/source revision; it does not recompile authority itself.
- A test fixture becomes \`tests/\` evidence with source SHA and status label.
- An architecture proposal becomes a reviewable ADR in \`docs/architecture\` **only after owner decision**.
- A provider-specific browser experiment becomes a capability adapter only after actual signed-in Chrome behavior is observed and data access terms respected.
- A private personal knowledge asset remains user-local and is never copied to ARBOR Git.

## Source and owner audit before code moves

For every Lab path selected for graduation, the migration ledger must record:
\`lab_ref, branch_sha, path, claim_status, implementation_status, tests_run, data_class, existing_product_owner, proposed_target, required_adapter, risk, owner_ratification_evidence, acceptance_gate\`.

Use statuses:
\`OWNER_DIRECTION\`, \`DESIGN\`, \`SOURCE_EXPERIMENT\`, \`BUILD_VERIFIED\`, \`INTEGRATION_VERIFIED\`, \`DEVICE_ACCEPTED\`, \`PRODUCTION_ACTIVE\`, \`REJECTED\`, \`RETAIN_LAB\`.

A stated goal or prior conversation does not certify a security-sensitive implementation. If no explicit recorded ratification evidence exists, list it as **review required**.

## Special caution: staged branches differ

The UI R007 work lives on Labs branch \`lab/nlp-visual-command-ui-20261111\`, while Node 0 Prime and the R001 Rust PRDs live on \`lab/user-io-rust-core-20261112\`. They are not necessarily all present on Labs \`main\`. ARBOR intake must pin exact branch SHA for each, not assume a single Labs checkout contains both.

## Open Labs-specific questions for competing proposals

1. Should TierFlow remain an upstream, pinned standalone package or should its source be imported into the product monorepo as a vendor tree/subtree while retaining upstream development?
2. Which DCB algorithms are genuinely source-backed and bench-tested versus candidates in the lifecycle design?
3. Which voice-to-tasks and Chronos runtime code pieces are already committed, testable, and compatible with one native Host?
4. Which R001/R002 Z.ai bundle changes are on canonical main, and which are only staged?
5. Which Labs experimental plugins are really required for Day 0 versus later extension support?
6. Which content/source permissions apply to past AI chats and historical personal exports?
7. Which artifacts require only documentation or test promotion, rather than source migration?

None of these open questions should block a **proposal**. They should block uncontrolled source copying or certification.
