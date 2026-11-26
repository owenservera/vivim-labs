# P04 — VWin execution handoff

**Status: materialized; current build blocked; no migration or activation.**

## Source and package
- Product baseline: `bd8d33693a61021a81a2721f18fa87f3069e6836`.
- Labs donor: `ccd15368a20aa3651c64f73ac8571a07950ebfd4`.
- Materializer: `96ee6c4248f788e2dd2048202624ca073003a095`.
- Isolated scratch: `C:/0-BlackBoxProject-0/scratch/p04-96ee6c42/execution`.
- Materialization receipt: `ctl-9ce8b7bb0f226b90`, 126 pinned blobs, 30 deterministic edits. Current build attempted with `cargo test --offline`; exit 101, MSVC linker unavailable. Follow-up discovery receipt `ctl-57a9d64673b9241e` failed before application.

## Adoption boundary
Direct Product candidates: resource accounting, pure scheduler decision, resident bounded policy, source-bound workload profiling. The existing TaskPlanner remains the logical task/campaign authority. Resource feasibility complements semantic risk and context admission; it cannot grant OS effects. The isolated adapter is not a second production scheduler.

Host-only: native sampling, process/job ownership, effect permission, terminal receipt, release reconciliation. P05 concurrence required before any installed adapter. Retain capacity for uncertain effects after logical lease expiry; only an authenticated terminal receipt or reconciliation releases it. No retry after uncertain effect without reconciliation.

## Resume
1. Read MANIFEST.json, closure.json, adaptation.json and EVIDENCE.json; do not overwrite original donor or Product.
2. Resolve existing MSVC toolchain environment via authorized Windows operation. No installation or toolchain change inferred.
3. Re-run exact isolated package tests; capture build and test counts, actual resource samples and owned-job terminal receipts. Preserve failures and distinguish historical donor results.
4. Integrate the reservation decision with the existing planner's exact lease/context identity and durable Host lifecycle only in a subsequent authorized adoption phase. Verify no widening, stale/unknown samples, overflow, retained holds, replay, cancellation, review and restart.
5. Reconcile the actual staged intel-201–210 definitions before assigning exact task IDs. Do not invent acceptance. Require independent verification of intel-200/intel-210 and owner authorization before adoption.

No Product, ARBOR, or original VWin modifications; no provider call or production deployment.
