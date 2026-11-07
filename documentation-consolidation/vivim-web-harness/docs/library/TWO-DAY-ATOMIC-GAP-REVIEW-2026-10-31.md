# October 29–31 VIVIM atomic capability gap review

**Basis:** User's recent VIVIM discussion history accessible in this conversation, retrieved prior-chat context, and supplied local-agent transcripts; compared against the 222 original PRD atomic rows and prior 28 cross-project candidate IDs. **Important limitation:** This is a broad semantic retrieval, not a guaranteed exhaustive export of every private conversation or every original message. Dates/topics are evidence pointers, not machine-verifiable message URLs.

## ERA 1 — What was actually missing

Added **76 distinct atomic sub-capability records** to the master registry, **X-029–X-104**. These refine user-discussed behaviors missing as separate trackable contracts. They do **not** mean 76 new independent services or implementation commitments. Existing feature IDs and linked roadmap tasks are retained; no 10-PRD rewrite and no runtime status promotion.

| Area | Missing atomic contracts added |
|---|---:|
| bootstrap | 14 |
| development | 10 |
| world | 9 |
| harness | 9 |
| mobile | 8 |
| dcb | 7 |
| linux_direct | 7 |
| operations | 6 |
| companion | 4 |
| platform | 2 |

**High-priority gaps preserved:** first-seconds safe theme compilation, the difference between inferred dislikes and owner hard NOs, multi-tab bootstrap with one Chrome owner, SQLite typed/idempotent receipts, profile-specific startup, Direct/cloud/Android UI truth, read-only provider readiness, root-file agent context, live-test baseline comparison, and Linux Direct-only Grokbot safety/recovery.

## ERA 2 — Provenance and deduplication

- Each new entry contains an immutable ID, precise measurable acceptance, parent PRD capability IDs, related existing roadmap task IDs, scoped user-conversation topic/date, candidate source files pinned to Git blob SHAs when present, conservative symbol candidates, decision state and draft file/function hints.
- Parent PRD features were **not duplicated as independent requirements**; added refinements deliberately map to them. The main registry remains [atomic-registry.json](../capabilities/atomic-registry.json), and [roadmap.json](../roadmap.json) alone owns work status.
- `human_review_needed` is retained for unresolved user authorization changes, multi-profile policy, schema promotion, historical rights semantics, and optional dev approval flow. `execution_dependent` means not verified on a real phone/provider/native Linux environment.
- Source file existence, symbol declarations and old test receipts do not prove end-to-end functionality. New candidate source paths are *potential implementation seams*, not instructions to build another controller or duplicate Host.
- Original concepts already represented (e.g. unified inbox, Build canvas, seven passes, Android pairing, G0/G2 gates, Auto Chat, VAWS, offline World export, public share) were linked as parents instead of re-added.

## ERA 3 — Decision and release constraints

The repository's [activePhase](../roadmap.json) currently allows a narrowly scoped **Linux Direct/Grokbot** implementation lane under an owner release, while broader runtime work remains outside scope. This gap review does not expand that authorization. No Vercel deploy, provider send, Android real test or new runtime module was performed for this update.

**Required evidence before treating the whole last-two-days corpus as exhaustively covered:** a full export/index of the original private conversations, source-date list and deduplicated machine check. Here we accessed searchable recollections and attached transcripts, so some conversations may remain unretrieved. Do not make false coverage or implementation claims.

**Next documentation step:** use `node tools/docs/query.mjs --id X-029 --json` for lookup; if adding a new execution roadmap task later, first check whether the linked parent task already owns it. Do not convert a high product priority into an unapproved implementation lane.
