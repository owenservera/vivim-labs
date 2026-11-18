# H1 — Dependencies and precise W2–W6 gates

**Status `PREPARED_NOT_PROMOTED`.** Read-only cross-owner roadmap, not a request to change any active branch. Wave mapping comes from [`owenservera/ARBOR:.integration/proposal5/09-EXECUTION-PLAYBOOK.md`](https://github.com/owenservera/ARBOR/blob/6ab7e6976c02cac2ee71f030deda3f6541f8102f/.integration/proposal5/09-EXECUTION-PLAYBOOK.md) and existing hard gates [`owenservera/ARBOR:.integration/proposal5/08-BETA-GATES.json`](https://github.com/owenservera/ARBOR/blob/6ab7e6976c02cac2ee71f030deda3f6541f8102f/.integration/proposal5/08-BETA-GATES.json). **No fresh tests executed here.**

## Truth-level dependencies (current sources)

```text
 W1-G source provenance and internal lock (NOT VERIFIED GO)
      |
 W2-G coherent in-ARBOR build, Host/HQ/TierFlow/native/Studio wire, security patch
      |                         |
   R005 inert strip [H1-01]   measured Rock baseline G11/B12 [H1-02]
      |                         |
 W3-G B05 Node0 / B06 Host / B07 HQ / B08 Personal source / B09 visual truth
      |                         |
  R007 producer trace       Rock protected read/degraded status
      |                         |
  R006 two Tab                W4-G privacy/revocation/no-donor/low-storage 
      |
 R008 screen mirror + authorized source + reversible state
      |
 W5-G blocker closeout → W6-G real Windows/PWA beta acceptance
      |
 Post-beta user programs, Queen, advanced DCB, personalized prediction
```

| ID | Required current owner ports | Gate dependency and stop | Collision prohibition |
|---|---|---|---|
| H1-01 R005 | B `intent-planner.mjs` / `action-catalog.v1.json`; D `annotation-v2.mjs` + inventory; E `command/ir.ts`; F acceptance | **G2 first**, G3 optional only on owner approval; G4 keyboard/mobile acceptance if shipped | No new parser, Host permission or TierFlow publisher |
| H1-02 Rock | B World/receipt/journal; D actual index/wiki; A graph journal; E device cache; F storage tests | **Existing hard** G11 + B12; W4-G physical low-disk and G5 closure; advanced retention modes *after* W6 | No compactor outside owner writers, no deletion of irreproducible bytes |
| H1-03 AutoWiki | D HQ wiki/search/index source vector; B authorization for personal; C exact bound provider thread; E knowledge client | G3 B07/B08/B09 + G4 privacy; after W6 first user mirror | No personal World ingestion into HQ dev, no Markdown flood |
| H1-04 R007 | D producer/source census; A pure join; B memory/DCB read; E view; F coverage | G3 source truth and G4 privacy; post W6 | Producer list is not live runtime, no hidden-source existence leak |
| H1-05 R006 | A Node0 bounded hints; B exact catalog/planner; D source semantics; E input; F latency | G3 B05/B06 and G4 IME/privacy; post W6 | No new `full update` action by inference or Tab effect |
| H1-06 App builder | A `core/programmability/`; B command program/catalog; E TierFlow | G3 canonical IDs, G4 grants/resource controls, post W6 | No arbitrary program runner/new action registry |
| H1-07 Voice | E Android recorder; B one task receipt; C Studio import; F real device/source | G3 donor continuity audit, G4 privacy, then G6 if already operational and approved; else after W6 | No ASR claim from `PocketRecorder`; no duplicate task DB |
| H1-08 Queen | B current Host mission/receipt; D HQ mission intel; F source inventory | G4 journal failure guard, post W6 | No second scheduler/executor or auto-permission |
| H1-09 DCB | B personal World, consent/definition state; A scoped graph read; D dev-only context; E projection | G3 B08 authorized source, G4 privacy/erasure, post W6 | No dev-index copy of owner personal memory |
| H1-10 SWC | D annotation v2/wikis + research compiler; B Host verified catalog; A pure computation; E read-only UI | G3 source-truth baseline; post W6 to compare verified digest and held-out tests | No compiled comment granting action |
| H1-11 Hook atlas | F source/truth manifest; owners A/B/D/E validate | After G2 source lock, existing G10 inventory proof | 153 proposed surfaces ≠ plugins/installed hooks |

## G's immutable owner and truth boundaries

- **A** owns `native/hqkg/`, `core/programmability/`, `src-tauri/`: typed Rust graph/compute and source-bound Node0; native cannot admit Host effects.
- **B** owns `apps/vivim-host/`: active capability catalog, current permissions, World grants, effect journal/claim/receipt, task commits, personal data custody; no UI shortcut becomes effect.
- **C** owns `studio/`, `harness/`: signed-in browser source thread and send; browser observation may be partial; no provider API or second sender.
- **D** owns `hq/`: deterministic dev index/search/context/wiki, annotation v2 and possible compiled semantic **read** model; dev database is not personal World.
- **E** owns `packages/tierflow/`, `apps/mobile-relay/`: visual command/knowledge projection, PWA; no separate publisher, personal memory or effect writer.
- **F** owns conformance/capability truth and independent tests: source-observed vs installed/available-now vs owner-authorized are distinct.
- **G** alone merges/updates integration baseline, locks, releases and source status. H1 proposes only, no merge/cherry-pick/activation.

## Adversarial acceptance matrix (all currently UNRUN)

| Concern | Synthetic negative proof then physical owner test |
|---|---|
| Semantic input | quoted/negated `do not send`; Host refusal wins; stale revision, Tab/undo/IME; zero dispatch |
| Interpretation coverage | producer skipped_privacy/not_run distinguish from no meaning; no unauthorized source IDs |
| Personal World/DCB | revoked source hides derived context; no HQ.dev shadow copy or assumed persisted memory |
| AutoWiki | old screen result dropped; screen→wiki→back preserves focus; bound thread only; no page flood |
| Rock | protected user-original/journal never evicted; Full cannot bypass volume reserve; partial compaction rollback |
| Prediction | unregistered `full update` abstains; device latency p50/p95/p99 measured, not guessed |
| Voice | pending upload ≠ committed task; duplicate recovery idempotent; consent and offline behavior |
| App builder/Queen | visual config cannot grant effect; no second writer/scheduler; uncertain send never replayed |

## Change and source-delta triggers before promotion

When G2 lands, compare the pinned Labs UI `67e76c5b730dcc2cd3ab3919c3ba273a6c260db8` against the pinned A/B/D/E ARBOR import roots above. Rebuild actual source inventories and mark each card `SAME_SOURCE`, `PURE_NEW_FIXTURE`, `OWNER_ADAPTER_REQUIRED`, `DESIGN_ONLY` or `REJECTED_COLLISION`. **If any new upstream SHA differs, do not silently use the latest branch—re-pin and rerun the delta/conformance review.** No external Labs checkout/runtime may be needed by an ARBOR beta.

Additional actual baseline provenance (used for collision assessment): Labs main `022608f5797e7b4a14ccc4c393bd3c5a4704a4be`; Labs R001 Node0 `61d9fc15c55144c4f0b79afa248981c2b0ca9bd6`; Labs Chronos `b87ea71c653c3833451bd0af19d9ef1c4327728e`; Labs readiness `f5edc9fc45c6c3208621d9ca172b36e9a0d547c0`; HQ main `9b922ea049dbadc94e504b5b9432743995c020ca`; Harness main `cae97afe848d79957c9d6c698a90996a758bfd56`; TierFlow main `50822f11ee63c3d80f631f8fea7dcb9b19bffc38`. The three HQ research branches have separate pinned refs in `cards.json`.
