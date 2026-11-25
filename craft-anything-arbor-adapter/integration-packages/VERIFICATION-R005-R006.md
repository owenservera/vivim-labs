# R005 + R006 — implementation and verification receipt
2026-11-25. Labs only; no ARBOR source changes, runtime mutations or provider actions.
Source pins: ARBOR cc4f001fe835b1ceaa20e8cb45fa0353081f41ab, Craft e64d7cb73d7f39323b7040a984e0f9b3f7981146.
R005 source commit 53671571f4d1504618cbba3a783e18e92cbfb7af; R006 source commit 278e73eee1fb62a57b8b6850f3e62e7c1c865b73.
Cloud test environment Node v22.16.0. Workstation native scratch directory was created, but subsequent staging was refused because VIVIM was draining for maintenance; no attempt to bypass. Direct cloud GitHub network unavailable. Reconstructed code locally from retrieved GitHub file content and checked exact git blob hashes before running. All seven hashes matched.

| File | Verified blob hash |
|---|---|
| R005/targets.mjs | e975847e29becaa5fbae412c7599e2d2a687f5e4 |
| R005/compiler.mjs | 6b9c499e3b282709092a7a40a52da65619e325cc |
| R005/studio-index.mjs | 953ba9ccf7cb1e8d0723cc47bbd0147f89deeadd |
| R005/cli.mjs | bb38a18740530809d3fac99e92545cc1b1ab087f |
| R005/compiler.test.mjs | 9202cc21ccaea73ada33d97616108ca78fe257fb |
| R006/handoff.mjs | 1801bd3d271fe32241b334cce694102ef40079fa |
| R006/handoff.test.mjs | aefc8f76c36c3f9800d4df8d66f0635c2f94251a |

## Executed
- node --test compiler.test.mjs: 10 pass, 0 fail.
- node --test handoff.test.mjs: 5 pass, 0 fail.
- CLI template, compile, diff, index: 4 commands exit 0; output readback assertions pass; existing output overwrite refused EEXIST as expected.
- 65 planned surface entries and 27 provider-specific operation proposals across ChatGPT (16/7), Gemini (17/7), Claude (16/7), Facebook (16/6). Counts refer to source-defined investigation targets, not actual measured coverage.

## Pending (explicitly not verified)
Genuine Craft capture, native Craft bundle parity, actual incumbent ARBOR Studio projector invocation and HTTP readback, full provider-specific output schemas, semantic registry binding, authenticated account scope, Rust port/benchmark, live provider effects, GPUI/PWA. R006 tested against contract fixture only; exact source projection interface reviewed. No provider registered or promoted. Existing upstream importer remains source of evidence; this package is not another registry or actor.

## Copy order
1. Existing Labs importer output -> R005 compiler and source index.
2. Existing Studio projector + R005 compiler -> R006 typed handoff.
3. Existing Studio/HQ semantic registry owner resolves candidate IDs, schemas, source scopes and independent evidence; integrate one real read per provider independently.
4. Existing Host execution and Studio browser custody for separately authorized actions; GPUI/VLANG/AutoWiki projections after verified bindings.
5. Native Rust migration of pure pieces after actual capture parity and benchmarks. Rust toolchain unavailable in isolated cloud environment; no Rust build claim.