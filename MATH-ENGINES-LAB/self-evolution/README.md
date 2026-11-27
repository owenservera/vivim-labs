# R002 — Self-evolving mathematical verification laboratory

## Scope and design
Implement Design 10 (hybrid evidence replay + witnesses), with Design 11's use of registered mathematical operations for ordinary ledger calculations. Standard calculator only. All work in Labs; frozen kernel, compatibility simulator, original calculator and prior proof-mining binary remain unchanged. No Product/ARBOR source changes. No real consensus, dynamic code installation or autonomous activity outside this bounded program.

The loop generates test workpieces, discovers counterexamples, retains them as admission fixtures, searches an executable grammar of compiled repair recipes, registers accepted revisions on a cumulative snapshot, and replays past revisions. In addition, it generates verifier-rule omission mutants and matching corrupt receipts. Full verification uses SHA-256 receipt integrity, predecessor/epoch/recipe identity/suite binding, completeness, ordered case identities, two distinct witness slots, independent bounded integer/rational oracle and algebraic invariant, and local replay. The ledger's epoch advances via registered ADD. The trust anchor, independent oracles and admission policy are not under the authority of a candidate.

## Ten experimental miners
The same 12 injected primitive defects (3 fault patterns each for add, subtract, multiply, divide), 3 deterministic seed trials per defect, and maximum 48 unique input cases per trial. Discovery seeds are not cryptographic entropy. Three repeated seeds on the same mutation are not three independent defects. Miner count is actual registry operation calls until first failure or budget; excludes fixture setup, repair and ledger checks. No learned evidence in first comparison (Novelty then falls back to Uniform). Bound universe: integer pairs [-10,10]^2, including division by zero. Planted fault patterns are small and correlated. This tests comparative search strategies, not all calculator correctness.

| Miner | Concept | Detected / 36 trials | Checks |
|---|---|---:|---:|
| Uniform | Random baseline | 16 | 1244 |
| Frontier | Numeric boundary exploration | 24 | 840 |
| Mirror | Sign/symmetry twin inputs | 16 | 1160 |
| Inverse | Identity/zero/-1 inputs | 12 | 1488 |
| Combinatorial | Deterministic cross-interaction lattice | 0 | 1728 |
| Novelty | Explore neighborhoods around retained failures | 16 | 1244 |
| Evolution | Mutating input random walk | 8 | 1380 |
| Reverse | Start with desired output and generate input pairs | 13 | 1303 |
| Committee | Interleave diverse input families | 12 | 1488 |
| AdversaryOfAdversary | Transform previous counterexamples and attack zero axes | 20 | 1620 |

Frontier is the strongest measured initial miner on this corpus; no claim that it generalizes. Combinatorial's sampled schedule missed every injected trigger: useful negative evidence. Novelty's initial score equals Uniform by construction. The adaptive loop uses historical successes/checks and explores unused miners before choosing a previously successful one; it found 6/12 within the initial search, then fell back to deterministic complete finite search for the other six. Adaptive search checks 409, fallback checks 1797. No speedup over exhaustive search has been established.

## Verified closed-loop results
Source pin 80d84adbf13324cf5d6208e93a1c5e761e405a98. Native Windows execution, session ctl-65a0ac7a9d330eac, exit 0. Sixteen tracked blobs checked before compile, no local checkout modification.
- 360 miner/mutant/seed trials; ten algorithms executed with real registered Rust arithmetic.
- 12 distinct counterexamples mined, 12 defective reproposals rejected after adding regression fixtures.
- 75 candidate recipe attempts, 12 replacements selected and registered; 5292 finite-domain recipe checks, plus receipt/replay checks and another 5292 historical-revision checks.
- Registry grows from 9 to 21 capabilities, generation 2 to 14. Revisions 10–12 per operation coexist and replay; earlier registered capabilities remain unchanged. Recipes selected: commuted add, add with negation for subtract, commuted multiply, direct divide. This is selection/synthesis within 10 precompiled templates, not arbitrary generated Rust source. Candidate probes use v2; final receipts and replays bind newly admitted revisions. Original v1 calculator interaction profile still computes 0.1 + 0.2 = 0.3 on cumulative registry. It has not been rewired to consume new revisions.
- Eleven of eleven checker-rule omission mutants distinguished from full checker: parent, epoch, identity, suite, digest, witness scope, uniqueness, complete claims, ordered cases, passing claims and local replay. Uniqueness omission yields defer instead of reject, not an unsafe acceptance. Two forged passing transcripts for a genuinely faulty provider are rejected by full gate, accepted by the deliberately replay-disabled mutant. One witness slot -> defer. Witnesses are local roles, not authenticated signatures or independent network nodes.
- First iteration b09dbf8fd44c1b944eb8578c2ddb88e39f00dbd9 also passed, with direct repair choices, 30 attempts and eight receipt corruption cases; final iteration removes unused helper, adds adaptive policy, composed recipes and checker mutation tournament.

## Wild next-generation variants to explore, still inside calculator
W01 Immune system: synthesize attack antibodies from minimal counterexamples and evolve them into adjacent domains; retain provenance and decay redundant tests.
W02 Dream machine: invent adversarial calculator environments and replay long stateful key sequences while idle; minimize any divergence before registration.
W03 Mathematical inverse universe: generate problems backward from required results, including singular and impossible cases.
W04 Proof predator/prey: coevolve candidate implementations and checkers; reward new discriminating evidence, not number of passing tests.
W05 Parallel mathematical worlds: isolate conflicting arithmetic models (rounding, percentage semantics), exchange only typed invariants proven compatible.
W06 Evolutionary archaeology: resurrect historical capability revisions and test future changes against all prior workpieces, including rollback and recovery.
W07 Proof compression organism: identify redundant witnesses and create smaller certificate sets; require that omitted-rule mutants still fail and retain a full replay escape.
W08 Counterfactual ledger: fork a verified checkpoint, run deliberate primitive corruption, measure downstream state divergence and compare recovery paths.
W09 Synthetic theorem factory: infer candidate algebraic laws from examples, then try to falsify them with an independent search or formal checker; never treat inference as proof.
W10 Self-designing verifier: synthesize minimal checker programs from immutable policy contracts, test against generated mutants and independent holdout attacks, only promote with predecessor/external-anchor verification.
These are research designs, not ten additional completed implementations. The ten miner prototypes above are the measured round; the wild variants define the next broader exploration.

## Limits and next action
No unbounded self-modification, self-granted authority, production source edits or external effects. Finite test evidence is not proof for all rational inputs or arbitrary programs. Candidate identity is recipe-label hash, not source/build artifact digest. The two witnesses run in the same process and share the tested provider; independent algebraic checks are bounded, not independent authenticated infrastructure. Ledger head and learned corpus live in memory for this run; not persisted or recoverable across restart. Candidate revision binding is explicit, but no actual live activation of a replacement calculator method, no source generation and no network. Next implement a durable content-addressed evidence/recipe store with replay on restart, then a typed expression grammar interpreter/compiled provider binding that can synthesize new compositions, and a version-selectable calculator adapter. Continue with independent hidden faults, decimal/fraction inputs and stateful calculator sequences. Keep bootstrap/recovery and safety anchor independent. Do not proceed to scientific calculator until standard conformance is qualified.

Run: cargo run --offline --manifest-path MATH-ENGINES-LAB/self-evolution/Cargo.toml

## Governing continuation (R003)
[Constitution R003](../CONSTITUTION-R003.md) governs subsequent self-evolution: each contestant must generate its own bare-metal seed, bootstrap, adversarial and implementation machinery, consume qualified mathematics in its chain, and improve hardware-native promotion ability before advancement. Existing recipe-selection results remain historical evidence.
