# ARBOR Proposal 4 — One Programmable System

**Codename:** Living System Architecture (LSA)  
**Status:** INDEPENDENT ARCHITECTURAL PROPOSAL — not adopted, shipped, built, or merged  
**Date:** 2026-11-12  
**Scope:** First full-stack integration, future-proofing of the *existing* capabilities, not a feature rewrite.  
**Branch:** `architecture/proposal4-composable-system-truth`; ARBOR `main` is unchanged.

## Thesis

**A VIVIM experience is a projection of a single, programmable, typed living system — not a frontend communicating with a separately designed backend.**

The user sees a conversation, visual canvas, wiki, workflow, task, or app. These are not independent applications with separate truths. They are *different views and interaction languages over the same identity, capability, dataflow, provenance, state and effect contracts*. The system stays programmable, composable, extensible and configurable without allowing every contribution to invent another kernel or authority.

The first integration **does not rewrite every implementation into Rust**. It makes existing production-relevant engines interoperable through native *capability adapters* and one typed composition/execution protocol. We progressively replace adapters with native implementations only when correct behavior and safety have been demonstrated. The one-time change is in the **architecture and canonical wiring**, not in rewriting every line of code.

The key inversion from Proposal 1:

> **Proposal 1 organizes a new product around apps and Rust infrastructure crates. Proposal 4 organizes the product around one system model, composable capabilities and live views; individual implementations (Rust, TypeScript, existing JS, TierFlow) sit *inside* that model.**

No runtime UI/backend distinction in capability identity, declarative program model, action names, source links, configuration or state transitions. There **are** unavoidable physical and security boundaries: local browser editing, native Host trust, device networking, Chrome adapter isolation and privileges. "One system" never means "trust every view" or "use one undifferentiated database."

## Seven principles

1. **One system truth; multiple source authorities.** A fact has a canonical semantic identity, source owner, typed provenance, revision and status. The Host owns personal World/effects; HQ owns development intelligence; TierFlow owns independent renderer code; user devices own unpublished drafts. A queryable logical graph joins only authorized facts. It does not override source ownership or create a second authoritative store.
2. **Everything useful is describable as a capability.** A capability has stable identity, typed ports, preconditions, supported placements, source/effect scope, semantics, version, health and evidence. A visual control and a text command resolve to the **same capability**.
3. **Everything can compose into typed programs.** A program is a versioned, inspectable graph of data sources, pure transformations, conditional/temporal operations, view projections and *effect proposals*. Side effects cross one separate Host-owned admission/receipt boundary.
4. **Views are part of the system, not a second truth.** UI is an observable, parameterized projection of the same graph; an edit is a typed input event; a widget is a view with explicit allowed commands. TierFlow owns rendering details, not canonical interpretation, grants or World state.
5. **Extension is a port, not a backdoor.** Existing proven extension hooks become first-class typed ports. Declarative assets are easiest; trusted native engines can be version-pinned; untrusted computation requires real isolation. No arbitrary native plugin grants.
6. **Configuration is inspectable, reversible state.** Profiles, preference rules, capability bindings, view recipes, privacy levels, storage budgets and program versions are owner-scoped data with diffs, preview, activation and rollback. No hidden settings logic in separate UI/backend flags.
7. **Integrate before replacing.** Wrap, contract-test, project and measure current engines. Promote only the necessary shared foundations (Rust graph, Node 0 event fabric, semantic IR). Retire old routes at one controlled cutover; no blind dual-write.

## What this changes about ARBOR's shape

Instead of first creating eight new Rust feature crates and independently planning all UI apps, Proposal 4 begins with four product backbone concepts:

- **System model:** canonical IDs, source-ownership handles, typed signals/resources, versioned capabilities, provenance/evidence, configuration and effect states.
- **Composition engine:** a native, bounded, versioned graph of *existing* compute engines, adapter ports, transformations, temporal dependencies and view projections; no new graph DB.
- **Capability modules:** product domains co-locate their manifest, source adapters, semantic bindings, view descriptors, tests and migrations rather than scattering their identity across frontend/backend directories.
- **Surfaces:** one responsive application shell on desktop/PWA, CLI, optional external integrations. Each surface renders or emits the same contract. A renderer may cache, but never become a second source of truth.

Rust is the trusted native substrate and long-term home of hotpath semantics. Existing JS Host/control, Studio Chrome driver, HQ graph/indexing and TierFlow visual runtime are **source-backed bridge modules in the first integration**; they are not automatically rewritten.

## Key distinction: no "frontend/backend" split *in the product language*

| Conventional approach | Proposal 4 |
| --- | --- |
| Backend endpoint `getTasks()`, frontend separate `TaskCard` | One `task.list@version` capability; typed result and views both bind to the same source ID |
| UI independently decides action risk | One Host-owned action/effect descriptor, projected with current capability and policy state |
| Wiki content stored separately from source graph | AutoWiki is a versioned read projection over actual accepted, source-linked facts |
| Natural language and visual controls have distinct command meanings | Both compile to the same typed operation/program node |
| Workflow built as bespoke agent script | A declarative composition of existing typed capability ports, with Host effect admission |
| Extensions install opaque privileges | Extensions declare contributions/ports, data scope and compatibility; owner grants are separate |
| Personalization hardcoded as app settings | Owner-scoped, versioned configuration and accepted learning signals transform projections |
| New feature requires new service + UI + APIs | Register one capability module, compose existing primitives and publish permitted views |

## First-integration commitment

**Do not write a new application by reimplementing the current components.** First make an ARBOR build that can ingest a revisioned user input, inspect current capability registrations from their original owners, resolve source data from existing Host/HQ bridges, compute with the existing HQ Rust graph kernel, produce a TierFlow-readable view, and enforce the current Host journal on any effect. Then migrate only the hotpath pieces that need replacing.

The first goal is **one end-to-end integrated system that knows which parts are native, bridged, unavailable or not yet verified**. It must be able to use that same truth to drive its user interface, auto-wiki, CLI and test/inspection views.

## Deliverables in Proposal 4

1. [01 — Exact capability-first repository backbone](01-BACKBONE-AND-DEPENDENCIES.md)
2. [02 — One system truth, graph state and projection contracts](02-ONE-SYSTEM-TRUTH.md)
3. [03 — Programmability, composition, extensions and configuration](03-PROGRAMMABILITY-COMPOSABILITY.md)
4. [04 — Existing-source-to-capability transformation and Labs graduation](04-SOURCE-PLACEMENT-AND-LABS.md)
5. [05 — One-time integration sequence, non-rewrite migration and acceptance](05-INTEGRATION-WAVES-AND-ACCEPTANCE.md)
6. [06 — Challenges to Proposal 1 and review rubric](06-TRADEOFFS-AND-REVIEW.md)

## Explicitly *not* approved by this proposal

- No new global data store, authority/governance layer, semantic router, event broker, plugin marketplace, cloud backend or independent microservice per module.
- No assertion that the current Labs extension manager, R007 VIEW ALL, AutoWiki, Native R001 or Rust graph integration is production ready.
- No statement that all existing implementations are compatible; compatibility must be measured.
- No Rust rewrite of signed-in browser automation or TierFlow rendering just for language uniformity.
- No loss of Host's exclusive effect writer or its uncertain-send protection.
- No requirement that phone PWA can execute effects while the PC is offline.
- No automatic provider API integration. Signed-in browser apps remain the provider control model.
- No unapproved reuse of old private `vivim`/`vivim-platform` code, secrets or tracked build artifacts.

## Success definition

At the end of first integration, the user experiences **one system**, each action/view/graph calculation speaks the same typed identity language, existing capabilities operate in place through validated adapters, and every claimed source/effect state is observable. Competing proposal reviewers can still reject this architecture; this document commits no decision on the owner's behalf.
