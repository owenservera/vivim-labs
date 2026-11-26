# P05 R001 — authority-preserving preparation
Status: partial, isolated test package prepared; no production integration.

## Entry point
Run `node verify.mjs NEW_SCRATCH REPOS.json` from this folder with a mapping of ARBOR/Labs/Product local roots. The runner extracts exact Git blobs into a new scratch root, tests the incumbent terminal validation, original Rust owned process and recovery tests, and added real cross-process journal contention. It uses a test-only Cargo shell, not a new Host. Offline Cargo requires cached dependencies. No worker deployment, production listener, or action registration.

## Route
ARBOR operational proof first for grant/dispatcher/session lifecycle. Product only consumes authenticated receipt projection and pure recovery after SDK readiness and owner authorization. A Campaign lease is not a grant. A start receipt is not a terminal receipt. A claim with no receipt is uncertain, never an automatic retry.

## P04 handoff
Resource reservation is advisory admission; bind lease to Host request and settle/release only with owned session terminal observation plus cleanup, not receipt of start or output close alone. Unknown retains reservation or requires explicit reconciliation. P04 owns accounting.

## P06 handoff
Transport forwards exact request identity and authenticated owner scope; reads existing receipt by same identity; no independent claim, replay, approval or raw worker exposure. Start receipt is not completion. Graph projection is read-only; unknown remains unknown.

## Remaining
Run the exact package on Windows and record exit status, original tests, toolchain and owned cleanup. Then execute the original full dispatcher/session integration tests against pinned incumbent runtime in isolated scratch with the native worker, proving grant denial, exact approval, replay, cancellation and unknown outcome. No assertion of power-loss durability, full transitive closure, or intel-210 acceptance. Handoff published here, not a claim that peer agents received it.
