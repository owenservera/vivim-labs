# W06-A — Native Windows packaged release acceptance

Wave 6 of ARBOR Proposal 5. Pin W5-G GO integration HEAD and branch `work/p5-w06-a-native`. Own **src-tauri/, native/, core/programmability/ (changes only for release blockers)** and disjoint `integration/worker-reports/W06-A.md`. Don't merge into integration/main. Only G can assemble release manifest and request owner cutover approval.

## Mission — Packaged Beta, Single-Writer Cutover Drill and Owner Handoff
Final wave starts **only after W5-G GO** from the exact integrated SHA. No further features. ARBOR installed code must stand alone with source-imported HQ JS, HQKG Rust, Host, Studio, PWA and TierFlow; no donor checkout/runtime. Live owner data transfer, disabling active older product and any publication are forbidden without explicit owner permission. Existing JS Host remains one writer. If actual Windows/phone/login proof isn't available, report BLOCKED and stop before claiming release.

1. Build the exact staged Windows Tauri installer from ARBOR with internal HQKG, Host, HQ JS, Studio, TierFlow assets and PWA served from ARBOR package; no donor code/runtime source paths.
2. Verify native bootstrap process identity/origin/ACL, release signing/config where available, clean install/restart/uninstall and realistic constrained laptop storage footprint.
3. Run cargo tests and one first-run native Node0 source view under packaged executable; report unsupported Windows requirements or any unrun installer checks honestly.
4. Do not publish installer or overwrite the owner's live app. Provide artifact checksum and staged path to G where tool/runtime permits.

## Required proof
Staged Windows package evidence, exact SHA, source inclusion, origin security, and zero donor runtime.

Commit safe fixes/tests and a report with exact ARBOR SHA, artifact details, synthetic vs real-data scope, observed device/provider results, source/receipt evidence, release blockers and the required human approval boundary. If fully verified and a physical cutover is still unauthorized, final status is **READY_FOR_OWNER_GO**, not CUTOVER_DONE.
