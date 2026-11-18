# PG01 proof plan — exact existing-crate test harness

## Harness installation (temporary read-only-source evidence; NO commit to A)
Create a separate **disposable** checkout of the A worker at dda546a11294920d6521a0f64ab7ee62ec993c70. Its Git HEAD must equal that SHA; do not push or commit changes to A. Copy the two PG01 test files **only within that disposable workspace** into native/hqkg/tests/pg01_engine_abi.rs and core/programmability/tests/pg01_program_abi.rs respectively. Keep the canonical ARBOR worker branch untouched.

Example POSIX commands after downloading/checkout of both refs:
~~~
git -C /tmp/pg01-A rev-parse HEAD
test "$(git -C /tmp/pg01-A rev-parse HEAD)" = "dda546a11294920d6521a0f64ab7ee62ec993c70"
mkdir -p /tmp/pg01-A/native/hqkg/tests /tmp/pg01-A/core/programmability/tests
cp .integration/proposal5/foundations/proofs/PG01-native-abi/rust/pg01_engine_abi.rs /tmp/pg01-A/native/hqkg/tests/
cp .integration/proposal5/foundations/proofs/PG01-native-abi/rust/pg01_program_abi.rs /tmp/pg01-A/core/programmability/tests/
cargo test --manifest-path /tmp/pg01-A/native/hqkg/Cargo.toml --test pg01_engine_abi -- --nocapture
cargo test --manifest-path /tmp/pg01-A/core/programmability/Cargo.toml --test pg01_program_abi -- --nocapture
cargo test --manifest-path /tmp/pg01-A/native/hqkg/Cargo.toml
cargo test --manifest-path /tmp/pg01-A/core/programmability/Cargo.toml
cargo check --manifest-path /tmp/pg01-A/native/hqkg/Cargo.toml
~~~
If cloning from ARBOR, use the full pinned commit SHA. Do not add a Git/path dependency to production manifests. For offline Cargo resolution, use the owner's existing approved cache; do not invent a PASS without actual exit codes. Commands above were **NOT executed** in this PG01 run.

## Expected assertions in existing interface
1. Initially install trusted v1 at semantic.grammar with interface hqkg.transform.pure/1 and capability interpret, expecting None -> SwapReceipt generation 1.
2. Pin v1; CAS replace with v2 expecting Some(1) -> generation 2, old lease still deterministic v1 and new lease v2.
3. Refuse stale CAS, interface change, capability removal and empty descriptor; active generation remains 2.
4. Roll back by **reinstalling reviewed v1** with expected Some(2) -> generation 3. Earlier pinned leases still return v1/v2. Never decrease generation or rewrite history.
5. Existing Scheduler max_tasks and max_output_bytes refuse oversized workloads; cancellation before work refuses.
6. Existing DagExecutor with namespace+graph epoch+source binding: Complete caches; changed source revision requires recomputation (source assertions only), pre-cancel yields Cancelled, tight output/input budget yields Backpressure, cross-namespace source and stale graph epoch error. The graph mismatch is Err, NOT a RunState.
7. Synthetic **fixture-only** read projection equality gate rejects an old actor/session/revision/graph epoch/sourceEpoch/policy/engine generation. This is NOT proof that E live reducer implements it.
8. Program validates inert candidate; EffectCandidate with explicit true allow_effect_candidates remains execution_authorized=false/effect_performed=false. Invalid source digest and policy revision refuse.

## Explicit missing tests that cannot pass until owner work
- Manifest owner review, compiled bundle digest attestation and per-engine admission are not implemented. Do **not** pretend malicious manifest owner=host or granted=true can be safely loaded or rejected by a nonexistent entrypoint.
- Raw ComputeEngine::execute may run arbitrary trusted Rust; pure ABI names are not a sandbox. Prevention of effectful/unreviewed code requires build review and controlled registry instantiation, not a Runtime Manifest boolean.
- Timeout cannot preempt a blocked engine. Deadlines must fence acceptance and any isolation/worker termination choice needs resource-measured implementation; no detached thread leak/unbounded concurrency.
- Need real Host/Tauri authenticated ingress and source grant verification; N0 draft, F mirror, A Namespace and E View adapter are not yet a jointly admitted ABI.
- Need source revocation during in-flight computation, policy/World version change, user-visible preview trace, actual Host receipt separation and cross-language parity under accepted G HEAD.

## Post-G test expansion (by owners)
A/F run Rust unit+integration against the accepted G Cargo workspace. B/F run Host forged actor/exact plan/uncertain send tests. E/F run revisioned PWA/World render and IME/device tests. Build owners record command, environment, exit code, output summary, SHA and no-donor dependency check. Any nonzero result blocks promotion. G is the only integrator.
