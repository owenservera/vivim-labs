# R001 → R004 engineering handoff (not an alternate roadmap)
## R001: Labs landing (this commit)
1. Candidate catalog and stream classes: broad coverage, default NOT WIRED.
2. Machine-readable metadata-only envelope and Host request-proposal schema.
3. Responsive Studio/PWA preview with synthetic Messenger inbox, source tiles, permission review, message draft preview and disabled send.
4. Catalog tests and source-pinned integration map. No external accounts/sessions accessed.

## R002: source-owned inventory and browser feasibility
1. In **Harness** inspect exact current Studio registry/capture/factory, source permission ledger, provider action broker, Host World receipt writer, browser target/account controls and single phone app routes; reuse existing code.
2. For Facebook's 2026 `facebook.com/messages`, verify live supported account + region + provider terms; consented read-only thread list/read fixtures (including E2EE and partial/locked threads); reject account/profile swaps. Never rely on `messenger.com`.
3. Add provider capability descriptors to **existing** adapter registry. Learn data-only read candidates using existing Studio factory and holdout evaluations. Never promote learned selectors to sender.
4. Bind first provider/source read states to existing Host World/graph journal and TierFlow projections. Static Labs catalog remains non-authoritative.

## R003: Facebook Messenger first trusted outbound vertical slice
1. User selects thread/account and composes locally. Host materializes a private draft; projection shows typed proposal, not direct send.
2. Host verifies source access, destination origin, unique account identity, thread/recipient binding, current draft hash, fresh grant and current visible page.
3. Owner approval covers exact target, attachment set, exact rendered message body and third-party disclosure. Distinguish approval and effect execution.
4. **Only existing Host-controlled browser actor** types text; rechecks typed readback and destination before send; never auto-retries uncertain sends. Existing signed-in site adapter must be **hand-verified**, not learned-data send adapter.
5. Capture independent post-action evidence tied to provider-recognized identity/thread/message and attach Host-issued receipt. If provider confirmation cannot be established show `OUTCOME_UNKNOWN`, not success.
6. Verify negative cases: wrong account, wrong recipient, thread route changed, logout/2FA, unsupported E2EE, blocked user, stale UI, rate-limit, draft drift, duplicate click, network loss during send, crash after send, revoke mid-flight.
7. Run real consented Windows Chrome + PWA paired tests; note browser service restrictions and ToS. Send to test account only until independent proof.

## R004: cross-provider growth
Instagram Direct, WhatsApp linked-web, X DMs, Telegram and email enter independently after feasibility and terms review. Preserve each platform's account/device/session controls, supported operations, receipt semantics, E2EE and message limits. Text-only paths precede attachments, edits/deletes, group targeting, reactions and advanced workflows. No provider APIs without a deliberate product-policy revision.

## Acceptance scenarios
- A guest launching Studio sees the full catalog but zero falsely connected accounts.
- Facebook posts can be previewed only after an explicit read/import grant; Messenger messaging is separately scoped.
- UI shows `APPROVAL_REQUIRED` and cannot report `SENT` on a synthetic preview.
- A switched Facebook account invalidates prior thread/draft send eligibility.
- Sending to wrong thread/recipient, revoked grant or unverified session fails closed.
- Refresh/re-import preserves provenance and deduplicates by stable source identity; undo/delete follows Host grant semantics.
- Real provider send is never claimed by fixture-only tests.

## Source handoff paths
- Harness Studio: `studio/src/studio/adapter-factory-service.mjs`, `adapter-studio-bridge.mjs`, `adapter-capture-to-signals.mjs`, `studio/src/capture/conversation-browser-adapter.mjs`.
- Existing Studio UI: `studio/src/studio/ui/`.
- Existing phone: `apps/mobile-relay/`.
- Labs personal intake: `personal-knowledge-lifecycle/runtime/`.
- Labs base design: `personal-knowledge-lifecycle/{SOURCE-ATLAS,PIPELINE-CONTRACTS}.md`.
- HQ shared graph and TierFlow renderer: inspect current owner source before any integration.
