# Oracle → AutoWiki existing-surface port — R003

Date: 2026-11-22. Owner directive: port/reuse what already exists, no new app surface, no new database, no fresh provider adapter.

## What is wired in canonical ARBOR main
1. Existing Studio browser/conversation API and exact-once journal remains sole provider-web owner. The already implemented seven passes remain scheduled only after owner review/approval, with Host as writer.
2. Existing Host bootstrap SQLite `apps/vivim-host/bootstrap/service.mjs` exposes owner-private, bounded `oracleCorpus()` claims with provider/generation/thread/provenance/confidence/status. No model transcripts are falsely represented as verified.
3. Existing Host runtime `apps/vivim-host/bootstrap/runtime.mjs` exports that service read, without creating a second corpus store.
4. Existing owner-private AutoWiki endpoint `GET /api/host/control/wiki-context` now composes `oracleCorpus` from that runtime after owner authorization, in the same response as source-journal semantics and HQKG metadata. Failed corpus read is visibly `unavailable`, never synthetic.
5. Existing `/wiki` page `apps/vivim-host/public/pages/wiki.js` renders bounded Oracle claims alongside existing private observations and semantic graph. DOM text uses textContent; there is no independent browser API poller, no extra data writer, no new display route.
6. Public `/api/wiki` is unchanged: these provider-reported claims do NOT enter the committed Wiki journal or revisions, and sensitive D06 relationship material is excluded.
7. Studio boot default is platform-correct: Windows VIVIM.ps1 launcher=38730; other Studio relay default=38787; explicit `VIVIM_STUDIO_URL` always wins. No duplicate browser owner.

## Exact Windows evidence
- Clean temporary clone `%TEMP%\arbor-oracle-zcode-scout` pinned to source `0cfd4c9a81d3cb910d5dc1e02bed216adfd05927` on 2026-11-22, with concurrent ARBOR changes also present.
- `node --test apps/vivim-host/test/oracle-wiki-wiring.test.mjs apps/vivim-host/test/wiki-visual-contract.test.mjs apps/vivim-host/test/product-routes.test.mjs apps/vivim-host/test/oracle-onboarding-prompts.test.mjs apps/vivim-host/test/onboarding.test.mjs apps/vivim-host/test/bootstrap.test.mjs apps/vivim-host/test/bootstrap-grant.test.mjs`
- Result: **58 tests, 58 pass, 0 fail, 0 skipped**.
- Dedicated integration test creates one isolated Host, ingests controlled provider-reported D03 and sensitive D06 data using the existing runtime, reads owner-only AutoWiki HTTP response, proves D03 source/provenance, no D06 disclosure, unauthorized/cross-origin denial and unchanged public Wiki revision.
- Existing owner-check tests, visual DOM tests, seven-pass simulation, and source-index tests all remained green.

## Environment truth and next operational acceptance
- Actual running local Host `127.0.0.1:43187` was live, onboarding phase idle and Studio unavailable; no real seven-pass generation started.
- Studio on Windows launcher default `127.0.0.1:38730` unavailable during read-only probe. `38787` and `7878` unavailable in earlier probe.
- Current production Host/Studio not restarted; existing GPUI/phone link and dirty local source checkout left unchanged. No physical PC↔phone screen acceptance or live signed-in ChatGPT extraction is claimed.
- For deployment use existing Windows VIVIM launcher, Studio owner lock, signed-in profile, existing Host rollout. Then owner must review exact prompts and approve before any send; accept actual provider response and read same claim in AutoWiki. Do not start competing Studio/CDP owner.
- ZCode free upstream Baidu ERNIE model previously returned HTTP 429; no model-driven DWF run completed. Deterministic read-only Node verification remains runnable without model/provider budget.

## Follow-on implementation targets (incremental only)
- Operational bringup of existing Studio using original launcher and verified profile; no browser session duplication.
- Roll out verified source using owner deployment path, no reset/patch of dirty local checkout.
- After owner-authorized ChatGPT extraction, test complete live send-once → Host receipt → AutoWiki same-context projection and existing GPUI/PWA views.
- Reuse existing HQKG/AutoWiki graph projection; provider claims remain provider-reported and cannot silently become owner-confirmed.
