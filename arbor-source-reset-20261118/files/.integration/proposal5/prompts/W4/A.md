# W04-A — Native Tauri and Rust security/performance red team

Wave 4 of ARBOR Proposal 5. Base exact W3-G GO commit from `integrate/proposal5-beta-r001`, branch `work/p5-w04-a-native`. Write only **src-tauri/, native/, core/programmability/ and native security fixtures** plus `integration/worker-reports/W04-A.md`. Root lockfiles and cross-owner merges belong to serial W4-G.

## Mission — Independent Security, Recovery and No-Donor Verification
Read Proposal 5 and W3-G reports. This is the red-team wave: demonstrate proof, don't guess. Source imported **inside ARBOR**, no external HQ/TierFlow/Harness runtime, one Host JS journal, no provider APIs, no unsafe native trust. Do not redesign UI or add functionality. Fix only domain-local security defects and report others. Unavailable physical devices/provider sessions are BLOCKED, never simulated PASS.

1. Adversarially test Tauri trusted command origins and window navigation/remote iframe access; prove currently patched supported Tauri source/lockfile and restricted capabilities, not merely version text.
2. Test preoccupied localhost Host port and spoofed health response with synthetic owner token; verify process identity before credential exchange; report exploit preconditions and precise observed behavior.
3. Fuzz native input/session identity, resource permissions, source version, oversized payload, 500-edit storms and hanging pure reactor. Verify no actor leak, lock stall or effect grant.
4. Verify HQKG Development/Personal scoped cache isolation and Rust tests/ASan or fuzz where practical; measure CPU/RAM/disk on Windows representative hardware.
5. Fix only security-critical defects in A-owned code within wave; every other finding goes to assigned W5 repair backlog.

## Required proof
Observed Windows/native security evidence, precise advisory/source version and reproducible P0 failure or PASS; no invented exploit.

Commit reproducible tests/appropriate local fixes and a report listing base/head SHA, exact commands/exits, OS/device, severity, source evidence and assigned owner of each remaining blocker. Never use a real personal token/data in synthetic adversarial tests. W4-G may issue a documented **REPAIR_HANDOFF** to W5 with a finite bug list; fundamental unscoped security or unknown effect writer remains NO_GO.
