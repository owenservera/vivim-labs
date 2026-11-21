# Multi-surface Craft capture index (R002.1)

The first importer accepted only one `--capture-dir` and one `--states-dir`. An application with many routes requires an explicit **evidence-ID → local artifact directory** map, not a heuristic that picks the latest folder or scans a user's disk.

## Contract

Use `schemas/craft-artifact-index.schema.json`:

```json
{
  "schema": "vivim.craft-artifact-index/1",
  "captures": [
    { "evidenceId": "ev_<ACTUAL_CRAFT_64_HEX_ID>", "directory": "C:\\captures\\page-a" }
  ],
  "states": [
    { "evidenceId": "ev_<ACTUAL_CRAFT_64_HEX_ID>", "directory": "C:\\captures\\states-a" }
  ]
}
```

**Replace placeholders with actual IDs and paths from the same original Craft evidence bundle.** Do not commit the filled index if it includes personal paths or private captures. The adapter validates each ID's operation class, rejects duplicates/foreign IDs, verifies the exact Craft-recorded directory, validates `capture.json`/`states.json` against original evidence, and checks screenshot SHA-256 digests where Craft embedded them.

Use `--artifact-index` instead of either single-folder flag, not alongside them:

```powershell
node src/cli.mjs import --bundle 'C:\captures\craft-evidence.json' `
  --site vivim-mvp --origin 'http://127.0.0.1:4173' `
  --artifact-index 'C:\captures\index.json' `
  --out 'C:\captures\atlas-multi-first'
```

A first-party investigation may additionally supply `--owned-target --include-labels` after inspecting privacy. Other sites should usually omit labels.

## Expected results

- `atlas.source.captureArtifacts` / `stateArtifacts`: number of distinct source-linked artifact directories actually processed.
- `website-capture.pageRef` / `state-capture.pageRef`: stable **hashed route identity**, derived from the origin/pathname and never the raw URL. It is a grouping hint, **not a thread or effect identity**.
- `coverage.md`: route fingerprints, capture counts, state counts, accepted evidence IDs and remaining unknown/UNRUN statuses.
- Up to 100 capture folders and 100 state folders; 5,000 control observations and 10,000 total observations. The bridge refuses overflow rather than emitting an atlas violating its own schema.

This index has no automatic discovery authority. A second directory with the wrong evidence ID, a symlink escaping an expected folder, a mismatched state payload, or an altered screenshot must fail closed. It does not search account storage or generate application operations.

## Source plan for the actual MVP

Inspect a range of *authorized* first-party screens: the PWA chat state, navigation, Wiki state and revision panel, settings, pairing dialog, and Studio view, where those screens are actually running. Produce a separate Craft record per page/state folder, then one evidence bundle from the same MCP session. Do not assume any screen exists until captured; do not attach to Studio/Harness's existing Chrome debugging port.

The Claude.ai baseline is a **source-only inventory** of existing adapter configuration, not a permit or a live capture. Full automated exploration of an authenticated third-party service requires appropriate authorization under the applicable terms.
