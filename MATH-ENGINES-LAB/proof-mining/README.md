# Proof mining — R001 calculator-only closed-loop ledger comparison

## Owner objective
Mine useful verification evidence and counterexamples, not tokens or nonce puzzles. Compare ten designs plus the owner's eleventh: the mathematical source methods themselves execute the ledger's computations. Lab only; frozen kernel, calculator and compatibility simulator remain unchanged. No Product/ARBOR changes.

## Experimental controls
Native Rust binary using the actual existing registry and calculator. sha2 0.10.9 resolved offline; Context7 /websites/rs_sha2_sha2 documented Digest API. Length-prefixed SHA-256 links bind candidate label, evidence, dependency label, parent and epoch. Candidate revision 2 percent provider can be correct, always wrong, or wrong only at 37%. Caller fixtures admit the hidden bug but reject the always-wrong version. Each admitted candidate has 12 independent exact-percent cases and six real unchanged calculator sequences. Percent v2 calls are direct registry workpieces; the unchanged calculator profile still binds percent v1. Five valid and eleven invalid scenarios exercise missing/truncated/forged evidence, replay, wrong head, revoked or stale dependency, three colluders, offline peers and bad mathematics. Labels for known truth are authored scenario expectations. This is a deliberately small controlled experiment, not a realistic adversarial network.

## Native results
Source pin 47f7c1ecd08d3dac6318fd37448ca373ca684e4e, Windows session ctl-db590bf0695036aa, exit 0. Thirteen tracked blobs verified before compilation. 11 policies x 16 scenarios = 176 decisions. Policy case checks are executed against real operations, but counts exclude common trial preparation, admission, hash helpers and separate lifecycle experiments. Votes are simulated slots, not signatures. No wall-time or network-performance comparison.

| Design | Valid accepted / 5 | Invalid accepted / 11 | Valid deferred | Invalid deferred | Replay/operation checks |
|---|---:|---:|---:|---:|---:|
| Hash-only | 5 | 9 | 0 | 0 | 0 |
| Smoke replay | 5 | 9 | 0 | 0 | 14 |
| Full replay without state binding | 5 | 5 | 0 | 0 | 234 |
| 3/5 majority | 3 | 7 | 2 | 0 | 1044 |
| 4/5 supermajority | 2 | 6 | 3 | 0 | 1044 |
| Optimistic challenge (one selected workpiece) | 4 | 5 | 1 | 1 | 12 |
| Dependency DAG + claimed evidence | 5 | 2 | 0 | 0 | 0 |
| Bound checkpoint replay | 5 | 1 | 0 | 0 | 162 |
| Certificate replay abstraction | 5 | 4 | 0 | 0 | 198 |
| Bound hybrid replay + two online witness slots | 4 | 0 | 1 | 0 | 162 |
| Embedded recursive verification + independent anchor | 5 | 0 | 0 | 0 | 106 |

The checkpoint accepted a truncated transcript despite correct math. The embedded policy explicitly checks evidence length and compares predecessor/candidate on the planted failure case before full replay. Its lower count is partly early rejection of the known defect; this is NOT a general efficiency win over a checkpoint with the same evidence and early-rejection rules. The hybrid defers the valid one-online-peer case by design. No distributed consensus guarantee follows from these results. The certificate policy is a replay abstraction, not a SNARK, STARK or formal proof. The DAG policy is a dependency-binding abstraction, not a distributed DAG protocol.

## Closed-loop mining and up/down
Exhaustive ordered search of integers -100..100 discovered the planted 37% defect at case 138. Expected 37/100, defective result 38/100. Feed this counterexample into the caller-owned admission fixtures. Bad reproposal rejected atomically; corrected provider admitted. Four revision activations 1 -> 2 -> 1 -> 2 -> 1 retained correct outputs; epochs advance rather than rewriting history.

## Eleventh design: math IS ledger computation
embedded.rs demonstrates two linked experiments:
1. A naive candidate verifying itself accepts its own wrong 38/100. Predecessor + small independent invariant rejects it; four up/down transitions preserve the chain. Injecting a bad active percent binding halts ordinary progress; explicitly limited recovery to the known predecessor appends a recovery transition at epoch 6. Altering a committed field changes the digest.
2. Actual ledger state is calculated through the existing registered source methods: ADD for block height and verified-work accumulations, SUB for outstanding cases, DIV for coverage, MUL for weighted verified work. The resulting numeric state is hashed into the block. New ADD revision 2 delegates to registered ADD revision 1; faulty variant has a planted 5+1 -> 7 bug that passes smoke qualification. Four normal ledger transitions give height 4, work 72, remaining 0, coverage 1, weighted work 36. Injecting defective ADD v2 changes the actual next state and block hash. A minimal independent state invariant rejects that transition, retains the previous head/state, and registered predecessor ADD recovers to height 5 and verified work 90. This is executable code reuse, not just a record of verification.
 
## Recommendation
Choose embedded recursive verification with a small independently controlled genesis/recovery anchor for future research. Combine it with complete dependency- and source-bound evidence, immutable snapshots, counterexample feedback, predecessor/candidate agreement and incremental checkpoint replay. Add authenticated witness agreement only when actual multi-node authority is needed. Do not make a candidate the sole verifier of itself. Ordinary math execution must use registered source methods; anchor checks remain separate from ordinary execution. Hashing uses RustCrypto, not the calculator's generic arithmetic. Provider selection is compiled in this experiment; hashes bind candidate labels, not complete source/build artifact digests yet. A production upgrade needs signed or authenticated source/build identities, pinned executable artifacts, durable evidence, reproducible state replay, deterministic arithmetic across architectures, an independently controlled recovery policy, real node simulations and a broader oracle corpus. No source self-modification, token mining, real distributed consensus or production installation asserted.

Run from Labs root:
cargo run --offline --manifest-path MATH-ENGINES-LAB/proof-mining/Cargo.toml

## Governing continuation (R003)
[Constitution R003](../CONSTITUTION-R003.md) governs future proof mining. Evidence receipts do not alone advance an evolutionary chain: actual independently qualified improvement in bare-metal promotion is mandatory. The earlier eleven-policy results are historical local simulations.
