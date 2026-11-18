# ARBOR Acceleration R002 — W1 Five-Lane Integration

**Disposition:** SOURCE_CONVERGED / MERGE_AUTHORIZED_BY_OWNER / INDEPENDENT_NODE24_BROWSER_DEVICE_ACCEPTANCE_UNRUN.
**Base main SHA:** `2a42f1af025db8aacb965326b11c73da671ffb3f`.
**Combined five-parent-source merge (staged):** `af94a34e0d4b914bd612ac16242fb0a68acc6e91`.
**Source lanes:** A `13a77e617ef1f009f985cdf9fe9642001c969b18`, B `d888945718a5f69a7a0b357436eff5529be5b1a2`, C `46e20152882a70dc85095effe9986d2705c49542`, D `9cf98329e979598661aa437b3a7dac072309b29a`, E `81ba6d3aa4a8017589820c9758239925140a9789`.
**Final integration/main SHA:** Git revision to be read from GitHub after the guarded promotion. No committed document can truthfully contain its own eventual SHA.

## ERA 1 — Merge inventory and integration corrections

All five branches were based on the same main SHA and had disjoint changed-file sets: A=5, B=4, C=5, D=5, E=6. The integration merge preserves all five branch-head parents in the Git commit graph and includes their 25 changed paths without dropping historical receipts, modifying the original lane branches or creating an alternate runtime.

Integration-corrected cross-lane contracts:

- **Host ↔ Wiki owner authentication:** A intentionally refuses privileged Wiki edits without a real owner bearer or existing short-lived authenticated browser session. C's original editor used only Origin and a custom header. The integrated Wiki now gets the existing `/api/host/control/ui-state` proof or exchanges a one-time `#control-session=<code>` launch fragment via `ui-exchange`, uses `credentials:'same-origin'` and `x-vivim-ui-csrf`, and clears cached proof on HTTP 401. It preserves the user's draft and sends no effect when session is unavailable. No owner bearer enters JavaScript or URLs.
- **Host ↔ runner:** B now creates a **random in-memory test-only owner capability** and passes it only to its temporary Host. Its negative checks assert actor+Origin+header-only editing is refused HTTP 401. The positive path uses a legitimate owner bearer against the isolated Host.
- **Runner ↔ independent Wiki evidence:** D's live CLI now requires an explicit isolated Host owner capability as well as isolated loopback address. B passes the temporary capability by child process environment only. The verifier never prints the token or promotes source checks to browser proof; mocked regression transports remain independently testable.
- **Node24 CI:** The lane B workflow includes the integrated main branch and new C/D/E/integration test suites, preserves literal exit files/stdout/stderr, and does not manufacture a lockfile or a passing CI result.
- **Test inventory:** C, D, E and integration test files registered in `scripts/test-manifest.json`. All historical release ledgers and receipts remain unchanged.

## ERA 2 — Evidence and gate limitations

- **Previously observed local Node22 tests, inherited from lane reports (not rerun after integration):** A journal 8/8, C controlled DOM-model 5/5 (the integration adds a new owner-session test), E feedback compiler 18/18. B real Node24 Host UNRUN, D independent Node24 verification UNRUN, real browser screenshots UNRUN, Windows/paired-Android UNRUN. GitHub Actions previously returned `startup_failure` with zero jobs, not a test-process exit.
- **Integration session static checks:** V8 parse checks after ESM import stripping are source-only and have no OS process exit. Run the actual command list below in a complete checkout; capture exit status, Host URL, runtime, journal and exact SHA. Do not infer Node24 pass from Node22 unit checks.
- **Open security blockers:** A's single-process journal serialization does not establish inter-process lease/one-writer safety. A torn tail requires owner recovery; no silent truncation. Browser session grant via actual native Host and physical Windows/PWA checks remain UNRUN.
- **No acceptance promotions:** `A00`, `A01`, `A02` remain in progress with `verification=UNRUN`, `ownerAcceptance=PENDING`. `A03` feedback compilation is source-prepared only, still blocked by A00 independent acceptance. `A04` Wiki visual projection is source-prepared only; the real NL→VLang/VEC compiler and independent A02 acceptance remain unproven. A05–A08 remain queued.
- **Release gate truth:** `integration/beta-gates.json` UNBLOCKED claims still conflict with `.integration/proposal5/08-BETA-GATES.json` PROPOSED_UNRUN and `integration/release-manifest.json` BLOCKED_NOT_BUILT. No beta or physical acceptance claim is authorized by this merge.

## ERA 3 — Exact executable integration acceptance protocol

From a complete checkout of the **integrated main SHA**, first execute:

```sh
node --version
git rev-parse HEAD
node scripts/roadmap/acceleration-queue.mjs --check
node scripts/roadmap/evidence-truth.mjs --check
node --test apps/vivim-host/test/product-journal.test.mjs apps/vivim-host/test/product-routes.test.mjs apps/vivim-host/test/chat-box-baseline.test.mjs
node --test apps/vivim-host/test/wiki-visual-contract.test.mjs scripts/batch-evidence/wiki-evidence-regression.test.mjs scripts/roadmap/evidence-truth.test.mjs scripts/feedback-loop/compiler.test.mjs scripts/acceleration/w1-integration-contract.test.mjs
node scripts/acceleration/live-host-check.mjs
node --test scripts/acceleration/live-host-check.test.mjs
```

Use real Node 24. Observe and record literal exit codes, platform, and raw receipts, not guessed results. After local/CI Host tests pass, independent keeper verifies A00/A01/A02 and resolves source-versus-release contradictions before flipping any queue. A real native Host owner-session launch, Wiki edit confirmation/readback, stale revision/revocation and real desktop/mobile visual checks are required separately.

**Next integration handoff:** A/D review owner authorization and evidence; B/local Host checks; C native launch+real UI proof; keeper sole-writer roadmap acceptance; E A03 proposal activation only after A00 independently accepted. A work order's acceleration impact stays NOT_YET_MEASURED until the next verified repair is demonstrably faster.

**No Playwright/Puppeteer, provider APIs, second Host interpreter/publisher, unauthorized source effects, autonomous roadmap writes, APK or release gate acceptance were introduced.**
