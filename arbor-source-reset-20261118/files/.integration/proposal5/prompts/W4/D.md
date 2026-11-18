# W04-D — HQ namespace, indexing, privacy and low-storage red team

Wave 4 of ARBOR Proposal 5. Base exact W3-G GO commit from `integrate/proposal5-beta-r001`, branch `work/p5-w04-d-hq`. Write only **hq/ and owned development-intelligence QA** plus `integration/worker-reports/W04-D.md`. Root lockfiles and cross-owner merges belong to serial W4-G.

## Mission — Independent Security, Recovery and No-Donor Verification
Read Proposal 5 and W3-G reports. This is the red-team wave: demonstrate proof, don't guess. Source imported **inside ARBOR**, no external HQ/TierFlow/Harness runtime, one Host JS journal, no provider APIs, no unsafe native trust. Do not redesign UI or add functionality. Fix only domain-local security defects and report others. Unavailable physical devices/provider sessions are BLOCKED, never simulated PASS.

1. Index a synthetic project through internal HQ under ARBOR, measure actual local index/wiki size and verify source freshness/updated commit invalidation.
2. Test secret file filters, denial of private World data to development index, source citation integrity, stale references and bounded context packet budgets.
3. Verify developer HQ code never imports/starts external donor checkout, database or service, even when removed, and that its Node runtime/SQLite dependencies are reproducible in packaged release.
4. Exercise local disk pressure and cache reindex/recovery; preserve protected World/effect receipt data under quota/compaction policy.
5. Report specific build/size/performance and source-truth defects for W5.

## Required proof
HQ index+wiki deterministic fixtures, quantified disk usage and development/personal isolation proof.

Commit reproducible tests/appropriate local fixes and a report listing base/head SHA, exact commands/exits, OS/device, severity, source evidence and assigned owner of each remaining blocker. Never use a real personal token/data in synthetic adversarial tests. W4-G may issue a documented **REPAIR_HANDOFF** to W5 with a finite bug list; fundamental unscoped security or unknown effect writer remains NO_GO.
