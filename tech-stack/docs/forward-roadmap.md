# Continuous stack intelligence — forward roadmap

**Phase 0 (this commit):** Pinned ARBOR stack, evidence links, workspaces, dependency edges, choice-specific research folders, release watch, optimization register, adoption horizon, on-demand drift checker. Not deployed automation; build and latest-release verification remain UNRUN.

**Phase 1 — repeatable resolution:** On actual ARBOR checkout, generate and commit root Cargo/Bun locks via owner-controlled CI; resolve dependency graphs; capture transitive versions, licenses, SBOM and security advisories; run Windows and PWA/security gates. Do not mark accepted until real receipts exist.

**Phase 2 — source-sync service:** Connect ARBOR event notifications and daily reconcile to a review PR; normalize changelogs/advisories and measure time-to-detection. Maintain immutable snapshots and a staleness alarm. Use a least-privilege token and no secrets in artifacts.

**Phase 3 — HQ native version intelligence:** Integrate with ARBOR HQ N-way version pin/source identity machinery and TierFlow's read-only visual inspector. HQ owns the derived data model; Labs only carries evidence, experiments and owner-reviewed technology choices.

**Phase 4 — optimization labs:** Benchmark selected hot paths (Bun install, Rust low disk, Chrome CDP, signer/offline, Blob vs Redis REST, Tauri patch), propose staged changes, capture protected migrations and rollback results.

**Phase 5 — horizon scanning:** Track confirmed upstream releases and deprecations, proposed/experimental tech and new hardware/browser capabilities; create hypotheses 3/6/12 months ahead, with **no invented ship dates** or auto-adoption.

## First acceptance checklist
- [x] Current ARBOR main pinned to exact source SHA.
- [x] JS/Rust workspace manifest evidence captured.
- [x] Research folder for every initial selection.
- [x] Source-vs-runtime and declared-vs-latest distinctions documented.
- [ ] Root Cargo/Bun locks generated and verified.
- [ ] Full transitive graph and SBOM extracted.
- [ ] Official latest versions checked with date/source.
- [ ] Upstream advisory evidence verified.
- [ ] Continuous source events and scheduled sync implemented.
- [ ] Accepted HVI interface and Host/TierFlow read-only view.
- [ ] Windows, Chrome and Android device gate acceptance.
