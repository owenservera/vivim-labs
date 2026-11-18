# W04-B — Host effect, World and recovery red team

Wave 4 of ARBOR Proposal 5. Base exact W3-G GO commit from `integrate/proposal5-beta-r001`, branch `work/p5-w04-b-host`. Write only **apps/vivim-host/ and owned Host recovery tests** plus `integration/worker-reports/W04-B.md`. Root lockfiles and cross-owner merges belong to serial W4-G.

## Mission — Independent Security, Recovery and No-Donor Verification
Read Proposal 5 and W3-G reports. This is the red-team wave: demonstrate proof, don't guess. Source imported **inside ARBOR**, no external HQ/TierFlow/Harness runtime, one Host JS journal, no provider APIs, no unsafe native trust. Do not redesign UI or add functionality. Fix only domain-local security defects and report others. Unavailable physical devices/provider sessions are BLOCKED, never simulated PASS.

1. Inject crash before/after plan admission, exclusive claim, provider attempt and receipt, including response-loss uncertain effect; prove no blind resend or second writer.
2. Verify Host action catalog ID/risk/approval parity with source donor and no capability-space code granting effects.
3. Test personal World export/import in temporary synthetic data folders, revocation during active plan, stale device permissions and one safe rollback drill; never mutate real user data without authorization.
4. Test Host startup under missing/competing local service, restricted IPC and missing Studio; keep status/local editing usable and errors honest.
5. Give concrete P0/other defects to next wave's B owner; preserve every existing safe fence.

## Required proof
Fault injection transcripts and single-writer journal/World evidence with no real owner-data destruction.

Commit reproducible tests/appropriate local fixes and a report listing base/head SHA, exact commands/exits, OS/device, severity, source evidence and assigned owner of each remaining blocker. Never use a real personal token/data in synthetic adversarial tests. W4-G may issue a documented **REPAIR_HANDOFF** to W5 with a finite bug list; fundamental unscoped security or unknown effect writer remains NO_GO.
