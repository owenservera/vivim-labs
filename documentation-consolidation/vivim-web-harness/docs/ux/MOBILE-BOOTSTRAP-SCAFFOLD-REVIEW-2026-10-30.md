# Review — Mobile Bootstrap Scaffold and UX Interaction Contract

**Reviewed:** 2026-10-30. **Scope:** PR [#35](https://github.com/owenservera/vivim-web-harness/pull/35) additive Build tab (`apps/mobile-relay/app.js`, `index.html`) plus `docs/ux/schema/mobile-interaction.schema.json` and `mobile-interaction.catalog.json`.  
**Disposition:** **CHANGES REQUIRED before production mobile acceptance.** Review only; no runtime code edited.

## Findings (severity-ordered)

1. **[BLOCKER] `apps/mobile-relay/app.js` — `buildPromptToTalk()` overwrites the existing Talk composer.** It calls `buildToTalk()` then unconditionally sets `$('text').value = prompt.slice(...)`. Any already-written unsent Talk draft is lost, contrary to the product's draft-ownership requirement. **Fix:** Preserve the existing draft; offer explicit Replace / Append / Keep options, never silently overwrite. Add regression for an occupied Talk draft and a pending multi-line draft.

2. **[BLOCKER] `apps/mobile-relay/index.html` — `#v-bootstrap { display:flex }` conflicts with the element's `hidden` attribute.** Author CSS `display:flex` can override the UA `[hidden]` display rule, so Build can remain rendered when another tab is selected despite `renderTabs()` setting `hidden`. **Fix:** add `#v-bootstrap[hidden]{display:none}` (and test tab switching / desktop/mobile CSS cascade). Check any other author display declarations on hidden surfaces.

3. **[MAJOR] `app.js` — activation label does not reflect capability activation.** `saveBuild([...B.enabled,item.id])` only changes an on-device array; cards then say `Active` and toast `activated` for Workspaces / Agent team / Auto Chat even though no host-backed capability was enabled. **Fix:** use `Added to my plan` / `Preview` until host capability state and an actual activation receipt exist. Distinguish design proposal, local UI configuration and executable active capability.

4. **[MAJOR] PR #35 is not wired to the 28-binding formal UX catalog.** `buildCard()` makes a single tap add/remove an item directly, while `mx.build.capability.tap`/hold specify **inspect**, then an independent configure/activation review. No hold recognizer, context sheet, gesture collision handling, `interaction.intent` dispatch or validation is present in this PR. **Fix:** implement an adapter from UI controls to registered semantic actions, and make taps preview by default. Keep gesture support optional; standard button parity first.

5. **[MAJOR] `app.js` — provider readiness is only `p.signedIn === true`.** The UI's `buildVerified()` and `build-connection` label use manifest flags without checking a host-issued verified timestamp, account scope, provider capabilities or expiry. **Fix:** consume the read-only verified provider manifest under planned B02; display `reported`, `verified at`, `stale` and `unavailable` distinctly. A signed-in flag alone is insufficient to claim live usable capability.

6. **[MAJOR] `app.js` — localStorage history is a UI-only stub, not the product's time travel.** `BUILD_KEY` saves just `enabled[]`, `revision`, and a last-20 history; the records have no mode, source evidence, commit binding, host synchronization or CAS. Reinstall/clear-storage and multiple phones can lose/diverge state. **Fix:** preserve as a demo scaffold with truthful wording and later bind to a single versioned local World config store, append-only revision events and conflict detection. Do not claim a real rollback of provider/host effects.

7. **[MAJOR] `docs/ux/schema/mobile-interaction.schema.json` — schemas validate message shapes, not runtime authority or cross-document relationships.** `Intent` accepts a `bindingId` and `semanticAction` independently; JSON Schema cannot verify that the binding actually matches, that the target belongs to the user, or that an `approvalRequestRef` was granted for the exact payload. **Fix:** a host-enforced dispatcher must revalidate binding lookup, surface/role/action match, capability revision, owner authorization, target identity and dedupe key. Never dispatch arbitrary client-origin `semanticAction` to CDP/host operations.

8. **[MINOR] `schema/mobile-interaction.schema.json` — `device_report.supports` uses `uniqueItems`, which does not require uniqueness by `feature`.** Contradictory records such as two vibration entries with different statuses would validate. **Fix:** runtime key uniqueness or replace array with a feature-keyed map in a future schema version; add a negative test.

9. **[MINOR] `app.js` — `loadBuild()` trusts `revision` and each historical entry after a shallow array test; `saveBuild()` has no surfaced localStorage failure/revision conflict.** **Fix:** validate bounded integer revisions, known unique IDs, snapshot shape and storage errors. Persist atomic comparison-and-swap on future host-backed store.

10. **[MINOR] `docs/ux` remains formal research, not a runtime integration test.** The catalog's 28 entries are all marked `research`; the prior structural subset validator is not Draft 2020-12 conformance proof. **Fix:** run an independent schema validator, negative authorization fixtures, Android Chrome/TalkBack and iOS Safari/VoiceOver interaction tests before changing maturity statuses.

## Existing good boundaries to preserve

- Build is additive and existing Talk remains the primary working path.
- Build prompt currently moves to a **draft** rather than being automatically sent.
- No direct AI-provider API is introduced in this patch; approvals/send-once remain harness responsibilities.
- Only harness-owned browser effects; no gesture or client schema bypasses provider/host authorization.
- Reusable machine-readable UX schema and catalog should remain separate research assets until real-device verification.

## Minimal targeted remediation before UX prototypes

1. Fix #1 and #2 with regression tests, preserving the current phone chat baseline.
2. Fix #3 and #5's truthful language/state; make `Preview` the default click rather than `Activate`.
3. Add binding-to-semantic-intent adapter and safe inspector bottom sheet (MX02, MX03), using the existing host authority for effects.
4. Run a full JSON Schema validator and hostile-client negative tests; add Android physical-device and accessibility receipts.
5. Promote user-visible behavior only after a recorded live test; leave `docs/roadmap.json` untouched until the owning agent reconciles priorities.

**Evidence limitation:** PR diff and relevant repo schema documents were inspected; no PWA installation, physical-device session, end-to-end host send, CSS browser automation or full regression suite was executed in this review. Some requested fixes are forward-looking rather than defects in the intentionally local-only demo's stated scope.
