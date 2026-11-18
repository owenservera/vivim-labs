# W06-E — Paired phone + primary user experience release acceptance

Wave 6 of ARBOR Proposal 5. Pin W5-G GO integration HEAD and branch `work/p5-w06-e-ui`. Own **apps/mobile-relay/, packages/tierflow/ (release blockers only)** and disjoint `integration/worker-reports/W06-E.md`. Don't merge into integration/main. Only G can assemble release manifest and request owner cutover approval.

## Mission — Packaged Beta, Single-Writer Cutover Drill and Owner Handoff
Final wave starts **only after W5-G GO** from the exact integrated SHA. No further features. ARBOR installed code must stand alone with source-imported HQ JS, HQKG Rust, Host, Studio, PWA and TierFlow; no donor checkout/runtime. Live owner data transfer, disabling active older product and any publication are forbidden without explicit owner permission. Existing JS Host remains one writer. If actual Windows/phone/login proof isn't available, report BLOCKED and stop before claiming release.

1. Pair a real phone PWA against installed ARBOR Host, test authentication, version negotiation, revocation, offline state and current source-linked command status.
2. Complete simple user journey from desktop composer to Host status and phone receipt, with local responsive input and minimally integrated TierFlow view; do not enhance UI beyond already accepted basics.
3. Check existing media/mission/context screens that were operational in donor remain available or are documented as owner-approved exclusions.
4. Record exact phone/browser/device versions, screenshots/command evidence where allowed; mark device access unavailable as DEVICE_BLOCKED.

## Required proof
Real paired device beta experience and same source effect status across surfaces.

Commit safe fixes/tests and a report with exact ARBOR SHA, artifact details, synthetic vs real-data scope, observed device/provider results, source/receipt evidence, release blockers and the required human approval boundary. If fully verified and a physical cutover is still unauthorized, final status is **READY_FOR_OWNER_GO**, not CUTOVER_DONE.
