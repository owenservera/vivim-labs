# W2 — launch six parallel lanes, then G

**Wave objective:** STREAMS, IDENTITY AND UI.

Start six ChatGPT/GitHub conversations in parallel using the exact linked worker prompts, all from accepted W1 gate head.

- [A — Identity and source binding](./A.md): Implement provider/product/account/session/thread/recipient identity model, freshness/epoch and account-swap invalidation; URL does not establish identity.
- [B — Normalized stream processing](./B.md): Implement source-qualified units, dedup, bounded cursors, deltas, partial/stale/denied states, resumable import against fake World receipts.
- [C — Inbox/feed/media read models](./C.md): Create source-backed inbox, timeline, media and search models with provenance/freshness and missing-data states, no unapproved model inferred content.
- [D — Studio UI](./D.md): Add source explorer/consent plan/read-model panels to existing Studio through mock Host route; do not create new Studio app.
- [E — Existing PWA UI](./E.md): Add provider tabs, inbox/feeds/thread viewer and local draft preview to existing PWA; no direct provider effect and no second phone app.
- [F — Observability and job state](./F.md): Build bounded mock source sync/import/pause/revoke/repair status and sanitized metrics through existing Host concept, no new durable ledger.

After all six commit and supply reports, run [G serial integration](./G.md). Do not assume worker completion or tool success; use actual branch heads.
