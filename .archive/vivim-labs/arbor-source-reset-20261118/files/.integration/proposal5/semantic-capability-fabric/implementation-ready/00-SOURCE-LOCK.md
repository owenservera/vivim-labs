# Source inventory and drift reconciliation before coding
## Audited 2026-11-12
- Original SCF PR #13 branch head `90bd81028fab71c6fadef608404f216e84b1629a` at kit start; it already contains SV-R012 (three additional commits) that must be preserved. This kit is an additive continuation, not a reset.
- ARBOR `main` had advanced to `98f15e327957956ec94189772d5e5813e7b6b07b` while SCF remained a draft; main changes include new HVI governing scope and prompts. Therefore **no code worker may fork an obsolete draft branch or rebase blindly**.
- Real contract: HVI `10-NATIVE-TOOLING-ONLY-SCOPE.md` overrides historical audit execution plans: native tool implementation and agent discoverability now, real donor comparisons later on explicit request.
- Existing SV claim/proof/explanation schemas are at `verifier/contracts/*.v0.schema.json`; **reuse**, never clone incompatible proof meanings.
- Current v2 parser `parseAnnotationV2(source,{path,contentHash,projectId,symbolId,registry})` is pure and exports only parser/grammars; scanner private. Unknown refs emit diagnostics AND records; reject source on any error.
- Current HQ `extractAnnotations(lang,text)` only covers v1 tags; indexer writes those to `annotations`. SCF typed fact side tables must not use that table as a v2/verifier proof source.
- `openDb` /`openMemoryDb` call `migrate(db)`, `SCHEMA_VERSION=1`, SQLite WAL; preserve all legacy tables and use additive `scf_schema_version` metadata with `CREATE TABLE IF NOT EXISTS` only.
- `buildWiki({db,root,projects,outDir,force,maxFilePages,graphIndex,persistHashes})` is existing one wiki writer. No competitor generator or external UI channel.
- `createDevIntelligence` Host mode requires actorId, sessionId, authorize callback, approvedRoots; existing `permit` rechecks per operation. Any SCF host read must go through it, not a new bypass.
- `compileSemanticWiki(input)` research accepts at most 128 supplied source bodies, uses `parseAnnotationV2`, has injected registry and Host catalog, and returns read-only `vivim.semantic-wiki-ir/0.1`; source contentHash is **caller-supplied** and not verified. SCF must verify Git blob/source bytes **before** calling research adapter.
- Semantic registry `SEMANTIC_KINDS` has only type,capability,action,schema,effect,event,contract; v3 IDs require a reviewed additive extension before using them.
- Node >=24 is required by `hq/package.json`.

## Prerequisite reconciliation (S0 worker MUST perform)
1. Resolve actual current ARBOR main, SCF draft branch, HVI feature/integration branch and SV changed paths at start; record complete SHAs and compare newer commits to this base.
2. Confirm HVI's installed API and native-discovery registry with *actual source*, not work-order descriptions. If missing/unaccepted, SCF HVI bridge stays adapter-only and integration gate BLOCKED.
3. Confirm exact v2 parser/getSemanticId APIs and schema tables still match; if changed, stop and file adjustment in S0 proof rather than edit active owners silently.
4. Inspect current Host auth/read and TierFlow display owner before implementing H/U; if unresolved PWA native-loopback/replay problems, remote SCF surface stays disabled.
5. Never compare real donor repository contents in this wave; synthetic Git-object fixtures only.
6. Freeze one new accepted `workerBaseSha` from current main after document review; all A–F feature branches fork that SHA, no implicit branch-tip work.

## Conflict rules
Source pins identify what was inspected, not guaranteed future branch heads. Worker changes of same file: A owns annotation-v2; B owns semantic-registry/schema; C owns wiki; E owns protocol/arbor-service/diagnostic CLI; F owns tests; G is sole serial merger. SV proof schema is authored in parent PR and is read-only for workers until separate owner-review. No merging PR #13 into current main merely because the near-code kit exists.
