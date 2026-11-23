# Verified native/ZCode regression convergence — 23 Nov 2026

**Exact tested source:** ARBOR `0cd77a2185afe204e41e04fe3d3d588133a3eac6`. **Labs runner fix:** `78472401a1c9c7b75c8688c81ef22379aae3732f`. Separate, newer ARBOR test-hardening commit `7991f675d8bbcd4d31daf0de76d1cebab8a03a56` has **only 5/5 targeted Chat UI tests** proven; do not attribute the earlier 215-case suite to 7991.

## Exact source-linked evidence
- Existing isolated Windows Task Scheduler `VIVIM-VLANG-Observer` built a Rust `vivim-product-pure.exe` at SHA 0cd, SHA256 `389C67DB9FA6913E1B257A5339846BCEA5BA96FB4429DC168059A0D2B33CEC14`.
- The observer's local receipt: `local-logs/native-verifier/latest.json`, schema `vivim.native-parallel-verification/1`, state **PASS**, verificationContract `native-sha-bound/1`, testReceipt `vlang-2026-11-23T19-13-20-587Z`, runnerSHA256 `5D929411261DB2D365F422AE80E5F2E13848F24130000F351F8277F1C569D6CB`, finished `2026-11-23T19:14:41Z`.
- Four disjoint no-effect test lanes at **maxConcurrentLanes=1** (sequential on this PC): Z-VLANG **22/22**, Z-SAFETY **160/160**, Z-AUTOWIKI **12/12**, Z-VISUAL **21/21**. **215/215 PASS; 0 failed; 0 skips**. No sourceWrites, providerSends or hostEffects; `releasePromoted=false`. This is an isolated *source regression* result, **not deployed native UI, AI delivery, or real Android sync**.
- Failure diagnosis preceding fix: at 4 lanes the same kind of isolated run had **212/215**, with transient native intents and approval UI readiness. At 2 lanes it had **210/215**, VLANG/Safety failures under CPU/Rust subprocess pressure. Three independent failing test files then **149/149 + 10/10 + 5/5** passed when run alone without modifying product behavior. Existing Host sync Rust subprocess timeout is 1200ms. Serializing lane verification eliminated observed failures without skipping tests or relaxing assertions.
- At ARBOR **7991f675**, actual Windows command `node --test --test-concurrency=1 apps/vivim-host/test/chat-command-surface.test.mjs` returned **5/5 PASS**. The commit waits for asynchronously rendered Host approval button before invoking uncertain-response proof, preserving *no auto retry* semantics.
- In separate **VIVIM Continuous Dev Cycle** automation (not this verifier), GPUI Rust **61/61** and `cargo check` native desktop PASS at ARBOR `0cd77a21`; packages/phone unrun, no six proven working cost-permitted ZCode models established.

## Agent handoff
- Code: [ARBOR UI-test fix 7991f675](https://github.com/owenservera/ARBOR/commit/7991f675d8bbcd4d31daf0de76d1cebab8a03a56).
- Operations runner: [Labs 78472401](https://github.com/owenservera/vivim-labs/commit/78472401a1c9c7b75c8688c81ef22379aae3732f).
- Keep the existing four exact test suites. **Never restart free-model workflows blindly**; dynamic ZCode scouts still require a verified, cost-permitted model and a genuine accepted workflow run ID. Parallel development agents can do unrelated no-effect source investigation, but native-heavy verification is serialized on this Windows host.
- Next: actual current ARBOR HEAD 7991+ source-pinned full 215-case run via existing schedule, then real native GPUI typing/Host receipts, #32 real provider turn, #31 phone ACK and rollback-safe release. Physical proof remains UNRUN.
