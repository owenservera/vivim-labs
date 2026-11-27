# Mathematical Evolution Championship — governing constitution R003

**Status:** owner-directed research constitution, 2026-11-27. Governs all subsequent mathematical evolution experiments in this directory. It supersedes earlier tournament scoring and promotion criteria, not historical results. Labs only, standard calculator only. This is a requirement specification, not a claim of implementation or qualification.

## 1. Foundational constitution

| ID | Binding requirement |
|---|---|
| F01 | Begin with universal atomic mathematical operations: addition, subtraction, multiplication, division, negation, remainder, comparison; define representation, domains, error and overflow semantics explicitly. |
| F02 | Atomic computational implementations originate in hardware-native instruction sequences (assembly/machine code), not Rust reference arithmetic. Bare metal here means CPU-native computational code; no requirement to boot without an OS. |
| F03 | All 64 independent blockchain contestants generate their own seed bare-metal mathematical core proposals. A common specification and independent oracle are allowed; a common supplied implementation is not a substitute for independently generated seed proposals. |
| F04 | Each contestant's actual blockchain arithmetic and state transitions execute through its own qualified seed implementations. Merely certifying external math is insufficient. |
| F05 | Each contestant designs and independently validates the entire machine-code-to-Rust bootstrap: ABI, calling convention, representation, alignment, register preservation, memory boundaries, error semantics, artifact identity, linking and execution. Revalidate at every relevant upgrade. |
| F06 | Rust binds to, consumes and composes qualified hardware-native capabilities rather than duplicating their computation. Independent reference/oracle implementations are expressly permitted for verification, not silent production fallback. |
| F07 | Extend through versioned capability registration, typed contracts and dependency resolution; discover and supply gaps without continually rewriting the foundational kernel. |
| F08 | Provide the same qualified mathematical utility to user-facing and internal VIVIM consumers; production integration remains provisional until source-owner authorization and compatibility verification. |
| F09 | Bidirectional coupling: blockchain qualifies mathematics; qualified mathematics computes actual blockchain state. Faults in active computational primitives must be observable as chain failure or divergence. |
| F10 | Evolutionary advancement is conditional on independently measured improvement in the chain's ability to create, verify and promote hardware-native computational capabilities, relative to its predecessor. A valid block or winning a match alone is insufficient. |
| F11 | Mine reusable mathematical evidence, counterexamples, qualified capabilities and improved verification/generation techniques, not currency or nonce puzzles. |
| F12 | Each contestant generates and improves its own adversarial checking systems, including adversaries targeting its verifier. |
| F13 | Each contestant generates and improves its own implementation-generation and hardware-promotion machinery. |
| F14 | Long-term direction is closed-loop computational self-creation, including generation, verification, composition and reconstruction of its own machinery. |
| F15 | No candidate is the sole authority for its own qualification. Separate predecessor, independent oracle, holdout evaluation and minimal safety anchor. |
| F16 | Maintain independently controlled minimal genesis and recovery foundations. No self-granted authority, unrestricted execution or silent relaxation of correctness. |
| F17 | Capability identity, version, domain, ABI, source/executable artifact, dependencies, evidence and activation/rollback lineage must be traceable and reproducible. |
| F18 | Failed experiments yield retained counterexamples and useful negative evidence, including failed architectures and promotion attempts. |
| F19 | Mathematical correctness, ABI integrity and safety are non-negotiable. Performance is not compensation for incorrectness. Finite test evidence is not universal proof. |
| F20 | Standard calculator is the entire initial research universe. No scientific computing expansion until the owner qualifies the foundational cycle. |

Hardware-specific optimizations must state their target CPU/ISA and feature requirements, and compare equivalent numerical semantics. Machine code is not intrinsically faster than compiler output; use measured results. Cryptography and OS/runtime infrastructure are not automatically required to be reimplemented using calculator arithmetic. Record independently controlled exceptions and do not confuse a candidate with an authorized production backend.

## 2. Independent genesis deliverables for every contestant

| ID | Required artifact |
|---|---|
| D01 | Independently generated bare-metal universal arithmetic seed and exact executable bytes |
| D02 | Machine-code execution model, isolation and target ISA |
| D03 | Machine-to-Rust bootstrap, ABI contract and conformance evidence |
| D04 | Mathematical representation, domains and failure semantics |
| D05 | Versioned capability registration mechanism |
| D06 | Dependency graph, compatibility preview and atomic admission |
| D07 | Blockchain architecture that actually consumes its seed arithmetic |
| D08 | Mathematical evidence, provenance and durable receipt representation |
| D09 | Independent verification and trust/recovery anchor |
| D10 | Adversarial test generator and checker-attack generator |
| D11 | Implementation generator and search grammar |
| D12 | Hardware promotion mechanism, baselines and measurement protocol |
| D13 | Recovery, rollback and requalification mechanism |
| D14 | Persistent evolutionary memory and replay |
| D15 | Resource allocation, budgets and measurement |
| D16 | Closed-loop self-improvement protocol, including improvement of generation and verification machinery |

64 contestant manifests, distinct architecture/strategy and seed proposals, pinned identities, isolated histories, explicit known limitations. Shared initial mathematical requirements, not a shared supplied seed implementation. Prior 21 research designs are candidate inspirations; remaining 43 must be materially distinct architectures/strategies, not merely names or random seeds. Recombination is valid when actual architectural behavior differs. Each entrant is responsible for its own bootstrap, candidate generation, adversarial machinery and evidence. Independent evaluator controls holdouts; contestant may not read them before proposal freeze. No claim that existing policy approximations satisfy these deliverables.

## 3. March Madness championship

Four conferences of 16, 64 entrants, 6 elimination rounds, nominally 63 matches. Seeding based on independently measured calibration, with holdout evaluation separated. Every surviving entrant receives progressively harder tasks. A match may produce no qualified winner; never force a promotion to fill a bracket. Where attrition yields an empty slot, record it explicitly rather than treating a bye as qualification.

| Round | Initial entrants | Required challenge |
|---|---:|---|
| R1 — Genesis stress test | 64 | Stress-test each independent seed, mathematical and ABI correctness, machine-to-Rust bootstrap and actual chain arithmetic. Every entrant must show qualifying advancement under F10. |
| R2 — Computational evolution | 32 | Generate and qualify superior hardware-native implementations; quantify promotion capability improvement. |
| R3 — Sweet Sixteen | 16 | Compose higher-order standard-calculator operations, identify and register missing dependencies, preserve prior contracts. |
| R4 — Elite Eight | 8 | Improve adversarial generation, challenge candidate arithmetic, ABI, ledger and verifier itself; retain novel witnesses. |
| R5 — Final Four | 4 | Improve the chain's own generator, verifier and promotion machinery; demonstrate recursive improvement against independently controlled requirements. |
| R6 — Championship | 2 | End-to-end self-evolving standard calculator, hardware-native execution, bootstrap, recovery, evidence and performance. |

### Match requirements

T01 Equal hardware, time/compute/search budgets and comparable measurement conditions.
T02 Independent holdout workpieces, sealed from contestants before proposal commitment; no training contamination.
T03 Increasing challenge complexity; prior obligations remain in force.
T04 Candidate-versus-predecessor and independent-reference equivalence, including errors and boundaries.
T05 Actual mathematical and actual blockchain computation stress tests.
T06 Inject incorrect arithmetic, state corruption, ABI failures and verifier faults; demonstrate detection and containment.
T07 Test machine-to-Rust interoperability on every relevant promotion, not merely a successful compile.
T08 Test absent dependencies, incompatible registrations, failed and atomic upgrades.
T09 Test upward promotion, downward restoration and re-promotion; append recovery events, do not rewrite history.
T10 Generate adversaries against verification and adversarial checking systems themselves.
T11 Measure actual machine execution: reproducible workloads, repeated timings, resource use, instruction/code size where appropriate; record uncertainty, target, compiler and conditions.
T12 Reward novel independently verified evidence, not duplicated tests or inflated claims.
T13 Compare contestant to its own previous generation on the same independently controlled promotion challenge.
T14 Incorrect or unsafe promotion disqualifies the candidate/entrant from advancement; fastest incorrect code is still incorrect.
T15 No automatic winner or advancement when neither entrant qualifies.

### Governing advancement decision

Ordinary observation, rejected proposal and counterexample receipts can be appended to an experimental evidence journal without implying evolutionary advancement. The evolution head advances only when ALL of the following pass: independent correctness and ABI qualification; complete evidence and dependency binding; actual use by chain and Rust consumer; previous qualified capability regression and recovery; independently measured positive improvement in promotion ability; no unsafe accepted promotion. Promotion ability means better success/coverage of genuinely new or improved hardware-native candidates under an equivalent resource budget, or equivalent success with demonstrably reduced resources, under comparable independent tasks. Quantitative threshold, repeat count and workload must be specified before each tournament round, not changed after results. More tests, faster hashing, more proposals or a good score alone do not qualify. Record rejected/deferred states and retain previous active head. Performance tradeoffs may be represented as Pareto improvements rather than forced one-dimensional speedups.

Among eligible contestants only, use a predeclared tie-breaking/scoring rubric: promotion success, execution gains, generation and adversarial discovery quality, verification resource efficiency; preserve exact measurements rather than a weighted score alone. No weighted score can override the hard requirements. Earlier 30/25/20/10/10/5 proposal is illustrative and not a settled threshold.

## 4. Closed-loop evolution (each entrant)

1. Discover mathematical gaps, inefficiencies and missing dependencies.
2. Invent implementations, instruction sequences and compositions.
3. Attack candidate, bootstrap, ledger and verifier with self-generated adversarial checks.
4. Learn from minimal counterexamples; retain evidence and update future fixtures.
5. Repair through autonomous bounded synthesis, composition or new generation.
6. Verify correctness, contracts, artifacts, independent oracles and holdouts.
7. Promote demonstrably superior hardware-native implementations.
8. Bootstrap into Rust, prove ABI correctness and bind the qualified revision.
9. Assimilate into actual blockchain computation and calculator consumers.
10. Reconstruct/improve generation, verification and promotion machinery using the evolving math.
11. Advance only on F10 and all mandatory checks; otherwise retain previous qualified state. Repeat.

Every proposed capability has explicit requirements; missing capabilities trigger another registration cycle; do not mutate frozen reference source to evade the registry. Receipt provenance includes candidate and executable digest, target ISA/ABI, dependency versions, oracle and suite version, measured results, preceding generation, decisions and rollback lineage. Keep independent verifier/anchor control separate; generated native code must run inside appropriately bounded isolation, not arbitrary unrestricted execution on an owner workstation. Never rewrite prior proof history.

## 5. Inheritance and seasons

After a match, independently qualified discoveries from any contestant enter the shared research evidence pool with origin, scope, limitations and reproduction instructions. Eliminated entrants retain their evolutionary history; invalid discoveries remain counterexamples, not accepted capabilities. During a season keep sealed holdouts and entrant isolation; share discoveries at declared boundaries only. Future seasons may generate 64 architectures via recombination, mutation and independent invention. Requalify inherited implementations on receiving ABI, hardware and semantic domain. No unapproved Product/ARBOR effect, new production graph or duplicate canonical registry.

## 6. Current evidence and implementation obligations

The prior compatibility experiment, eleven proof-mining policies, self-evolution experiments and initial 64-player championship are preliminary research. They used mostly Rust arithmetic and shared registry, policy approximations and bounded fault suites. The initial 64-player championship at fc8e896d6f4d84d2aa5222e9e9b8495ad45afee5 completed 63 matches; relative winner Predator+Archaeology+SelfVerifier accepted 8 invalid holdout proposals, so no qualified champion under R003. Earlier self-evolution selected from precompiled recipes and simulated witnesses. The bare-metal directory is a preliminary x86-64 Windows assembly/ABI experiment, not 64 independent seeds or a qualified backend. Do not infer assembly correctness, actual bootstrap qualification, independent code generation, distributed consensus or implementation of this constitution from these records.

Next: freeze this constitution, produce a 64-entry architectural/genesis manifest, common independent math/ABI stress specifications and sealed evaluation design, then generate entrant-specific seeds and bootstraps, build and run R1 under the governing advancement decision. Record exact commands, exits, source and artifact digests, per-entrant gaps and remediation. Do not start later rounds until their entrants qualify. All decisions are research-only; no production integration implied.

## Governing principle

The blockchain earns the right to evolve by becoming better at creating, verifying and promoting the computational machinery that makes its own evolution possible.
