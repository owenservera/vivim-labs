# ARBOR — Integration Proposal 1: Native Product Monorepo

**Status:** PROPOSAL ONLY — one candidate architecture for independent review, comparison and eventual owner ratification.
**Date:** 2026-11-12
**Destination:** `owenservera/ARBOR`, branch `architecture/node0-prime-backbone-r001`, `.integration/proposal1/`.
**No product migration, code cutover or replacement has been performed.** ARBOR `main` remains untouched.

## Thesis

ARBOR should be the **one canonical, buildable, locally owned VIVIM product**, not a new service next to existing systems. Consolidate the product's authoritative runtime into one resident native Rust Host and Cargo workspace; share typed contracts with desktop, PWA and TierFlow. Preserve separately valuable HQ development intelligence, independent TierFlow research/runtime and Labs experiments as upstream collaborators, rather than permanently coupling the installed product to their development servers.

**Node 0 Prime is the anchor**: user expression stays locally editable; an authenticated, bounded Rust event fabric derives multiple source-pinned interpretations without a central semantic intent router; explicit committed intent is distinct from predictive text and effect admission; Host remains the sole effect authority; all outcomes are observable with honest uncertainty.

**This proposal does not claim that the Labs designs are all ratified or production-ready.** It separates user-established architectural decisions from Lab hypotheses and requires source, tests and owner acceptance for promoting each component.

## Proposed repository skeleton

```text
ARBOR/
├── .integration/                # comparative proposals, evidence, migration ledgers — NOT shipped
│   ├── proposal1/               # this proposal, immutable review baseline once submitted
│   ├── proposal2/               # reserved conceptually, not created by Proposal 1
│   ├── proposal3/               # reserved conceptually, not created
│   └── decision/                # created only after cross-proposal owner decision
├── Cargo.toml                   # one Rust workspace, one lockfile
├── rust-toolchain.toml          # pinned supported stable toolchain
├── bun.lock                     # one JS package lock
├── package.json                 # Bun workspaces; package root scripts
├── apps/
│   ├── desktop/                 # Tauri v2 shell, WebView2, tray, installer, native bootstrap
│   ├── web/                     # canonical responsive TypeScript UI, also packaged as local PWA
│   ├── cli/                     # native Rust CLI for diagnostics, headless workflows
│   └── browser-worker/          # isolated signed-in Chrome/CDP adapter (Node/TypeScript)
├── crates/
│   ├── contract/                # canonical typed protocol, actor/capability IDs, revisions, receipts
│   ├── node0/                   # interaction sessions, edit events, parallel semantic trace
│   ├── graph/                   # one graph-node engine, DAG, scheduling primitives, invalidation
│   ├── intelligence/            # command semantics, prediction, DCB/context, retrieval adapters
│   ├── world/                   # authoritative owner state, evidence, personal graph lifecycle
│   ├── host/                    # runtime assembly, secure ingress, mission supervision
│   ├── effects/                 # Host-owned admission, grants, claims, receipt/recovery journal
│   └── devices/                 # authenticated PWA pairing, sync, offline reconciliation
├── packages/
│   ├── protocol/                # generated TS clients/types from Rust contract; no hand-maintained fork
│   ├── ui/                      # reusable design primitives and TierFlow presentation integration
│   └── browser-bridge/          # typed boundary for browser-worker, no authority
├── schemas/                    # canonical schema source and migrations; generated artifacts labeled
├── assets/                     # audited icons, themes, fonts metadata; not secrets
├── tests/
│   ├── contract/                # cross-language fixture parity / schema evolution
│   ├── node0/                   # IME, Unicode, Tab, negation, 500-edit burst, cancellation
│   ├── authority/               # hostile origin, actor binding, grants, uncertain effect, recovery
│   ├── full-stack/              # real user journey on installed Windows + paired PWA
│   ├── migration/               # read-only source-to-destination fixtures & reversible data migration
│   └── performance/             # p50/p95/p99 budgets, lower-end Windows/phone
├── tools/
│   ├── dev/                     # build, bootstrap, doctor
│   ├── codegen/                 # one Rust→TS contract generation/check flow
│   ├── integration/             # source audit and compatibility analysis
│   ├── release/                 # supply chain, signing, packaging, rollback
│   └── migrations/              # export/import and one-time cutover tooling
├── docs/
│   ├── architecture/            # accepted ADRs only
│   ├── contracts/               # generated contract explanations
│   ├── security/                # threat model, principal boundaries, storage and secrets
│   ├── operations/              # release, backups, upgrade, incident recovery
│   └── user-journeys/           # accepted user experience contracts
├── .github/workflows/           # reproducible Windows build + Rust/Bun checks
├── .gitignore
├── .gitattributes
├── AGENTS.md
└── README.md
```

**Scaffold status:** This tree is a proposed *future* ARBOR root, not actual directories in the repository. Do not generate empty placeholder crates, duplicate package managers or ship experimental Labs directories merely to match the diagram.

## Proposal documents

- [01 — Exact scaffold and dependency rules](01-SCAFFOLD-AND-DEPENDENCIES.md)
- [02 — Source-to-destination map and transformations](02-MIGRATION-MAP.md)
- [03 — Labs component graduation map](03-LABS-GRADUATION.md)
- [04 — Runtime, dataflows and migration acceptance](04-RUNTIME-DATAFLOWS-AND-GATES.md)
- [05 — Independent review questions and decisions](05-REVIEW-QUESTIONS.md)

## Repository ownership contract

- **ARBOR:** product runtime, executable native session/effect authority, packaging, personal owner data authority, desktop/PWA/CLI contracts.
- **repos-hq:** development-intelligence index, source graph/census, engineering wiki, docs generation and reusable algorithm evidence. It does not become a private user World store.
- **tierflow:** upstream domain-neutral visual and interaction runtime. Product uses a pinned artifact, a tested thin adapter and **one** rendered application identity.
- **vivim-labs:** research, benchmark fixtures, design hypotheses and migration evidence. Only inspected/risk-reviewed source graduates; Labs remains non-authoritative.
- **vivim-web-harness:** donor and live baseline until ARBOR verified cutover; then archive/retire product entrypoints, no dual effect writers.
- **legacy private `vivim`, `vivim-platform`:** separate donors; audit and quarantine secrets/old provider-API assumptions before selecting any code. Their existing `main` branches are not the scaffold.

## Guardrails

1. No direct provider APIs in the product; signed-in Chrome webapp sessions only. No Playwright/Puppeteer as product control surface.
2. No new graph DB, semantic router, permissions owner, native daemon per subsystem, shadow action catalog or second phone app.
3. Explicit source authorization before read fan-out; no sharing unsent drafts with unrelated producers.
4. One Host approval/claim/receipt writer. User-confirmed commands never inherit authority from autocomplete.
5. One shipped PWA + Windows desktop; APK only after independent release acceptance.
6. One product build/release identity; historical repo code is not copied blindly.
7. Do not collapse `designed`, `source_present`, `fixture_tested`, `native_built`, `integrated`, `device_accepted`, `production_active`.
8. Security fixes precede private source binding: Tauri update, exact origin/IPC scopes, process-bound Host session identity, source-owner proofs.
9. The user-facing interface accepts ordinary conversation, not just registered commands.
10. ARBOR should reduce permanent infrastructure, not multiply it.

## Comparison policy

Other opinions should use the same source anchors, distinguish decisions from hypotheses, and score against the review rubric. Do not silently treat this proposal as ratified or merge it with alternatives until the owner decides.
