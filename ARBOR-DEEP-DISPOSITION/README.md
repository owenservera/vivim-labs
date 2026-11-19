# ARBOR Deep Disposition — Five Parallel Lanes

**Baseline:** ARBOR `main`, commit [e07e61385a53f30890b4a7f7441fe25d22a79b1c](https://github.com/owenservera/ARBOR/commit/e07e61385a53f30890b4a7f7441fe25d22a79b1c), 2026-11-19.  
**Mission:** convert the entire ARBOR repository into a minimal, evidence-backed, dependency-ordered product implementation programme. Do not start another software architecture, build another tracker, or mistake inventory coverage for completed disposition.

## 1. Canonical ledger and baseline

The only master ledger in this folder is `ARBOR-MASTER-INTEGRATION-TRACKER.csv.gz`, a lossless gzip container for the full `ARBOR-MASTER-INTEGRATION-TRACKER.csv`. The direct uncompressed upload exceeded the connector's transport capacity. Decode into a local working directory, not another tracked master.

```bash
python -c "import gzip,pathlib; p=pathlib.Path('ARBOR-MASTER-INTEGRATION-TRACKER.csv.gz'); pathlib.Path('ARBOR-MASTER-INTEGRATION-TRACKER.csv').write_bytes(gzip.decompress(p.read_bytes()))"
```

Baseline CSV SHA-256: `e18a5ebc55dcd4c90c9cff5b1b953708a01197f80b3ebfad63aa2a4be397800a`. Uncompressed Git blob: `3f64dbe3e4c091e4d7f3f4cb8a79a6a1ec58d2fb`. Compressed Git blob: `92074ea249d0c30741790cf11bb1a49d17c97796`. The original 75 fields were preserved and eight fields added. Baseline: 4,741 unique rows, 2,558 file records (complete pinned tree), 2,183 symbol records, 83 columns. Every symbol is linked to its parent file. Prior DR01–03 and FB01–02 are retained: 2,500 rows triaged or assessed, not 2,500 fully verified; 2,241 pending initial disposition. Of all rows, 303 currently carry NEXT; these are candidate rows, not distinct work orders. Runtime tests remain UNRUN unless new execution evidence is attached.

### Merge log — 2026-11-19 / A-F01

A-F01: 459 stable-ID row updates applied, 49 prior proposals included unchanged, 410 continuation proposals included. Status: FULL_ROW_COVERAGE_STATIC_PROVISIONAL. Action counts: REUSE 24; RETAIN 190; CHANGE 6; VERIFY 233; DEFER 6. Lane A-F01 NEXT: 26 rows, six existing implementation units. Global NEXT: 306 rows (distinct nonempty implementation-unit keys: 15; other unkeyed rows not counted as distinct units). All 4,741 identities and 84 columns retained; 4,282 non-target rows unchanged field-for-field; runtime tests UNRUN. Source pin: `e07e61385a53f30890b4a7f7441fe25d22a79b1c`. Prior overwritten fields and all appended evidence retained in per-row tagged history. No product source edited, and no exhaustive caller or runtime certification asserted.

Post-A-F01 CSV SHA-256: `4c768c3d0828eb15e2c2b26a91208488b62222ca2103458e24af5fbf9beedac0`. Post-A-F01 uncompressed Git blob: `49fbf95c069865bf2707f926b20cb1809abd8f6f`. Post-A-F01 compressed Git blob: `011bb8d6af37f0127e3133ccd8a2106164114f90`. Previous compressed Git blob: `76ec801d4b6ca4733433dd89ff61dcccacddbe1d`.

### Merge log — 2026-11-19 / D-F01

D-F01: 338 stable-ID assessment updates applied, including 36 earlier proposals and 302 continuation proposals. Status: FULL_ROW_COVERAGE_STATIC_PROVISIONAL. Resolved action counts: REUSE 47; RETAIN 226 (including 171 ISOLATE variants); VERIFY 64; DEFER 1. Phase counts: NEXT 1; RETAIN 271; VERIFY 64; LATER 1; DEFER 1. Fifteen existing implementation-unit keys represented; no new NEXT row promoted. Global NEXT: 306 rows, 16 distinct nonempty implementation-unit keys. All 4,741 stable IDs, 2,558 unique file paths, 2,183 symbol-parent relationships and 84 columns retained; all 4,403 non-target rows unchanged field-for-field. Prior values archived in tagged D-F01 entries in the existing per-row history/evidence column (its legacy header is `B-F01 review evidence`). Source pin: `e07e61385a53f30890b4a7f7441fe25d22a79b1c`. Corrected two transient transcription errors against the pinned source tree and original digest: relay-core test key `AT-4BE1BA1FED4` to canonical `AT-4BE1BAA1FED4`; VEC package blob SHA for `AT-8399A1FC9A14` from `c82387b56e9ae3c5b63c369353ebea9553176f42` to `c82387b56e9ae3c5b73c369353ebea9553176f42`. Zero unresolved row conflicts, no product source edits, no exhaustive caller-closure or device acceptance certification. Runtime tests UNRUN. CSV serialization and gzip decompression round-trip verified byte-for-byte.

Post-D-F01 CSV SHA-256: `1b35b2e5c130e2335da25e94fa0394d838bfe8f0fc8c37f955ea22f362ec0ba4`. Post-D-F01 compressed Git blob: `a6868815f3383d73c5dc9159b55befc5460c0944`. Uncompressed bytes: 11,026,480; compressed bytes: 716,160.

### Merge log — 2026-11-19 / D-S01 part 1 of 2

251 symbol rows across ten pinned PWA/NLCL source files, recorded directly by stable ID; 159 remaining. Counts: {"REUSE":167,"RETAIN":71,"VERIFY":12,"CHANGE":1}. Phases: {"RETAIN":238,"VERIFY":12,"NEXT":1}. Source-anchor matches 251; lexical same-file references 224; selected imported-module references 18; declarations and bounded source segments are not exhaustive caller verification. Source-backed revocation defect: pending prompt text in sessionStorage is not cleared by clearPrivatePhoneState; one existing D14.device-pairing unit NEXT, refer Host/B for receipt status preservation. No runtime tests run. All 4,741 identities and 84 columns retained; 4,490 non-target rows unchanged. Global NEXT 307; distinct nonempty keys 17. Source pin `e07e61385a53f30890b4a7f7441fe25d22a79b1c`. Gzip/CSV roundtrip verified.

Post-D-S01 part 1 CSV SHA-256: `4c1fa28dd817f84a0b8db9f8716a8ca35fcded25138c86c67dd3a040166211fa`. Post-D-S01 part 1 compressed Git blob: `7af50feba00714126b230d7727f5284d8597c295`. Uncompressed bytes: 11766946; compressed bytes: 766734.

### Merge log — 2026-11-19 / D-S01 part 2 of 2 — LANE D COMPLETE ROW COVERAGE

159 additional symbol rows across thirteen pinned renderer, VEC, knowledge and revision files; all 410 D-S01 rows now have decisions. Part 2 counts: {"RETAIN":68,"REUSE":40,"VERIFY":31,"DEFER":20}. Part 2 coverage: 159 exact source anchors; 53 type declarations, 28 constant/data declarations, 78 callable/class anchors; 141 lexical same-file references, 6 selected cross-import references; 11 named counterpart evidence entries (includes historical callers, not all production). Entire D-S01 action counts: {"REUSE":207,"RETAIN":139,"VERIFY":43,"CHANGE":1,"DEFER":20}; phases: {"RETAIN":346,"VERIFY":43,"NEXT":1,"DEFER":20}. Entire Lane D (338 files + 410 symbols): {"REUSE":254,"RETAIN":365,"VERIFY":107,"DEFER":21,"CHANGE":1}. 15 existing implementation-unit keys represented by D rows, not 748 independent changes. No new NEXT promotion in part 2. Live template recipe and engine compiler reuse verified through existing PWA callers; historical APK adapter, proof orchestration and independent revision writer are not baseline product authorities. Global NEXT 307, 17 distinct nonempty keys. All 4,741 IDs, 2,558 distinct file paths, 2,183 linked symbols and 84 columns retained; 4,582 non-target rows unchanged in this commit. Source pin `e07e61385a53f30890b4a7f7441fe25d22a79b1c`. Source anchor and bounded lexical analysis is not full caller closure, runtime or device acceptance. No product source modifications; tests UNRUN. Candidate blob independently re-read, decompressed and compared against parent after connector call-limit interruption.

Current CSV SHA-256: `54ee9ed46c56ad0bcf5fba4498cf66b18dee9c6ea7373fc60f94f70589175335`. Current uncompressed Git blob: `34ebd7056523df3672b9e0b4283f568b6a1d3c3a`. Current compressed Git blob: `46a896fb419861ade4d6eda804a33a3277e72555`. Previous compressed Git blob: `7af50feba00714126b230d7727f5284d8597c295`. Uncompressed bytes: 12243917; compressed bytes: 800770. CSV and gzip roundtrip verified.

### Merge log — 2026-11-19 / C-F01

C-F01: 380 file rows merged by stable ID (59 earlier proposals and 321 continuation proposals); 809 C symbol rows remain outside this batch. Status: FULL_ROW_COVERAGE_STATIC_PROVISIONAL; no exhaustive caller/runtime verification. Action counts: {"REUSE":80,"RETAIN":208,"CHANGE":15,"VERIFY":76,"DEFER":1}. Phase counts: {"NEXT":30,"LATER":66,"RETAIN":208,"VERIFY":76}. Global NEXT: 318; distinct nonempty implementation-unit keys: 22. All 4,741 IDs and 84 columns retained; 4,361 non-target rows unchanged field-for-field; exact touched-cell base comparisons and supplied original row digests checked. Superseded fields and source anchors retained in tagged C-F01 history in the existing evidence column. Runtime tests UNRUN. No product source edited.

Post-C-F01 CSV SHA-256: `13693e38fcb9fa10447e3e11c20b813b2b1803961e5ef0ea0a1fd039c85d2e33`. Post-C-F01 compressed Git blob: `8e57be49ce20bedcfae8306ae16acc0b5e882e8f`. Previous compressed Git blob: `46a896fb419861ade4d6eda804a33a3277e72555`. Uncompressed bytes: 13503303; compressed bytes: 1055424. CSV/gzip round-trip verified.


Recompression-only follow-up: identical CSV SHA-256 and row values; compressed Git blob `bdde887f6463da8264c7f596158fee372c692d30`, 880621 bytes, below 1 MiB Contents API limit. Supersedes compressed container `8e57be49ce20bedcfae8306ae16acc0b5e882e8f`; no dispositions changed.

## 2. Parallel assignments — filter the ledger, do not clone it

| Lane | Exclusive responsibility | Files | Symbols | Total | Pending initial | Verification revisit | Prior decision revalidation |
|---|---|---:|---:|---:|---:|---:|---:|
| A | Native Rust, HQ and HVI | 459 | 617 | 1076 | 175 | 219 | 682 |
| B | Host, Harness, execution authority | 331 | 344 | 675 | 222 | 251 | 202 |
| C | Studio, browser, personal intelligence | 380 | 809 | 1189 | 341 | 319 | 529 |
| D | PWA, TierFlow, VEC, NLCL and visual interaction | 338 | 410 | 748 | 566 | 114 | 68 |
| E | Foundations, contracts, packaging, historical material | 1050 | 3 | 1053 | 937 | 99 | 17 |
| Total | | 2558 | 2183 | 4741 | 2241 | 1002 | 1498 |

Assignment follows the existing file's subsystem label, inherited by its symbols, rather than a second architectural taxonomy. Existing global Review bucket values remain intact. Use **Parallel lane**, **Lane batch**, **Lane batch item**, **Lane batch size**, **Lane review priority**, **Lane review need**, **Lane assignment rationale** and **Lane ownership rule**. Within each lane file buckets precede symbol buckets; NEXT/VERIFY evidence and priority guide ordering. Every batch contains at most 500 rows:

| Lane | File-first batches | Symbol batches |
|---|---|---|
| A | A-F01: 459 | A-S01: 500; A-S02: 117 |
| B | B-F01: 331 | B-S01: 344 |
| C | C-F01: 380 | C-S01: 500; C-S02: 309 |
| D | D-F01: 338 | D-S01: 410 |
| E | E-F01: 500; E-F02: 500; E-F03: 50 | E-S01: 3 |

Launch A-F01, B-F01, C-F01, D-F01 and E-F01 simultaneously. Review every row in each batch; original decisions are hypotheses to validate, not authority to change real product code. Within a batch follow dependency/priority order when useful. File analysis can resolve its symbol children by explicit evidence; do not count inherited classification as independent body inspection.

### Lane A — Native intelligence and development knowledge
Trace existing Rust graph, typed DAG, engine registry, invalidation/cache, durable descriptions, data ingestion and recovery; HQ semantic source indexing, VLANG source evidence, HVI comparison and authorization boundaries. Establish actual source-to-native-to-consumer paths. Prefer existing HQKG operations; separate durable indexing from bounded retrieval. Reuse scoped tombstones, stable identities and revisions. No new database, graph, scheduler or mandatory Git executable. Reconcile D01/D02/D03/D04/D07 and send actual Host integration changes to B. Identify pure compute worth native execution with benefit and tests; preserve portable computation when appropriate.

### Lane B — Host authority and Harness convergence
Trace product journal, dispatcher, approval binding, exact effects, startup, native adapters, recovery, DCB ports and Harness execution. Preserve Host as sole effect/World authority; Studio executes browser effects only through authorized existing ports. Investigate alternate send and dev-auto-approve paths; do not relax genuine authority, identity, replay or data privacy protections. Remove artificial workflow constraints only with concrete source proof. Distinguish legacy baseline/demo generators from live product. D05/D06/D12 cross-lane dependencies require single owner. Prefer reusing existing ports; do not introduce a central router, another journal or another command interpreter.

### Lane C — Browser, Studio and personal intelligence
Trace real browser capture, conversation API/runtime/service/journal/adapter, profile ownership, recorder/replay, adapter learning, evidence, personal memory, World projections and recovery. Reuse verified Studio journal evidence in Host; never resend uncertain effects automatically. No provider APIs or Playwright/Puppeteer. Confirm the mounted missing browser-companion protocol import and propose minimal source-closure repair or retirement; do not revive obsolete systems by default. Native integration is for authorized pure compute, memory search, lineage and diagnostics, not CDP actions. Preserve browser custody, credential protection, read-only learned adapters and single persistence ownership.

### Lane D — PWA and visual interaction
Trace the actual mobile-first Talk/Build/Wiki journey through NLCL/VLANG/VEC, previews, exact committed revisions and real renderer consumption. Distinguish working TierFlow renderer and engine catalog from inert fixtures/legacy standalone executors. Keep browser rendering/offline operation portable; use Rust only when concrete deterministic compute reuse is justified. Keep PWA-first, APK deferred, and Host-owned persistence. Do not rebuild another visual editor or central command router. Validate source-bound UI, tokens, templates, accessibility, component engines, local capture, pairing/revocation and revision readback. Hand authority changes to B and native computation changes to A.

### Lane E — Repository foundations and release truth
Review contracts, build/package scripts, CI, tests, fixtures, historical proofs, design documents and other files. Determine actual production reachability rather than treating all historical files as product work. Verify missing root bun.lock, native binary staging, empty Studio test target and actual test-manifest omissions; check test reality rather than launch receipts. Preserve useful security and deterministic fixtures but isolate stale/demo/historical sources from production imports. Identify retirement candidates only after dependency and provenance checks. Keep release verification honest; no invented gates or synthetic acceptance. Route product owner-specific fixes to A–D, retaining exclusive ownership of E rows.

## 3. Deep disposition protocol — evidence before wiring

For every row:

1. Read current ARBOR main HEAD and compare against the inventory pin; do not silently mix revisions. Record new source SHA and source ranges. If changed, preserve prior history and mark revalidation. New/deleted paths are explicit inventory delta, not silently dropped. Stay on the pinned baseline or a clearly stated new pin for that batch.
2. Inspect executable body, exports, real imports, reverse callers, route/registry mounting, configuration, packaging, persistence and tests as applicable. Check dynamic imports, feature flags, and external boundaries. Path/name or declarations alone are insufficient for a verified decision. For historical or non-code rows use actual dependency checks; source value and archival status are independent of production reachability.
3. Record producer, consumer, existing owner, effect authority, data class, read/write behavior, source revision, verification evidence, missing dependency and counterevidence. Current wiring and desired target are distinct; mark unknown rather than inventing. Classify direct body+caller evidence vs body only vs declarations vs path/manifest only; runtime evidence is a separate field. Mark actual test outcomes only if run.
4. Set a concrete disposition: CHANGE (specific source defect or necessary wiring), REUSE (existing capability through parent port), RETAIN (correct local/authority boundary), CONSOLIDATE (verified duplicate and canonical owner), ISOLATE (fixture/dev/historical boundary), RETIRE_CANDIDATE (verify callers/data/provenance before removal), VERIFY (unresolved evidence, excluded from next queue), DEFER (nonessential for baseline). Preserve original disposition fields. Use existing Resolved action vocabulary CHANGE/REUSE/RETAIN/VERIFY/DEFER for rollups, and express precise variant in Atomic decision / boundary.
5. Set revised should wire, priority, target depths for Rust/Host/Studio/PWA/HQ/VLANG/TierFlow/Harness/AutoWiki/Tauri, and acceptance test; do not automatically widen access. Target depth 0 = no new direct connection; 1 = contract/identity; 2 = read/consume; 3 = active production connection; 4 = owner. These are proposed architectural relationships, not % complete. Preserve owning authority and distinguish actual state from target.
6. Group necessary changes into an existing implementation package and a stable implementation-unit/dedup key. For cross-lane dependency record referred owner, exact source/target symbols, contract and dependency order; the other lane is informed, not given duplicate ownership. Update Phase decision NEXT only for supported high-value actual work; otherwise LATER, RETAIN, VERIFY or DEFER. Unknown/source-only claims do not become NEXT by default. Count distinct repair units as well as candidate rows. No product edits during disposition.
7. Record specific acceptance (source closure, caller integration, visible product outcome, durable readback, revocation/invalidation, negative case, restart/device behavior). Runtime status remains UNRUN where unavailable. Do not mislabel fixture results as production/device acceptance.

### Baseline objectives and narrowing rules

- One natural-language product interaction loop: typed interpretation → preview → exact approval → Host execution → persisted authoritative state → live PWA/Studio/Wiki readback.
- One native graph/compute foundation, no redundant graph/index/cache/journal or routing system when existing components suffice.
- Local-first, owner-controlled workspace, minimal authorized disclosure, explicit effect grants, revision pins, provenance, revocation and safe uncertainty recovery.
- Reuse actual code first; no provider APIs, mandatory Git installation, central I/O router, invented approval layers, or unnecessary Rust ports for DOM, browser or ordinary UI work.
- Prioritize direct user value and source closure. Narrow the next phase to distinct concrete repair units with dependencies, paths and executable acceptance, not thousands of independent tasks.
- Every tracked file ultimately has a reviewed disposition; symbols inherit only justifiable parent conclusions. Historical material is retained, isolated or retirement-reviewed, never implicitly promoted.

### Starting defect/connection hypotheses (recheck current source)

D01/D02: bounded native graph snapshots currently discard omitted nodes; distinct storage vs context budget, explicit tombstones and query-aware retrieval. D03/D04: native validation is not full DAG execution; reuse native executor and revision invalidation rather than forge effects. D05: root bun.lock absent; mounted Studio ingress imports absent apps/browser-companion/protocol.mjs; verify exact current packaging and test targets; Studio test path absent; central manifest 42 additional test paths at baseline. D06: existing Studio journal verification and uncertainty reconciler; extend existing receipts, no second writer. D07: HQ graph import invokes cargo; HVI requires live source adapter/grant, no forced Git. D12: DCB default disclosure denied, stale roadmap/atomic registry refs; use real authorized source ports, never relax privacy. D09 and other prior package decisions remain candidates in ledger. These are review starting points, not automatic tickets or newly verified tests.

## 4. Concurrent update discipline

All lanes work on ARBOR source read-only during audit. The shared ledger stays in this folder on vivim-labs main. **Never overwrite it with an older copy.** Five sessions can review concurrently, but a single merge operator serializes changes by stable ID (not row position). Each session produces a transient keyed row patch with base ledger Git blob/CSV hash, lane, batch, reviewed source pin, expected old field values or row digest, changed fields, evidence and cross-lane referrals. Do not persist five alternative trackers. On merge, reload current master, check expected old values, reject actual conflicts only, preserve untouched fields and IDs, reconcile any same-package discoveries by dedup key; serialize and recompress the complete CSV. Commit directly on main respecting concurrent changes, no force-push and no new branches or PRs. Use expected HEAD or retry after reconciliation. The patch may be exchanged locally or in the agent's response; discard it after incorporation, no duplicative tracking files in Labs. Record new hash, count, and change summary in the commit. Only merge operator updates the canonical ledger; lane workers may propose changes but must not independently replace it. Cross-lane referral is not a second assignment.

Verify on every merge: IDs unique; every file path unique and all symbol parents exist; all 4,741 baseline identities retained unless an explicit inventory delta is authorized; no lane overlap; batch <=500; original review decisions retained as history or explicitly superseded with reason and new evidence; NEXT rows and unique repair units counted separately; no unsupported promotion of provisional rows.

## 5. Launch instruction for each parallel session

```text
ARBOR — DEEP DISPOSITION / LANE <A–E> / FIRST FILE BATCH
Read this README and load the current single master ledger. Verify ARBOR main,
compare it to the baseline pin, and filter only your Parallel lane and first
Lane batch. Inspect actual source and callers, not just old documentation.
Review every assigned row; preserve all original fields and stable identities.
Give evidence-backed top-level and deep dispositions, target integration depths,
remaining unknowns, specific acceptance and deduplicated work units. Reuse
existing capabilities and authority owners. Refer cross-lane changes to their
owner. Do not change ARBOR product source during this round. Return a keyed
patch for the single merge operator; do not overwrite the master or create
another permanent tracker. State exactly which rows were body/caller verified,
which remain provisional and which tests were actually executed.
```

First wave is A-F01, B-F01, C-F01, D-F01, E-F01. Subsequent waves advance each lane independently until all file and symbol batches are disposition-reviewed. Then reconcile the minimal dependency-ordered implementation phase. 
