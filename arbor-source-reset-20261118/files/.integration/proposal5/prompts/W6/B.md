# W06-B — World, Host effect data migration and rollback drill

Wave 6 of ARBOR Proposal 5. Pin W5-G GO integration HEAD and branch `work/p5-w06-b-host`. Own **apps/vivim-host/, Host-owned temporary synthetic migration scripts** and disjoint `integration/worker-reports/W06-B.md`. Don't merge into integration/main. Only G can assemble release manifest and request owner cutover approval.

## Mission — Packaged Beta, Single-Writer Cutover Drill and Owner Handoff
Final wave starts **only after W5-G GO** from the exact integrated SHA. No further features. ARBOR installed code must stand alone with source-imported HQ JS, HQKG Rust, Host, Studio, PWA and TierFlow; no donor checkout/runtime. Live owner data transfer, disabling active older product and any publication are forbidden without explicit owner permission. Existing JS Host remains one writer. If actual Windows/phone/login proof isn't available, report BLOCKED and stop before claiming release.

1. Dry-run migration with synthetic or user-approved backup-copy Host World, grants, identity settings, session references and effect journal into ARBOR release paths; preserve exact IDs and claimed/uncertain states.
2. Verify one writer before/after switch, no duplicate claim replay, consistent plan/receipt signatures and no private data written to Git or CI artifacts.
3. Prepare owner-visible backup, preflight, reversible restore and rollback steps; do not mutate actual owner's World or kill existing active Host without explicit approval.
4. Run source-compatible Host restart/status, current catalog and one safe effect recovery; surface unmet permissions/migrations to G.

## Required proof
Reversible synthetic data transfer and one-writer handoff protocol, actual private cutover pending owner GO.

Commit safe fixes/tests and a report with exact ARBOR SHA, artifact details, synthetic vs real-data scope, observed device/provider results, source/receipt evidence, release blockers and the required human approval boundary. If fully verified and a physical cutover is still unauthorized, final status is **READY_FOR_OWNER_GO**, not CUTOVER_DONE.
