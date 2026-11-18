# HVI Lane E — deterministic read-only migration reporting

**Status:** `PREPARED_NOT_PROMOTED`; pure source-only projection. **Authority:** existing HQ is development intelligence; Host alone authorizes effects/personal World and TierFlow alone publishes UI.

Import `projectVersionComparison`, `validateVersionComparison`, and `exportVersionComparison` from `./index.mjs`. No web, DB, filesystem, sender, scheduler, graph, Host mount, authorizer, permission or UI state is created by the module. All input is an already-computed N-way `VersionComparison` (`arbor.hvi.version-comparison/1`); an explicit schema adapter is needed at G1 if C/F decide another envelope. Unknown envelopes fail closed (`SCHEMA_MISMATCH`). This does **not** validate repository bytes or authorizations; A/C/F and G1 own those upstream obligations.

## API

```js
import { projectVersionComparison, exportVersionComparison } from './index.mjs';
const view = projectVersionComparison(comparison, { offset: 0, limit: 50, coverageFilter: 'all' });
const markdown = exportVersionComparison(comparison, { format: 'markdown', limit: 50 });
const csv = exportVersionComparison(comparison, { format: 'csv', limit: 50 });
const json = exportVersionComparison(comparison, { format: 'json', limit: 50 });
const evidence = exportVersionComparison(comparison, { format: 'evidence-json', limit: 50 });
```

`coverageFilter` may be `all`, `complete` or `incomplete`. Each paginated row includes `id`, `kind`, category + migration category, SHA-resolved source pin, path/line/blob, target paths, per-pin cell states, similarity confidence, uncertainty, owner and evidence refs. A complete pin with no explicit cell value is still `UNOBSERVED`. Incomplete input never becomes absence or tombstone. `replaced_confirmed` is possible only with an explicit owner decision plus typed complete evidence, not similarity or document-only claims. Report summary/counts and owner-review merge plan are deterministic; no write or auto-merge command is generated.

Limits: 2–16 pins; 20,000 source rows and 20,000 capability rows maximum; max 20,000 evidence records; page <=200 rows; merge-plan max 100 rows; report returns explicit `truncated` flags and total counts. Evidence index is page-scoped. Report reproducibility digest is SHA-256 of canonically ordered JSON metadata (not a cryptographic attestation of external facts).

## Security

No raw source bytes, comments, arbitrary labels/descriptions, host session data, test stdout or personal data appear in projections. Supplied path/repository/ID/owner metadata is restricted and sanitized; untrusted paths, dot-segments, secret-looking names and Markdown/CSV injection are redacted. CSV formula prefixes are inert. Unknown evidence is explicitly incomplete. A receipt labeled `RUNTIME_VERIFIED` is only a *reported claim* unless separately attested and approved at the Host/device gate; this report always returns `runtimeAcceptance: NOT_DETERMINED_BY_SOURCE_REPORT`.

## Tests / independent acceptance

- `node --test hq/version-intelligence/report/test/report.test.mjs`
- `node hq/version-intelligence/report/test/benchmark.mjs`
- `node hq/version-intelligence/report/test/generate-samples.mjs /absolute/output/directory`

The local isolated suite was exercised with Linux Node v22.16.0, **not** the stipulated integrated Node 24 toolchain. G1 must rerun all tests in an accepted checkout, reconcile A/B/C/D/F schema seams, and record actual exit codes. H/U may consume a projection only after G1 and live Host read grant checks. Existing `integration/beta-gates.json` is unaffected.

## Source research (not runtime acceptance)

- ARBOR worker base `f9823a5131e73e7562db5a4ec14e1ec481557d69`; `hq/lib/catalogue-delta.mjs` blob `6849e6df26bd136ee50ae08a260d8ab9e9445ac7`; `hq/lib/capabilities.mjs` existing static capability inventory.
- ARBOR `integration/reports/W01-GATE.md` blob `95365a19cb634f77264b8050f9e823d2470f7fa7` and `integration/reports/MAIN-CONSOLIDATION.md` blob `3400b213e030388c1fd901e46791151626b10d85` establish source merged / device-security blocked / toolchain tests unrun.
- VIVIM Labs observatory source `owenservera/vivim-labs@022608f5797e7b4a14ccc4c393bd3c5a4704a4be:capability-observatory/lane-d/data/seed-snapshot.json`, blob `0a28e90bb2bf2c05f5ba0584af2d4e152c0ce98c`: 10 seed capabilities, zero production-certified; not product acceptance.
- PG00 preparation `owenservera/ARBOR@f8415bb793b1a88b920b3ac509f4820e7a4f5e46:.integration/proposal5/foundations/proofs/PG00-source-truth/CAPABILITY-CROSSWALK.json`, blob `4876db7ca0d1f29f629a797350623967047bc2dc`: 19 declared Host control IDs, zero observed physical tests; research-only, not promoted.
