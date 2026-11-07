# VIVIM Labs — Capability Observatory (seed v0.1)

**Status: SEED ONLY; NO LIVE TRACKER OR COMPLETE AUDIT.** This is a non-authoritative, machine-readable **dev/pre-prod** tracker plus a separate **production** evidence tracker, together with provenance and task crosswalks. Labs observes, benchmarks and reports; canonical deployment, World, Host security, UI document and KB authority remain in their owning repos. No mirroring or replacing production stores.

## Scope and owners

The three combined product repositories are **TierFlow** (`owenservera/tierflow`), **VIVIM Web Harness** (`owenservera/vivim-web-harness`) and **Repos HQ** (`owenservera/repos-hq`). Labs itself tracks observations and proposed convergence. Identity, status and verification must be pinned to a repository SHA and source locator. Each capability can have multiple implementations/surfaces and multiple tracker references, but ONE stable canonical capability key.

**Two distinct ledgers**:
- `data/dev-preprod.json`: design/research/prototype/implemented/tested/integrated/release-candidate progress, dependencies, target end state after final eight lanes and evidence quality. A claimed successful lane is not proof of live production.
- `data/production.json`: actual shipped/build-installed/activated/observed/revoked/retired facts. Production is not inferred from source code, passing synthetic tests or a plan. Empty production certification is intentional until verified.

`data/capabilities.json` is the common identity/dimensions index; `data/source-catalog.json` holds exact source/tracker discovery roots; `schemas/*.schema.json` supply portable validation. `scripts/validate.mjs` checks key relations without network or third-party packages.

## Comparison dimensions

Architecture domain, capability family, feature/tool/algorithm type, source repo, active consumer surface (Windows/Android/Studio/Host/KB/Oracle), owner authority, lifecycle, end-state change, test type, risk, provenance, scope/privacy, dependencies, task/feature IDs, delivery stage, release deployment, observed user availability, accessibility, rollback, and evidence confidence. Multi-dimensional view should support **baseline vs final-wave target vs post-wave verified vs production available**, with count denominators and unknown/unverified visible. Do not collapse `planned`, `code_present`, `tested_fixture`, `hardware_verified`, `production_active` into a single checkmark.

## Intake: the 600+ lists and other trackers

Do **not** invent the 600+ count or mark these records imported. Search the three repos and Labs for machine-readable feature inventories, 600+/2000/217/100/200-series task lists, roadmaps, backlog databases, test matrices and orphaned-work mappings. Add every discovered source to `source-catalog.json` with SHA, path, format, count **measured from content**, owner and sync policy. Import with stable `source_item_id`, crosswalk to `capability_id` (many-to-many), preserve original source text, use `unmapped` and `conflict` rather than guessing. Avoid silently deduplicating unrelated tasks; preserve aliases and provenance. Allow manual adjudication.

## Evidence policy

A source file proves at most `code_present`; passing fixtures prove `tested_fixture`; a successful native test proves `hardware_verified` only with device/build/OS identifiers and raw logs; production activation requires Host/publisher/install or observed-user receipts and explicit owner scope. A future-lane target is a hypothesis until backed by a published handoff and integration result. Track blockers, negative tests, regressions, revoked availability and historical/retired records.

## Initial known context

TierFlow final-wave dispatch: https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/FINAL-WAVE/DISPATCH.md

TierFlow Wave 4 Integration 4 result: https://github.com/owenservera/tierflow/blob/main/VIVIM-VISUAL-RUNTIME/WAVE-4/INTEGRATION-4-RESULT.md — **BLOCKED** at seed: Lane B `render_root` guard incompatible with real renderer; physical Windows/Android/provider browser acceptance UNRUN; repository tests passing is **not production evidence**.

Final-wave eight lanes A–H are *planned work allocations*, not accepted production features. Keep the user-supplied known final goal in `docs/AGREED-END-STATE.md` and reconcile with later authoritative contracts.

## Start the dedicated session

Read [Dedicated session brief](DEDICATED-SESSION.md). Run `node capability-observatory/scripts/validate.mjs` from the repo root (requires Node), re-pin upstream sources and build two trackable ledgers plus a genuinely interactive comparison visualization from the normalized index. Never claim a visualization is already implemented by this seed.
