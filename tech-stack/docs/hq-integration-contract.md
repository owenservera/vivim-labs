# Future ARBOR HQ integration contract (non-authoritative draft)

ARBOR is the one product source; Labs is a specialized **selection-research evidence producer**. The existing ARBOR `hq/` source index and proposed [HQ Version Intelligence](https://github.com/owenservera/ARBOR/blob/f9823a5131e73e7562db5a4ec14e1ec481557d69/.integration/proposal5/hq-version-intelligence/03-NWAY-CONTRACT.md) should own derived graph/index/search and cross-version comparisons. No new graph store, watchdog daemon, product authority or competing wiki in Labs.

## Minimal input record (from Labs to accepted HQ ingestion)
```json
{
  "schema": "vivim-labs.tech-stack.selection/1",
  "repository": "owenservera/ARBOR",
  "source_sha": "f9823a5131e73e7562db5a4ec14e1ec481557d69",
  "manifest_path": "src-tauri/Cargo.toml",
  "package": "tauri",
  "declared": "=2.0.0",
  "resolved": null,
  "upstream_release": null,
  "evidence": {"path": "src-tauri/Cargo.toml", "blob_sha": "3985b0749263a336fa5f8049f8ad239132151a84"},
  "comparison": {"baseline_sha": null, "candidate_sha": null, "verdict": "NOT_RUN"},
  "decision": "RESEARCH_PENDING"
}
```

## HQ responsibilities (future acceptance only)
- Treat `repository+commit+manifest_path+package+feature/config` as identity, not only package name.
- Support N-way pinned comparison (ARBOR old/new, internal workspaces and upstream versions).
- Compute reverse-dependency and impact views using source references and lock resolution, not hand-entered narratives.
- Distinguish imported historical donor provenance from current ARBOR source ownership.
- Return a generated, searchable version atlas to Host/TierFlow read-only surfaces; no new effect authority.
- Require explicit consent and source-scope authorization for private repositories. Do not ingest owner personal World or browser sessions.

## Required status vocabulary
`DECLARED`, `RESOLVED`, `OBSERVED_SOURCE`, `RUNTIME_TESTED`, `DEVICE_ACCEPTED`, `UPSTREAM_CHECKED`, `VULNERABILITY_REVIEWED`, `ADOPTED` are separate evidence dimensions. An upstream release and an internal product commit have distinct identities and clocks.

**This file is design only.** No HVI runtime binding is claimed or installed from Labs.
