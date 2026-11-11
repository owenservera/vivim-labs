# T=0 acquisition staging implementation
Status: LABS PROTOTYPE, not product-wired or real-account verified.

## Purpose and ownership
This package contains a pure source-acquisition state machine and explicit, injectable Host persistence port. It is intentionally incapable of browser navigation, network fetching, ChatGPT export requests, provider API access, personal graph persistence, and external AI disclosure.

Production adoption MUST wire the existing Host transactional ledger as the storage port, existing Host onboarding readiness and grants as authority, and existing Studio browser owner as the only browser effect executor. The shared Rust scoped graph is the computation backend; Host alone commits the Personal World. TierFlow projects status and evidence. Labs holds tests and synthetic fixtures, never actual exports.

## T0 semantics
T0 is exact Host/Studio verified ChatGPT sign-in/account binding; an HTTP 200 or a browser profile name is insufficient. Source scopes are independent:
request_export, read_profile, read_index, run_bootstrap, capture_thread, import_archive, disclose_context.
Sign-in must never auto-grant data ingestion or disclosure.

Proposed staged choreography:
- T0: initialize scoped coordinator; record account/World identity.
- Grant review: owner explicitly approves selected operations, retention and destination scope. This prototype validates operation/grant but does not yet enforce detailed retention or disclosure destinations; production MUST enforce them through Host.
- Immediately offer one official export request, via an owner-approved UI flow. prepare('export') records an intent only. Dispatch must be delegated to Studio after UI support verified. reconcile only from verified receipt.
- Profile, index and bootstrap are logically concurrent but Studio browser effects remain serialized by current owner adapter.
- Intake of any content happens only after Host has committed the corresponding source evidence and accepted the batch.
- Progressive local export import uses Host-owned archive parser and actual supported schemas; this prototype does not include ZIP parsing.
- Rust receives strictly typed, bounded change sets via Host. The per-source batch record is metadata, NOT evidence storage or an applied graph transaction.

## Security and correctness properties
- No implicit owner approval; no auto-send; no personal raw text; no cookie or token use; no private endpoint.
- Effect action IDs bind to immutable payload digest. Pending and uncertain statuses prevent blind resend.
- Verified receipts are required to mark effects complete; actual Studio receipt verification is an external Host integration requirement.
- Record batches only after explicit Host acceptance; exact account/source/native ID/content digest dedup.
- Revoking prevents subsequent new intake, but deletion of previously accepted Host evidence, graph and DCB caches must be wired and tested separately.
- Memory fixture is NOT transactional across processes and is not a production store.
- Retry behavior, grant expiration, crash semantics, cross-store atomicity, outbound disclosure, complete archive backfill and device verification are incomplete until integrated.

## Contract checklist before promotion
1. Map Host authoritative ledger transaction API and give coordinator atomic storage.
2. Wire source grants and account-binding receipts; add retention/destination enforcement.
3. Bind existing Studio bootstrap scheduler and chat index reader (existing partial metadata only).
4. Add verified user-guided export request and separately official ZIP selector/parser.
5. Persist evidence then apply scoped Rust change sets under exact revision CAS.
6. Emit receipt-based TierFlow projections and source coverage.
7. Crash/restart, account swap, expiry, revoked export, corrupt ZIP, low disk and DCB exfiltration tests.
8. Mark separately: source implemented, integrated, automated tested, real-account verified and device verified.

## Run tests
From this folder: `node --test tests/*.test.mjs`
