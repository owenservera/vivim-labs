# Frozen contract — nine-section model intake

**Schema:** `hq.vivim-model/1` · **Status:** frozen for waves 1–3 · **Owner:** L2 (build), L0 (change)
**Gate:** D2 · **Contract source:** `docs/VIVIM-KB-INTAKE.md` §8 (specified, never implemented)
**Consumers:** L3 (parity), D5 (identity parity), every later projection

---

## 1. The law this contract enforces

`docs/VIVIM-KB-INTAKE.md:207-213`: **the KB owns the domain model.** HQ does not invent, redefine or
override entities, types, flows, actions, authority rules, features or use cases. HQ validates the
model, materialises it, and reports where its own code drifts from it.

Two consequences that are easy to get wrong:

1. A conflict between the model and an HQ table is **drift against HQ**, not an error in the model.
2. **A model file is never a document.** If model intake is unavailable, the package is refused. It
   is never quietly classified as `documents` — that is GAP-005, and it turns a hard failure into a
   silent one.

## 2. Package shape

```text
package/
  manifest.json                 # hq.vivim-kb/1, required (unchanged)
  model/
    index.json                  # hq.vivim-model/1, required when model/ exists
    entities/*.json
    types/*.json
    schemas/*.json
    flows/*.json
    sources/*.json
    actions/*.json
    authority/*.json
    features/*.json
    usecases/*.json
  docs/**/*.md                  # unchanged
```

`model/index.json`:

```json
{
  "schema": "hq.vivim-model/1",
  "id": "vivim",
  "version": "2026-11-03",
  "producer": "vivim-kb-agent",
  "authority": "kb",
  "entities": ["entities/*.json"],
  "types": ["types/*.json"],
  "schemas": ["schemas/*.json"],
  "flows": ["flows/*.json"],
  "sources": ["sources/*.json"],
  "actions": ["actions/*.json"],
  "authorityRules": ["authority/*.json"],
  "features": ["features/*.json"],
  "useCases": ["usecases/*.json"]
}
```

All nine section keys are **required**, even when the glob matches zero files. A missing key is
`MODEL_SECTION_MISSING:<key>`; an empty section is valid and reported as `empty`.

## 3. Section shapes

| Section | Required keys | Notes |
| --- | --- | --- |
| `entities` | `id`, `title`, `kind`, `owner`, `fields[]`, `relations[]`, `invariants[]`, `source` | `relations[].target` must resolve |
| `types` | `id`, `kind ∈ primitive\|enum\|union\|alias` | `of` required for alias/union; `values[]` for enum |
| `schemas` | `id`, `version`, `target`, `jsonSchema` | Versioned JSON Schema, stored verbatim |
| `flows` | `id`, `title`, `actors[]`, `steps[]`, `inputs`, `outputs`, `authority[]`, `source` | Every step names an action id or an explicit `user-decision` |
| `sources` | `id`, `kind ∈ file\|table\|api\|capture`, `trust`, `owner`, `description` | Provenance graph input |
| `actions` | `id`, `title`, `actor`, `effects[]`, `authority[]`, `idempotency`, `inputSchema`, `outputSchema` | Effects are declared, never inferred |
| `authority` | `id`, `subject`, `may[]`, `mayNot[]`, `requiresOwner`, `rationale` | Data about permission; grants nothing |
| `features` | `id`, `title`, `status ∈ planned\|partial\|shipped\|retired`, `flows[]`, `actions[]` | Drives coverability |
| `usecases` | `id`, `title`, `actor`, `feature`, `flow`, `given`, `when`, `then`, `evidence[]` | Traceability spine |

## 4. Validation order (fixed; each step may refuse)

```text
1  manifest schema + authority ceiling
2  model/index.json schema + all nine keys present
3  id uniqueness per section; ids namespaced by section
4  every field type resolves to a declared type
5  every entity relation target exists
6  every flow step names a declared action or `user-decision`; flow DAG acyclic
7  every action maps to a declared component; effects declared
8  every authority subject exists; may/mayNot are known action ids
9  every feature references existing flows/actions
10 every use case cites an existing feature and flow
11 drift against HQ tables (report, never refuse)
```

Steps 1–10 refuse. Step 11 always reports and never blocks an import.

## 5. Transaction and refusal semantics

- **One package, one transaction.** Refusal at any step writes zero rows and returns a receipt.
- A refusal names the exact code and the offending id or path.
- Partial import is impossible by construction: sections are staged, validated, then committed.
- Re-importing an unchanged model is a no-op with an identical digest.
- A changed model is a **new whole snapshot**, never a mutation of the previous snapshot.
  `modelDiff(versionA, versionB)` reports added / removed / changed across all nine sections.

## 6. Refusal codes

| Code | Meaning |
| --- | --- |
| `MODEL_SCHEMA_MISMATCH` | `index.json.schema` is not `hq.vivim-model/1` |
| `MODEL_SECTION_MISSING:<key>` | One of the nine keys is absent |
| `MODEL_ID_DUPLICATE` | Same id twice in one section, or across sections where namespacing forbids it |
| `MODEL_TYPE_UNRESOLVED` | A field type has no declaration |
| `MODEL_RELATION_MISSING` | Entity relation or flow/feature reference points at nothing |
| `MODEL_FLOW_STEP_UNKNOWN` | A flow step names neither an action nor `user-decision` |
| `MODEL_ACTION_EFFECT_UNDECLARED` | An action has no `effects` array |
| `MODEL_AUTHORITY_INVALID` | An authority rule names an unknown subject or action |
| `MODEL_USECASE_UNBACKED` | A use case cites no feature or no flow |
| `MODEL_DOWNGRADE_FORBIDDEN` | A `model/` path reached document classification |

## 7. Drift report

```json
{
  "schema": "hq.vivim-model-drift/1",
  "modelVersion": "2026-11-03",
  "ok": false,
  "entitiesMissingTable": [],
  "tablesMissingEntity": [],
  "fieldsMissingColumn": [{ "entity": "session", "field": "outcome", "table": "sessions" }],
  "fieldsUndeclared": [{ "table": "notes", "column": "confidence" }],
  "actionsWithoutComponent": ["hq.action.send-message"],
  "useCasesWithoutEvidence": ["UC-014"],
  "authorityViolations": []
}
```

`ok: false` with named gaps is a **valid import**. Drift is information, not a refusal.

## 8. Acceptance

1. A fixture package with all nine sections ingests; counts match the files.
2. A package missing `useCases` is refused with `MODEL_SECTION_MISSING:useCases` and writes nothing.
3. An entity field typed `foo` with no declared type is refused with `MODEL_TYPE_UNRESOLVED`.
4. A flow step naming `hq.action.nope` is refused with `MODEL_FLOW_STEP_UNKNOWN`.
5. A use case citing a missing feature is refused with `MODEL_USECASE_UNBACKED`.
6. A `model/` file never appears in `documents` after any intake outcome.
7. Re-ingesting the identical package is a no-op with an identical digest.
8. `modelDiff(v1, v2)` reports a removed entity and an added action by id.
9. Drift names a real missing table without refusing the import.
10. No model file can grant authority or cause an effect; a `browser:effect` action is ingested and
    refused at execution.

## 9. Out of scope

- Executing model actions.
- Auto-generating HQ tables from entities (drift is reported, not auto-migrated).
- Rewriting model meaning to fit HQ.
