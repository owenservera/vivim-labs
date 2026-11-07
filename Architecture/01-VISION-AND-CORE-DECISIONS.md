# One Logical Rust Graph — Architecture Vision

## Core premise
Rust HQKG is not just a knowledge graph engine parallel to the UI engine. It was designed as a graph containing *engines*, *database nodes*, *datasets*, algorithm definitions, executable engine generations, provenance and operational relationships. The platform's unified system model should reside in or be represented through this graph.

A source file, KB fact, database, UI document, engine, CLI command, schema, transformation, job, receipt and release may each be modeled as a node. Typed edges express dependencies, derivations, ownership, implementation, consumption and status. An operation connects a capability to a real executor; the graph can model it without independently having authority to run it.

## Distinctions that must survive consolidation
- **One logical truth ≠ one physical database.** SQLite remains excellent for FTS and structured records; native Host/TierFlow/Visual document stores own their state; graph nodes can reference entire external databases.
- **Everything on graph ≠ everything runs inside Rust.** Trusted native Rust drivers can implement ComputeEngine; UI engines and device/Host runtimes can be represented as external engines and accessed through controlled adapters.
- **Knowledge and executable authority differ.** A document, inferred fact, algorithm paper, capability registry entry or graph edge does not approve a provider/device effect.
- **An engine slot differs from an implementation generation.** A logical slot has stable identity; in-process replacements use CAS generations and leases; graph lineage records swaps.
- **Engine generation differs from dataset revision.** Production dependencies need independently pinned data/schema resources and verified bytes.
- **Search indexes and materialized visual views are rebuildable.** Curated knowledge revisions and receipts are durable.
- **No unverified production claims.** Source-level implementation does not imply native tests, Windows/Android acceptance or deployed use.

## One core vs multiple cores
### A — Single physical core (preferred starting point)
One Rust graph address space and initial journal owner; engines, resources, transformations, operational references and views connect through adapters. Easier identity, consistency, recoverability and integration. Caveat: journal throughput, single-writer contention and failure isolation.

### B — Multiple specialized cores now
Separate knowledge, visual, engine and runtime graphs. Benefits include isolation and scale; costs include distributed identity, replicated authority, multi-writer conflicts, routing, operational complexity. Not recommended before one end-to-end path works.

### C — One logical graph, federated physical Rust cores later
Stable global identifiers; partitions and specialized execution nodes behind the same logical graph. Attractive future evolution, but requires tested federation, cross-core revisions, routing, reconciliation and consistency rules.

**Directional choice:** A now, evolvable into C. No wholesale migration or extraction until evidence justifies it.

## Conceptual planes inside the same graph
1. Data: data stores, datasets, ingestion, streams, transformations, materializations.
2. Knowledge: entities, terms, decisions, facts, assertions, source/provenance, temporal revisions.
3. Compute: engine slots, implementations, algorithms, graph queries, visual computation, workflows.
4. Control: CLI, typed operations, natural-language intent, permissions, health, configuration, lifecycle.

These are conceptual roles within one graph, not four new cores or new source-of-truth registries.

## Proposed graph relationships (illustrative, not frozen contracts)
consumes, produces, implemented_by, depends_on, derived_from, writes_to, provides, requires_authority, verified_by, supersedes, visualizes, invokes, owned_by, sourced_from, served_by, active_engine.

## Principle
**One Rust graph kernel that knows the entire VIVIM system, operates its native graph/compute engines, and coordinates other engines through their actual owners.**
