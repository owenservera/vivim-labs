# P05 R001 — authority-preserving preparation
Status: 34 distinct isolated tests passed on actual Windows; full Host dispatcher/session integration remains blocked; no production integration.

## Entry point
Run `node verify.mjs NEW_SCRATCH REPOS.json` from this folder with a mapping of ARBOR/Labs/Product local roots. The runner extracts exact Git blobs into a new scratch root, tests the incumbent terminal validation, original Rust owned process and recovery tests, and added real cross-process journal contention. It uses a test-only Cargo shell, not a new Host. Offline Cargo requires cached dependencies and an initialized MSVC x64 developer environment. The runner explicitly sets the target directory to scratch. No worker deployment, production listener, or action registration.

## Route
ARBOR operational proof first for grant/dispatcher/session lifecycle. Product only consumes authenticated receipt projection and pure recovery after SDK readiness and owner authorization. A Campaign lease is not a grant. A start receipt is not a terminal receipt. A claim with no receipt is uncertain, never an automatic retry.

## P04 handoff
Resource reservation is advisory admission; bind lease to Host request and settle/release only with owned session terminal observation plus cleanup, not receipt of start or output close alone. Unknown retains reservation or requires explicit reconciliation. P04 owns accounting.

## P06 handoff
Transport forwards exact request identity and authenticated owner scope; reads existing receipt by same identity; no independent claim, replay, approval or raw worker exposure. Start receipt is not completion. Graph projection is read-only; unknown remains unknown.

## Evidence and remaining
See EVIDENCE.json for exact receipts. 11 Node, 16 Rust and 7 original recovery-adapter tests passed; native worker built. Original dispatcher and native session suites did not load because the initial snapshot omitted hq/lib/meta-vlang-v1.mjs. Incremental hq/lib materialization was requested, but Host returned HOST_EFFECT_NOT_APPLIED. Do not bypass. Retry only under a valid grant with a fresh approved request. Complete full dispatcher/session integration before operational migration. No assertion of power-loss durability, full transitive closure, or intel-210 acceptance. Handoff published here, not a claim that peer agents received it.
