# W-AUTOWIKI — compatible contracts /0.1
**Schema authority remains in the repos-hq AutoWiki donor branch** (see SOURCE-MAP.md); this file is a mapping, not a replacement standard or schema registry.

## ERA 1 — Wire compatibility
`ScreenContext` is exactly `vivim.screen-context/0.1` with `contextId`, integer `revision`, ISO `observedAt`, `source` (native/bound-web/manual/unsupported, surfaceRef/viewRef/bindingState/contentCoverage, optional world/view/conversation revisions), `focus` (title, objectRefs, projectCandidates with trusted/candidate/unresolved, topicTerms, activity, draft state/digest), `scope` (host-trusted-scope; ownerRef/worldRef/purpose=autowiki-read/policyRevision), and `coverage` (complete/partial/unavailable/denied; olderHistory/truncated/unavailableSources). Crucial: ScreenContext.scope is **comparison metadata**, NEVER authorization; only the injected trusted Host `readScope()` grants a read, and W checks it both before and after source lookup.

`AutoWikiPage` is `vivim.autowiki-page/0.1`: pageId, contextId, contextRevision, projectionRevision, asOf, status, focus, bounded sections, bounded items, coverage/readerStates, sourceRevisions, omissions, `readOnly:true` and `effectPerformed:false`. Source cards retain upstream item IDs and `source.nativeRef`, revision, digest, optional repo/path/line; they never remint HQ keys, truncate identities into substitute IDs, or claim a new graph source. HQ `kb.mjs` document IDs use `doc:${project}:${path}` and can contain `#` or `/`; W now permits source-native IDs up to donor schema maximum 191 characters while Host owner/policy tokens retain a stricter opaque-ID validator. `projectRef:null` and `unresolved` express ambiguity, not a guessed owner. Drafts are in-memory synthetic `draft_intent` status `proposal` and do not copy actual typed text into the view.

`AutoWikiDelta` is `vivim.autowiki-delta/0.1`, preserving contextId/contextRevision, prior/next projection revisions, kind `changed`/`resnapshot-required`/`revoked`/`denied`/`source-reset` and bounded changed/removed item IDs. A source feed event is never an access grant: revalidation/re-read first.

## ERA 2 — Source ports, exact owner separation
Synthetic port (`createAutoWikiMirror`):
- `readScope(): Promise<{ authorized, revoked, ownerRef, worldRef, policyRevision, purpose, sessionRef, deviceRef, allowedAudiences }>` is **trusted Host supplied**. Screen input cannot mint it; no HTTP or browser payload should be accepted as this port.
- `readSources({ contextId, contextRevision, ownerRef, worldRef, policyRevision, focus:{objectRefs, projectRef, topicTerms}, limit:20 })` is a **read-only Host/HQ adapter**; returns `{status, readerId, contextRevision, records, sourceRevisions}`. W filters each result again by owner/world/audience/project, source nativeRef/revision/digest, source status and authority.
- Record adapter contract is `{id, kind, title, summary, authority, ownerRef, worldRef, audience, projectRef, source:{nativeRef,revision,digest,repoRef?,path?,line?}, status, rankReasons, sourceTimes}`. `id` is unchanged in the page. Personal source authorizations and HQ development provenance stay in their owning stores.
- `open(screen)` returns a page, `close()` invalidates pending reads and drops held projection; `current()` is an in-memory presentation pointer. No editor state is passed or retained.

ARBOR F `SourceRef{id,domain,ownerRef,revision}` is not identical to a Git/HQ nativeRef. The W adapter retains `id` and `source.nativeRef` as authoritative source-owned strings; any numeric `revision` crosswalk must be resolved by owner F, not invented. E `KnowledgeClient` and `ScopedKnowledgeFeed` are display/read-port precursors, not a new broker.

## ERA 3 — Fail-closed obligations
1. Unknown/invalid screen, unsupported/unbound browser, wrong owner/world/policy/purpose, revoked grants and expired sessions must produce denied/no-source or be rejected **before** source read.
2. Policy/session/device changes during async reads discard results; later screen generations return stale and cannot replace newer pages.
3. Bound browser transcript `complete:false` and incomplete historical coverage force page `partial`; never infer full conversation from visible messages.
4. Multiple/non-trusted project candidates remain unresolved without opportunistic source lookup. Different source audience is omitted, not silently promoted.
5. The projection is a reader, not a command parser, wiki writer, UI publisher, effect broker, or storage governor. Caller must prove in native integration that `readScope` and `readSources` are genuine signed-in owner read ports.
