# R003 constitutional championship — executable development scaffold

Read [Constitution R003](../../CONSTITUTION-R003.md) and [Math Engines agent rules](../../AGENTS.md) first. This is the first implementation scaffold for R003, independent of the historical R002 tournament. **No entrant is qualified; no evolutionary head is advanced; no actual championship match has been played.** Initial roster contains the first 32 conceptual designs and 32 vacant reserved slots. Do not conflate P IDs here with earlier R002 roster IDs.

## Architecture and entrypoints

- roster.json: 64 fixed positions, four conferences, 32 distinct architecture descriptions and seed/adversary directions, D01–D16 each OPEN, 32 RESERVED, null genesis/head. One canonical roster; no duplicate production registry.
- src/workspace.rs: validates roster, creates a new explicit workspace with 32 per-entrant manifests and separate directories for genesis, bootstrap, architecture, registry, generator, adversary, ledger, evidence, recovery, memory, measurements, promotion and receipts; evaluator layout with development, holdout placeholder, submissions, receipts, bracket and inheritance. No overwrite. Directory segregation is NOT security isolation, holdout secrecy or authentication.
- seeds/ and build.rs: initial native implementations. Each is statically linked, reviewed, checked against the same disclosed bounded oracle. No arbitrary dynamically generated code execution. P024's build-time emitter is a small legal instruction grammar, not yet a self-evolving generator.
- src/main.rs: actual x86-64 machine arithmetic through a typed Rust Win64 ABI, signed i64 development domain, operations 0 add, 1 subtract, 2 multiply, 3 truncating quotient, 4 negate, 5 truncating remainder, 6 comparison (-1,0,1), invalid opcode status 3; success 0, overflow 1, divide-by-zero 2. Inputs a/b by value, op u32, aligned writable output i64; leaf volatile registers; failure leaves output untouched. i64 is a deliberately bounded development subset, not the complete rational/decimal standard calculator or final representation. No Rust arithmetic fallback. Independent Rust checked arithmetic is permitted only as oracle. Canary and status probes do not prove all ABI properties.
- src/protocol.rs: 64 position bracket, six named rounds, fail-closed admission predicate requiring complete D01–D16, artifact identity, independent suite and evaluator receipt identity, all hard qualifications and real positive comparable promotion improvement. Measurement comparison requires same nonzero suite/conditions, same budget, sufficient repetitions, and either qualified-promotion count gain or nonzero prior yield preserved with threshold resource-cost reduction. Example test thresholds are NOT frozen championship policy. Match predicate recomputes eligibility, cannot award two unqualified entrants, rejects mismatched rounds/same entrant, leaves ties unresolved. Receipt digest references are identities, not authenticated evaluator records. The real independent evaluator, sealed corpus, receipt resolution, calibration seeding, durable evolution state and execution budgets remain unimplemented.
- Evidence journal and evolution head are distinct by design; preview heads are not committed evolution states.

## Initial seed wave
| Entrant | Native seed hypothesis | Source |
|---|---|---|
| P011 Embedded | Position-independent dispatch table | seeds/P011.S |
| P022 Reversible Chronicle | Forward/inverse checks and division reconstruction | seeds/P022.S |
| P024 Executable Genome | Build-time instruction-template emitter | build.rs |
| P026 Cellular Proof Fabric | 32-bit limb carry/borrow cells | seeds/P026.S |
| P027 Shadow Twin | Bitwise carry and shift-add shadow computations | seeds/P027.S |
| P028 Negative-Space Atlas | Sign-partitioned bounds before add/sub | seeds/P028.S |
| P029 Transplant Architecture | Distinct arithmetic leaf symbols behind dispatcher | seeds/P029.S |
| P032 Instruction Witness | Immediate overflow flag capture and branchless comparisons | seeds/P032.S |

Eight separate authored proposals, not independently autonomous generators or complete independent blockchain architectures. Some hardware instructions necessarily overlap. Development harness shares the ABI/oracle and links the eight seeds into one executable; separate child processes contain crashes but are not a security sandbox. Actual per-entrant chain preview uses registered native function dispatch for height, work, weighted work, remaining work, quotient and remainder. Hashing is conventional SHA-256. The same small preview state model is intentionally shared for development; the entrant-specific ledger designs, native artifact manifests, register-preservation probes, rational/decimal representations, native generation, adversarial systems, recovery, measured promotion and hidden evaluation must follow.

## Native development evidence
Pin 428119d0058e09a89677586f5d239941790e7bdf, Windows native release run, exit 0 (12 tracked source blobs verified before compilation), 6/6 unit tests passed, 8/8 seed development suites passed. Every seed executes 30,984 checks: 17 x 17 boundary pairs x 8 opcodes, plus 4096 mixed-width deterministic random pairs x 7 opcodes. 247,872 total actual seed calls in this suite, plus 32 chain transitions with six native calls each. Failure output/canaries checked on every call. Source hashes, full executable hash and preview head emitted in output. No hidden independent holdout, formal proof, timing comparison, or complete qualification. Earlier pin 7c89f5feff344f91041a1641d864f05ad23400fa passed 2 unit tests and four seeds. Unused scaffold API warnings were observed, not compile failures. Later evidence is appended below after verification.

## Run
From Labs root:
```powershell
cargo test --offline --release --manifest-path MATH-ENGINES-LAB/championship/r003/Cargo.toml
cargo run --offline --release --manifest-path MATH-ENGINES-LAB/championship/r003/Cargo.toml
cargo run --offline --release --manifest-path MATH-ENGINES-LAB/championship/r003/Cargo.toml -- scaffold <new-explicit-workspace-directory>
```
No existing directories overwritten. Scaffold is reproducible and does not activate the entrants.

## Development lanes / next concrete acceptance
1. Finish the remaining 24 candidate-specific native seed proposals, preserving meaningful architectural differences; do not manufacture difference by renaming identical templates. Generate/validate 32 more concepts before complete 64 roster.
2. Establish per-entrant artifact and ABI manifests, truly isolated independent generation and evaluator-controlled hidden suites; validate full D01–D16. Each entrant must develop its own actual ledger, adversary and generator architecture.
3. Native bootstrap stress: independently test all registers/stack/memory/flag/representation and error contracts, executable identities and Rust composition; qualify rational/decimal standard-calculator behavior. Include known-bad seeds as negative controls, no silent fallback.
4. Implement durable evidence store, typed dependency/registration preview, independent receipt resolution, recovery and evolution-head CAS. Reject unsafe promotions; include deliberate primitive failure and recover/requalify.
5. Freeze comparable promotion workloads and thresholds with independent evaluator before R1. Run paired predecessor/successor promotion machinery under equal budgets, demonstrate positive yield/cost improvement (F10), then seed and execute actual bracket. No automatic winner.
6. Preserve historical source evidence, use shared discoveries only at declared inheritance boundaries. Never alter Product/ARBOR or frozen math reference kernel for this research.

All unclosed deliverables stay OPEN. This folder is a development scaffold, not proof that the 32 architecture designs have been implemented.

## Final verified development run R001
Source pin `5c49d502e83b49793553c935a30ccdea6b8fdd96`, Windows session `ctl-23498dd7964bc720`, exited 0. Thirteen tracked blobs verified; 8/8 unit tests, 8/8 native seed development suites (247,872 checks), 32 preview transitions, 32 per-entrant development workspaces materialized and counted; 32 reserved slots. [Machine-readable evidence](DEVELOPMENT-EVIDENCE-R001.json) records all eight source hashes, common executable digest and preview heads. No qualification or advancement, no sealed holdout. Dependencies resolved offline including serde_json 1.0.154; full dependency lock not yet retained. Protocol predicates tested with controlled fixture inputs; real evaluator authorization, persistent per-entrant artifacts, independent seed generation and F10 measurement remain open. Unused scaffold warnings remain.

## Independent external research teams — R003-V1
The single architecture-neutral handoff is [EXTERNAL-TEAM-BRIEF-V1.md](EXTERNAL-TEAM-BRIEF-V1.md). Give teams that document alone, not our roster or seed implementations. Team-local E01–E32 IDs are namespaced by team_id; central admission and 64-slot selection are independent. The native seed ABI is identical to the development evaluator, with a fixed external symbol for each separately assembled entrant. The portable request/response and package contract is versioned; it does not dictate the entrant's internal architecture.

Structural intake: `cargo run --offline --release --manifest-path MATH-ENGINES-LAB/championship/r003/Cargo.toml -- validate-team <extracted-team-root>`. It checks 32 entrant paths/manifests, namespaced identities, obligations, evidence digests and unsafe path forms without building or executing candidate source. Results are STRUCTURAL_PASS_UNQUALIFIED only. Independent review, truly isolated native probing, external runner orchestration, evaluator authorization, promotion qualification and official selection are still separate requirements. Do not confuse the presence of a native label or evidence file with a functioning candidate.
