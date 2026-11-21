# Paired PWA UI programmability — executable acceptance matrix

## Already implemented on this branch (source level)
- Pure VEC three-domain compiler contract with only UI installed.
- Existing phone Build composer returns deterministic UI proposal for exact syntax, independently of the model sender.
- UI preview only: `vivim.ui-proposal/1` -> `compileUiProposal` -> `vivim.ui-preview/1`; `persisted:false` and `authorization:required`.
- Existing PWA Activate -> `sendProposal` -> Host `apply-proposal` command; Host surface head read guard untouched.
- New static modules allowlisted in `apps/vivim-host/direct-phone.mjs`; offline network-first service-worker v5 caches shell **only**, no API/auth.
- Synthetic tests authored in `test/vec-ui-compiler.test.mjs`.

## Must actually execute (NO claim of passing yet)
```sh
node -v  # Node 24 preferred for ARBOR integration, source JS is browser-safe ESM
node --test apps/mobile-relay/test/vec-ui-compiler.test.mjs
node --test apps/mobile-relay/test/build-composer.test.mjs
node --test apps/mobile-relay/test/generative-ui.test.mjs
node --test apps/mobile-relay/test/host-build-consumer.test.mjs
node --test apps/mobile-relay/test/host-static-allowlist-contract.test.mjs
node --test apps/mobile-relay/test/service-worker.test.mjs
node --test apps/mobile-relay/test/build-vec-routing.test.mjs
node --test apps/mobile-relay/test/*.test.mjs
node --check apps/mobile-relay/vec/compiler.mjs
node --check apps/mobile-relay/vec/ui-adapter.mjs
node --check apps/mobile-relay/build-canvas.mjs
```
Record exact exit status, Node version, source HEAD and failing test names. Missing runtime tooling = UNRUN, not PASS.

## Required negative cases
- Different domain (command vs UI) cannot silently compile one another.
- UI text known grammar uses zero model sends; unknown goes to existing explicit AI design pipeline.
- Invalid title, HTML, link or dangerous code-bearing properties refuses before proposal.
- Same-visible-name ambiguity and unknown target require clarification (no arbitrary guessed deletion).
- Source surface missing/invalid, `pc-private` scope, stale `baseRevision`, same revision inconsistent Host snapshot, unpaired/disconnected/revoked actor.
- A model or device cannot choose Host revision/action by placing content in UI JSON.
- Timeout/unknown Host `apply-proposal` result: never automatic resend, no premature success.
- PWA offline shell may display *non-authoritative* cached assets, never cached personal state or executable action.
- Host Direct static allowlist includes new scripts; cloud PWA build has same module closure.
- Screen-reader labels, font scaling, one-thumb touch targets and Android Chrome first paint.
- Undo after Host commit reuses existing history; UI preview never pretends to be committed.

## Real-device progression
1. Node24 and static CI checks green on exact source SHA.
2. Windows Host PC pairs phone; verify key/enrollment, PWA initial UI head and Direct static asset imports, no external provider request for known UI command.
3. Type 'Add a card called Research' on phone; see local Preview and distinct `COMPILED LOCALLY · NO AI` label.
4. Confirm untouched current Host revision until Activate; accept, verify Host commit receipt and next PWA revision.
5. Rename/move/remove and Undo; reject stale conflicting surface; revoke pairing and confirm no editable cached old surface.
6. Verify service worker offline only displays shell, never replays `apply-proposal`; restart PC and phone, prove no duplicate writes.
7. Repeat on PWA direct and chosen deployment route with actual Android Chrome.

## Release blockers
This source branch has not executed CI or physical pairing tests in the GitHub editing environment. This first slice does NOT yet mean VEC owns Wiki/NLCL, dynamic TierFlow binding, secure custom Rust capsules or general freeform natural-language reprogramming.