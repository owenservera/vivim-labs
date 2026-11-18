# Lane D — ownership conflicts and required decisions
**Gate: PREPARED_NOT_PROMOTED.** Exact pinned source evidence; no runtime acceptance or automatic merge.

## C1 — HQ supervisor vs canonical Host supervisor (P0)
Donor `repos-hq@9b922ea049dbadc94e504b5b9432743995c020ca`:
- `hq/supervisor/supervisor.mjs` blob `0660b0bd90582313fed4bf3ce53177a25f818e1b`, imports `node:child_process`, exposes `launchLane`/`dispatchTasks`/`resumeLane`.
- `hq/supervisor/admission.mjs` blob `43f8e35bc7c824b4b3f6bb53793f4ca01f0fd13f`; `recovery.mjs` blob `6948e2416169965bf433b13e835b58d04c44e78d`, pure functions.

ARBOR Host pin `f9823a5131e73e7562db5a4ec14e1ec481557d69` has `apps/vivim-host/bootstrap/worker-supervisor.mjs` blob `86a4bf8598d8c51db490daa60f504877e2d39e47`, and `apps/vivim-host/runtime/supervisor.mjs` blob `26a27d93e6e10824a7d093d929fe991a9c276db3`. **Decision:** Host owner must forbid duplicate process launcher and may consider extracted pure admission/replay after compatibility review. No migration of old W0 effect controller.

## C2 — HQ live graph / Wave1 Rust graph vs native HQKG (P0)
Donor `hq/live-graph/wave1/master/build-graph.mjs` blob `5cd96cda803c39d5e2c0e8ded025eab77495647a` and `master/master-graph.mjs` blob `7d074c9c147dbba7097cf0a7d6642ad5969e1c72` compile evidence and produce native feed candidates. `hq/wave1-rust-graph/lane-b/round-2/bounded-reader.mjs` blob `7094fe9592ad1bf04223291976d6aa1834772bdb` and `sqlite-inspector.mjs` blob `97431f16a22af1a54036dcc0986a39aff77dc188` have read-only diagnostics.

ARBOR native `native/hqkg/src/lib.rs` blob `2f4eddd34e64a53430df97ada0e2bd3960592935`, `scoped_graph.rs` blob `586843b56eedd4a9926bb443cbbeda80d841bda4`, `engine.rs` blob `fbb1100999fd229fb5a4352a78bbb0d77b643988`. **Decision:** HQ/Rust owners determine missing *read behaviors* and Rust interface coverage before adopting utilities; no second graph store, writer or journal.

## C3 — Catalogue identity and semantic registry (P0)
Original `hq/catalogue-identity/crosswalk.mjs` blob `ed4cb6f561e055e1dd3145b61de6f50f2555687d` exports `buildCrosswalk`; only Lane B may port under its scoped ownership. Existing `hq/lib/semantic-registry.mjs` blob `ec982381c77147119f62e702a162a203df8e7d4e` and `hq/lib/catalogue-delta.mjs` blob `6849e6df26bd136ee50ae08a260d8ab9e9445ac7` stay canonical. **Decision:** explicit identity mapping, no duplicate registry, no owner-inferred tombstones.

## C4 — Host personal World, Studio sender, TierFlow publisher (P0)
Original HQ `hq/work/lane-a3-handshake.mjs` blob `1743f9c1fcd04fb4b4f1f3843a04e684428ccba9` imports channel broker for live provider turn; not an HQ parity feature to copy. ARBOR Host dispatcher `apps/vivim-host/control/dispatcher.mjs` blob `723865482b8a0f98ab3214b2ec6a83aaea629c43` owns effect authorization; Studio signed-in browser sender remains exclusive. Personal intake `apps/vivim-host/corpus/intake/host-intake.mjs` blob `1db63cf3b0cc2dca09590a203b93fc895d4c936c` is an existing isolation boundary; Labs ZIP imports may only feed it through Host-approved authority. TierFlow `packages/tierflow/src/visual-runtime/command/ir.ts` blob `a6e581476af9d5b091b1939591ad5d5ba8a32a45` is inert. **Decision:** no effect executors, browser sender, personal data authority or second visual publisher in HQ.

## C5 — Rock Core, Chronos, semantic compiler/AutoWiki (P1)
- HQ Rock `storage-policy.mjs` blob `4606b90c2635564e74c712e19771c5d244576e3e` pure planner; per-store owners decide cache eviction, ENOSPC safety, protected originals and receipts. Not a new database.
- Labs Chronos `chronos-toolkit/wave2/.../supervisor-timeline.mjs` blob `d24b645939467a0d00e0dff088da87890f5310c2` references old HQ supervisor replay. Existing ARBOR staged contract `contracts/chronos/staged-contract.v1.json` blob `a74a612458e1854a95bc8331e6ed14747fec2742` is **not activated**.
- HQ SWC prototype `research/semantic-wiki-compiler/semantic-wiki-compiler.mjs` blob `83c2eab7921c84dd55f0503518e7fc7e4a466657` imports retained `hq/lib/annotation-v2.mjs`; it neither executes commands nor registers authorities.
- AutoWiki research architecture `research/autowiki-screen-mirror/ARCHITECTURE-AW-001.md` blob `b849f3beff9da8ac2ac17fac0856882dbfd075b9` remains design-only.

## C6 — Tests and release claims (P0)
Original 144 missing `hq/test/**` path owners must inspect imports, fixtures, subprocess and private data risk before reusing. Node/Bun tests **UNRUN**. No accepted evidence for physical device, Host read/revoke, Node24 compilation, Cargo/Bun lock generation, Windows build, browser sender security; separate existing `integration/beta-gates.json` stays **BLOCKED_NOT_BETA**.
