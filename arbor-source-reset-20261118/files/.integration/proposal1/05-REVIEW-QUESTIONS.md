# Proposal 1 — Independent Review Brief, Decision Register and Comparison Rubric

Date 2026-11-12. Proposal 1 is one opinion, **not ratified**. ARBOR \`main\` remains protected by process convention (branch is not a GitHub protected branch).

## Reviewer instructions

Read these in order: [README](README.md), [scaffold](01-SCAFFOLD-AND-DEPENDENCIES.md), [migration map](02-MIGRATION-MAP.md), [Labs promotion](03-LABS-GRADUATION.md), [runtime and gates](04-RUNTIME-DATAFLOWS-AND-GATES.md). Also read the source-owned:
- [Labs Node 0 Prime](https://github.com/owenservera/vivim-labs/tree/lab/user-io-rust-core-20261112/user-io-rust-core/node-0-prime)
- [Labs UI R007](https://github.com/owenservera/vivim-labs/blob/lab/nlp-visual-command-ui-20261111/nlp-command-ui-lab/docs/R007-TOTAL-INTERPRETATION-VIEW-ALL-AND-HQ-HANDOFF.md)
- [HQ interpretation producer audit](https://github.com/owenservera/repos-hq/blob/research/interpretation-xray-intelligence-surface-20261112/research/semantic-interpretation-surface/HQ-SURFACE-AND-GAP-AUDIT.md)
- [Harness R001 draft PR #55](https://github.com/owenservera/vivim-web-harness/pull/55)
- [Harness main](https://github.com/owenservera/vivim-web-harness), [HQ main](https://github.com/owenservera/repos-hq), [TierFlow main](https://github.com/owenservera/tierflow)
- [Previous private VIVIM](https://github.com/owenservera/vivim) and [previous VIVIM platform](https://github.com/owenservera/vivim-platform) as historical donors, not authority.

**Do not edit Proposal 1 to express another opinion.** Create \`.integration/proposal2/\` or another numbered folder on a separate branch/commit, preserving its own assumptions and evidence. A comparison/ruling document should be written only after the owner requests one.

## Proposal 1 architectural decisions to challenge

| ID | This proposal selects | Alternative worth comparing | Decision condition |
| --- | --- | --- | --- |
| ADR-P1-01 | ARBOR as complete canonical product monorepo | New native core repo + existing independent product app repos | Integration risk, authority duplication, build/rollback |
| ADR-P1-02 | One resident Rust Host process and Tauri shell | Native Host service separate from desktop process | background reliability, Windows privilege separation, installer complexity, trust |
| ADR-P1-03 | Eight Rust crates | Five broader crates or larger domain modules | compile and ownership coherence, risk of circular dependencies |
| ADR-P1-04 | Rust-native semantic/event computation | Retain JS/Omega as canonical interpretation until deeper Rust parity | language corpus fidelity, performance, version skew |
| ADR-P1-05 | Rust Host ultimately implements admission and effect journaling | Keep existing JS Host as permanent effect authority | proven crash durability, migration burden, trust boundaries |
| ADR-P1-06 | Isolated Node/Bun Chrome browser worker | Integrate browser automation directly in Rust using CDP | complexity, page adaptability, failure isolation, security |
| ADR-P1-07 | One shared responsive TypeScript UI/PWA bundle | Separate native desktop UI and mobile web UI with shared semantic libraries | platform fidelity, maintenance, accessibility, load |
| ADR-P1-08 | TierFlow upstream pinned dependency/adapter | Vendor TierFlow into ARBOR with upstream subtree strategy | visual runtime independence, build reproducibility, UI parity |
| ADR-P1-09 | One canonical graph crate based on HQKG algorithms, packaged in ARBOR | Canonical graph crate remains in HQ and is pulled by exact version | source ownership, publication, security, API stability |
| ADR-P1-10 | Rust→TS generated source-of-truth contract | Language-neutral IDL as master source | safe validation, tooling, cross-language evolution |
| ADR-P1-11 | Owner data under one Host domain with distinct stores/namespace projections | Separate personal graph process/DB | security, transactions, performance, recovery |
| ADR-P1-12 | Single product cutover after staged build/test | Continuous product-by-product migrations | user disruption, ability to roll back, trust and evidence |
| ADR-P1-13 | Proposed static web build plus Rust service; no required cloud | Cloud-assisted optional sync/network at first release | privacy policy, cost, device reachability, owner directives |
| ADR-P1-14 | UI display over source-filtered multi-facet trace | UI computes semantic truth locally | high-frequency performance, parity, offline honesty |

## Ten questions that matter more than style

1. Can a clean checkout of this scaffold build Windows and a paired PWA without launching any HQ/Labs/TierFlow development servers?
2. Does the same utterance produce source-consistent semantics, and can the UI always paint even while all native semantic engines hang?
3. What exactly is the process and credential trust boundary when an untrusted local listener or web origin tries to impersonate the Host?
4. Is an existing command/effect owner replaced or accidentally duplicated anywhere?
5. Does the graph have one computational implementation while retaining domain-specific **authoritative source data** and privacy?
6. Can the full product recover after crash during intent commit, grant approval, effect claim, provider send and receipt UI delivery without double sends?
7. Are Labs ideas promoted to source owners with proof or wholesale imported because they sound compelling?
8. Does the data migration cover old personal memory, DCBs, provider sessions, Studio captures, grants, receipts, indexes and user-generated artifacts with owner consent?
9. Does the scaffold retain real essential capabilities (onboarding, conversation, streaming, phone, app builder, voice/tasks, World, personalization) without rebuilding multiple experiences?
10. Can the codebase be maintained by a very small team and a large AI worker pool without changing architecture every week?

## Quantitative comparison rubric (scores are judgments, NOT observed metrics)

Each independent proposal should score 1–5 with evidence references, never fill an unknown field as 5.

| Dimension | Weight | Required evidence |
| --- | ---: | --- |
| User I/O integrity and perceived responsiveness | 20% | edit/IME independence, bounded reactors, source revisions, measured latency strategy |
| Effect/security/owner authority correctness | 20% | threat model, process auth, grants, one-writer journal, crash recovery |
| Architectural cohesion / fewer duplicate owners | 15% | precise process and crate boundaries, retirement map, no new router |
| Lab and legacy capability preservation | 15% | source-to-destination table incl. status and tests |
| Data portability / safe single cutover | 10% | schema migration, rollback, data lineage and owner provenance |
| Desktop/PWA/visual renderer experience | 10% | same semantic contract, accessibility, offline/phone flow |
| Buildability, local hardware and operating cost | 5% | toolchain, deterministic CI, memory/storage/scheduler budgets |
| Developer/AI coding throughput and maintainability | 5% | bounded package structure, source ownership, generated types, tests |

A proposal with a P0 violation in Host effects, source privacy or first-hop auth fails regardless of aggregate score.

## Source baseline and nonclaims

At proposal design review, examined:
- Harness main \`cae97afe848d79957c9d6c698a90996a758bfd56\` and R001 branch \`fa6dea9576c83496d95562b98b7a815fda4aec88\`.
- HQ main \`7efb52b251fd56c9257a3184ac39dc19d84340b5\`.
- TierFlow main \`50822f11ee63c3d80f631f8fea7dcb9b19bffc38\`.
- Labs Node 0 branch \`lab/user-io-rust-core-20261112\`; UI R007 on \`lab/nlp-visual-command-ui-20261111\`.
- Earlier private \`vivim\` and \`vivim-platform\` source-level topologies.

Exact file SHAs, latest branch heads, source-code test coverage and runtime status must be rechecked when executing. This proposal does not assert any tests passed in ARBOR, migrated source code, completed security patch, compiled Rust workspace or device acceptance.

## Required output from an independent reviewer

Respond with:
- Top-level recommendation: accept Proposal 1 / modify / reject, with up to 5 decisive reasons.
- A complete *alternative directory scaffold*, not a list of criticisms.
- Changes to process topology, language, graph owner, data storage, protocol and effect writer.
- Labs inclusion/exclusion with clearly identified source status.
- 10 highest-risk migrations, with proposed proof.
- Source-anchored migration and retirement map for canonical repos.
- Contradictions/ambiguities in Proposal 1 (especially crate cycles and who owns graph release).
- A score table against the common rubric.
- A proposed full one-time cutover gate list.

No proposal should claim that a UI design, a draft pull request or a historical test count proves product readiness.
