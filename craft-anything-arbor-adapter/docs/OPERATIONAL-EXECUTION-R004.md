# Craft operational execution receipt R004
2026-11-25. Owner authorized activation. See [plan](OPERATIONAL-ACTIVATION-R004.md), commit c17605bce6f902edac883553b4f9237efa2081ab.

## Actual execution
- Workstation accessed using Desktop Commander; Craft local checkout e64d7cb73d7f39323b7040a984e0f9b3f7981146; no changes reported at preflight. Rust/cargo 1.99.1, 12.26 GB C: free at preflight. No existing CLI at expected default debug/release path.
- C01 ACTIVE: started PowerShell PID 26112 in Craft checkout, `CARGO_BUILD_JOBS=2`, `CARGO_PROFILE_RELEASE_DEBUG=0`, `cargo build --locked --release -p craft-anything-cli`. Observed dependency downloads and real compilation (proc-macro2, syn etc.). Exit and binary not yet verified. Check process output before restarting; no competing cargo build or destructive cleanup.
- Adapter local baseline 5c496ca4ac27a9c25e31ccc2550d224b5298fd69: 22 PASS / 1 FAIL, `CAPTURE_COUNT_MISMATCH: outline`. Test fixture declared one control but contained two; validator correctly refused. Fixed fixture count and added explicit mismatch-rejection assertion; no production validation changed. Commit e76502801d2aaad705f3f8dd014abdae4b46d174 on Labs main.
- Fetched exact repaired commit, extracted adapter into separate temp directory, ran `node --test test/*.test.mjs`: 23 PASS / 0 FAIL, exit 0. No local shared checkout reset or overwrite.
- Host port 43187 HTTP and subsequent TCP checks timed out. Actual live capture endpoint remains unresolved; avoid restarting active shared services. No third-party app investigated.

## Resume
1. Inspect PID 26112 and actual exit/binary version. If failed, record exact error; resource-aware repair.
2. Build native example using exact-SHA Labs helper and existing Craft evidence crate per [ROUND-2-BUILD](ROUND-2-BUILD.md), preferably after CLI build to reuse artifacts. Preserve source provenance.
3. Confirm live first-party app route and actual ownership; use [CRAFT-MCP-FIRST-RUN](CRAFT-MCP-FIRST-RUN.md) to initialize isolated persistent MCP session, list tools, capture, export and verify. No fake evidence.
4. Import and read back genuine atlas via existing Studio R003 routes. C02–C10 remain UNRUN. No provider effect or raw capture committed.
