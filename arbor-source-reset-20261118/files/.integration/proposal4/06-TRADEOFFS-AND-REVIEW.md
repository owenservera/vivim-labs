# Proposal 4 — ADRs, Tradeoffs, Risks and Independent-Opinion Review

**Status:** comparison-ready independent proposal; nothing here is automatically accepted.  
**Comparison baseline:** ARBOR `.integration/proposal1/` on `architecture/node0-prime-backbone-r001`.

## 1. How Proposal 4 differs from Proposal 1

| Question | Proposal 1 answer | Proposal 4 answer |
| --- | --- | --- |
| What organizes the repository? | Rust product crates + apps + bridges | **Capabilities and programs first**; kernel only shared mechanisms, surfaces only deployment/rendering |
| What makes it one system? | One resident Rust Host, one generated contract and source data boundary | **One programmable semantic model** across all existing engines, data sources, programs, views, commands and effects; source owners remain explicit |
| How does first integration start? | New Rust substrate and progressively port app/runtime paths | **Bind existing implementations immediately**, then promote/replace selectively where tests justify |
| How much is rewritten? | More decisive native migration expected in the initial end-state | **Minimum needed for unified contracts, graph composition, first-hop security and UI/dataflow integration** |
| Where is programmable behavior? | Mostly Rust `intelligence`/`node0`/`host` + visual app layer | Typed **ProgramIR** and composable capability ports are the *foundation* of all operations and views |
| Where do UI semantics live? | TypeScript UI consumes native Rust semantic state | Same, **but views are declared as program graph projections**, not a semantic "frontend tier" |
| How are new capabilities added? | New Rust module, JS adapter or UI package within existing ownership | Source-backed capability module co-locates descriptor, ports, visual projection, config, tests and adapters; no new service |
| How is Host authority handled? | Long-term native Rust rewrite of effect controls | **Existing JS Host remains the one effect writer in first integration**; optional Rust reimplementation later after parity |
| How are Labs integrated? | Migrate selected Labs code and contracts into corresponding product crates/apps | **Graduate Labs algorithms, descriptors, views, program recipes and tests** into one capability structure; experimental code never becomes a separate service |
| Main concern | Risk of building a new product largely before source integration | Risk of excessive abstraction and adapters that never retire |

**These are not fundamentally incompatible**, but the order and architectural center differ substantially. Proposal 4 tries to eliminate repeated future full-stack wiring by making the capability contract and programming model the default unit of integration.

## 2. ADR-P4 decisions (to be challenged, not silently approved)

| ID | Proposal 4 decision | Rejected default / alternative | Consequence |
| --- | --- | --- | --- |
| P4-01 | One logical source-bound system truth, not one database | New master graph DB stores copies of all source domains | Retains source ownership and privacy; requires revision/provenance joins |
| P4-02 | Capabilities are the unit of product composition | Apps and endpoints are the primary integration unit | Fewer semantic duplicates; requires disciplined port descriptions |
| P4-03 | Runtime views are program projections over canonical resources | UI stores its own authoritative view models | Avoids frontend/backend drift; requires efficient incremental projections |
| P4-04 | Module folder co-locates contracts, views, programs, source adapters, tests | Scattered frontend/backend directories per feature | Improves traceability; module descriptors must not become a second registry |
| P4-05 | Five stable native mechanism crates initially | Eight domain Rust crates from day one | Smaller first kernel; risk of broad `runtime` or `composition` crates |
| P4-06 | Promote HQKG's existing engine/typed DAG with source-preserving tests | New native scheduler/graph implementation | Reuse, generation leases and real lineage; existing engine may need bounded concurrency upgrades |
| P4-07 | First integration keeps current Host one-writer effect implementation | Immediate Rust rewrite of admission/journal | Safer first cutover; creates temporarily retained JS authority port |
| P4-08 | First integration uses validated source adapters | Whole codebases copied or immediately rewritten into ARBOR | Faster truth convergence; adapters require retirement governance |
| P4-09 | One generated Rust/TS type contract, runtime validation | Hand-authored protocol forks in React/SolidJS/Node/Rust | Schema parity; codegen/toolchain investment |
| P4-10 | Trusted native and declarative extension tiers first | Generic native user-code plugin installation on Day 0 | Avoids unsafe extension privilege; advanced extensibility deferred |
| P4-11 | TierFlow remains rendering/runtime upstream, pinned and adapted | Full upstream rewrite or second TierFlow live server in product | Reuses existing visual investments and prevents duplicate product app |
| P4-12 | HQ dev source intelligence remains external | Installed app bundles full HQ development indexer | Keeps privacy/product size; authorized dev views require a read bridge |
| P4-13 | One responsive UI/PWA with platform-specific transport | Separate semantic desktop and mobile UI codebases | Parity on one contract; needs careful offline behavior |
| P4-14 | AutoWiki, VIEW ALL and app builder share capability graph | Each feature has bespoke backend, state and UI truth | Huge leverage; depends on strongly typed source/view model |
| P4-15 | Owner-configurable quota/retention and Rock Core | Unbounded index/wiki/cache growth | Resource reliability; needs measurable memory and storage budgets |
| P4-16 | Full product cutover once, staged verified integration beforehand | One huge unreviewed commit or perpetual hybrid product | Honest rollback and source continuity; needs disciplined migration proof |
| P4-17 | LLMs and AI agents propose programs/claims, not grants | Model text directly triggers effectful tool invocation | Preserves Host safety, creates explicit review boundary |
| P4-18 | One permission owner even when unifying UI/backend semantics | Everything with access to program graph has effect capabilities | Essential protection against untrusted views and plugins |

## 3. Criticisms of Proposal 4 that a serious reviewer must test

1. **Too generic too early.** A universal capability/IR model could require months of schema discussions. Countermeasure: start with actual Host catalog/status, HQKG pure DAG and TierFlow view; add only fields demanded by real source data.
2. **Artificial uniformity.** Streaming media, browser screenshots, small text updates and graph edges have very different memory/performance needs. Countermeasure: typed ports support opaque streams, chunked evidence and resource handles; don't force megabytes into one JSON graph object.
3. **Descriptor duplication.** A `module.yaml` may be another stale action registry. Countermeasure: capability descriptors are generated/adapted from existing owners; duplicate manual definitions fail contract checks. There is exactly one canonical *action risk/permission* authority.
4. **"No frontend/backend" masking trust.** A single system model cannot make a webview trusted. Countermeasure: contract unity, privilege separation and explicit source-owner authentication are simultaneous requirements.
5. **Legacy forever.** Stable adapter ports might justify never cleaning up JS Host complexity. Countermeasure: every adapter records retirement/measurement, call depth, owner and "replace only if net gain"; migration end-state can retain a necessary browser worker, not obsolete semantic routers.
6. **Graph as false fact store.** An unverified relation could appear authoritative simply because graph connected two objects. Countermeasure: original source refs, claim status and producer coverage mandatory; no graph-edge grants.
7. **Plugin security optimism.** If arbitrary user-defined programs can reach shell/browser, composability becomes an execution vulnerability. Countermeasure: pure operations first, sandbox-specific code only when demonstrably isolated, effects via exact Host journal.
8. **Loss of streaming responsiveness.** A graph-only UI projection could require costly cross-process roundtrips for every motion. Countermeasure: local DOM/TierFlow microstate remains entirely local; only semantic/source transitions use the shared model.
9. **Complex release state.** An adapter to existing Host during first integration may create competing process/installer versions. Countermeasure: exact compatibility matrix, no dual effect writer, source-qualified version pins and cutover dry-run.
10. **No absolute future-proof guarantee.** Versions, platform constraints and source contracts change. The promise is safe substitution and traceable migration, not eternal binary ABI compatibility.

## 4. Seven explicit architectural questions for owner decision

1. Do we want **capability-first directory structure** as the canonical ARBOR organizing principle, or only capability manifests while keeping a conventional apps/crates tree?
2. Is a first-integration **temporary JS Host authority** acceptable if it is the only writer and obeys the new unified contract, with Rust migration deferred? Proposal 4 says yes.
3. Should the shared Rust HQKG library physically live in ARBOR after promotion, or be versioned in HQ and pinned by ARBOR? Either can satisfy one compute implementation; release ownership needs explicit decision.
4. Should module definitions be an ARBOR build-time generated capability manifest from source owners, or static reviewed versioned descriptors with generated comparisons? Proposal 4 favors **source-owned descriptor + generated normalized view**, not independent YAML catalog authoring.
5. Which user-editable configuration categories can safely activate without Host approval? Layout/ranking may be reversible config; any broader data access/effect must be admitted separately.
6. Is the first user-app builder a **view/program composer over trusted existing ports**, or does it include arbitrary user JS/WASM execution? Proposal 4 defers arbitrary code execution.
7. Which source-recovery/compaction policies are acceptable for the smallest user machines, and what Rock Core data must never be evicted without user intent?

None of these are runtime-blocking for writing a proposal. They **are** decisions that must be pinned before declaring the scaffold final.

## 5. Independent proposal scoring

Use the same weighted rubric as Proposal 1, with scores treated as subjective architectural judgments, **not** measured implementation performance:

| Dimension | Weight | P4 strengths | P4 risks |
| --- | ---: | --- | --- |
| User I/O integrity / responsiveness | 20% | Node 0 typed local-edit + bounded graph contract | Needs actual native scheduling implementation |
| Effect/security/authority correctness | 20% | Retains one Host writer, provenance and explicit policy refs | JS↔Rust bridge until migrated; local process auth must be fixed |
| Cohesion / fewer duplicate owners | 15% | One typed capability/state/view truth | Abstraction may become a new catalog if implemented poorly |
| Labs / existing product preservation | 15% | Adapter-first avoids reimplementing working source | Verification of each legacy port is substantial |
| Data portability / cutover | 10% | Source-authority-preserving, one cutover | Complex owner-version migration still required |
| Desktop/PWA/TierFlow experience | 10% | One view projection model | Real IME/renderer parity untested |
| Hardware/resource economics | 5% | Rock Core, bounded cache and pure scheduler | Real latency/memory budget not measured |
| AI dev throughput/maintenance | 5% | Module-contained port/test/view ownership | Danger of ceremonial manifests |

**No numerical readiness score is appropriate yet.** Both proposals are designs, and neither has passed Windows native build, private source ACL, paired PWA or product cutover tests. A P0 failure in security/effect ownership overrides any aesthetic architecture score.

## 6. What a second reviewer should deliver

A reviewer evaluating Proposal 4 should answer:
- Is this coherent as a *single runtime truth* when source owners are separate?
- Which proposed universal types are redundant with existing HQKG/Host/TierFlow contracts?
- Can source-owned module descriptors really be composed without creating another semantic registry?
- Which proposed module boundaries would cause forbidden Rust crate cycles or low performance?
- Does the existing Host data/effect model fit the program IR without lossy transformation?
- Which unchanged code paths must be formally retired at cutover?
- Is one responsive UI enough, or do native mobile/desktop need distinct view components?
- What exact minimal subset can compile and run without private data?
- Can the user build an app/view from existing capabilities with no new authority?
- Are storage/compaction and producer-coverage policies implementable with modest hardware?
- Does Proposal 1's infrastructure-first design win on risk, speed or maintenance compared with this capability-first design?
- Provide a full alternative with exact scaffold, not only a critique.

Keep Proposal 1 intact. Keep Proposal 4 independent. The owner decides whether to ratify one, combine explicit ADRs or request further alternatives.
