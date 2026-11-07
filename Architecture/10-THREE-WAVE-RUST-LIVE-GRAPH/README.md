# VIVIM — Live Rust Architecture Graph: Three Waves × Six Lanes
**Date:** 2026-11-07 · **Owner intent:** execute, test and converge, not produce only another plan.
**Status:** Wave 1 executable campaign ONLY. Wave 2/3 are UNAPPROVED hypotheses until the Wave 1 runtime has been exercised and an evidence-backed blind-spot dossier reviewed. NOT a claim of an already running graph.
**Repository:** vivim-labs (campaign authority and evidence hub); code owners: repos-hq, tierflow, vivim-web-harness, and other source owners as explicitly assigned.

## Mission
The long-range intention is to produce over three eventual waves **one live, running Rust HQKG graph** for the eligible VIVIM code, canonical KB, semantic language/action/command system, actual CLI commands, generated/programmable wiki, engine slots/implementations, databases, dataflows, ownership, callers/callees, input/output/write paths and their source evidence. Make it queryable from a short CLI recipe and visible through a continuously refreshed **actual-vs-target** topology/wiki/report.

**Live** means: on the real local workspace, while source/KB/command changes occur, the refresh mechanism detects updates, publishes a source-pinned graph revision without a competing writer, invalidates/updates derived views and exposes staleness/refusals. It does not mean only one demo snapshot or a CI-only mock.

**Source of truth:** One logical graph namespace (one physical Rust HQKG runtime initially), but source-native ownership for code, approved KB revisions, executable actions, Host/Visual/TierFlow state. Never invent runtime wiring from source-file presence.

## Existing code to reuse first
- HQ code index + graph + FTS: `repos-hq/hq/lib/indexer.mjs`, `schema.mjs`, `graph.mjs`, `extract.mjs`, `hq/bin/hq.mjs`.
- KB/context and durable kernel: `hq/lib/kb.mjs`, `kb-intake.mjs`, `context.mjs`, `kernel/store.mjs`.
- Semantic IDs/action registry/NL routing: `semantic-registry.mjs`, `semantic-optimizers.mjs`, `action-registry.mjs`, `action-router.mjs`, `comment-wiki-bridge.mjs`.
- Automatic wiki and programmable view: `wiki.mjs`, `wiki-program.mjs`. **Wiki is downstream evidence of correct A+B+C integration, not a peer data source whose mere import establishes success.**
- Rust HQKG Graph/EngineSlot/ComputeEngine/Database/Scheduler/Journal: `research/rust-knowledge-graph/src`.
- Already existing HQKGX1 data-only exporter/importer: `research/agentic-knowledge-graph/scripts/export-hqkg.mjs`, `research/graph-kernel-integration/scripts/export-unified-hqkg.mjs`, `research/rust-knowledge-graph/src/import.rs`, `src/bin/hqkg-import.rs`.
- Existing research exporter does **not** yet export live HQ indexing/KB/actions/wiki: extend via a separately source-owned live projection, do not mislabel historical research export as live.
- Existing Rust importer requires canonical `hqg1:<sha256>` IDs, manifest digest, source ownership, scalar metadata and node-before-edge ordering; data only, not execution.

## Wave structure
| Wave | Theme | Key proof | Deliverable |
|---|---|---|---|
| **1 · Bootstrap** | Connect live code intelligence (A), KB/semantics (B) and commands/CLI (C) inside Rust; generate the wiki as an integration quality test (D) | Real A+B+C linked and queried from Rust; derived wiki exposes integration completeness and gaps | First runnable graph + measured wiki quality + source-backed blind spots |
| **2 · Convergence** | Make the graph a unified, navigable command/KB/code/engine model | Actual caller/callee and inputs/outputs/writers connected; same semantic operation across surfaces | System-wide topology + actual/target views |
| **3 · Live operation** | Continuous refresh, end-to-end UX and robust operation | Change → index → Rust revision → query/wiki/UI → receipt, repeatable and recoverable | Living architecture observatory + launchable dev/product profiles |

Each wave contains six lanes A–F. Parallel work **within** a wave; wave integration gate only after six handoffs and independent tests. Subsequent wave begins against gated main, not a stale assumption.

**Executable order NOW:** [Wave 1 actual execution charter](EXECUTE-WAVE-1-FIRST.md) and [six copy-paste lane prompts](WAVE-1-LAUNCH-PROMPTS.md). The old [Wave 1 notes](WAVE-1-BOOTSTRAP.md) are background only. [Wave 2 concept](WAVE-2-CONVERGENCE.md) is DRAFT/FROZEN and must be redesigned from Wave 1 measurements; do NOT launch Wave 2 or Wave 3.

## End-user questions this system must answer
- Where is a capability? Which repo/path/symbol/commit and semantic ID?
- What does it do? Inputs, outputs, transformations and permitted writes?
- What does it talk to? Inbound/outbound dependencies, engines, DBs, events, CLI/Host/Visual routes?
- How can I use it? CLI command, wiki menu, typed action, natural-language mapping and effects/approval status?
- What exists now versus desired? Implementation, linked vs orphaned, declared vs wired, integrated vs test fixture vs device-verified.
- When did it change? Last indexed source SHA, graph revision, provenance, receipts, freshness and confidence.
- What will a change affect? Reverse dependency/impact traversal.
- What isn't known? Unindexed repo, ambiguous identity, unsupported adapter, stale data or unverified runtime.

## Nonnegotiables
Main only, fresh-head non-force CAS on shared code, no PRs/branches/Actions. Parallel lanes own disjoint files and never silently modify one another's owner paths. Do not reset existing HQ DB/profile or replace a signed-in browser session. No Playwright/Puppeteer, no provider API shortcuts. The Rust graph is a **read and compute kernel**, not a backdoor Host action authority. Single journal writer; no duplicate engine registry; no second curated KB writer. DEV and PRODUCT configurations use **identical semantics** but different installed adapters, source scope, debugging privileges and privacy policy. End product must be independent of lab path fixtures.

## Wave sequencing law
Every lane must leave code/tests, an evidence record, and a precise reusable interface consumed by a named lane in the *next* wave. If it produces only a document without an implementation/test improvement, call it incomplete except explicit inventory-only lanes.
