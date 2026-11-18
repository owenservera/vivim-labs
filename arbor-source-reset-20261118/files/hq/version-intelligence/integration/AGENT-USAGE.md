# ARBOR HQ Version Intelligence — Native Tool G1

Stable capability ID: arbor.hvi.compare.pinned/1
Library: hq/version-intelligence/integration/index.mjs, export runHviTool
CLI: node hq/version-intelligence/integration/cli.mjs --synthetic-stdin
MCP tool: hvi_compare_pinned (hq/lib/mcp.mjs, stdio JSON-RPC; listed by tools/list, run by tools/call)
Host action: versions.compare.pinned (apps/vivim-host/control/action-catalog.v1.json)
Host binding: apps/vivim-host/control/hvi-capability.mjs, createHviHostReadPort().invoke
Input: arbor.hvi.tool-request/1
Output: arbor.hvi.tool-result/1

## Intent and trigger examples

- Compare commits A, B, and C of the same explicitly selected repository.
- Compare explicitly authorized donor repositories against a target pin.
- Explain which source files were identical, changed, relocated, physically absent or unobservable.
- Report source provenance, incompleteness, lineage hints and evidence, not assumed runtime parity.

## Where it is registered, and where it is not

This capability has THREE existing registrations. None of them is a second registry.

- **HQ MCP tool `hvi_compare_pinned`** — the native agent-discovery surface. `tools/list`
  advertises it and `tools/call` runs it. `tools/call` is awaited, so the async engine is
  reachable here. In the native-tooling wave it accepts `synthetic/` fixtures only.
- **Host action `versions.compare.pinned`** — present in the canonical Host action
  catalog with `readiness: "enforcement_missing"` and `executor: null`. The Host control
  planner already routes natural language to it (`planText('compare versions')` yields
  this action id) and reports `outcome: 'unavailable'`, because the Host control plane is
  synchronous by construction and this engine is async. Promoting that row to
  `source_wired` is a Host-owner change, not a documentation change.
- **Host read port** — `createHviHostReadPort()` binds the catalog row and performs the
  read through the same HQ code paths, with a live grant recheck on every source read.

It is NOT registered in the HQ action registry (`hq/lib/actions/*`), which cites only
real `hq`/`sync` CLI verbs; and it must NOT be added there, because the HQ action
registry and the Host action registry are authoritatively different and are never merged.

## Typed call and approval

**`runHviTool` is `async`. It always returns a Promise and must be awaited.** Every
failure is a promise rejection carrying `e.code`; a `try { runHviTool(...) } catch {}`
without `await` catches nothing.

Call runHviTool with:
- schema: arbor.hvi.tool-request/1
- sourceManifests: array of 2..16 validated Lane-A arbor.hvi.manifest/1 objects, each including its signed-off immutable pin copy. Acquisition occurs separately in Lane A.
- targetPinId: one of the supplied immutable SHA256 pin IDs; branch names are only provenance.
- pairwise: optional boolean, false by default.
- scope: bounded approved source prefixes.
- budgets: maxPins, maxEntries, maxFindings, maxPairs.
- includeParity: optional physical D atlas join, false by default.
- identityRecords/identityLinks: optional source declarations, NEVER owner approval.
- identityDecisions: must be absent or empty until separate authenticated Host receipt acceptance.
- reportOptions: optional, passed through to the report projection.

Supply options.authorize for each source, options.signal for cancellation, and
options.context (free provenance, echoed into every authorize payload).

Budget defaults and ceilings: maxPins 16/16, maxEntries 100000/1000000,
maxFindings 20000/100000, maxPairs 120/8128. A non-safe-integer or `<1` budget is
`RESOURCE_LIMIT`.

Error codes: INVALID_REQUEST, SOURCE_UNAUTHORIZED, RESOURCE_LIMIT, SCOPE_ESCAPE,
PIN_NOT_FOUND, EQUIVALENCE_UNPROVEN, plus bridge codes INCOMPLETE_SCAN, ID_CONFLICT and
DIGEST_CONFLICT.

Each call checks every source against the provided authorize callback; denial is terminal.
Host H must replace this with live actor/session-grant validation, including revoke/refuse,
prior to any remote agent routing. No guessed privileges, no extra Git readers, no ambient
checkout, no worker orchestration.

The CLI accepts ONLY synthetic/* repositories from stdin with explicit --synthetic-stdin.
It NEVER resolves an actual Git ref, reads credentials, clones a donor, accesses a provider
app, writes files, or launches scheduled scans. Its exit codes are 0 (success, stdout
carries the result plus `provenance: "SYNTHETIC_ONLY"`), 2 (any argument other than
exactly `--synthetic-stdin`, checked before stdin is read) and 1 (any other failure, one
`HVI_TOOL_<CODE>` line on stderr). stdin is capped at 4 MB.

## Result shape and what it never claims

The result carries `schema`, `capabilityId`, `status`, `runtimeAccepted: false`,
`grantProvenance: 'CALLER_SUPPLIED_NOT_HOST_ATTESTED'`, `hostBound: false`,
`sourceAuthority: 'READ_ONLY_DEVELOPMENT'`, `sourcePins`, `adapterReceipts`, `comparison`,
`evidenceReport`, `identity` (null unless requested) and `parity` (null unless requested).

`limitations` always contains: `not runtime verified`, `Host agent route not yet
registered by H`, `no donor file execution`, `path absence is not capability absence`.

## Trust and completion

Native A manifest SHA256 and projected C SHA256 are computed independently; bridge receipts bind both to immutable repo/commit/tree/pin. Native A provenance is retained without being re-typed as a C digest. The C comparison envelope is kept intact with full donor coverage; E is a sanitized pagination view, not lossless truth itself. D physical file absence NEVER means missing capability. B cross-repo identity candidates are source claims, not signed equivalence. No runtime or beta evidence may be manufactured.

The G1 integration gate is GO_FOR_W2_TOOLING only after observed real Node24 synthetic tests, strict Draft 2020-12 schema checks, independent source review and normal PR merge. Host H and TierFlow U are separate W2 work. Actual ARBOR/VIVIM/Labs comparison is explicitly deferred until a later user-triggered request and fresh grants.

## Machine-discoverable definition

HVI_TOOL_DEFINITION is exported by the native index module, with capability ID, input/output schemas, trigger examples, grant obligations and read-only effects policy. H must bind that definition into the existing canonical Host registry; no second registry, graph, scheduler, action executor, browser controller or TierFlow publisher is permitted.

## Development-source authorization

`apps/vivim-host/control/hvi-source-grant.mjs` is the Host-owned development-source
grant. It reuses the EXISTING bootstrap ledger `meta` store and the EXISTING bounded-grant
vocabulary; it creates no new store, no new authority, no new endpoint and no scheduler.

- A grant names WHICH repositories the owner authorised, and for how long. Every new grant
  carries an absolute `expiresAt` bounded by the existing 24-hour maximum.
- Evaluation order is fixed: revoked -> not granted -> expired -> repository scope.
  Absence, expiry and revocation all fail closed; an absent or unreadable expiry is
  treated as expired, because this grant type has no legacy unbounded form.
- Revocation writes a tombstone under its own key, so `HVI_SOURCE_GRANT_REVOKED` stays
  distinguishable from `HVI_SOURCE_GRANT_REQUIRED`.
- `createHviSourceGrantVerifier(store)` adapts it to the HVI read port's injected
  `verifyGrant` contract. The Host re-checks the grant on every source read, so a
  revocation denies the next read immediately and no stale authority is cached.
- The grant authorises REPOSITORIES, not identities. Who the caller is stays the Host
  owner's business: `createHviLocalOwnerGate` verifies the owner token or live UI session
  on every step, and a valid owner session alone NEVER grants repository access.

## UI revision verification

The Host observes the current UI revision from the canonical owner
(`runtime.uiRevisionStatus()`), never from the caller. The HVI request schema is closed
and carries no revision field, so no caller can claim a revision; the observed revision
travels with the result as `uiRevision`. A failing observer fails closed.
