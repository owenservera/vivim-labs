# PG00 negative acceptance — deny-before-effect contracts

**Status:** test sources and expected verdicts only; **not executed** here. The sibling `negative-contracts.test.mjs` exercises a minimal read-only PG00 metadata guard with `node:test`; it is **not** an application regression or a claimed Host/device security test.

| ID | Input attack/failure | Expected refusal/evidence | Exact owner path to prove after G |
| --- | --- | --- | --- |
| NEG-01 | duplicate or unknown v6 Host action ID; `source_wired` promoted to `verified` without runtime evidence | reject unknown, duplicate, changed risk/args; do not emit permission | B `control/{action-catalog.v1.json,intent-planner.mjs,dispatcher.mjs}` |
| NEG-02 | HQ `@hq:action` + semantically current ID (or claimed HQ `wired`) used as Host executor | **NO_HOST_HANDLER**, no claimed effect, even with nice wiki description | D `annotation-v2.mjs`, `semantic-registry.mjs`, `action-registry.mjs`; B dispatcher |
| NEG-03 | TierFlow `availableNow=yes` or fake injected CrosswalkRow but action not present/owned in v6 | `ACTION_NOT_REVIEWED`/refuse; no plan or approval from UI | E `visual-runtime/command/{ir,compile,crosswalk}.ts`; B catalog |
| NEG-04 | source sha spoof (64 valid hex but different actual bytes), stale digest, owner mismatch, revoked root | deny read/meaning binding; never substitute probable source | D `arbor-service.mjs` and S `inert-adapter.mjs` |
| NEG-05 | R3 `machine.network.disable_all` or R2 approval via forged phone/local text | R3 unsupported, R2 desktop-owner-token and exact digest required; no effect | B `control/dispatcher.mjs` |
| NEG-06 | changed plan digest, expired catalog, replay, journal claim without receipt | no re-execute; `uncertain` visible; do not infer success from missing error | B `control/journal.mjs` and dispatcher |
| NEG-07 | rogue server owns 127.0.0.1:43187 before Tauri launch; sends HTTP 200 status | native must NOT leak owner bearer to rogue service; process attestation required | A `src-tauri/src/main.rs`; B `control/browser-session.mjs` |
| NEG-08 | missing/rotated browser device key, expired P-256 challenge, stolen active grant+session bearer, nonce replay | no silent replacement key; no privileged request without valid current per-request proof, reject double nonce | E `browser-device-key.mjs`; B `apk/enrollment.mjs`, `direct-phone.mjs` |
| NEG-09 | mobile SW offline or hostile API response attempts replaying approved control/relay mutation | shell-only cache; API/authorization-bearing requests uncached, mutations never auto replay | E `sw.js`, `transport.mjs` |
| NEG-10 | Studio unknown provider delivery with same turn key; recipe retries despite journal uncertainty | journal fences; only read/reconcile, never re-send | C `thread-turn-journal.mjs`, `conversation-runtime.mjs`, recipe service |
| NEG-11 | malicious/unknown UI publish; stale draft revision or non-UiStore fixture source | deny publish/rollback; no second SQLite writer; no Host action claim | E `revisions/port.ts`, `wave3-atomic-replay.ts`, `fixture-store.ts` |
| NEG-12 | arbitrary `../`/symlink/junction source path to personal World, or revoked source grant while wiki is open | fail closed; no private bytes or cached page leak | D `arbor-service.mjs`, B Host grants |
| NEG-13 | simulated low disk deletes Host journal receipts, personal originals, SQLite -wal, or unrecoverable index bytes | refuse compaction/eviction; preserve protected classes and recovery evidence | D HQ index/writer, B Host World/effect journal; Rock is design constraint |
| NEG-14 | HQ dev watcher / native pure Scheduler / Labs Cron job tries end-user message or timed Host effect | does not execute; only Host future authorized action path may claim and receipt | D `automation.mjs`, A `engine.rs`, B `dispatcher.mjs` |
| NEG-15 | model/README/test fixture claims 90% interpretation, SECURITY_ACCEPTED or RUNTIME_VERIFIED with no exit/receipt | source-only/UNRUN classification; no beta GO | F interop and G gate |
| NEG-16 | G branch absent, root manifest 404 or original W frozen head changed | no auto-create integration or rewrite frozen HEAD; block any post-G code task | G sole `W1/G.md`, F source lock |

**Execution order after accepted G:** deterministic source-level guard → real negative unit tests (Node/Bun/Cargo with tool versions+exit) → OS/process attack → signed-in browser and Android PWA physical tests. Never equate this proof plan with actual pass.
