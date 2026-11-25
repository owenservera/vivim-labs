# VIVIM V1.0 — Rust graph convergence work program R001

Date: 2026-11-25. Status: proposed, ready for reconciliation with existing ARBOR work. No implementation completion implied.
Parent: [Future-proof intelligence](FUTURE-PROOF-PERSONAL-INTELLIGENCE-R001.md); [source review](RUST-REPROGRAMMABLE-GRAPH-ARCHITECTURE-R001.md).
Production tracker: [ARBOR #33](https://github.com/owenservera/ARBOR/issues/33). Before claiming work, reconcile existing active lanes and current main; reuse their tasks rather than create competing implementations. Rust-first, main-only, existing source ownership, no duplicate graph/scheduler or effect authority. Private World remains separate from development HQKG.

## Era I — Establish the actual baseline
| ID | Deliverable | Acceptance |
|---|---|---|
| RG-A01 | Inventory native product operations and graph drivers | Machine-readable source-pinned map: operation, owner, input/output, consumers, evidence, effect class |
| RG-A02 | Map existing functions to EngineRegistry | Existing operator exercised through registry; no duplicate function or scheduler |
| RG-A03 | Typed value/port contracts | Reject incompatible composition, missing parameters and taint downgrades |
| RG-A04 | Product kernel/DAG convergence | Real product transform invoked through typed DAG; output parity with direct implementation |
| RG-A05 | Meaningful pure operations | Replace fingerprint-only paths where domain implementation exists; test real outputs |
| RG-D01 | Graph memory baseline | Report nodes/edges, peak RSS, disk growth, cold/warm query and import times on owner hardware |
| RG-D02 | Execution latency baseline | p50/p95 direct vs DAG, cold/warm, engine generation, workload size |
| RG-D03 | Compact identity and adjacency investigation | Measured comparison, no migration unless meaningful improvement |
| RG-D04 | Transaction durability design | Fault injection at graph/journal/ledger/registry publication boundaries; restart yields one consistent revision |
| RG-D05 | Engine replacement recovery | Active leases finish, future runs use new generation, restart loads correct manifest, incompatible swaps rejected |

## Era II — Persistent behavioral intelligence
| ID | Deliverable | Acceptance |
|---|---|---|
| RG-B01 | Behavior and Experience versioned records | Typed identity, lineage, applicability, dependencies and evaluation; scoped to owner |
| RG-B02 | Parameterized program persistence | Learn one real task, restart, resolve new argument, run same registered operators |
| RG-B03 | Correction-to-pattern lifecycle | Actual user correction changes next resolution; prior revision retained or deleted per owner policy |
| RG-B04 | Outcome and confidence | Record real success/failure receipts; calibration against held-out tasks |
| RG-B05 | Revision and rollback | Restore previous behavior without changing historical receipt or privileges |
| RG-C01 | Fine-grained dependency invalidation | Unrelated mutation reuses result; changed relevant source, query membership, engine or policy recomputes/refuses |
| RG-C02 | Studio semantics | Real authorized external software state contributes source-bound semantics; no inferred authority |
| RG-C03 | VLANG internal semantics | Source/symbol/capability mapping uses existing semantic extractor and current revision |
| RG-C04 | World/AutoWiki projection | Real local source and correction produce consistent revision/readback; private data absent from HQ exchange |
| RG-C05 | Continuous computation | Bounded resource consumption, coalescing, cancellation, stale-state rejection; no new scheduler |

## Era III — Learning and evolution experiments
| ID | Deliverable | Acceptance |
|---|---|---|
| RG-E01 | Natural language evaluation corpus | Actual registered capabilities and real authorized commands; held-out paraphrases, negation, ambiguity, corrections |
| RG-E02 | Model-free routing evaluation | Exact intent/argument accuracy, abstention, false execution rate, coverage and latency |
| RG-E03 | AI-assisted reusable workflows | First authorized task composed with assistance; subsequent equivalent task uses durable program without LLM |
| RG-E04 | Alternative engine evaluation | Compare compatible implementations on identical task/evidence; publish measured trade-offs and rollback |
| RG-E05 | Inference reduction benchmark | Repeated-task external calls/cost, quality, latency and CPU/RAM/disk compared to baseline |

## Implementation order
1. Reconcile the live task tracker, active Core-IP sessions and current source. Identify exact implementation owners; preserve in-flight changes.
2. A01/D01/D02 establish ground truth; D04 specifies the publication/recovery invariant.
3. A02–A05 deliver a vertical slice using one existing product operation, one typed DAG and exact real inputs.
4. B01/B02/B05 and C01 establish persistence and correctness before automatic learning.
5. B03/B04 and C02–C05 extend the vertical slice; E01/E02 run alongside semantic work.
6. E03–E05 prove repeatable learning and engine portability. Advance only by actual evidence; no artificial release bureaucracy.

First recommended real workflow: deterministic project status/impact computation using the existing development graph engine, followed by a Host-owned Wiki update. Do not introduce a new Wiki writer. Persist the reusable pure program and resolve current authorized target at invocation. After restart and an unrelated source change, measure correct reuse; after a relevant source change, measure recomputation. Actual external effects remain separately authorized and receipt-backed.

## Evidence contract
For each task record source SHA, files, exact command, exit code, test workload, timings, limitations, and status: proposed / source implemented / tested / deployed / physically accepted. Record benchmark distributions and hardware. No synthetic execution receipts or substitution of a catalog entry for a working engine. Use the existing ARBOR tracker as execution authority, this document as research and acceptance crosswalk. All 25 tasks start unverified by this document.

## Long-term invariants
Personal intelligence remains portable independently of any LLM, neural architecture, accelerator or provider. Use existing engine slots for future symbolic, neural and hybrid operators. Record program and operator revisions; verify current provenance and authorization before execution. Support owner inspection, correction, rollback, export and deletion. Do not claim general intelligence from bounded command success.
