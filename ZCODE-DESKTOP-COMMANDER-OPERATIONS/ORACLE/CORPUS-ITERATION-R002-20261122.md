# Oracle corpus — R002 implementation and ZCode/Desktop Commander verification

Date: 2026-11-22, Europe/Madrid. This is a bounded code/test receipt, NOT evidence of live ChatGPT extraction.

## Scope / current source
- Canonical code owner: ChatGPT Cloud Dev via GitHub `owenservera/ARBOR` `main`. Latest exact test snapshot: `7f42f8b38e9650931fc1cedc7532ddbfbe638eb7` (concurrent lanes also edited `main`).
- New read model: `apps/vivim-host/bootstrap/service.mjs` `oracleCorpus({limit})` reads EXISTING Host SQLite claims with provenance; no second graph/database, no model calls. D06 sensitive relationship/value content is excluded from default claim results.
- Runtime bridge: `apps/vivim-host/bootstrap/runtime.mjs` exposes existing Host-owned service method.
- Owner-only read endpoint: `GET /api/host/bootstrap/corpus?limit=1..100`, same-origin guard, owner bearer required, malformed/unauthorized/cross-origin refusals, no-cache.
- Existing onboarding UI now projects real persisted Host claim/observation/contradiction counts; absent ledger progress stays unknown (not a fabricated zero). No second onboarding product surface.
- Fixed Host default Studio loopback mismatch: `apps/vivim-host/server.mjs` now uses `DEFAULT_STUDIO_ORIGIN=http://127.0.0.1:38787` from `apps/vivim-host/onboarding/studio-port.mjs`, matching the Studio relay default. Prior Host default was 7878.

## Real local environment findings
- Desktop Commander device `DESKTOP-9KB03LU` online.
- Running Host `http://127.0.0.1:43187`: onboarding page/status HTTP 200. Sanitized onboarding state: `studio.reachable=false`, no configured primary provider, phase `idle`, no discovery passes. Studio HTTP at 7878 and 38787 unavailable. These were read-only checks; no ChatGPT prompts dispatched.
- User's working `C:\0-BlackBoxProject-0\ARBOR` checkout had unrelated dirty files and was NOT modified/reset.
- Test-only Git clone `%TEMP%\arbor-oracle-zcode-scout` fetched current `origin/main` and checked out a clean detached commit for post-push verification; no application source editing occurred locally.

## Executed test receipt
- Command (isolated clean checkout):
```text
node --test apps/vivim-host/test/oracle-onboarding-prompts.test.mjs apps/vivim-host/test/onboarding.test.mjs apps/vivim-host/test/bootstrap.test.mjs apps/vivim-host/test/bootstrap-grant.test.mjs
```
- At exact source SHA `7f42f8b38e9650931fc1cedc7532ddbfbe638eb7`: **44 PASS, 0 FAIL, 0 SKIP**, Windows device.
- Tests include seven exactly-once mocked discovery passes, consent expiry/revocation, mismatched generation refusal, correct bounded Host corpus source provenance, D06 sensitive exclusion, and owner token/origin/query refusal for the new HTTP read.
- No live provider sends, no real personal corpus records, no physical GPUI/phone acceptance proof.

## ZCode model pilot truth
- Launched bounded `zcode --mode plan --workflow-mode onDemand` from isolated checkout to invoke saved `.zcode/workflows/arbor-oracle-corpus-verify.dwf.ts`. ZCode model `baidu/ernie-5.5-flash-preview:free` on OpenRouter returned **HTTP 429 upstream rate-limit**. Spawned PID was explicitly terminated. The ZCode workflow did **not** complete or produce a verified DWF run receipt; do not claim otherwise.
- Deterministic `node --test` lane remained independent and green. Do not silently spend money, change provider routing or restart the 429 model in an automatic loop.

## Remaining acceptance
1. Deploy an isolated, verified build/Host update under existing owner deployment process; do not overwrite dirty local checkout or change canonical phone link until accepted.
2. Bring up the actual Studio conversation API at configured 38787 with its approved browser session and verify signed-in ChatGPT readiness. Current Studio is unavailable.
3. With owner review/approval of exact seven prompts, observe Studio's one-send journal and real provider replies, validate and ingest into Host SQLite with durable receipts. Do not claim full ChatGPT-history access.
4. Read the real accepted claims back from `/api/host/bootstrap/corpus` with owner token; inspect provenance/confidence/staleness and privacy.
5. Connect accepted claims to existing Oracle/AutoWiki/HQKG read port and revisioned PC GPUI/phone PWA surfaces, run device evidence, and build ZCode improvement loops only after bounded model admission or model-free testing.
