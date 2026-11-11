# Personal Knowledge T=0 — Labs integration implementation
Updated: 2026-11-11. Stage: **LOCAL SYNTHETIC TESTED, PRODUCTION HOST/RUST WIRING NOT ACCEPTED**.

## Mission and boundaries

This runtime folder is a staging implementation for VIVIM's ChatGPT-first onboarding and continuous personal knowledge acquisition. It is **not** another service, second graph store, browser automation owner, or authorization authority.

Owners:
- Host `apps/vivim-host/onboarding/service.mjs`: provider readiness, local owner identity, permissions and exact user approval. T=0 begins after a qualified ChatGPT sign-in is observed by Studio.
- Host `apps/vivim-host/bootstrap/ledger.mjs`: durable SQLite `transaction(fn)` and `meta.get/set`. `src/host-ledger-port.mjs` binds the stage worker to that existing API and creates **no additional database**.
- Host bootstrap scheduler: F02/F01 prompts, approval and Studio effect reconciliation; archive worker never sends a prompt.
- Studio: only browser effects and bounded read-only chat index. This package never attaches to Chrome, uses CDP or private provider endpoints.
- Host evidence/World: sole authorized personal evidence writer and commit journal. The worker calls a Host-injected sink and **requires verified receipts**.
- HQKG Rust `scoped_graph.rs` and `graph_change_set.rs`: computational graph apply backend. `src/scoped-graph-proposal.mjs` makes deterministic **proposals**, not a Rust FFI call; the typed bridge is an explicit integration gate.
- TierFlow: progress, provenance and source coverage display only.
- Labs: code, contracts and **synthetic fixtures only**. Never commit real conversation exports, credentials or sensitive personal evidence.

## Code map

| File | Function | Status |
| --- | --- | --- |
| `src/t0-core.mjs` | T=0 grants, actions and receipt state machine | source-implemented, synthetic tests authored previously |
| `src/ports.mjs` | fixture and owner-path reference map | source-implemented |
| `src/zip-reader.mjs` | bounded ZIP32/ZIP64 directory parsing, safe streaming, CRC and expansion caps | tested synthetic |
| `src/json-array-stream.mjs` | strict UTF-8 incremental top-level JSON object-array parser | tested synthetic |
| `src/chatgpt-normalize.mjs` | preserves roles, message parent/children, timestamps, text, omissions, digests | source-implemented, exercised by import fixtures |
| `src/chatgpt-import.mjs` | archive fingerprint, explicit permission checks, batching, pending journal, resume/reconcile | tested synthetic |
| `src/host-ledger-port.mjs` | maps existing SQLite ledger interface to checkpoint transactions | tested against mock ledger; real Host ledger not integration-tested here |
| `src/scoped-graph-proposal.mjs` | source-backed scoped node and edge proposals based on current Rust contract | tested synthetic; not Rust-executed |
| `src/host-archive-wiring.mjs` | injection glue Host ledger/grants/evidence/journal/Rust proposals/T0 accounting | tested synthetic; production port methods remain to be wired |
| `bin/inspect-chatgpt-export.mjs` | user-invoked archive counts, no raw content output | code committed; real export test unrun |
| `bin/bench-synthetic.mjs` | self-contained synthetic-only throughput measurement | exercised locally |

## Archive intake state and transactional choreography

Input: **user-selected absolute local ZIP path** + current owner World/account identity + exact archive import grant. The worker never downloads, scrapes or requests ChatGPT exports itself.

1. Permission preflight -> read ZIP directory, reject unsafe paths, symlink entry types, duplicate names, invalid ZIP64, unsupported algorithms, encryption, excess entries/uncompressed bytes/ratio.
2. Select exactly one `conversations.json`; scan/hash the **same open file descriptor** to bind checkpoint to precise archive content.
3. Stream deflated/stored content, verify size + CRC; parse a top-level JSON array object by object with bounded object size.
4. Normalize conversation/message IDs, parent/child relations, role, creation/update time and nontext omission flags. Text remains local to authorized Host sink, not the metadata journal or Labs repo.
5. Group bounded batches; sort *within* each batch by recent update. Batch identity derives from source-qualified archive SHA256 and normalized content digests, never filename or title alone.
6. Under existing Host transaction store, reserve `pending` batch before effect. Query Host's authoritative `inspectEvidenceBatch`; **uncertain blocks**. Only a confirmed-absent entry may proceed to `commitEvidenceAndGraph`.
7. Host sink authenticates grant, source, expected Rust graph epoch and provenance; commits evidence and graph through its existing authority. An externally verifiable journal receipt must bind exact key and batch digest.
8. After receipt, optional metadata accounting updates T0 coordinator (idempotently); checkpoint cursor advances only after that succeeds. If process dies after commit but before checkpoint, restart inspects receipt, reconciles, and avoids duplicate commit.
9. On every batch and pre-commit recheck current source permission; AbortSignal pauses at a safe boundary. Re-import of identical archive returns existing completion checkpoint.

**No claim of full cross-store atomicity is made here:** it must be demonstrated by the real Host/Rust adapter. The test Host sink simulates a durable journal only.

## Graph proposal exactness

The proposed graph uses `personal:<UTF8 owner byte length>:<owner>/` namespace, matching `scoped_graph.rs` prefix; all node/edge/change IDs are deterministic and scoped. Nodes carry `hqkg.contract = hqkg.scoped-record/1`, `hqkg.namespace`, provenance source/revision/digest, and source-linked evidence IDs. Message raw text is **not copied** into graph properties. Each proposal contains all nodes before edges. Host must translate the proposal to Rust `GraphChangeSet` / `Node` / `Edge`, authenticate provenance and enforce exact `RevisionToken` CAS.

The proposal's graph epoch and source revision must come from the real Host/Rust bridge; the current repository cannot prove such a bridge is already active.

## User-facing T=0 workflow

- T=0: qualified ChatGPT sign-in, permission review; offer an official export request once, without blocking the app.
- T+minutes: existing approved F02/F01 bootstrap and partial authorized sidebar discovery are used independently. No assumed full account history or automatic permission from login.
- When ZIP arrives: user picks archive; Host invokes `createHostArchiveIngestion({ledger,host,coordinator,worldId,accountId}).importArchive({archivePath})`.
- Progressive import reports `processed`, `batches` and `cursor` for the **selected archive** only, not whole-account completeness.
- Continuous approved Studio capture should eventually use the same evidence/graph sink, but this package does not currently implement that live capture adapter.

## Operational commands

At `personal-knowledge-lifecycle/runtime`:
```sh
node --test tests/*.test.mjs
node bin/inspect-chatgpt-export.mjs /absolute/path/to/chatgpt-export.zip
node bin/bench-synthetic.mjs 3000
```
The inspector emits only counts and size metrics; no message bodies. The benchmark generates **synthetic** data and should never be interpreted as real-world archive throughput.

## Verified in this sprint

- 20 synthetic Node.js tests passed locally (Node v22.16.0), including ZIP32/ZIP64, path traversal, CRC, decompression limits, JSON chunking and UTF-8, archive import, crash-after-commit reconciliation, duplicate re-import, uncertain receipts, revocation, pause/resume and scoped graph proposals.
- One synthetic 3,000-conversation, 1.03 MB uncompressed benchmark measured roughly 7,800 conversations/s and about 10 MB heap on the sandbox host. This says **nothing** about production-sized conversations, disk, full-text indexing or Rust execution.
- All new code is committed under Labs only; Host, Studio, Rust and TierFlow repos are **unchanged** by this sprint.

## Missing production gates

1. Verify exact current main SHAs and Host/Studio/Rust contracts; prove qualified account identity rather than assuming a signed-in browser implies account ID.
2. Wire Host `authorizeSource`, `inspectEvidenceBatch`, `commitEvidenceAndGraph`, `graphRevision` to existing durable authorities; these are **required port names**, not claims the functions already exist.
3. Prove the typed Rust bridge can actually receive this JSON proposal, validate and atomically persist with evidence; CAS/retry on stale epochs remains an integration task.
4. Bind T0 coordinator to real user-scope grants, retention, account swap, deletion/invalidation, external disclosure policies. Staging `acceptedByHost:true` is metadata only and not a security boundary.
5. Validate on consented real ChatGPT export schema variants, attachments, deleted/archived/project data, very large individual conversations and Windows file handles.
6. Device/end-to-end acceptance: Host Chrome, first knowledge, exported archive, graph query, TierFlow progress, DCB external send receipt, and revocation cascade. **UNRUN**.

## Known limitations

- Only `conversations.json` is normalized; other exported account files/assets are inventoried but not ingested.
- Individual conversations above configured JSON object/batch limits fail closed; a sub-conversation streaming mode is future work.
- Supports bounded ZIP64, stored and raw deflate; intentionally rejects encrypted, multi-disk, unsafe or unsupported compression variants.
- Full-archive SHA256 adds one sequential read before ingest; optimize only after profiling real data without weakening identity or crash safety.
- No silent email-monitoring/export-request automation and no provider APIs.
- The current metadata checkpoint advances by *conversation index*, so recovery rereads the archive stream from the beginning. It avoids duplicate commits but is not yet a random-access offset index.
