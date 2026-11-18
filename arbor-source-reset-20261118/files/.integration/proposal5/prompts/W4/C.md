# W04-C — Studio provider and browser failure red team

Wave 4 of ARBOR Proposal 5. Base exact W3-G GO commit from `integrate/proposal5-beta-r001`, branch `work/p5-w04-c-studio`. Write only **studio/, harness/ and owned fixtures** plus `integration/worker-reports/W04-C.md`. Root lockfiles and cross-owner merges belong to serial W4-G.

## Mission — Independent Security, Recovery and No-Donor Verification
Read Proposal 5 and W3-G reports. This is the red-team wave: demonstrate proof, don't guess. Source imported **inside ARBOR**, no external HQ/TierFlow/Harness runtime, one Host JS journal, no provider APIs, no unsafe native trust. Do not redesign UI or add functionality. Fix only domain-local security defects and report others. Unavailable physical devices/provider sessions are BLOCKED, never simulated PASS.

1. Test exact provider/thread/body correlation and send-once under UI DOM drift, browser crash, prompt injection in provider content and missing signed-in session.
2. Check no path uses provider model API, Playwright/Puppeteer or a hidden alternative browser sender; all effects remain Host-admitted.
3. Test lease recovery and loss of acknowledgment after possible external send, ensure status is uncertain instead of automatic replay.
4. Run supported provider live smoke only when owner explicitly provides logged-in session; otherwise report LIVE_LOGIN_BLOCKED, with precise manual acceptance instructions.
5. Report only source-backed issues with reproduction and effect risk; do not feature-expand provider support.

## Required proof
Provider adapter fault-case evidence and current supported site acceptance status, honest login dependency.

Commit reproducible tests/appropriate local fixes and a report listing base/head SHA, exact commands/exits, OS/device, severity, source evidence and assigned owner of each remaining blocker. Never use a real personal token/data in synthetic adversarial tests. W4-G may issue a documented **REPAIR_HANDOFF** to W5 with a finite bug list; fundamental unscoped security or unknown effect writer remains NO_GO.
