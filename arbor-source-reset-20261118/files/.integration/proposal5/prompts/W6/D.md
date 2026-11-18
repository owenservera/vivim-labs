# W06-D — Internal HQ and native graph standalone acceptance

Wave 6 of ARBOR Proposal 5. Pin W5-G GO integration HEAD and branch `work/p5-w06-d-hq`. Own **hq/ and HQ owned tests (native modifications through A)** and disjoint `integration/worker-reports/W06-D.md`. Don't merge into integration/main. Only G can assemble release manifest and request owner cutover approval.

## Mission — Packaged Beta, Single-Writer Cutover Drill and Owner Handoff
Final wave starts **only after W5-G GO** from the exact integrated SHA. No further features. ARBOR installed code must stand alone with source-imported HQ JS, HQKG Rust, Host, Studio, PWA and TierFlow; no donor checkout/runtime. Live owner data transfer, disabling active older product and any publication are forbidden without explicit owner permission. Existing JS Host remains one writer. If actual Windows/phone/login proof isn't available, report BLOCKED and stop before claiming release.

1. With donor HQ checkout/process unavailable, run imported HQ index, semantic registry, context and source-linked wiki inside packaged/installed ARBOR on synthetic local project.
2. Verify native HQKG graph crosswalk from in-repo selected development source with one shared graph crate and separate Personal World scope; no source data leakage.
3. Test quota/storage reindex and index rollback/deletion of disposable caches, protecting persistent owner facts/effects.
4. Record exact source revisions, running executable origin and disk footprint; mark any missing native connection as BLOCKED, not hidden external bridge.

## Required proof
Internally running source HQ/HQKG with no outside repo dependency and bounded persistent storage.

Commit safe fixes/tests and a report with exact ARBOR SHA, artifact details, synthetic vs real-data scope, observed device/provider results, source/receipt evidence, release blockers and the required human approval boundary. If fully verified and a physical cutover is still unauthorized, final status is **READY_FOR_OWNER_GO**, not CUTOVER_DONE.
