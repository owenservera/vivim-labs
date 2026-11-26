# P01-NATIVE-CORE — isolated qualification R001

**Disposition: selected slice compiled and tested; reproducible, not authorized for Product adoption.** SDK-first remains mandatory. No ARBOR, Product, original VWin or other lane source changes were made.

## Entry point

From this lane's checked-out folder, with the pinned parent materializer present:
```powershell
node prepare.mjs C:\absolute\repos.json C:\absolute\scratch\p01-new
cargo test --jobs 2 --locked --offline --manifest-path C:\absolute\scratch\p01-new\candidate\Cargo.toml
```
Use a fresh scratch destination outside all three repository roots. repos.json maps ARBOR, Product and Labs to exact local Git roots; their pinned objects must exist. On Windows with an uninitialized MSVC environment, set the session PATH/LIB to the installed MSVC and Windows SDK as recorded in EVIDENCE.json. The entrypoint assembles only; Cargo is a separate explicit command. It invokes the existing parent materialize.mjs and emits expanded-manifest.json, materialized/MATERIALIZATION.json and CANDIDATE.json. No production effect, registration or deployment occurs.

## Exact evidence

- ARBOR donor b13b8d67b4adb84ec91612196c8b4257df71968b; Product baseline 11812e6b1c06ac047329169ef5d848da3f85788c; observed Product main ff9c7159933a9fbc8e045fface44c14d553493b5.
- Initial manifest 11 files, 58,956 bytes; full donor closure 67 files including exact Host catalogue compile-time test fixture; Product core six files; 22 locked Cargo packages. Expanded replay 77 unique pinned files, 634,947 bytes per run.
- Product original baseline 8/8; isolated candidate 15/15 (8 original + 7 new Labs control contracts); those same 7/7 passed against exact donor.
- Full donor original suite 192/193; 152/153 unit tests plus 40/40 original integration tests. Failure: control_catalog_compat::tests::published_catalog_reconstructs_exact_fingerprinted_predecessors at line 95, catalogue version 25, expected 15. Do not hide or rewrite this failure. It is outside the selected two-module slice.
- Replay receipt ctl-099ca5328bbdd033, exit 0: two fresh assemblies produced identical CANDIDATE.json and all ten files byte-identical to the tested candidate.

## Candidate and reuse

Retain M01 source_digest and development_projection (exact declared overlay confirmed); retain resident_semantic. Do not replace Product's lib.rs or Cargo.toml wholesale. Selected exact donor modules control_planning.rs and control_plan_presentation.rs use existing serde_json; add two exports only in the isolated root and the Labs fixture. The donor standalone lock is qualification-only, never a Product workspace replacement. Current Product lock contains all 22 donor name/version pairs; current core tree and targeted control schema/name grep reveal no direct collisions. Semantic equivalents and real graph impact still require current SDK verification. Host remains authority; planning and presentation are inert. P05 owns live receipts/effects.

## Next authorized action

Use closure.json (all exact paths and dependency graph), adaptation.json (D01–D05 and S01–S06, per-task intel-201–210), and EVIDENCE.json as an immutable candidate intake to the incumbent SDK. Start with the smallest fresh planning-only sample for intel-210 if presentation intersects P05. No broad crate incorporation. Re-pin before any integration and verify current semantic graph, Wiki readback, original contracts, full current Product workspace and authority compatibility. Resolve internal license/ownership and third-party notices; no top-level license file was found, Cargo metadata alone is not legal clearance. The catalogue regression needs independent historical fingerprint disposition, not a forced green test. SDK intel-200 and intel-210 acceptance and explicit owner authorization remain required. No claim of SDK conformance, live Host acceptance, deployment or final migration readiness.

## Files

- closure.json: exact source/fixture/dependency provenance, destination collisions, licence status.
- adaptation.json: deterministic rules and explicit semantic/remediation obligations.
- EVIDENCE.json: original failures, actual Windows test receipts, replay proof, candidate file digests.
- prepare.mjs: exact isolated assembly entrypoint.
- tests/control_contract.rs: seven Labs-authored native Rust qualification fixtures, not original donor tests.
