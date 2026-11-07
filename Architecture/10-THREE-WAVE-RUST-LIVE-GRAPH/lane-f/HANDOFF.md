# Wave 1 / Lane F HANDOFF — 2026-11-07

**Status: BLOCKED for integration gate; verification software implemented and locally tested.** This is NOT a claim of a running VIVIM Rust graph or a completed Lane A reset.

## Delivered under Labs ownership
- `acceptance.mjs`: independent Node read-only evidence comparator + opt-in native subprocess runner, complete raw exit-code and bounded performance receipts, source/graph/wiki chain verification, NL/complier/ownership conformance and machine reports.
- `test/acceptance.test.mjs`: six executed tests, all PASS on Node.js 22.16.0 in isolated tool runtime.
- `evidence/blocked-run.json`: explicit source heads with `indexed:false`, no fabricated graphs or measurements.
- `evidence/observed-repo-sources.json`: GitHub-source observations with pinned blob hashes, not execution proof.
- `evidence/blocked-reports/*`: actual generated reports from fail-closed invocation.
- `README.md`: exact local entry points and acceptance evidence schema.

## Executed commands and results

```text
node --check acceptance.mjs
  exit 0
node --test test/acceptance.test.mjs
  exit 0; 6 pass, 0 fail
node acceptance.mjs --manifest blocked-run.json --out evidence/blocked-reports
  exit 2; status BLOCKED, 46 evidence gaps, 0 actual A+B+C+E command receipts
```

The tools environment has Node.js but no Cargo executable and no access to the user's Windows HQ index/KB/SQLite or Rust DEV journal. No local scoped reset, Rust native tests, fresh import, wiki regeneration, integrated graph traversal, restart or source edit was executed. Stub/illustrative `graph` data used inside checker unit tests are labelled tests, not runtime evidence. No Rust/SQLite migration is recommended from source-only observations.

## Evidence source pins at initial inspection
- vivim-labs main `1e5edaa6b8310f000f3eed23b6e448cb4c0f268f`
- repos-hq main `2c9bfb4c515cfabce6f8775473e461cd16418e46`
- tierflow main `24dc3c5904a6d493c3459415d0075778eb5424c2`
- `repos-hq/research/rust-knowledge-graph/src/bin/hqkg-import.rs`: blob `34c855908712123e7ef12e09618923e2b3520245`
- `repos-hq/research/rust-knowledge-graph/src/import.rs`: blob `c9983e0d9085ae78409d01b173a18c7142edef0b`
- `repos-hq/hq/lib/action-router.mjs`: blob `f7911158a18d73029850013a76fc91207b69ab51`
- `repos-hq/hq/lib/annotation-v2.mjs`: blob `7e94dfbd4fbca8ef0fdb9e0610053364ce2c38c8`
- `repos-hq/hq/lib/wiki.mjs`: blob `3efcde513bf5fdf538b413dce209ec9e6bcf0253`

## Integration owner and lane dependencies
1. Lane A: supply local before-after reset archive receipt, all source-root denominators, excluded/retained data and current indexing intent; do not wipe `.hq` wholesale.
2. Lane B: supply source-linked KB/semantic record and surviving curated record proof.
3. Lane C: supply observed CLI/router corpus including NO_MATCH, negation, quotes, conjunction and exact registered handler states.
4. Lane E: supply real source-A+B+C HQKGX1 input, DEV journal, source→KB→command Rust query nodes/edges, canonical IDs, importer no-op/stale/collision/refusal/restart results, and native `cargo test` receipts. Lane E is sole journal writer.
5. Lane D: supply generated wiki from the combined graph and source after E, not a separate input authority.
6. **Integration owner** runs `acceptance.mjs` against fresh output, reviews all source SHA/digest pins, retains raw exit-code files, confirms no extra writers or DEV→PRODUCT contamination, and only then assesses gate. Any missing fundamental evidence means BLOCKED, not PARTIAL or GO.

## Known checker limits (must be updated only after E contract lands)
- Graph query JSON inspection shape (`nodes`, `edges`) is provisional: source-owned E may publish a narrower CLI format; update F's read adapter, not E's graph schema.
- Source-path snapshot, reset durability, runtime effects, stale revisions and migration decisions cannot be independently inferred from GitHub code inspection.
- Command recipe uses an owner-trusted local manifest; no shell, but it is **not** an isolation sandbox for arbitrary imported code. Never use provider-origin manifests.
- Checker reports do not self-certify from synthetic fixtures; missing observations are explicit BLOCKED.

## Wave 2 boundary
`WAVE-2-INPUT-DOSSIER.md` currently contains **unresolved evidence needs**, not a Wave 2 plan. Its goals remain document-led, and the detailed wave design is forbidden until executed Wave 1 evidence is reviewed. Retain Wave 3 visual-language metadata obligations without implementing a competing interpreter or visual writer now.