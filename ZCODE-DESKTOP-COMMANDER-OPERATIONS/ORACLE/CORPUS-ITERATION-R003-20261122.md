# Oracle → AutoWiki → GPUI/PWA | R003 porting receipt

**Date:** 2026-11-22 (Europe/Madrid)  
**Source:** `owenservera/ARBOR` canonical `main` (ChatGPT Cloud Dev directly committed).  
**Intent:** port verified Oracle discovery into EXISTING product read paths, no parallel store or new interaction surface.

## Actual wiring added
- `apps/vivim-host/bootstrap/service.mjs`: seven-pass F02/F01/D03-D07 existing scheduler writes grounded claims into existing SQLite Host ledger; `oracleCorpus` is a bounded, provenance-backed owner read.
- `apps/vivim-host/server.mjs`: owner-private `/api/host/control/wiki-context` now includes `oracleCorpus` from the same runtime, without changing journal semantics or revision or granting a Wiki mutation. `/api/host/bootstrap/corpus` remains owner-token-only. No new REST authority.
- `apps/vivim-host/public/pages/wiki.js`: Oracle section within existing AutoWiki context panel; provider reports labelled unverified.
- `apps/mobile-relay/app.js`: same Oracle section rendered ONLY in existing `HOST_PREVIEW` owner-private `renderWikiContext`, reusing the already-required `/api/host/control/wiki-context` read. No extra fetch, no new state store, no owner-private content in the paired Direct phone view. Invalid/sensitive-shaped reply fails existing private-context validation.
- `native/gpui-desktop/src/lib.rs`: Rust `OracleProgress` derives ONLY aggregate counts and seven pass statuses from existing authenticated `vivim.bootstrap-phone/1` projection already in `host-pwa-view`. Strict shapes, duplicate/refused passes, no copied claims, observations or new API.
- `native/gpui-desktop/src/main.rs`: aggregate progress appears within the existing contextual AutoWiki (only while composing) and within the GPUI Wiki/Overview inspector. The empty First Contact canvas stays blank.
- Existing `Studio` port is platform-aware: Windows launcher 38730 and non-Windows relay 38787, explicit `VIVIM_STUDIO_URL` overrides both.

## Evidence actually executed
- **73/73 Node Host + mobile UI tests PASS, 0 fail**, clean isolated exact Git revision `179f46be3f40625ee8c9296a7d595567f3994b4d`. Scope includes original bootstrap 7-pass mocks, owner permission and expiration, live Host HTTP wiki-context private/pwa response, preview DOM read-path checks, private-source refusal, Direct phone safety and existing Wiki editor tests.
- **49/49 GPUI Rust library tests PASS**, same isolated revision, `cargo test --manifest-path native/gpui-desktop/Cargo.toml --lib`; new Oracle progress source test included.
- **Follow-up post-push recheck: 73/73 Node tests PASS, 0 fail, 0 skipped**, at exact clean archived ARBOR SHA `c67894f26c757c16881aab6ac2789cf293111d76`, including `c454781d7f931ce7394e59b17cb1d2a199bcf2b9` which verifies authenticated generic GPUI/PWA summaries contain only progress metadata, never private Oracle statements.
- GPUI executable with desktop feature has a **separate compile/physical acceptance gate**; Rust *library* pass is not executable acceptance. No real Chrome ChatGPT prompt extraction performed or claimed.
- `cargo check --manifest-path native/gpui-desktop/Cargo.toml --features desktop --bin vivim-gpui-desktop` was launched in the isolated temp checkout and advanced through GPUI dependencies, but **did not complete**. Another agent's independent stable-toolchain Cargo build was active in parallel. To avoid duplicating expensive native work and interfering with the concurrent agent, only our `cmd.exe` process tree (PID 12016 / Cargo 16928) was terminated with `taskkill /T`. **Desktop feature compile = UNVERIFIED**, not PASS or FAIL; the other agent's build was left running.
- Existing `.zcode/workflows/arbor-oracle-corpus-verify.dwf.ts` was **extended in place** at ARBOR `a767921ea4dc7bbb1ce76823b344fda4e67438de`: source-pinned guard remains, nine deterministic Host/PWA suites now covered, no browser/account sends. The ZCode model-based workflow itself was **not executed successfully** due previous OpenRouter rate-limit; only the independent test lane was run.
- Offline Cargo initially lacked pinned `core-foundation 0.10.0`; a normal isolated Cargo resolve retrieved pinned dependencies without editing tracked repository dependency declarations.

## Current machine boundary
- Authorized Desktop Commander machine online. Observed `http://127.0.0.1:38730/api/v1/health` **HTTP 200**: existing Studio Windows listener is running. `/api/v1/conversation-targets` was not confirmed within bounded read timeout; logged-in ChatGPT readiness is **UNVERIFIED**.
- `http://127.0.0.1:43187/api/onboarding/state` unavailable in latest read: running Host is currently not verified alive. Do not infer live Oracle onboarding from static tests.
- User's canonical working tree remained untouched; source was edited only via GitHub `main`, tests built from a temp clean checkout. Do not start a competing Studio, resend an uncertain provider ask, or reset the owner's dirty working directory.

## Next actual acceptance step
1. Reconcile existing native Host supervisor/Host health and Studio port 38730 without creating a second service or source owner.
2. Verify Studio conversation target and ChatGPT sign-in read-only; owner must review seven exact prompts before sending. Studio's send-once journal owns send receipts.
3. After real accepted responses, verify Host's provenance and private corpus readback, existing AutoWiki owner view, native GPUI counts and Direct PWA's non-disclosure.
4. Run physical GPUI/phone acceptance and exact-source Rust executable compile separately; never promote a library test to physical proof.
