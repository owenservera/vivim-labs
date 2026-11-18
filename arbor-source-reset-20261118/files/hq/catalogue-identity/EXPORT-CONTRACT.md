# C4 export contract — `hq.catalogue-crosswalk/1`

This is a pure HQ **read-only projection**. It carries no permission to access
Host, Studio, provider accounts, device data, or production activation APIs.

## Input

```json
{
  "as_of": "2026-11-07T01:00:00Z",
  "sources": [{
    "id": "harness:atomic", "repository": "owenservera/vivim-web-harness",
    "revision": "<40 lowercase hex commit SHA>",
    "completeness": "partial", "authority": "harness_atomic_registry",
    "lifecycle": "active"
  }],
  "records": [{
    "source_id": "harness:atomic", "source_item_id": "CAP-01",
    "type": "tracker", "name": "Example capability",
    "canonical_id": null, "authority_claim": "harness_atomic_registry",
    "lifecycle": "active",
    "evidence": [{"kind":"source_record", "ref":"owenservera/vivim-web-harness@<sha>:path#CAP-01"}],
    "links": [{"target_source_id":"harness:journey", "target_item_id":"J001", "relation":"references"}]
  }],
  "previous": null,
  "decisions": []
}
```

`completeness` is `complete | partial | missing`. It means completeness **for
that exact catalogue**, not every repository or product capability. IDs are
source-scoped and stable across renames and branch-head changes. No content hash
or display name is used as identity. `canonical_id` is optional and accepted
only when explicitly declared by the source; support `cap:*` and HQ semantic
`name@version`. Existing different IDs are not auto-merged.

Record types: `capability`, `tracker`, `implementation`, `file`, `consumer`,
`algorithm`, `test`, `research`, `release`, `deployment`, `schema`, `catalogue`.
Relations: `implements`, `tracks`, `consumes`, `tests`, `researches`, `releases`,
`deploys`, `depends_on`, `documents`, `references`, `equivalent_candidate`.
Evidence types are enumerated in README and are *not* automatically treated as
runtime receipts. Any `tombstone` requires `{owner_confirmed:true,receipt:"..."}`.

## Output

- `schema`, `as_of`, `sources`, `records`, `edges`, `evidence`, `decisions`.
- `duplicate_candidates`: pair IDs, type, confidence, reason, review disposition.
- `authority_conflicts`: divergent authority claims under one declared canonical ID.
- `orphan_findings`: `unmapped_identity`, `researched_unadopted`,
  `code_untracked`, `tracker_without_code`, `missing_reference`, `stale_work`,
  `source_unavailable` with `certainty` and source revision.
- `missing_since_previous`: retained records, never implicit deletion.
- `delta`: added / changed / unchanged / retained_unobserved / explicit tombstoned.
- `counts`, `limits`, `policy`, `snapshot_sha256`.

`sha256` is SHA-256 over canonically sorted JSON (excluding the digest field).
The same input yields the same output; a replay with `previous` produces a new
delta but identical record identities and content when sources have not changed.
The previous export must have a valid self digest. The current export is not a
signed receipt or an execution authorization.

## Collision, uncertainty, privacy and recovery

- Duplicate source IDs or record IDs with differing contents fail closed.
- Parallel evidence/references preserve many-to-many edges, with edge keys
  stable on `(source row, relation, target source row)`.
- Semantic matching is bounded token-Jaccard candidate discovery, never
  owner-approved equivalence. Manual decisions are annotations only.
- Complete-source omission is `unobserved` awaiting review, not deletion.
  Missing-source retention is `source_unavailable`. Recovery reuses stable IDs.
- Exact duplicate output is bounded to 10,000 pairs; candidate comparisons
  bounded to 15,000; truncation flags are explicit.
- Secret paths/credential-looking refs are rejected. No raw provider state or
  secret content should be passed to intake. Labs public export is allowlist-only,
  with all non-allowlisted source records and evidence withheld.

**Ownership:** Labs can display the projection, not rewrite canonical IDs or
activate production; HQ owns identity/provenance; existing source repositories
own original task/code truth and Host effects.
