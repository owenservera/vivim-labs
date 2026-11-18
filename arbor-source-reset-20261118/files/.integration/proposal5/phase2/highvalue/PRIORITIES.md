# Proposal 5 — H1 High-Value Labs Triage (source-only)

**Verdict: `PREPARED_NOT_PROMOTED`.** 2026-11-12. Isolated branch `work/p5-p2-highvalue-labs`, created from `architecture/proposal5-beta-first-centralization` at `6ab7e6976c02cac2ee71f030deda3f6541f8102f`. All writes restricted to this directory; **no W1/G, donor, runtime, feature, build or root changes**. Source inspection via connected GitHub only. **Tests executed in this H1 session: ZERO; all candidate acceptance: UNRUN.** Earlier reported Labs 28/28 and 100/100, HQ SWC 22 V8, voice workbench 10/10 are historic/research-author claims, not this session's tests or product acceptance.

## Five ranked investments — no extras on beta critical path

| Rank | Initiative | User value / effort / risk | Disposition and first allowed gate |
|---:|---|---|---|
| 1 | **H1-01 Source-truth composer semantic strip (R005)** | high / low-to-medium / medium | **OPTIONAL-LOW-COST-DURING-BETA**; After W2-G GO; optional W3 acceptance only if G approves zero-critical-path cost |
| 2 | **H1-02 Bounded storage and protected Rock baseline (R009)** | critical-safety / medium / critical-data-loss | **CORE-BETA-BLOCKER**; W2 source owner wiring → W3 G3 read-path proof → W4-G low-storage acceptance; must close by W5-G, W6-G release |
| 3 | **H1-03 One-screen reversible AutoWiki (R008)** | very-high / medium-to-high / high-privacy | **POST-BETA**; After W3-G verified internal HQ+Host source reading; security G4 source/permission proof; full trial after W6 beta |
| 4 | **H1-04 One multi-producer VIEW ALL trace (R007)** | very-high / high / high | **POST-BETA**; After W3-G source-grounding and G4 privacy+Node0 checks; expansion only post-W6 |
| 5 | **H1-05 Grounded two-Tab predictive command proof (R006)** | high / medium-to-high / high-source-staleness | **POST-BETA**; After W3-G Node0 and source IDs safe; require W4-G security/latency then post-W6 personalized features |

**Highest-value quick win: H1-01 R005 inert semantic strip**, *provided* existing TierFlow/Host/HQ contracts and G2 are verified first. Do not replace Host planning. **Biggest consequence risk: H1-02 Rock baseline**—compacting the wrong byte could erase irreplaceable World/receipts/originals. The beta requirement B12/G11 is an **existing hard gate**, not proof that Rock compaction exists or permission to expand it into a new system.

## Source-truth comparison: R005–R010 versus incumbent owners

| Feature | Actually source-present | Unsupported or research-only | Safe convergence |
|---|---|---|---|
| R005 semantic command UI | Labs `semantic-lens.mjs`, `bridge.mjs`, `compiler.mjs`, tests & static UI; historical author 28 Node unit / 100 probes | Native W1/UI, real device IME, contacts grants, runtime availability | E inert projection, B pure planner & catalog, D HQ source truth, F fixtures |
| R006 prediction | R005 session-local menu precursors; Host command planner source | 100 promised predictive features, two-Tab `full` real capability, persistent model | A Node0 bounded candidate → B admission only after human confirmation |
| R007 VIEW ALL | HQ 43-facet / 22-gap research and 31 observed producer-source seed; TierFlow inert IR | No universal producer subscription, span→memory→DCB→effect trace or visual renderer | Single source-vector trace; show NOT_RUN / SKIPPED_PRIVACY accurately |
| R008 AutoWiki | W1-D generated `hq/lib/wiki.mjs`, HQ search/context and E knowledge client | No native screen mirror, reversible flip or personal authorized corpus join | Ephemeral `ScreenContext → read → AutoWikiPage`; no second wiki writer |
| R009 Rock Core | HQ pure `storage-policy.mjs`, `rock-membership.mjs`; Host/HQ/native owners and B12/G11 safety requirement | No runtime quota agent, disk-pressure measured acceptance, live safe compactor | Owner-specific write enforcement and disk reserve, protected records never evicted |
| R010 semantic/wiki compiler | HQ `annotation-v2.mjs`, `wiki.mjs` and isolated inert research compiler | v3 meta tags, trusted source digest, native online compiler and “90%” benchmark | D source compiler → A pure typed computation → B catalog/admission → E projection |

Concrete anchor files (pinned Git blobs, **not** mutable main refs):
- [`owenservera/vivim-labs:nlp-command-ui-lab/docs/R005-PROGRESSIVE-SPRINT.md`](https://github.com/owenservera/vivim-labs/blob/67e76c5b730dcc2cd3ab3919c3ba273a6c260db8/nlp-command-ui-lab/docs/R005-PROGRESSIVE-SPRINT.md) blob `e8d8aeb663a8d3828b228ca1114f203a7bb4a6ce`; [`owenservera/vivim-labs:nlp-command-ui-lab/src/semantic-lens.mjs`](https://github.com/owenservera/vivim-labs/blob/67e76c5b730dcc2cd3ab3919c3ba273a6c260db8/nlp-command-ui-lab/src/semantic-lens.mjs) blob `d5de11edf693eb67ffed23e1fd26e68240d4420c`.
- [`owenservera/vivim-labs:nlp-command-ui-lab/docs/R006-100-PREDICTIVE-REALTIME-COMPOSER-CAPABILITIES.md`](https://github.com/owenservera/vivim-labs/blob/67e76c5b730dcc2cd3ab3919c3ba273a6c260db8/nlp-command-ui-lab/docs/R006-100-PREDICTIVE-REALTIME-COMPOSER-CAPABILITIES.md) blob `a8300777bc4676f83eef789fe07984e3302e49f6`.
- [`owenservera/vivim-labs:nlp-command-ui-lab/docs/R007-TOTAL-INTERPRETATION-VIEW-ALL-AND-HQ-HANDOFF.md`](https://github.com/owenservera/vivim-labs/blob/67e76c5b730dcc2cd3ab3919c3ba273a6c260db8/nlp-command-ui-lab/docs/R007-TOTAL-INTERPRETATION-VIEW-ALL-AND-HQ-HANDOFF.md) blob `cba4d05b853ae4de0cc69a0caee6fb7692fe867e`.
- [`owenservera/vivim-labs:nlp-command-ui-lab/docs/R008-AUTOWIKI-SCREEN-STATE-MIRROR-VISION.md`](https://github.com/owenservera/vivim-labs/blob/67e76c5b730dcc2cd3ab3919c3ba273a6c260db8/nlp-command-ui-lab/docs/R008-AUTOWIKI-SCREEN-STATE-MIRROR-VISION.md) blob `a819245e0c13b4cf7838129340ca5e04c34f5853`.
- [`owenservera/vivim-labs:nlp-command-ui-lab/docs/R009-ROCK-CORE-STORAGE-BOUNDED-INTELLIGENCE.md`](https://github.com/owenservera/vivim-labs/blob/67e76c5b730dcc2cd3ab3919c3ba273a6c260db8/nlp-command-ui-lab/docs/R009-ROCK-CORE-STORAGE-BOUNDED-INTELLIGENCE.md) blob `1c82e411503967364bfdaf25bd702733bf6f2cdc`.
- [`owenservera/repos-hq:research/semantic-wiki-compiler/semantic-wiki-compiler.mjs`](https://github.com/owenservera/repos-hq/blob/7cb2fadd88c5a08c0ec7e084410e150a5b6e0f3d/research/semantic-wiki-compiler/semantic-wiki-compiler.mjs) blob `83c2eab7921c84dd55f0503518e7fc7e4a466657`; [`owenservera/ARBOR:hq/lib/annotation-v2.mjs`](https://github.com/owenservera/ARBOR/blob/ad55fe28c2c331841675f308f9199ee4c2add86a/hq/lib/annotation-v2.mjs) blob `7e94dfbd4fbca8ef0fdb9e0610053364ce2c38c8`.
- [`owenservera/repos-hq:research/rock-core-storage/storage-policy.mjs`](https://github.com/owenservera/repos-hq/blob/0a3c11a6df72434b46931f90b25bf1f93da09f92/research/rock-core-storage/storage-policy.mjs) blob `4606b90c2635564e74c712e19771c5d244576e3e`; [`owenservera/repos-hq:research/rock-core-storage/rock-membership.mjs`](https://github.com/owenservera/repos-hq/blob/0a3c11a6df72434b46931f90b25bf1f93da09f92/research/rock-core-storage/rock-membership.mjs) blob `9023bd026a311cadf9f9d6d024562166a3176a3c`.
- [`owenservera/ARBOR:packages/tierflow/src/visual-runtime/command/ir.ts`](https://github.com/owenservera/ARBOR/blob/1a5cf26ce14ebce4e31d9a4373060ace61852645/packages/tierflow/src/visual-runtime/command/ir.ts) blob `a6e581476af9d5b091b1939591ad5d5ba8a32a45`; [`owenservera/ARBOR:apps/vivim-host/control/journal.mjs`](https://github.com/owenservera/ARBOR/blob/78a5105a9c1f1880d658f350e130b1e2632d3532/apps/vivim-host/control/journal.mjs) blob `b9739a00c8345a0513dba32ddc0361e96bb9b16f`; [`owenservera/ARBOR:native/hqkg/src/engine.rs`](https://github.com/owenservera/ARBOR/blob/dda546a11294920d6521a0f64ab7ee62ec993c70/native/hqkg/src/engine.rs) blob `fbb1100999fd229fb5a4352a78bbb0d77b643988`.

## Scope accounting, deliberately outside ranked top five

- **Visual app builder H1-06:** real typed DAG/Host program and TierFlow IR pieces, not source proof of a user-facing builder. Post-beta. No arbitrary executor or marketplace.
- **Voice H1-07:** Android `PocketRecorder` + pure Labs task evaluator are real source; ASR→review→single Host task commit→PWA journey **unproven**. Audit conditional operational continuity at G3; do not assume source existence means shipped feature.
- **Automation Queen H1-08:** Labs discovery/copy-only; HQ automation code ≠ verified running autonomous Host scheduler. Post-beta, no second scheduler.
- **Personal DCB/Oracle H1-09:** Host personal source/contracts exist, but optional/advanced learning, stored definition crosswalk, consent and erasure must be proven. Post-beta; no personal data into HQ.dev or public Labs.
- **Semantic Wiki Compiler H1-10:** high-leverage enabling research, not a sixth ranked build. Build on HQ annotation v2 and existing Host catalog, never code-comment-as-permission.
- **Extensions H1-11:** verified hook map and source-present ports differ from proposed **153** surfaces. F may preserve source-truth metadata; no installed plugin marketplace or dynamic loader.

## Minimum runnable proof order

1. **Pure source proof (outside this branch):** on a real checkout run `node --test nlp-command-ui-lab/tests/*.test.mjs` plus `node nlp-command-ui-lab/scripts/semantic-bench.mjs`; record exit codes and actual branch SHA. The R005 README test record is not enough.
2. **After G2:** E renders just one revision-checked, source-identified, non-executing semantic strip using the actual B catalog and D HQ source IDs. Negative test: `do not send` never invokes dispatch, Tab inserts no grant, Host refusal overrides lexer; stale output discarded. This is the smallest useful **product** proof.
3. **Existing beta B12/G11:** only source-owner-controlled synthetic low-space dry-run and then measured Windows/no-donor low-storage acceptance. Include non-rebuildable source/receipt guard and physical disk reserve. This is not license to promote HQ research planner as a data-deletion daemon.
4. **Post-beta:** one screen AutoWiki; one VIEW ALL trace; then one supported-action two-Tab proof. Never start with 43 facets/100 ideas.

See `cards.json` for each atomic value/effort/risk card, source pin, exact owner/package, minimum proof, negative tests, gate and explicit non-goals. See `DEPENDENCIES.md` for wave/collision sequencing and `HANDOFF-G.md` for promotion rules.

## Evidence limits

The inspected W1 A/B/C/D/E/F branches are **isolated imported source**, not a combined ARBOR build. The original `integrate/proposal5-beta-r001` branch returned 404 on inspection; W1-G is **not GO on that evidence**. Do not assert Windows, physical phone, source-authorized personal knowledge, signed-in provider or low-storage acceptance. The 2026-11-12 Labs first-pass readiness Actions jobs failed before startup; they did not execute tests. **Source present ≠ source active ≠ executed test ≠ release admitted.**

**Final disposition: `PREPARED_NOT_PROMOTED`.**
