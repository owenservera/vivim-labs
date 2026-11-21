# Semantic Capability ABI — one evidence graph over source, not a new database
**Proposed machine projection:** `arbor.semantic-capability-abi/0.1`, **NOT IMPLEMENTED**. Build a versioned adapter from `vivim.semantic-wiki-ir/0.1`, `hq.annotation/2`, current HVI `VersionComparison` contract and existing HQ SQLite/HQKG read projections. Do not replace their schemas in-place.

## IDs and cardinalities
- `RepoPin`: (canonical repo identity, exact commit SHA, root tree SHA, source completeness).
- `SourceUnit`: (repo pin, path, Git blob hash, symbol signature/anchor, stable optional `@hq:unit` ref). Exact file identity and stable semantic identity are **separate**.
- `Capability`: canonical owner-approved `name@version` ID, many files/modules can support it, a file can support many capabilities. Effective runtime state is separate.
- `Contract`: schema/ABI revision identity and owner; API edges may be inferred, directly declared or runtime-observed.
- `Evidence`: source hash/line, verified parser observation, tested command/exit, execution receipt, owner decision.
- `Relation`: typed `supports,implements_candidate,calls,imports,consumes,emits,tests_candidate,tests_observed,depends_on,declared_by,owned_by,source_replacement_candidate,verified_equivalence,displayed_by,admitted_by`.
- `TemporalLink`: immutable predecessor and successor `(repo pin,unitId)` across commits/branches, including split/combine candidates and explicit reviewed migration record.

## Fact is a join between declaration and verification
```json
{
  "schema": "arbor.semantic-capability-fact/0.1",
  "id": "fact:sha256:EXAMPLE",
  "subject": "unit:vivim.hq.hvi.sources@1",
  "predicate": "supports",
  "object": "cap:vivim.hvi.compare-revisions@1",
  "pin": {"repository":"owenservera/ARBOR","sha":"<40 lowercase hex>"},
  "source": {"path":"hq/version-intelligence/sources/snapshot.mjs","blobSha":"<40 lowercase hex>","lines":[1,8]},
  "claim": "DECLARED",
  "staticCheck": "UNRUN",
  "testStatus": "NO_EXECUTION_RECEIPT",
  "runtimeStatus": "UNKNOWN",
  "completeness": "PARTIAL",
  "evidence": [],
  "authority": "DEVELOPMENT_INFORMATION_ONLY",
  "confidence": {"identity":"declared_ref","equivalence":"not_asserted"}
}
```
The above is a schematic example with placeholders, not a valid production fact. Implementer must provide a strict JSON Schema, canonical hash, no user-controlled `id` override and exact ref parser before using it.

## Truth lattice (not one blended score)
1. `DECLARED`: agent-authored comment claims a relationship. No proof.
2. `PARSED`: token is real comment, grammar valid and source line known.
3. `RESOLVED`: semantic ref/owner status and exact source SHA confirmed.
4. `STATIC_OBSERVED`: symbol/graph/import/contract structure supports claim.
5. `TEST_OBSERVED`: exact executed test receipt with runner, exit, pin and signed/hashed output scope; may test only part.
6. `INTEGRATED_OBSERVED`: actual deployment/startup/read path or product call can reach implementation under owner.
7. `RUNTIME_VERIFIED`: observed bounded real runtime path/receipt; physical phone acceptance remains separate.
Failures/conflicts coexist independently. One verified test does not prove all claims, or effect authorization.

## Source and evidence ledger
All facts contain origin `repoSha,blobSha,path,sourceSpan,compilerSchema,semanticRegistryDigest,ownerCatalogDigest,observationTimestamp?`. Times are evidence metadata, never part of deterministic source content hash. Preserve explicit `coverage: complete|partial|missing|blocked`, `inference: exact|verified|candidate|unknown`. An absent tag is **not** proof of absent implementation.

## Capability Passport compiled view
`{capabilityId,owner,description,supportedBy: [{pin,unit,role,status}], publicContracts, consumers, testsDeclared, testsExecuted, runtimeReceipts, risk, protocolRefs, languageRefs, uiRefs, provenance, coverage, conflicts, previousRevisions, currentVerdict}`.
The wiki renders this; HVI snapshots/diffs it; agent task context selects it. These are projections from existing authority read ports, not new canonical stores.

## Safe separation
Semantic-registry remains the identity authority; native/hqkg existing graph is the only native graph compute; HQ SQLite is a rebuildable development index; `hq/lib/wiki.mjs` remains the wiki writer; Host remains exclusive execution, permissions, receipts and personal World owner. A `runtimeVerified` boolean from a comment/LLM is rejected outright.
