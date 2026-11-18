# W06-F — Independent no-donor audit and owner-ready beta dossier

Wave 6 of ARBOR Proposal 5. Pin W5-G GO integration HEAD and branch `work/p5-w06-f-verify`. Own **tests/, contracts/, capability-spaces/, QA reports (root release manifest G)** and disjoint `integration/worker-reports/W06-F.md`. Don't merge into integration/main. Only G can assemble release manifest and request owner cutover approval.

## Mission — Packaged Beta, Single-Writer Cutover Drill and Owner Handoff
Final wave starts **only after W5-G GO** from the exact integrated SHA. No further features. ARBOR installed code must stand alone with source-imported HQ JS, HQKG Rust, Host, Studio, PWA and TierFlow; no donor checkout/runtime. Live owner data transfer, disabling active older product and any publication are forbidden without explicit owner permission. Existing JS Host remains one writer. If actual Windows/phone/login proof isn't available, report BLOCKED and stop before claiming release.

1. Run complete donor removal audit after stopping all donor processes and making checkouts unavailable; verify installer, pure HQ/Rust, Host, Studio, phone and UI do not rely on outside VIVIM services.
2. Independently evaluate all G00–G13 hard gates and B01–B12 product journeys, with exact command, version, environment, exit output and unresolved owner dependencies.
3. Verify no committed secrets/user databases, no dual Host journal or graph engine, correct SourcePresent/Available/Verified capability labels and release artifact manifests.
4. Write a concise owner decision dossier: proven flows, beta limitations, outstanding hard blockers, safe backup/rollback, explicit real-data cutover approval request.
5. Do not publish or perform live data migration; G decides readiness after all six lanes.

## Required proof
Release acceptance evidence and owner GO/NO_GO package independent of implementation owners.

Commit safe fixes/tests and a report with exact ARBOR SHA, artifact details, synthetic vs real-data scope, observed device/provider results, source/receipt evidence, release blockers and the required human approval boundary. If fully verified and a physical cutover is still unauthorized, final status is **READY_FOR_OWNER_GO**, not CUTOVER_DONE.
