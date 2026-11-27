# Compatibility-driven Math Engines — standard calculator experiment R001

## Goal and boundaries
Prove that new mathematical capabilities can extend a frozen kernel by registration, rather than editing it. This is a Labs-only simulated host compatibility system, not the Product or ARBOR registry, a live host adapter, production admission authority or an independently deployed graph. Product and ARBOR source were not modified. The base kernel blob is unchanged: d59b3dc271d541d8a59c60aec15d7c8dc31a275d (Git blob SHA).

## Mechanism
1. A consumer declares exact required operation keys (ID, revision, numeric domain) and signatures (argument/output kinds, assurance).
2. Preview the candidate bundle without mutation; report missing/incompatible capabilities, attributed to their immediate consumer, including transitive requirements.
3. Supply new providers with declared dependencies and caller-owned positive/negative reference fixtures.
4. Reject duplicate keys, incompatible contracts, cycles, missing dependencies, wrong types and failed fixtures; commit the candidate snapshot atomically only when all checks pass.
5. Rebind the consumer explicitly; generation and immutable snapshot identity fence stale plans, including unrelated registries and divergent forks.
6. Execute only declared dependencies through the local dispatcher, with depth and call-fuel limits. Different revisions coexist; no overwrite of the original registered implementation.
 
The experiment accepts trusted, compiled Rust providers. It is NOT a sandbox for arbitrary code, proof of mathematical correctness, runtime dynamic library loader, or resource governor for code executed inside a provider. Fixtures test selected cases only. Numeric/text value kinds are closed for this first experiment; a future extensible typed schema and backend identity/evidence contract must precede arbitrary object types and production admission. The baseline mapping uses typed Rust operations and exact source pin; future integration must additionally verify host capability metadata against the incumbent registry. Production must reuse its existing registry, SDK, graph and Host.

## First consumer: standard calculator
The headless immediate-execution calculator declares nine requirements. Five are supplied by the frozen kernel: add, subtract, multiply, divide, negate. Four genuine initial gaps are supplied in extensions.rs: exact decimal parsing, display formatting, percentage (uses registered division), and bounded square root.

Interaction: digits, decimal point, backspace, four operators, equals and repeat equals, sign, percentage (add/subtract context uses registered multiply), square, square root, reciprocal, CE, AC, memory clear/recall/store/add/subtract, bounded 128-entry binary calculation history, history clear and one-step undo. No GUI or scientific functions. Errors are transactional; memory and prior state survive failures. AC preserves memory/history. Immediate chaining is deliberately not algebraic precedence. Decimal input is bounded; exact underlying rational results are retained. Display truncates to twelve decimal places and marks incomplete representations with ~. Square root uses a bounded floor approximation to six decimal places, conservatively marked approximate even for exact squares; approximation propagates through calculations and memory. These conventions are experimental, not a claim of parity with every physical calculator.

## Native verification
Tested source pin: 4aeed1678bda358419c79283c683450e93e7a1f1.
Executed on Windows in an isolated temporary directory populated from the fetched Git revision; nine tracked blobs were independently checked with git hash-object --no-filters before compilation. Local checkout and production repositories were not changed.
- cargo test --offline --manifest-path MATH-ENGINES-LAB/Cargo.toml: 11 passed, 0 failed, exit 0.
- cargo test --offline --manifest-path MATH-ENGINES-LAB/compatibility/Cargo.toml: 17 passed, 0 failed, exit 0.
- Combined terminal session receipt ctl-48aa7172c0980421: exited successfully, code 0. No private workstation logs or paths retained here.

Earlier iterations: initial archive extraction failed at the PowerShell archive module, remedied with Git blob extraction and verification; source ef8bdd676eb45fa1ed69256b46f6201bf65e3eb2 passed all 11 kernel tests but failed compatibility compilation on two Result alias type arguments; fix 777d051a410c0082abee4dc90c0a10cb38b6d00e compiled and passed 16/17 tests, exposing a decimal parser limit that rejected the valid signed i128 minimum. Shared input-bound correction through 4aeed1678bda358419c79283c683450e93e7a1f1 resolved it. All 28 tests passed at the final source pin.

## Next
Keep this baseline frozen and use the same protocol for subsequent algorithm proposals. Before any scientific expansion, qualify a larger independent calculator conformance corpus (including randomized decimal and key-sequence cases, rounding conventions, historical calculator behavior, resource limits and performance). Add open type/schema registration and independent provider identity/qualification only when the next consumer requires it. No live Product/ARBOR wiring or UI construction is authorized by this experiment.
