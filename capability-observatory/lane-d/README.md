# Lane D - Labs Capability Observatory

## Scope and data authority

This is an observer, not a second HQ graph. Owning product repositories retain source truth. Repos HQ owns identity, entity resolution, graph, provenance, tombstones and duplicate/orphan findings. Labs reads **explicit, pinned, bounded HQ exports** and its existing 10-capability development seed; it does not infer matches, merge identities, alter task status, publish, grant access or promote production from code/fixtures.

**Disjoint write owner:** `capability-observatory/lane-d/**` only. Root `data/`, `schemas/`, and `scripts/` are unchanged and remain available to C4 Lanes A and B. No Final Convergence Wave files changed. No network or provider API access is required.

## Run

From `vivim-labs/` (Node 20+):

```sh
node --test capability-observatory/lane-d/tests/observatory.test.mjs
node capability-observatory/lane-d/scripts/refresh.mjs
node capability-observatory/lane-d/scripts/refresh.mjs --hq /path/to/hq-catalogue-export.json --watch
node capability-observatory/lane-d/scripts/serve.mjs
```

Open **http://127.0.0.1:8764/** on that same computer. The loopback server serves only the standalone dashboard and committed seed preview; it deliberately does **not** serve private `.local` snapshots or the HQ export. Load a locally generated `.local/snapshots/snapshot-*.json` from the dashboard's **Load local snapshot** control. The viewer reads it in browser memory only. No data leaves the computer. It supports search, owner/domain/surface/lane/maturity/risk filters, detail drilldowns, duplicate/orphan evidence, source coverage and ageing, maturity bars, and CSV/JSON exports.

`refresh.mjs` writes content-addressed, versioned snapshots and an atomic `latest.json` pointer under `lane-d/.local/snapshots/` (gitignored). It tolerates a missing HQ input with explicit `missing` and `unknown` metrics. Invalid or incomplete HQ exports **fail closed without changing the latest snapshot**. `--watch` continuously checks HQ and ledger files for local changes (3-second polling); no external service, API or job scheduler is installed. `--as-of ISO8601` pins time for reproducible replay.

## Export contract (HQ -> Labs adapter profile v1)

A future HQ publisher can emit a file of shape:

```json
{
  "schema": "vivim.catalogue-export/1",
  "producer": "repos-hq",
  "revision": {"head": "0123456789abcdef0123456789abcdef01234567"},
  "sources": [{"source_id":"s1","repository":"owenservera/repos-hq","path":"research/example.json","revision":"0123456789abcdef0123456789abcdef01234567","status":"active","privacy":"public","measured_rows":4,"updated_at":"2026-11-07T00:00:00Z"}],
  "entities": [{"id":"hq:entity/1","privacy":"public","capability_ids":["cap:ui-canonical"]}],
  "links": [],
  "findings": [{"id":"find:1","kind":"duplicate_semantic","entity_ids":["hq:entity/1"],"status":"conflicting","confidence":0.6,"reason":"Needs human review","provenance":["s1"]}],
  "tombstones": [],
  "conflicts": []
}
```

Contract is **provisional** until the C4 integration owner reconciles the final A/B/C export schema. A mismatch is reported and never silently coerced. Entity-to-capability relations are **many-to-many and HQ-supplied**. All non-public or unknown-scope entity details are excluded from the serialized Labs view. HQ is never asked to expose unrestricted personal intelligence. Tombstoned entities have no displayed findings. Missing refs remain counted as invalid references, not automatically repaired.

Finding classes: `duplicate_exact`, `duplicate_semantic`, `authority_conflict`, `code_without_tracker`, `tracker_without_code`, `research_unused`, `missing_reference`, `stale_abandoned`, `orphan_other`. Findings retain reasons, provenance, status and confidence without merging. Source status and row counts must be measured by their owner or remain unknown.

## Baseline and release evidence

Read-only preview: 10 seed capabilities, 10 dev/preprod rows, 0 certified production rows, 5 source catalog stubs, of which 1 is unlocated; **0 measured external source rows**. This is a limited seed, not a complete repository inventory. The pinned TierFlow final master result (`cb16d2a...`) reports eight fixture-composed lanes and **Gate 5 / Gate 6 both BLOCKED**. No native Android/Windows/Host reachability or owner-approved activation can be inferred. Per-capability post-wave status remains not certified until HQ evidence maps it.

Known blockers: HQ export producer and contract integration await Lane C and final integration; complete source counts/duplicate/orphan figures await A/B; live hosting and production/device receipts are absent. No remote dashboard deployment was attempted or implied by this static viewer.
