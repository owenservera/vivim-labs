# PGZ — Readiness and acceptance matrix

| Lane / gate | Source status | Executed evidence | Blocker classification | Readiness decision |
|---|---|---|---|---|
| Original A–F frozen import | All 6 refs equal shared frozen contract, PREG-A–F FREEZE_READY | GitHub ref and file read; no combined build | Original G not run | **GO to original G source review only** |
| PG00 source truth | `f8415bb793b1a88b920b3ac509f4820e7a4f5e46`: 9 scoped files; 83 unique source records, 76 direct, coverage.complete=false | GitHub compare 0 foreign; Node negative-contracts UNRUN | Callgraph and source/license closure still incomplete | **GO as provisional source/owner inventory; NOT runtime GO** |
| PG01 pure native ABI | `78e43165f76c1f6ad18a63b725d75d40501259ce`: 9 scoped files incl 2 Rust harness files | Source inspection; Cargo/rustc UNRUN; 5+2 #[test] declarations only | Actor/source attestation absent; execute not preemptible; no verified manifest install | **BLOCKED / NOT YET VERIFIED** |
| PG02 semantic fabric | `021864846783d28679b29e563490d4bb100ac079`: 12 scoped files and one inert projection | **Worker-reported** Node.js v22.16.0: `node --test test/semantic-visual-crosswalk.test.mjs` exit 0, 20/20; PGZ inspected 20 declarations. Original suites UNRUN | No trusted walker/actor/current-Host replan/real UI; full-update catalog ID absent | **GO synthetic only; product BLOCKED** |
| PG03 PWA & Rock | `82fcffc1217ac58dd323627c698d71ed422a9534`: 10 scoped files and 25 negative fixtures | Source/negative read; PG03 synthetic Node UNRUN; Host/Android/Windows/ENOSPC UNRUN | **P0** static signer 404, stolen active bearer replay, native loopback, APK package; **P1** Rock disk/recovery | **BLOCKED security & device; prep accepted as design** |
| S/V/W/N0/T/K/C/H1 | Current isolated refs pinned in README; actual adapter/schema/fixture sources inspected | Historical isolated source-test reports not inherited as combined runtime tests | Different provenance/actor/version/span contracts, synthetic-only and personal-scope unknowns | **TEST_ONLY / ADAPT; no production promotion** |
| PGZ independent referee | Eight in-tool V8 content-policy assertions passed; four GitHub compares all scope-clean | **V8 check 8/8** (not Node). PGZ Node/Bun/Cargo, signed-in browser, device and disk **UNRUN** | No G merged source tree, combined integration tests unrun | **PREPARED_NOT_PROMOTED** |
| Original W01-G | `integrate/proposal5-beta-r001` absent 404 at independent check | No `W01-GATE.md` on accepted ref; no combined Cargo/Bun/Windows acceptance | Cannot authorize post-G patches or beta GO | **G_NOT_RUN; not G_ACCEPTED** |

## Combined strict gate order

1. **G acceptance**: original G serially integrates frozen A–F with an exact HEAD/source-lock, no donor runtime, root Cargo/Bun workspace and observable command exits. PGZ neither blocks nor performs G.
2. **Security P0**: actual A/B process-bound native launch and B/E per-request PWA signer/static/grant/replay tests, before privileged remote operations. PWA FIRST; APK not beta dependency. Any P0 negative failing = NO GO.
3. **Pure native proof**: real Cargo generation CAS/pinned old lease/hot swap/reviewed rollback, trusted compiler/source/actor admission, bounded compute/cancel and F ABI parity. Source fixture alone = NOT YET VERIFIED.
4. **One semantic read**: D original source walker & KB + B registered R0 Host original-text/current grant + E visual source-revision/IME projection. Require real Host receipt (if read path records one), no comment effect or unverified UI permission.
5. **Storage/safety/device**: D/B protected byte inventory, WAL/ENOSPC/recovery, F negative tests, no-APK Windows packaging and actual Android HTTPS PWA pairing. Explicit DEVICE_ACCEPTED only after physical test.

## Refusal vocabulary

`SOURCE_PRESENT` = Git blob at isolated owner ref; `PREPARED_NOT_PROMOTED` = staging proof; `TEST_EXECUTED` = exact observed test only; `RUNTIME_VERIFIED` = combined integrated live proof; `DEVICE_ACCEPTED` = physical platform result; `BLOCKED` = not authorized/known P0; `UNRUN` = no command/exit receipt. None of the PG work meets RUNTIME_VERIFIED or DEVICE_ACCEPTED.
