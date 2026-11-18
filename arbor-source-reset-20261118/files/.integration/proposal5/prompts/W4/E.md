# W04-E — Physical PWA, UI/IME and TierFlow acceptance

Wave 4 of ARBOR Proposal 5. Base exact W3-G GO commit from `integrate/proposal5-beta-r001`, branch `work/p5-w04-e-ui`. Write only **apps/mobile-relay/, packages/tierflow/ and owned visual tests** plus `integration/worker-reports/W04-E.md`. Root lockfiles and cross-owner merges belong to serial W4-G.

## Mission — Independent Security, Recovery and No-Donor Verification
Read Proposal 5 and W3-G reports. This is the red-team wave: demonstrate proof, don't guess. Source imported **inside ARBOR**, no external HQ/TierFlow/Harness runtime, one Host JS journal, no provider APIs, no unsafe native trust. Do not redesign UI or add functionality. Fix only domain-local security defects and report others. Unavailable physical devices/provider sessions are BLOCKED, never simulated PASS.

1. Use a physical paired phone (or explicitly report DEVICE_BLOCKED if no device) to verify pairing, grant revocation, stale offline state, reconnection and correct Host action/effect status.
2. Exercise desktop and PWA input with caret/undo/paste, emoji, Spanish/Catalan accents, IME composition and screen-reader/focus basics; no native stalling.
3. Verify in-repo TierFlow source/version and one active visual renderer, no permission-bearing UI actions, source-version guards and no unneeded visual feature changes.
4. Smoke existing working media/mission UI and mark any feature loss against donor; all Labs experimental UI is optional unless actually working in donor.
5. Report real device/browser/version evidence and minimal W5 fix list.

## Required proof
Physical phone/Windows visual proof or explicit device blocker; current UI functionality maintained.

Commit reproducible tests/appropriate local fixes and a report listing base/head SHA, exact commands/exits, OS/device, severity, source evidence and assigned owner of each remaining blocker. Never use a real personal token/data in synthetic adversarial tests. W4-G may issue a documented **REPAIR_HANDOFF** to W5 with a finite bug list; fundamental unscoped security or unknown effect writer remains NO_GO.
