# ONE-PASS LOCAL AGENT WORK ORDER — ACTIVATE SOURCE-AGNOSTIC HVI AND USE IT

**OWNER CORRECTION:** THIS SUPERSEDES THE PREVIOUS GIT-ADAPTER-FIRST PR TEXT AND THE LOCAL 10-SUBAGENT PLAN. NO MANDATORY GIT DEPENDENCY FOR CUSTOMERS. First read [SOURCE-CONNECTOR-CONTRACT.md](SOURCE-CONNECTOR-CONTRACT.md). The work is implementation and execution, not another preparation wave.

## Primary objective
One ordinary natural-language request must run HVI across **any two or more approved source snapshots**, including native folders, exports and optional immutable Git revisions where available. Produce explainable, evidence-grounded source and capability comparisons and executive reports. First developer case: compare authorized ARBOR and original VIVIM source snapshots.

## 1 — Verify installed system and preserve local work
- Inspect local ARBOR checkout, branch, dirty files, Node/Bun, available native sources and approved source grants. Do not reset or delete local work.
- Inspect `hq/version-intelligence/integration/{index.mjs,cli.mjs,source-manifest-adapter.mjs,AGENT-USAGE.md}`, `hq/lib/mcp.mjs`, `hq/version-intelligence/sources/`, `hq/version-intelligence/{compare,identity,parity,report}`, Host action catalog and grant owner, TierFlow inspector.
- Reuse the canonical engine and agent discovery. Existing CLI `--synthetic-stdin` and MCP `synthetic/` restriction are temporary wave guards; remove them from real production mode only once authorization + real source capture are enforced. Preserve synthetic fixture mode for tests.

## 2 — Implement native source capture WITHOUT Git prerequisite
- Implement a product-ready **source-neutral capture API** (local Host file APIs, user-selected folders and archives, existing saved snapshots) with verified SHA256, deterministic records, bounded traversal, secret/path exclusion, cancellation and current per-source Host authorization. Plain folders with no `.git` must work.
- Add a versioned adapter that can feed ordinary snapshot records into HVI **without faking Git revisions**, as specified by SOURCE-CONNECTOR-CONTRACT.md. Do not relax/erase Git-commit pin hash integrity for genuine Git sources.
- The optional developer Git connector is allowed only if verified Git commit comparisons are needed and a source is already accessible. It may be embedded or use an already installed tool, but must NOT block ordinary customer comparisons; do not add mandatory `simple-git`, `isomorphic-git`, `git` subprocess or checkout requirements.
- Provide source listing, access/permission prompts, snapshot capture, N-way compare, report/export and error handling through existing native agent/HQ discovery. Keep Host actor/session and per-source grant independent. If Host sync dispatcher cannot safely await HVI, preserve the canonical action's denied state and use the existing awaited native MCP path or implement reviewed async routing with tests; never launch a second action executor.
- Add a user-facing diagnostic (e.g. `hvi doctor`) that reports supported source connectors, installed/available functions, grants and disabled surfaces without inventing runtime readiness. Keep local snapshots/reports private unless approved to share.

## 3 — Execute the first REAL authorized comparison
- The target is actual ARBOR; the candidate donors are repos-hq, vivim-web-harness, tierflow and vivim-labs. Use existing available **authorized** versions. If clones expose genuine Git commit evidence, treat that as optional developer-specific fidelity. Otherwise native file/folder or approved archive captures are legitimate, honestly labeled snapshots; never forge a Git SHA or call folder capture a historical commit.
- With at least two approved real sources/snapshots, invoke the native HVI comparator and produce one actual report bundle per REPORT-CONTRACT.md. If only some donor sources are available, report missing source coverage rather than infer missing capabilities.
- Source state should distinguish exact content, modified, relocated, unknown and unobserved. Capability equivalence needs code/contract/import/test evidence; a same-name file or missing original path alone is not enough.
- Produce a machine-readable source/capability landing matrix, provenance, uncertainty, owner conflicts and prioritized recommended actions. **Do not auto-migrate code, push donor changes or run donor builds.**
- Use archived 676/315/314/1/361 HQ path census only as historical baseline, never substitute for current source analysis.

## 4 — Validate and ship
- Execute tests using real local Node/Bun runner with Git missing from PATH on the **ordinary native source** path; test folder and archive capture, content hashes, changed/moved files, denied/expired/revoked source grants, path escapes, partial capture and reproducibility.
- Preserve existing synthetic HVI fixture tests as regression tests. Test the optional developer Git connector separately, if implemented; no Git dependency for standard users.
- Run an actual approved read-only comparison; record observed commands/exits, snapshot types/IDs and output digests. Report missing evidence precisely; do not substitute fixture results or synthetic test success for real comparison.
- Commit source/docs/tests on a reviewable implementation branch/PR, leave local sensitive run artifacts off public GitHub, do not overwrite local uncommitted work.

## Completion contract
Return: `01-EXECUTIVE-SUMMARY.md`, `03-CAPABILITY-LANDING-MATRIX.md`, `06-PRIORITIZED-BACKLOG.md` and the full REPORT-CONTRACT bundle, along with a **normal-user Git-free invocation example**, developer-version path if present, machine source/format contract, test receipts and exact commit SHA. A real operational result is `OPERATIONAL_REAL_COMPARISON_COMPLETE` only if approved actual source data was compared. If blocked, report source/grant limitations, not bureaucracy or fabricated success.

**Hard rule:** If a proposed 10-subagent plan starts with `createRealGitAdapter`, `simple-git`, `isomorphic-git`, `--real-source` requiring `.git`, or `git`-installed assumptions as mandatory product prerequisites, REJECT/REPLACE that plan. The product must remain Git-free for ordinary customers.
