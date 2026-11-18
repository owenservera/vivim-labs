# W05-F — Release candidate packaging, evidence and hard gate closeout

Wave 5 of ARBOR Proposal 5. Start from W4-G GO/**REPAIR_HANDOFF** exact integration SHA; create `work/p5-w05-f-verify`. Own **tests/, contracts/, capability-spaces/, QA evidence; G owns root/release manifests**, and disjoint `integration/worker-reports/W05-F.md`. Only G merges, changes root lockfiles or declares beta status.

## Mission — Bounded Blocker Repair, Regression Closure and Release Candidate
Read W4-G exact domain-tagged blocker assignments (including REPAIR_HANDOFF) and only repair listed release-blocking defects. Source has already landed internally. Preserve current Host/Studio/HQ/TierFlow/phone behavior; **no feature or UI upgrades**. Any unassigned work = regression only. One Host writer, no outside donor repo/runtime, no secrets. Real test results, not optimistic status.

1. Convert W4-G finite blocker ledger into tracked gate IDs and test commands; rerun independent G00–G13 and B01–B12 suites after A–E fixes merge.
2. Run donor-removal build/install dependency scan, secret/license/source-lineage audit and version/protocol compatibility tests.
3. Package-check Cargo/Bun/Node/WebView2/Chrome prerequisites and quantify install/index/cache size; enforce Rock Core protection and quota policies.
4. Produce accurate release candidate limitations, working/deferred features and verification receipts, not a claimed finished product from source tests.
5. Submit root package/CI/release manifest changes to G only, not by editing shared root locks in parallel.

## Acceptance
Signed/staged release candidate evidence set and hard beta gate outcomes, zero P0 if candidate can advance.

Commit fixes/tests and a report enumerating every assigned blocker and proof status, base/head SHA, command/output, source risk, remaining P0 defects and requested G merges. If you find no assigned blockers, commit a small report with test observations, **not** invented implementation work.
