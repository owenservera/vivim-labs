# Recipe E1 — zero-friction coding agent protocol + diagnostic CLI + HQ service
**Owner E.** Modify `hq/lib/protocol.mjs` only as opt-in guidance, `hq/bin/arbor-hq.mjs` as diagnostic surface, `hq/lib/arbor-service.mjs` as Host-boundary adapter; create `semantic-capability/agent-context.mjs`. No edits to VIVIM Agent/Host command dispatcher before product gate.

## Contributor SDK behavior, minimal
Pre-edit task context `{repoPin,capabilityRefs,ownerCatalogDigest,sourcePaths,invariants,approvedContracts,neededTests,readScope,unknowns}`. Coding agent follows six operations:
1. `DISCOVER` existing capability passport and canonical owner, not create second handler.
2. `DECLARE` existing v1/v2 tags on touched first-party source, no auto-registration.
3. `IMPLEMENT` ordinary code/tests, no comment-granted permissions.
4. `COMPILE` semantic projection, fix errors and unknown refs, report warnings.
5. `VERIFY` requested independent verifier/real CI tests; if cannot, UNRUN.
6. `PUBLISH` exact Git SHA and metadata proof through accepted HQ/CI, without hand-editing wiki.
One file can support multiple features with repeated tags. Unchanged legacy file never becomes blocking solely because it isn't annotated.

## Diagnostic CLI
Existing `hq/bin/arbor-hq.mjs` is local-only and accepts repeated `--allow ABS_PATH` for every source read; it initializes `createDevIntelligence({dataDir,approvedRoots,mode:'diagnostic'})`.
Proposed commands, only **after tests**:
```sh
arbor-hq scf-check --allow ABS_PATH --project NAME --changed
arbor-hq scf-compile --allow ABS_PATH --project NAME --pin EXACT_SHA
arbor-hq scf-passport --allow ABS_PATH --id registered.capability@1 --pin EXACT_SHA
arbor-hq scf-delta --allow ABS_PATH --pins manifests/SYNTHETIC-PINS.json
```
A source pin may be from approved installed HVI snapshot but no implicit `HEAD` for historical. Inputs strict and local diagnostic; no provider API, no ambient home scanning, no personal data.
Exit code policy (proposed): `0` success, `2` invalid args/schema, `3` denied/unknown current grant, `4` incomplete source, `5` blocking semantic/contract/proof conflict. No silent stdout secrets. JSON `--output=json` and human text stable.

## HQ service boundary
```ts
// Add to returned Object.freeze inside createDevIntelligence():
scfCheck({project,changedPaths?}): ScfCheckResult;
scfCompile({project,pinId,force=false}): ScfCompileResult; // derived data only
scfPassport({project,pinId,capabilityId}): CapabilityPassport|Refusal;
scfDelta({pins,targetPinId}): SemanticDelta|Refusal;
```
Every method MUST call existing `permit('source.read',storedRoot(entry))` or equivalent narrow approved domain, then `withDatabase` for derived DB access; each source root independently authorized at use time. Verify actor/session from Host mode. Don't allow a user to pass arbitrary blob SHAs or paths that skip `permit` via cached SCF snapshots. Diagnostic local roots never automatically become Host grants.

## Agent context budget
`buildMissionContextPack` projects only requested feature neighborhood with maxTokens and maxDepth; stale/partial markers preserved. Do not place personal DCB, browser state, credentials, owner grants or sampled runtime private payload in source context. Deterministic tokens and output digest, safe truncation and sorting.

## Acceptance
No `scf-*` command exists before feature accepted; existing `arbor-hq help` identical if flag disabled; `--allow` required on every source. Pass/deny/revoke with live Host actor, including revoke between compile and passport query. Agent asked to "compare VIVIM repos now" during tooling-only install => refuse as out of wave, not auto-run. Registered tool description must be discoverable in accepted HVI native command registry only after product gate.
