# W06-C — Studio/provider final controlled smoke

Wave 6 of ARBOR Proposal 5. Pin W5-G GO integration HEAD and branch `work/p5-w06-c-studio`. Own **studio/, harness/ (release blockers only)** and disjoint `integration/worker-reports/W06-C.md`. Don't merge into integration/main. Only G can assemble release manifest and request owner cutover approval.

## Mission — Packaged Beta, Single-Writer Cutover Drill and Owner Handoff
Final wave starts **only after W5-G GO** from the exact integrated SHA. No further features. ARBOR installed code must stand alone with source-imported HQ JS, HQKG Rust, Host, Studio, PWA and TierFlow; no donor checkout/runtime. Live owner data transfer, disabling active older product and any publication are forbidden without explicit owner permission. Existing JS Host remains one writer. If actual Windows/phone/login proof isn't available, report BLOCKED and stop before claiming release.

1. Test installed ARBOR Studio worker can use supported signed-in Chrome webapp via the user's authorized existing profile without external Harness folder or service.
2. Perform one owner-authorized exact message receive/capture path and confirm single send/uncertain receipt with Host; if no provider login, report a defined manual BLOCKED test rather than fake proof.
3. Verify browser profile access never requires packaging owner credentials and missing provider/site leaves native Host/Node0/UI operable.
4. Record supported provider/browser version and any real-world breakage without adding new provider features.

## Required proof
Supported provider E2E acceptance or explicit, decisive owner-login blocker.

Commit safe fixes/tests and a report with exact ARBOR SHA, artifact details, synthetic vs real-data scope, observed device/provider results, source/receipt evidence, release blockers and the required human approval boundary. If fully verified and a physical cutover is still unauthorized, final status is **READY_FOR_OWNER_GO**, not CUTOVER_DONE.
