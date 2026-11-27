# Mathematical Engines Lab — R001
Experimental Rust foundation. This is a candidate implementation of mathematical computing, not a production runtime or a deployed graph node. Product and ARBOR remain unchanged by this round. Do not treat the candidate IDs as canonical registrations.

## Start from invariants, not a library collection
The lowest level owns scalar representations, explicit numeric domain and assurance, zero/one, checked arithmetic, division semantics, shape and input budgets, and error propagation. Exact and approximate domains cannot silently substitute for each other. A successful bounded exact operation returns an exact representable result; overflow can reject even a mathematically representable result if intermediate values do not fit. Finite f64 allows normal floating-point rounding and underflow; it is not a certified interval or proof. No unsafe code, network, persistent state, hidden global registry, graph scheduler or Host effects.

The current source is dependency-free and separates typed operations from their implementations. New numeric domains implement Scalar (and FieldDivision if appropriate); new operations implement Operation<I> with an explicit capability identity, input/output type and evaluation method. compose uses static type checking to connect compatible operations. Product-specific domain policies are not math primitives. An operation's identity includes ID, revision, numeric domain and assurance; additional full input/output signatures, units, shape constraints, algorithm/backend identity and conformance evidence must be qualified at host admission.

## Expand bottom-up
| Layer | Scope | R001 state |
|---|---|---|
| L00 | Errors, domain, assurance, type and size invariants | Initial source |
| L01 | Integer, rational and finite real arithmetic; explicit quotient/remainder | Initial bounded source |
| L02 | Shapes, borrowed vectors, sum, dot and reference mean | Initial source |
| L03 | Arbitrary precision, decimal, complex, intervals, units, tensors, checked conversions and stable reductions | Planned |
| L04 | Algebra, numerical linear algebra, transforms, geometry, distributions, graph primitives and autodiff | Planned |
| L05 | Statistics, optimization, equations, simulations, symbolic manipulation, numerical solvers | Planned |
| L06 | Specialist disciplines, formal verification, reproducible research compositions | Planned |
| L07 | Existing SDK/graph bindings, user and agent projections, calculator through research profiles | Provisional boundary only |

Extend a lower layer only when its dependency contract and failure semantics are explicit. Reuse qualified library implementations rather than reproducing mature algorithms. Backend upgrades must declare compatible domains, resource/accuracy behavior and versioning; no silent approximate fallback. Use caller-provided memory budgets and deterministic execution settings as applicable. No mandatory LLM or remote service in direct native computation.

## Product / ARBOR provisional adapter
The adapter module supplies a caller-owned Resolver trait, Target::Product and Target::Arbor, and a Binding that records the capability identity, an opaque existing host key and its generation. invoke re-resolves against the caller's current mapping and rejects unbound, stale or mismatched requests before evaluating. A binding is not permission, registry admission, graph registration, actual host communication or execution evidence. No hardcoded external registry keys; the adapter is deliberately internally replaceable. The host remains responsible for validating the adapter implementation, capability contracts, revision, operation qualification and authorization.

Future integration:
- Product: implement Resolver over the incumbent typed SDK/command definitions and expose pure operations to the existing computational graph and interaction layer. Use direct Rust calls for internal consumers. No new crate/graph/registry in Product without owner-approved integration.
- ARBOR: implement the same Resolver contract against its incumbent graph/command mapping. Map semantically equivalent operations to the same qualified kernel; do not copy a parallel graph or replace existing authority.
- Both: maintain host-side binding generation and actual consumer provenance; adopt via source-owner work order, check API compatibility, reject stale revisions, compare actual results and measure overhead. Reconcile physical graph ownership before migrating graph code.
- Until then, adapters are local contracts exercised by caller-owned test implementations, not live bindings. All effects remain with incumbent Host.

## Context7 technical research
- Resolved Rust: /rust-lang/rust. Consulted checked integer arithmetic and overflow semantics; source https://github.com/rust-lang/rust/blob/main/library/core/src/num/uint_macros.rs and https://github.com/rust-lang/rust/blob/main/tests/ui/consts/const-int-arithmetic.rs. Selected std checked operations for the small reference kernel.
- Resolved faer: /sarah-quinones/faer-rs. Consulted owned/view matrix types, shape checks and scratch storage. https://github.com/sarah-quinones/faer-rs. Candidate for L04; not a dependency or qualified backend yet. Dimension mismatch must be checked before dispatch.
- num-rational lookup did not return a matching Rust library in Context7. Arbitrary precision and production rational selection remain pending; bounded reference rational code is experimental, not a replacement for a qualified big-number library.

## Verification and next work
Run `cargo test --manifest-path MATH-ENGINES-LAB/Cargo.toml` at Labs root. Tests include exact small-domain arithmetic crosschecks, overflow, zero and nonfinite cases, shape/input limits, typed composition, both candidate target adapters and cross-domain/stale-binding rejection. Test source has been authored; execution is UNRUN: cloud runtime has no rustc/cargo, public Rust download endpoint DNS failed, workstation terminal command was denied. No claim of compilation, installed capability or live acceptance. Next: obtain authorized Rust execution in Labs, run and repair these tests, qualify exact arithmetic with independent arbitrary-precision oracles, add stable reductions, typed matrix views and the first qualified backend, then test a real internal consumer through each host with owner-approved wiring. No edits to either production repository in this experiment.

## R002 — verified compatibility experiment (2026-11-27)
The earlier UNRUN statement above describes the initial R001 round only. [Compatibility laboratory](compatibility/README.md) now records native Windows verification at source pin `4aeed1678bda358419c79283c683450e93e7a1f1`: 11 frozen-kernel tests and 17 compatibility/calculator tests passed, both exit 0. The original kernel source is unchanged. The standard calculator is headless; the registration system is a Labs-only simulation, not live Product/ARBOR integration. Subsequent capability proposals must use this compatibility cycle instead of modifying the base implementation.
