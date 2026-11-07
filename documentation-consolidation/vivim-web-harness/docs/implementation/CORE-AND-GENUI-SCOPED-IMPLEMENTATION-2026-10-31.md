# VIVIM core engines and Bootstrap Generative UI: scoped code integration

Source added 2026-10-31. This note is not a runtime acceptance certificate.

## Included
- Ten independent core engines (Round 1–4) and their 11 unit-test helpers/suites under `test/round{1..4}`.
- GUI01 strict component catalog/proposal compiler/DOM renderer, GUI02 seven-pass post-parse adapter.
- GUI02 semantic-projection layer: knowledge-object IDs separate from UI instance IDs and intent IDs; evidence-grounded links, layout intents, privacy projections, and read-only convergence of tap/language/glyph/gesture meaning. Historical eight mutation forms recorded as a compatibility matrix but **not executable**.
- GUI test files `apps/mobile-relay/test/generative-*.test.mjs`.

## Trust boundary
All new engines are standalone draft APIs. No launch process, provider messages, UI activation, auth grants, schema changes or current phone/Chrome ownership path is wired by this addition. Existing Host SQLite, primary Personal AI/World, Studio owner/CDP lock, send-once journal and Direct transport remain authorities. Provider replies are untrusted until passed through existing bootstrap parsers and Host ingestion. GUI preview is not Host authorization or an active revision.

## Verified offline tests
`node --test test/round*/*.test.mjs apps/mobile-relay/test/generative-*.test.mjs`

`node --check` on every new engine and GUI module.

## Required later integration
1. Read and satisfy exact existing owner-grant and Studio send-once authorization before connecting any bootstrap agent request.
2. Build GUI04 Host-owned revision/approval/CAS and GUI05 renderer mount; Direct projection requires private source/identity validation.
3. Retain `viewProposals` across restarts using existing trusted receipts or append-only authority, without a duplicate source of truth.
4. Test native paired Android F02/F01, Build continuation, Undo, profile preservation, and uncertain-send no-retry before marking GUI01/GUI02 or any core roadmap item as accepted.
5. Reconcile `docs/roadmap.json` once owners approve detailed integration and live receipts.

Do not activate these engine drafts by importing them into an existing background worker unreviewed.
