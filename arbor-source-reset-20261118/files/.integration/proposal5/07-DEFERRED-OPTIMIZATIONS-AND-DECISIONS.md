# Proposal 5 / 07 — Deferred Optimization Ledger and Decision Comparison

## P5 verdict

Choose **source-preserving internal consolidation first**, not an external-facade architecture and not a universal rewrite. This makes the initial ARBOR beta operationally self-contained and creates the typed substitution seams needed to improve each implementation later.

### What cannot be deferred

- Remove every build/runtime/deployment dependency on outside VIVIM repositories or development services.
- Physically import and test HQKG, operational HQ index/search/context/wiki, Host/Studio/PWA, TierFlow runtime and source-tested Labs contributions required for declared beta.
- Keep one authoritative Host effects/World writer and source-bound proof for user-visible claims.
- Patch native Tauri IPC/origin process identity/owner-token handling and secure paired phone.
- Bound native Node 0 computation; correct revision/actor/privacy handling; keep local editor independent.
- Establish one coherent contract for capability IDs, source refs, view projections, configuration and effect states.
- Prove Windows installer + physical paired PWA + real signed-in browser conversation and safe recovery.
- Cap local storage/index/wiki growth and protect essential Rock Core personal data.
- Run all acceptance tests with old donor repos and processes unavailable.

### Optimizations explicitly deferred past beta

| Debt | Beta implementation | Future upgrade and migration trigger |
| --- | --- | --- |
| Host JS → Rust admission/journal | Keep current JS one-writer Host; Rust reads/derives only | Port to Rust only after a shadow comparator and crash/recovery parity; retain single writer |
| HQ JS operational intelligence → Rust | Import selected HQ JS index/search/context/wiki into `ARBOR/hq` | Port specific measured hotpath algorithms into Rust; source results must remain identical |
| Full Node 0 semantic reactor fabric | Bounded native read-only hint/trace path + existing Host NLCL canonical execution | Expand subscriptions to all eligible interpretation facets when real source truth and time budgets justify |
| Universal program/composition engine | Thin typed operation/capability model, reuse existing Rust typed DAG and Host registered actions | Extend to declarative typed programs only after one working cross-domain composition proves necessity |
| Extension/plugin execution | Source-owned declarative capability space descriptors and trusted internal adapters | Sandboxed third-party/user code only with verified isolation/permission model |
| Rich R007 VIEW ALL | Limited source-linked visual understanding, existing UI test results | Full multi-facet hypergraph/visual fidelity with producer coverage after truth pipeline verified |
| AutoWiki universal mirroring | Internal HQ dev wiki for selected code sources; optional bounded personal-source read projection | Dynamic per-screen wiki with deep temporal cross-links, user feedback and quotas |
| Predictive personal model | Small on-device accepted-pattern ranker with honest cold start | More adaptive model only if correct selection/latency benchmarks justify it |
| Voice-to-tasks | Preserve source-confirmed voice workflow and typed task proposal | Real-time multimodal streaming and fine-grained corrections |
| Studio/browser automation | Existing signed-in Chrome/CDP owner with error/uncertain guards | Better provider compatibility and optional optimized capture; no provider API switch |
| UI reorganization | Existing desktop+PWA views with internal TierFlow adapter | One unified responsive UI codebase when screen parity proven, not beta prereq |
| Internal crate/module cleanup | Retain current source paths for import speed | Gradual relocation to capability-co-located modules, no semantic/API changes |
| Offline PWA intelligence | Cache and draft, clearly unverified, no effects | Optional local WASM semantic hints/rebase, without mobile effect authority |
| Media optimization | Preserve tested byte-range/resume/client decoding | Better adaptive prefetch/cache policies and hardware-specific optimizations |
| Rust scheduler performance | Existing bounded shared graph scheduling with cancellation | Work stealing/advanced fair queues/latency class tuning only after measurements |
| Data store consolidation | Existing Host World plus internal HQ dev index, scoped shared graph algorithms | Reduce storage duplication via proven indexing/materialization, not new master DB |
| Disk footprint | HQ low-debug build settings + quota-index/wiki cache | Smarter compaction, content-addressed dedupe and user-adjustable hot/warm/cold policies |

These are acceptable **engineering debts**, not authorization/security correctness waivers.

## Comparing P1, P4 and P5

| Dimension | Proposal 1 | Proposal 4 | **Proposal 5** |
| --- | --- | --- | --- |
| Primary objective | Long-term ideal Rust product shape | Universal programmable system | **Fastest safe self-contained beta** |
| Starting move | Move product into canonical Rust/apps crates | Establish capability/program ontology | **Physically import working source with existing paths** |
| Rust graph | Canonical crate in product | Shared kernel and composable program model | **Source-preserving HQKG import and compile now** |
| HQ indexing/wiki/context | Usually external development read owner | Existing source-owner adapter | **Required selected HQ code imported into ARBOR** |
| Host effect implementation | Aim native Rust | Keep source-owner admission initially | **Keep existing JS writer for beta** |
| TierFlow | Pinned upstream build/package | Visual projection of universal program | **Source-import internal runtime package** |
| Labs | Graduate into product owners | Typed capability contributions | **Import ratified, source-proven contributions with explicit beta status** |
| Schema extent | Broad type/domain crates | Universal typed ports/program/view graph | **Smallest sufficient identity/effect/source/view protocol** |
| Initial organizational churn | Significant | Moderate/high | **Minimal; old relative paths preserved** |
| Runtime dependency on donors | Not desired | Not desired | **Explicit hard FAIL if any donor wiring remains** |
| Long-term optimization | Clean product native architecture | General composability | **Replace internal components behind stable ports** |

**P5's primary disadvantage:** retention of some JS/legacy file paths and a narrower initial programming IR. This is acceptable only if there is a visible retirement/optimization ledger, source-level tests and no duplicate semantic or effect authority. An opaque wrapper pile is not a finished beta.

## Decision points reviewers should challenge

1. Is it safer to import HQKG and HQ operations as two in-repo packages than move only the native kernel and later rewrite developer intelligence? P5 says yes.
2. Should the existing Host/Studio Node runtime remain one internal child process for beta? P5 says yes if privileged IPC is hardened and all releases bundle it.
3. Can TierFlow source be vendor-imported license- and test-cleanly with one active renderer? Must verify.
4. Which Labs components are truly buildable and previously operational vs only owner-approved concepts? Inventory and feature gates decide.
5. Which exact Host/DAG/native API should serve the first runtime Node 0 semantic projection? Choose the smallest actor-scoped read-only integration and measure.
6. What user-facing functions are required for beta vs explicitly deferred owner-approved preview? State in product contract.
7. Can a single release manifest lock Tauri/Host/HQ/Studio/PWA/TierFlow versions and on-disk migrations without external service? Must prove with no-donor integration test.
8. Does the internal HQ package require Node while product already uses Bun? Preserve the existing verified entrypoint and align toolchain only after test parity; don't block beta for cosmetic manager unity.

## Beta-first strategic rule

> **Full centralization is nonnegotiable. Full reimplementation is unnecessary.**

We are trying to avoid a second integration project later. Therefore the *source code and operational contracts* must all physically land under ARBOR now, even if high-performance rewrites and idealized domain moves are deferred. All later changes are within the same repository and release pipeline, not cross-repository wiring.
