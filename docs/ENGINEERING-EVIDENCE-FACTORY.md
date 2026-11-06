# VIVIM Engineering Evidence Factory — full program of intent

**Record:** 2026-11-06. **Status:** program intent and proposed operating targets, **not** an implemented fleet, verified corpus, design of ZCode internals, or authority grant. This document preserves the full earlier high-volume program, not just wave boosters. The local team designs workflows from the existing ZCode/HQ environment.

## North star

Use ZCode's *demonstrated* 3+ parallel threads × 8 workers (24+ worker instances) and available OpenRouter/NVIDIA model options to **expand the breadth of capabilities VIVIM understands, the depth of situations it handles, and the number of combinations verified**. Never optimize for raw generated code volume alone.

**Division of responsibilities:**
- **ChatGPT:** architecture, canonical contracts, critical decisions, source implementation in explicitly reserved Wave 3 paths, integration review, priority and long-term coherence.
- **ZCode/Labs:** local Git/test operations, large-scale experiment/asset/corpus creation, integration verification, environment/device validation when available, benchmark/process/tooling work, evidence collection, and reproducibility.
- **Canonical integration authority:** acceptance and promotion of verified, nonduplicative features into owning production repos. Labs cannot grant itself Host/KB/UI source or production authority.

## The 12 high-volume factories — preserve all twelve

These are **candidate work programs** and scale targets, not current inventory or task guarantees.

| # | Factory | Priority | Proposed scale | Coverage and outputs |
|---|---|---|---|---|
| 01 | **Natural Language Command Factory** | P0 | **100k+ utterances** | Synonyms, paraphrases, intent classification, correction, chained commands, ambiguity, multilingual phrases, typed IR conformance and refusal |
| 02 | **Catalogue of Catalogues** | P0 | **1,000+ integration candidates** | Federated index across components, commands, capabilities, models, providers, devices, algorithms, agents, assets, workflows |
| 03 | **Universal Asset Library** | P0 | **1,000+ reusable assets** | UI components, templates, workflows, command recipes, adapters, agent definitions, test fixtures, help patterns |
| 04 | **Adversarial Security Factory** | P0 | **10k+ scenarios** | Cross-owner leaks, forged receipts, privilege escalation, replay, revocation, prompt injection, unsafe transitions |
| 05 | **Integration Compatibility Lab** | P0 | **All critical interfaces** | HQ ↔ TierFlow ↔ Host ↔ Studio ↔ Android ↔ browser adapter contract conformance; version skew |
| 06 | **Production Scaffold Factory** | P0 | **Full release topology** | Clean packages/builds, build profiles, installers, migration manifests, configuration, release channels, reproducible artifacts |
| 07 | **Knowledge & Context Evaluation Lab** | P1 | Large synthetic corpora | Retrieval/graph, memory evolution, provenance, freshness, ranking, deletion, temporal consistency |
| 08 | **Provider Compatibility Observatory** | P1 | Broad app coverage | Signed-in web UI workflows, accessibility, sessions, recovery and safe adapter contracts; owner-consented research only |
| 09 | **State & Failure Simulation Lab** | P1 | Thousands of fault runs | Crash/restart, corrupt state, offline, network loss, stale cursor, concurrency, storage and ACK-loss |
| 10 | **UX & Accessibility Factory** | P1 | Many devices/viewports | Layouts, input parity, keyboard/TalkBack mappings, onboarding, readable denial and recovery |
| 11 | **Performance & Optimization Lab** | P1 | Continuous benchmarks | Latency/startup, CPU/RSS/storage, indexing, cache, query plans, routing quality/cost and energy where measurable |
| 12 | **Documentation & Recipe Factory** | P2 | Thousands of recipes | Executable how-to, capability explanations, deterministic CLI examples, troubleshooting, reusable agent playbooks |

Priority labels and volumes are **proposed**, to be re-ranked from evidence. Corpus counts must distinguish *generated, deduplicated, independently labelled, executable, passed, integrated, native/device verified*.

## Natural-language conformance, not synonym accumulation

One example underlying semantic operation: `workspace.panel.move@1`.
- “Move my tasks above activity.”
- “Put the task widget at the top.”
- “Sube el panel de tareas.”
- “Can you rearrange my dashboard?” (possibly ambiguous; clarification may be necessary)
- “Actually, move it back.” (requires conversational reference and prior state)

The deterministic typed command must include exact semantic identity/version, target, arguments, scope and revision preconditions. Outcomes must distinguish **accepted candidate**, **clarification**, and **safe refusal**; no text, model or UI action can mint a Host grant.

Generation and grading must not share an unchecked model-produced oracle. **Independently verify test expectations**. Record false acceptance, false refusal, wrong target, wrong scope, unauthorized action choice, ambiguous grounding and policy/version drift. A diverse 10k verified case corpus can be more valuable than 100k near-duplicates.

## Federated catalogue of catalogues

**Do not merge into one giant JSON file, monolithic DB or another independent authoritative registry.** Catalogue index links current owning registries with: **stable ID · version · canonical owner · source · relationships · compatibility · evidence · capability maturity**.

Catalogue domains: **Commands · Components · Capabilities · Providers · Agents · Models · Algorithms · Assets · Workflows**, with additional device/knowledge/integration registries where found. HQ remains knowledge/semantics authority, Host remains effect/grants authority, TierFlow owns UI schema/command runtime after cutover.

Oracle and ZCode should eventually be able to ask what exists, which source owns it, which environments support it, what is compatible, and what has actually been verified. Discovery metadata is never executable permission.

## Discover → Verify → Promote

- **Discover:** candidate capabilities, mappings, corpora, fixtures and reusable assets.
- **Verify:** executable unit/integration, property, mutation, adversarial, compatibility and environment proof, with independent reviewers.
- **Promote:** owner-approved incorporation into the relevant canonical repo with contract/version mapping, consumer migration and rollback evidence.

**Six-step capability maturity ladder** (distinct from the compact 3-stage workflow):
1. **Discovered:** item identified.
2. **Specified:** contract, inputs, outputs, risk and boundaries defined.
3. **Unit verified:** isolated deterministic tests pass.
4. **Integrated:** actual neighboring production modules interoperate; mocked neighboring modules do not count.
5. **Environment verified:** relevant real browser, OS or device executed.
6. **Production verified:** owner-authorized release and real operational receipts.

Never call fixture-only tests real Android/Host/browser proof. Every result has source SHA, environment, command, exit code, evidence and verifier.

## Five high-leverage test methods for parallel workers

1. **Combinatorial coverage:** commands × actors × devices × policies × versions × failure modes. Use pairwise/risk-based sampling; exhaustive checks where the critical state space is small.
2. **Differential tests:** compare legacy Harness UI/semantic interpretation with TierFlow canonical behavior before migrations.
3. **Property and mutation tests:** corrupt valid documents, typed commands, receipts, KB projections, permissions and revision states; require deterministic safe refusal.
4. **Cross-version tests:** old saved user state and old app adapters against newer UI/versioned commands and migration readers.
5. **Clean-checkout reproducibility:** independently reconstruct and rerun without secret machine-local files, caches or hand-patched config.

## Centralized decisions — not swarm-owned

Canonical schema/version changes, Host authorization or effect grants, live KB identity/deletion policy, shared authoritative registries, cross-repo migrations, production cutover and final Integration GO/BLOCKED remain with their respective owners. A swarm may research, test and propose changes; it may not self-approve them. No unapproved signed-in provider sends or bypasses.

## Proposed 3 × 8 worker cell allocation — illustrative, not imposed on local ZCode

| Cell | Proposed allocation | Primary evidence output |
|---|---|---|
| **Language & Catalogues** | 3 language, 2 catalogue federation, 2 reusable assets, 1 independent validator | Indexed, typed, searchable conformance and asset corpus |
| **Reliability Lab** | 2 security, 2 integration compatibility, 2 state/fault simulation, 1 performance, 1 test harness | Reproducible regression, fuzz/property and adversarial evidence |
| **Platform Factory** | 2 production scaffold, 2 clean build/release, 1 migration compatibility, 1 provider research, 1 documentation, 1 independent verifier | Reproducible production-foundation candidates |

Local ZCode must **reuse existing HQ orchestration** and measure actual concurrency, quota, disk/memory, reliability and throughput rather than assume this topology is automatically best. Separate owned paths and worker artifacts; serialize writes to any shared evidence database. Research read-only where source ownership is not authorized.

**Candidate task evidence record:** `task_id · source_sha · owner · contract_version · inputs · expected_output · allowed_paths · test_command · evidence_level · result · artifact_hash · reviewer`. This is a discussion-level data list, not a mandatory schema.

## First campaign proposed volume targets

- **10,000** quality-filtered natural-language test cases.
- **1,000** integration candidates classified.
- **500** reusable assets inventoried or specified.
- **2,000** adversarial cases.

These are first-campaign targets, **not already achieved, not guarantees and not substitutes for correctness**. The larger 100k+/1k+/10k+ factory scales above are aspirational longer-run workload sizes.

Production scaffolding must *build from a clean checkout, exercise canonical interfaces, produce verifiable release artifacts and demonstrate migration from current repos* before it is described as ready. Empty package folders are not a successful production scaffold.

## Automated capability/evidence map (research opportunity)

Queryable relation (not a new independent KB):

`Capability → Implementation → Contract → Consumer → Test → Evidence → Platform`

Desired future questions:
- Which commands work on Android but have never been tested under revoked permissions?
- Which old Harness renderer behaviors still block retirement?
- What share of reusable assets are verified on Windows, Android and narrow viewports?

An evidence index should link to Repos HQ's canonical graph, not copy fact authority. Import only approved, scoped metadata with provenance.

## Outcomes: breadth, depth, convergence

- **Breadth:** unique, correctly specified capabilities and situations covered.
- **Depth:** distinct environments, failures, permissions and states genuinely verified for each.
- **Convergence:** coverage that flows through canonical systems without duplicating active owners or implementations.

**Permanent division:** ChatGPT reduces uncertainty about what VIVIM should be; the local ZCode fleet reduces uncertainty about what VIVIM can *demonstrably* do. Both use evidence and explicit handoffs.

## Source and next steps

Discussion captured in [DISCUSSION-RECORD.md](../DISCUSSION-RECORD.md). Wave-aligned Labs responsibilities in [WAVE-BOOSTERS.md](WAVE-BOOSTERS.md). Existing local readiness baseline in [WAVE-3-READINESS-HANDOFF.md](WAVE-3-READINESS-HANDOFF.md). No tool installation, new process engine, worker scheduling, code mutation or model-provider quota has been verified by this program document.
