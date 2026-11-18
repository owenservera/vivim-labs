# Z1X — concrete incompatible interfaces
This short index supplements the exhaustive `INTERFACE-COMPATIBILITY-MATRIX.json`.

1. `A/sources/snapshots.mjs` computes `manifestDigestOf` from **A full native manifest payload**; `C/compare/index.mjs` computes `digestManifest` from **different C projection payload**. Different schema names and coverage/count fields prove their SHA256 values are **not aliases**. Separate independent verification and version-tagged projection required.
2. `F/schemas/v1/pin.schema.json` expects `arbor.hvi.pin/1`, object repoIdentity, mandatory owner/evidenceRef/requestedRef/digest; A supplies `arbor.hvi.source/1`, normalized string repo, direct-SHA null ref and snapshot-bound digest. F native pin schema is not A conformance.
3. `F/schemas/v1/manifest.schema.json` shares A's schema name but not root field types/entry parseStatus enum/symlink metadata. Do not pass by schema const alone.
4. `F/schemas/v1/comparison.schema.json` expects `arbor.hvi.comparison/1`, string pins, single coverage object, summary enums, scalar arrays, strict finding fields and root owner. C/E use `arbor.hvi.version-comparison/1`, structured 2..N pin objects, coverage per donor, source/capability finding objects, detailed evidenceIndex, limits and errors. **Full v1 validity is impossible without lossy reinterpretation.**
5. E `report/index.mjs` never reads C `originPath`/`originBlob`; without a narrow adapter, source-path and source-hash provenance disappear from export. E also omits candidate and pairwise diagnostic detail, some truncation/errors. Preserve these explicitly.
6. D `atlas.mjs` handles two `repo,commitSha` manifests and emits `source_state` separately from `capability_state`; a simple C sourceDelta mapping would lose `inspected` and owner-test provenance. The 338 unknown rows must remain unknown.
7. B `identity/index.mjs` makes source-scoped IDs and candidate equivalence annotations but C rejects every nonempty `identityDecisions`; do not adopt unsigned owner decisions or automatically merge aliases.

All observed actual source blobs and exact typed field mismatches are recorded in the machine-readable matrix. **No integration build or test execution was performed by Z1.**
