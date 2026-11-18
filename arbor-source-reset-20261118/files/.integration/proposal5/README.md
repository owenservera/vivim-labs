# ARBOR Proposal 5 — Internally Complete Beta Landing

**Version:** P5-R2, 2026-11-12  
**Status:** Independent proposal; no source migration or product acceptance performed  
**Branch:** `architecture/proposal5-beta-first-centralization`  
**Objective:** **one-time source centralization and integration** into one internally self-contained product repository, using working implementations rather than a wholesale rewrite. The result must be a functioning Windows + paired PWA beta with Node 0 Prime and a future-proof capability contract, while deeper optimizations remain clearly deferred.

## The owner's correction (non-negotiable)

**NO OUTSIDE WIRING. ARBOR MUST BE FULLY INTERNALLY CONSISTENT.**

In Proposal 5, HQ, TierFlow, Harness and Labs are **migration donors only**. ARBOR cannot require any of their checkouts, servers, CLI processes, SQLite files, Git branches, unbuilt packages, environment variables or network bridges at build time, launch time or while operating. Everything needed for the selected beta must be physically present in ARBOR, versioned, bundled and tested under one release manifest.

That explicitly includes:
- The existing HQKG Rust graph/typed DAG/scheduler code as one in-repo canonical product crate.
- **The HQ development-intelligence functionality required by VIVIM**: selected HQ indexing, source search, semantic registry, context-bundle, incremental wiki and evidence/capability code, adapted into an **in-repo** package with its own internal development data domain. No "HQ-A/B/C external read adapter".
- The working Harness Tauri/Host, action catalog, planner, journal, pairing, World and Studio/Chrome integration under retained internal paths.
- The TierFlow rendering/interaction/typed visual code selected for the beta, imported as an in-repo package with one compatible UI publisher. No runtime TierFlow service or source checkout.
- Ratified Labs contracts, source code, test fixtures and views moved into their actual owners; no Labs runtime or source path dependency.
- One paired PWA, one Windows release identity, one internal protocol, one source-owned effect writer and one canonical graph computation implementation.

**External systems do exist:** the user's Chrome/browser, signed-in ChatGPT/Claude/Gemini web applications, Windows OS, local user files and optional user-selected remote devices are legitimate external *sources/targets*. They are **not external VIVIM repository or orchestration wiring**. No provider APIs; signed-in user browser applications only. Package toolchains/third-party dependencies may be resolved during CI/build using pinned lockfiles; released artifacts bundle everything necessary that licensing permits.

## Core beta-first decision

**Centralize by physically importing, linking and proving existing code—not by running one project as a facade over three other projects.**

Keep enough source paths/behavior intact to minimize import churn. Build one Cargo + Bun workspace; import the needed source and tested dependency closures from donor repos, scrub secrets and generated artifacts, and make the internal dependency graph explicit. Introduce a small typed protocol and native I/O layer. Do *not* demand a rewrite of Host effect admission, HQ indexing, signed-in browser automation, TierFlow rendering, mobile UI or all native graph algorithms before beta.

The one-time job is to create a consistent product **without a second integration wave being needed just to make the components aware of each other**. Future optimizations replace an internal implementation behind versioned capability/operation interfaces.

## The conceptual product

```text
      Windows / Paired PWA / CLI / View / AutoWiki
                         │
      local editor + TierFlow presentation package
                         │
                   Node 0 Prime
        typed input, evidence, revisions, candidates
                         │
       in-repo capability contracts and programs
             /                   \
      HQKG native graph       Internal JS adapters
      DAG, source scopes      HQ intelligence, Host planner,
      scheduling, context     Studio provider, TierFlow bridge
             \                   /
                     Host
        ONE personal World/effect authority
        grants, plans, claims, receipts, recovery
                         │
         user-owned browser and files/OS/devices
```

Physical processes remain allowed for safety and technical necessity (Tauri, internal Node Host/Studio worker, Chrome). **Process separation does not imply separate system truth or a source repository dependency.**

## Seven Proposal 5 artifacts

1. [01 — Complete in-repo scaffold](01-FAST-LANDING-SCAFFOLD.md)
2. [02 — HQ Rust + HQ operational intelligence internal migration](02-RUST-CORE-AND-HQ-WIRING.md)
3. [03 — Source and capability-space placement](03-CAPABILITY-SPACES-AND-MIGRATION.md)
4. [04 — Node 0 and security-critical minimum](04-NODE0-SECURITY-AND-INTERACTION.md)
5. [05 — Complete beta release contract](05-FULL-BETA-RELEASE-CONTRACT.md)
6. [06 — One-time integration and cutover](06-EXECUTION-AND-CUTOVER.md)
7. [07 — Deferred optimizations and decision tradeoffs](07-DEFERRED-OPTIMIZATIONS-AND-DECISIONS.md)

Machine-checkable gates: [08-BETA-GATES.json](08-BETA-GATES.json).

## Turnkey execution package — 7 waves, no repeated ChatGPT prompts

- **[09 — Complete execution playbook](09-EXECUTION-PLAYBOOK.md):** source-to-beta sequence, six disjoint parallel lanes per wave and one serial integration gate, branch ownership, handoffs and GO rules.
- **[One-paste AUTO-RUN-ALL master prompt](prompts/00-AUTO-RUN-ALL.md):** autonomous local orchestrator may drive W0–W6, pausing only for real blockers and explicit owner cutover approval.
- **[Copy-link prompt index](prompts/INDEX.md):** 7 wave launcher prompts, 42 individual worker prompts (A–F), 7 serial integrator gate prompts (G). Every prompt is self-contained when read with this proposal's docs.


## What "full functional beta" means

A clean Windows installation and connected PWA must run **independently of the donor repositories** with the currently functioning capabilities: owner onboarding, authenticated pairing, signed-in-browser AI conversation, existing command/control plan/grant/receipt behavior, internal HQ development intelligence and graph reasoning for selected local projects, context evidence and truthful source status, visual interaction, and safe restart/recovery.

Media streaming, voice-to-tasks, mission state, personal knowledge intake, AutoWiki and user app-building are included **when current source and native acceptance prove they work**; incomplete Labs proposals are integrated as disabled/preview capabilities with exact blockers and are not represented as finished beta features. The release is a complete coherent beta of its declared capability set—not a claim that all aspirational features already ship.

## Source integrity and cutover

Donor SHAs and file-level ancestry are recorded inside ARBOR. During construction, the current Harness may continue running. At cutover the new ARBOR product is the **single** live application/effect writer; external donor checkouts are neither imported dynamically nor called. HQ development data and Host personal World data remain separately permissioned **inside ARBOR**, sharing graph algorithms and typed IDs, not one indiscriminate data store.

## Hard no-go

Any require/import/git path to ../repos-hq, ../tierflow, ../vivim-web-harness, ../vivim-labs; runtime HTTP/IPC calls to donor development processes; unbundled source programs; dual Host journals; second graph store/scheduler; fake "ready" sources; Tauri unpatched IPC/origin boundary; unverified owner bootstrap; PWA with its own effect authority; source-leaking debug telemetry; unconditional provider APIs.

## Phase 2 — Parallel Labs intake during active Wave 1

**[Three ChatGPT-ready Labs lanes](phase2/README.md)** · [Evidence-based assessment](10-PHASE2-LABS-INTAKE.md) · [Migration decision matrix](11-PHASE2-DECISION-MATRIX.md).

Phase 2 separates **easy source/test migration**, **core development/migration preparation**, and **high-value pending development**. It prepares disjoint source-based branches while W1 A–F are active. Phase 2 must not edit worker-owned product paths or merge into the shared integration branch without master G's collision review and approval. No new Labs runtime dependence, duplicate effect authority or feature creep.

## Truthful status

**Proposal only.** The ARBOR branch contains proposal documents, not a completed Cargo/Bun product. No graph import, HQ UI integration, Windows package, native test, source migration or device acceptance is claimed. That is the work this proposal defines.
