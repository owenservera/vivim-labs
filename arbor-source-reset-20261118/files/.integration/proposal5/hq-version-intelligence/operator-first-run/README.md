# HVI native, Git-free production activation — local agent mission

**Corrected product requirement (2026-11-13):** VIVIM/HQ is a **source-agnostic comparison and intelligence engine**. An end customer must never be required to install Git, Git CLI, GitHub CLI, a Git library, a development checkout or developer credentials merely to use the product. Git-specific source handling is an **optional developer connector**. The original PR's requirement to create a mandatory real-git-adapter was a design error and is withdrawn.

## Mission
Activate the already merged HVI N-way comparison system, make it callable through the existing agent/Host interfaces, and exercise it on **real, explicitly authorized sources** accessible to the local agent. For ARBOR's first developer-use comparison, prefer already available/authorized versioned repository records. Do not silently clone/download unapproved sources, fabricate history or synthesize real findings.

The first run must tell the owner **what landed in ARBOR from repos-hq, vivim-web-harness, tierflow and vivim-labs**, distinguishing source-file presence from capability equivalence and runtime proof. If exact historical revisions cannot be obtained with the supported source connectors, compare available immutable captured snapshots and report revision fidelity. A snapshot of a loose folder is NOT a Git commit.

## User-facing product invariants
1. **No Git prerequisite:** no Git installation/download prompt, no shelling out to `git` in core product, no `.git` directory requirement, no `simple-git` or `isomorphic-git` as a mandatory runtime dependency.
2. **One normalized source snapshot protocol:** readers supply immutable `sourceId, snapshotId, versionType, contentDigest, completeness, entries[], provenance` and optional `commitSha, rootTreeSha` only when VERIFIED by the reader. The existing HVI manifest/pin types need a compatibility layer, not invented Git SHAs.
3. **Multiple acquisition choices:** local files/folders through approved native file access; user-selected archives/exports; cached/saved VIVIM source snapshots; authorized signed-in web/project sources when supported; optional Git object provider only for developer commit-history semantics.
4. **No false equivalence:** generic file snapshots support content/version comparisons but cannot claim exact Git commit ancestry, Git blob hashes or cross-commit history unless verifiable.
5. **Existing security:** Host is owner of actor/session permissions and source grants; read-only comparison, bounded reads, symlink/path/secret exclusions, revocation and proof receipts. Never auto-merge, delete or migrate code.
6. **Agent-native discovery:** the user expresses a comparison in everyday language; the agent chooses available supported sources and reports unavailable data explicitly, without teaching a Git procedure.

## Documents
- [Execute local mission](EXECUTE-LOCAL-MISSION.md)
- [Source-neutral architecture contract](SOURCE-CONNECTOR-CONTRACT.md)
- [Candidate developer source set](SOURCE-SET.json) — names are NOT credentials or access grants
- [Report contract](REPORT-CONTRACT.md)
- [Agent starter](LOCAL-AGENT-STARTER.md)

**Do not run another strategic planning wave:** implement the minimal native source reader/bridge, run tests and authorized actual comparison, and produce the specified outputs. Preserve synthetic fixtures only under tests; never display synthetic results as real.
