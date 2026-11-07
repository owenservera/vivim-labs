# Dedicated-session launch brief — Capability Observatory

**Mission:** Build and operate a machine-readable, evidence-first, cross-repo capability inventory and two distinct ledgers in **vivim-labs**, plus a multi-dimensional interactive visual comparing current combined VIVIM capabilities against post-final-eight-lane capabilities and actual production access.

Read first:
- `capability-observatory/README.md`, `docs/AGREED-END-STATE.md`, schemas, example ledgers and source catalog.
- TierFlow `VIVIM-VISUAL-RUNTIME/FINAL-WAVE/{README,LAUNCH-PROMPTS,LAUNCH-ADDENDUM,MASTER-INTEGRATION}.md`.
- TierFlow Wave 3/4 integration results and original Wave 5/6 contracts/advance designs.
- Actual TierFlow, Harness and Repos HQ main trees and owning contracts; Labs orphaned-work and intelligence directories.

Execution sequence (one dedicated working session, not multiple planning rounds):
1. Fetch fresh repository SHAs. Discover inventories, roadmaps, skill/agent/tool registries, the user-mentioned 600+ lists and other 217/2000/100/200-series trackers; **measure their actual row counts**, formats and active status. Do not assume these lists all exist on main. Record missing sources as unlocated.
2. Build a normalized multi-dimensional graph/crosswalk: capability ↔ implementation/source ↔ product surface ↔ tool/algorithm ↔ tracker item ↔ final-wave lane ↔ tests/evidence ↔ deployed build/consumer. Preserve provenance and conflicts; avoid conflating historical research with active code.
3. Populate `dev-preprod.json` with measured existing vs target status and stage transitions; populate `production.json` only with confirmed built/installed/activated/accessible evidence, device/owner/policy/receipt references and rollback. Support both real-world runtime observed status and negative/unavailable status.
4. Add ingestion/mapping/validation scripts and machine-readable crosswalk export. Keep source data read-only by default, diff against SHA, refuse silent destructive merges, and do not duplicate canonical operational stores.
5. Create an interactive multi-dimensional visual (matrix/radar/flow/heat map as useful), filters by repo, domain, surface, owner, implementation, maturity, risk, wave lane, evidence and production access; compare BASELINE vs TARGET vs VERIFIED AFTER WAVE vs LIVE PROD, show unknown counts and disagreements rather than inflated checkmarks.
6. Benchmark importer scale and reconcile 600+ tracker records with stable IDs, provenance, duplicates/aliases and unresolved conflicts. Add raw proof logs, tested schema validator and a user-friendly dashboard/readme.
7. Publish only Labs files via fresh-head non-force CAS main; do not change TierFlow/Harness/HQ production owners or activate deployments. Deliver exact SHA, verified inventory counts, sources mapped/unmapped, state transition coverage, remaining unknowns, visualization link and tests.

Rules: one canonical product authority in owning repos; Labs is an observer, evaluator and development/pre-prod→prod release tracker. Production status cannot be inferred from code presence. Do not fabricate user reachability or device testing. Preserve the distinction **planned / implemented / tested / integrated / deployed / actually accessible**. No Playwright/Puppeteer, branches, PRs, GitHub Actions, force or provider APIs.
