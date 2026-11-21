# VEC personal program lifecycle and explicit source-mutation boundary

## Three distinct scopes
1. **Personal behavior configuration:** user-owned shortcuts, UI layouts, filters, preferences and event/schedule workflows. Default path for ordinary NL requests. Stored in existing Host personal World with revision/receipts, not product source code.
2. **Trusted program composition:** durable graph of installed SDK capabilities. Performs effectful actions only by invoking the existing Host-authorized action owner, with actor/session/grant checks at execution time and safe limits. No separate VEC authority.
3. **Source code mutation (future advanced capability):** deliberately granted development mode only, on isolated Git worktree/branch, bounded path/operations/test runner and independent review/merge gates. Cannot edit verifier/policy/own grants, forge receipts, bypass approval or overwrite main. User must knowingly request this mode; normal interface personalization must not silently edit the VIVIM repository.

## Lifecycle
Intent -> deterministic resolution / clarification -> typed program preview -> permission plan -> owner authorization -> install disabled -> enable -> bounded event execution with receipts -> observe -> edit (new revision) / pause / revoke / undo. A revoked grant stops future execution, denies cached sensitive outputs and does not imply impossible erasure of prior external effects.

## User-facing controls
Every program has 'what I understood', 'when it runs', 'data accessed', 'what it changes', 'what needs approval', 'what happens if offline/revoked', 'resource use', 'last receipt' and 'undo/pause'. Use a natural-language one-program editor; no need for users to see JSON/Rust except advanced inspectors. Provide deterministic rewording and a direct way to correct a mistaken interpretation.

## Effect and environment boundaries
- No code in user utterances, retrieved webpages, imported notes or comments acquires executable privilege.
- No direct filesystem/cloud/browser/provider I/O from sandboxed user-authored computations.
- Native Rust/Wasm capsules can be pure; effect proposals are typed data. Existing Host decides and executes.
- On phone, UI and input may propose programs, but Host retains effect authority; offline proposals can queue for later approved application without false success.
- Preserve action receipts, revision guards, crash recovery and idempotent retries. Bounded schedules use the existing clock/scheduler rather than introducing a second one.
- Draft recipes and grammar packs are not product runtime acceptance. Independent security, PWA and device gates remain separate.

## Initial proof-of-concept
Use 3 invented known intents: show tasks at a schedule, change a TierFlow preference, report a low storage threshold. Define mocked *capability stubs* and synthetic users/sources (not actual personal data). Verify exact deterministic matching and safe ambiguity refusal, generate data-only Program IR, evaluate dry-run and grants, replay in-memory with a fake Host, test revocation and undo; record that no model was called. This is a proposed future test matrix, not execution of tests now.
